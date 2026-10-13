//! Agent automation harness for headless gameplay testing, inspection, and verification.
#![cfg(feature = "agent_test")]

use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use avian3d::prelude::{Collider, LinearVelocity, Position};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

static TELEMETRY_GUARD: Mutex<Option<tracing_appender::non_blocking::WorkerGuard>> =
    Mutex::new(None);

/// Returns true if headless execution was requested via `--agent-headless` CLI flag
/// or the `AGENT_HEADLESS` environment variable.
pub fn is_headless() -> bool {
    std::env::args().any(|arg| arg == "--agent-headless")
        || std::env::var("AGENT_HEADLESS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
}

/// Configures the structured JSON log layer writing to `target/agent_telemetry.jsonl`.
pub fn setup_telemetry_layer() -> Option<bevy::log::BoxedLayer> {
    let _ = std::fs::create_dir_all("target");
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("target/agent_telemetry.jsonl")
        .expect("Failed to open target/agent_telemetry.jsonl");

    let (non_blocking, guard) = tracing_appender::non_blocking(file);
    if let Ok(mut g) = TELEMETRY_GUARD.lock() {
        *g = Some(guard);
    }

    use bevy::log::tracing_subscriber::Layer;
    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(non_blocking)
        .boxed();

    Some(json_layer)
}

/// Sets up a panic hook that emits structured JSON telemetry to `target/agent_telemetry.jsonl`
/// before invoking the previous panic hook.
pub fn init_panic_hook() {
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        let payload = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<dyn Any>".to_string()
        };

        let json_entry = serde_json::json!({
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64(),
            "level": "ERROR",
            "target": "panic",
            "fields": {
                "message": format!("PANIC: {}", payload),
                "location": location,
            }
        });

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("target/agent_telemetry.jsonl")
        {
            let _ = writeln!(file, "{}", json_entry);
            let _ = file.flush();
        }

        prev_hook(panic_info);
    }));
}

/// Inter-thread command payload transferred from background socket server into Bevy ECS.
pub struct CommandRequest {
    pub raw: String,
    pub responder: Sender<String>,
}

/// Resource holding the receiver channel for agent commands.
#[derive(Resource)]
pub struct AgentHarnessChannel {
    pub receiver: Mutex<Receiver<CommandRequest>>,
}

/// Resource for controlling frame-by-frame simulation step advances.
#[derive(Resource, Default)]
pub struct AgentTickControl {
    pub enabled: bool,
    pub remaining_ticks: u32,
}

/// Bevy plugin initializing background socket server and registering command handling systems.
pub struct AgentHarnessPlugin;

impl Plugin for AgentHarnessPlugin {
    fn build(&self, app: &mut App) {
        let (cmd_tx, cmd_rx) = channel::<CommandRequest>();

        // Spawn background TCP listener thread on 127.0.0.1:6550
        std::thread::spawn(move || {
            let listener = match TcpListener::bind("127.0.0.1:6550") {
                Ok(l) => {
                    info!("Agent harness listening on 127.0.0.1:6550");
                    l
                }
                Err(e) => {
                    error!("Agent harness failed to bind to 127.0.0.1:6550: {}", e);
                    return;
                }
            };

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let cmd_tx_clone = cmd_tx.clone();
                        std::thread::spawn(move || {
                            handle_client_connection(stream, cmd_tx_clone);
                        });
                    }
                    Err(e) => {
                        warn!("Agent harness incoming connection error: {}", e);
                    }
                }
            }
        });

        // Initialize Bevy Remote Protocol (BRP) HTTP server on 127.0.0.1:15702
        // enables bevy_brp_mcp, external LLM tools, and brp_cli.ps1 for deep ECS introspection & mutation
        if !app.is_plugin_added::<bevy::remote::RemotePlugin>() {
            app.add_plugins(bevy::remote::RemotePlugin::default());
        }
        if !app.is_plugin_added::<bevy::remote::http::RemoteHttpPlugin>() {
            app.add_plugins(bevy::remote::http::RemoteHttpPlugin::default());
        }

        app.insert_resource(AgentHarnessChannel {
            receiver: Mutex::new(cmd_rx),
        })
        .init_resource::<AgentTickControl>()
        .add_systems(Update, (handle_agent_commands, step_tick_control))
        .add_systems(OnEnter(crate::screens::Screen::Title), auto_enter_gameplay_if_headless);
    }
}

fn auto_enter_gameplay_if_headless(
    mut next_screen: ResMut<NextState<crate::screens::Screen>>,
) {
    if is_headless() {
        info!("Headless mode active: auto-transitioning from Title to Gameplay screen");
        next_screen.set(crate::screens::Screen::Gameplay);
    }
}

fn handle_client_connection(mut stream: TcpStream, cmd_tx: Sender<CommandRequest>) {
    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to clone client TcpStream: {}", e);
            return;
        }
    };
    let mut reader = BufReader::new(reader_stream);

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // Connection closed by client
            Ok(_) => {
                let trimmed = line.trim().trim_start_matches('\u{feff}');
                if trimmed.is_empty() {
                    continue;
                }
                let (resp_tx, resp_rx) = channel();
                if cmd_tx
                    .send(CommandRequest {
                        raw: trimmed.to_string(),
                        responder: resp_tx,
                    })
                    .is_err()
                {
                    // Bevy app terminated
                    break;
                }

                // Wait for Bevy ECS response
                match resp_rx.recv_timeout(Duration::from_secs(10)) {
                    Ok(reply) => {
                        if writeln!(stream, "{}", reply).is_err() || stream.flush().is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = writeln!(stream, "ERR_TIMEOUT {}", e);
                        let _ = stream.flush();
                    }
                }
            }
            Err(_) => break,
        }
    }
}

#[derive(bevy::ecs::system::SystemParam)]
struct HarnessQueries<'w, 's> {
    player: Query<'w, 's, (Entity, &'static Transform, Option<&'static crate::game::Inventory>, Option<&'static mut crate::game::Hotbar>), With<crate::player::Player>>,
    camera: Query<'w, 's, (Entity, &'static Transform), With<crate::camera::SceneCamera>>,
    all_camera: Query<'w, 's, (Entity, &'static Transform), With<Camera3d>>,
    buildings: Query<'w, 's, (Entity, &'static crate::game::Building, Option<&'static Mesh3d>, Option<&'static Transform>, Option<&'static Children>, Option<&'static Collider>)>,
    compressors: Query<'w, 's, (Entity, &'static crate::game::CargoCompressor, &'static crate::game::Building)>,
    autoclaves: Query<'w, 's, (Entity, &'static mut crate::game::ChromaticAutoclave, &'static crate::game::Building)>,
    bell_jars: Query<'w, 's, (Entity, &'static crate::game::ResonanceBellJar, &'static crate::game::Building)>,
    monoliths: Query<'w, 's, (Entity, &'static crate::game::AcousticMonolith, &'static crate::game::Building)>,
    tunings: Query<'w, 's, (Entity, &'static mut crate::game::HarmonicTuning, &'static crate::game::Building)>,
    transported_items: Query<'w, 's, (Entity, &'static crate::game::TransportedItem, Option<&'static Mesh3d>, Option<&'static Transform>)>,
    visual_tiles: Query<'w, 's, (Entity, &'static crate::game::VisualTileMarker, Option<&'static Mesh3d>, Option<&'static Transform>)>,
    all_meshes: Query<'w, 's, &'static Mesh3d>,
}

/// Drains incoming socket commands each frame and applies them to Bevy ECS state.
fn handle_agent_commands(
    mut commands: Commands,
    harness: Res<AgentHarnessChannel>,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut tick_control: ResMut<AgentTickControl>,
    mut app_exit: MessageWriter<AppExit>,
    mut camera_config: Option<ResMut<crate::camera::DioramaCameraConfig>>,
    tester_progress: Option<Res<crate::game::hud::TesterTasksProgress>>,
    screen_state: Option<Res<State<crate::screens::Screen>>>,
    mut next_screen: Option<ResMut<NextState<crate::screens::Screen>>>,
    island_grid: Option<Res<crate::game::IslandGrid>>,
    resonance_grove: Option<ResMut<crate::game::ResonanceGrove>>,
    mut q: HarnessQueries,
) {
    let Ok(rx) = harness.receiver.lock() else {
        return;
    };

    while let Ok(req) = rx.try_recv() {
        let raw = req.raw.trim();
        let parts: Vec<&str> = raw.split_whitespace().collect();
        if parts.is_empty() {
            let _ = req.responder.send("ERR_EMPTY_COMMAND".to_string());
            continue;
        }

        let cmd = parts[0].to_ascii_lowercase();
        let reply = match cmd.as_str() {
            "screenshot" => {
                if parts.len() < 2 {
                    "ERR_USAGE screenshot <relative_path>".to_string()
                } else {
                    let path = parts[1..].join(" ");
                    if let Some(parent) = std::path::Path::new(&path).parent() {
                        if !parent.as_os_str().is_empty() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                    }
                    commands
                        .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                        .observe(bevy::render::view::screenshot::save_to_disk(path.clone()));
                    format!("ACK_SCREENSHOT {}", path)
                }
            }
            "press" => {
                if parts.len() < 2 {
                    "ERR_USAGE press <Key>".to_string()
                } else {
                    let key_str = parts[1];
                    if let Some(code) = parse_keycode(key_str) {
                        key_input.press(code);
                        "ACK_KEY_PRESSED".to_string()
                    } else {
                        format!("ERR_UNKNOWN_KEY {}", key_str)
                    }
                }
            }
            "release" => {
                if parts.len() < 2 {
                    "ERR_USAGE release <Key>".to_string()
                } else {
                    let key_str = parts[1];
                    if let Some(code) = parse_keycode(key_str) {
                        key_input.release(code);
                        "ACK_KEY_RELEASED".to_string()
                    } else {
                        format!("ERR_UNKNOWN_KEY {}", key_str)
                    }
                }
            }
            "query" => {
                if parts.len() >= 2 && parts[1].eq_ignore_ascii_case("state") {
                    let player_entry = q.player.iter().next();
                    let player_data = player_entry.as_ref().map(|(entity, t, _, _)| {
                        let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::YXZ);
                        serde_json::json!({
                            "entity": entity.to_bits(),
                            "translation": [t.translation.x, t.translation.y, t.translation.z],
                            "rotation": [t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w],
                            "angles": {
                                "yaw": yaw,
                                "pitch": pitch,
                                "roll": roll,
                            },
                            "forward": [t.forward().x, t.forward().y, t.forward().z],
                        })
                    });

                    let inventory_data = player_entry.as_ref().and_then(|(_, _, inv, hotbar)| {
                        inv.map(|i| {
                            let items: Vec<_> = i
                                .slots
                                .iter()
                                .flatten()
                                .map(|s| {
                                    serde_json::json!({
                                        "item": format!("{:?}", s.item),
                                        "name": s.item.display_name(),
                                        "count": s.count,
                                    })
                                })
                                .collect();
                            serde_json::json!({
                                "capacity": i.capacity,
                                "items": items,
                                "selected_hotbar_slot": hotbar.map(|h| h.selected_slot).unwrap_or(0),
                            })
                        })
                    });

                    let cam_transform = q.camera
                        .iter()
                        .next()
                        .or_else(|| q.all_camera.iter().next());

                    let camera_data = cam_transform.map(|(entity, t)| {
                        let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::YXZ);
                        serde_json::json!({
                            "entity": entity.to_bits(),
                            "translation": [t.translation.x, t.translation.y, t.translation.z],
                            "rotation": [t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w],
                            "angles": {
                                "yaw": yaw,
                                "pitch": pitch,
                                "roll": roll,
                            },
                            "forward": [t.forward().x, t.forward().y, t.forward().z],
                        })
                    });

                    let current_screen = screen_state
                        .as_ref()
                        .map(|s| format!("{:?}", s.get()))
                        .unwrap_or_else(|| "Unknown".to_string());

                    let terrain_data = island_grid.as_ref().map(|g| {
                        let mut covered = 0;
                        let mut revealed = 0;
                        let mut flagged = 0;
                        for tile in g.tiles.values() {
                            match tile.prospect_state {
                                crate::game::TileProspectState::Covered => covered += 1,
                                crate::game::TileProspectState::Revealed
                                | crate::game::TileProspectState::Extracted => revealed += 1,
                                crate::game::TileProspectState::Flagged => flagged += 1,
                            }
                        }
                        serde_json::json!({
                            "total_tiles": g.tiles.len(),
                            "visual_entities": q.visual_tiles.iter().count(),
                            "covered": covered,
                            "revealed": revealed,
                            "flagged": flagged,
                        })
                    });

                    let placed_buildings: Vec<_> = q.buildings
                        .iter()
                        .map(|(ent, b, mesh_opt, tr_opt, children_opt, col_opt)| {
                            let child_count = children_opt.map(|c| c.len()).unwrap_or(0);
                            let child_mesh_count = children_opt.map(|c| {
                                c.iter().filter(|child| q.all_meshes.get(*child).is_ok()).count()
                            }).unwrap_or(0);
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "layer": format!("{:?}", b.coord.layer),
                                "type": format!("{:?}", b.building_type),
                                "has_mesh": mesh_opt.is_some(),
                                "has_collider": col_opt.is_some(),
                                "translation": tr_opt.map(|tr| [tr.translation.x, tr.translation.y, tr.translation.z]),
                                "children_count": child_count,
                                "children_meshes_count": child_mesh_count,
                            })
                        })
                        .collect();

                    let compressors_data: Vec<_> = q.compressors
                        .iter()
                        .map(|(ent, comp, b)| {
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "layer": format!("{:?}", b.coord.layer),
                                "facing": format!("{:?}", comp.facing),
                                "is_jammed": comp.grid.is_jammed,
                                "total_squishes": comp.grid.total_squishes,
                                "pending_ingots": comp.grid.pending_ingots,
                            })
                        })
                        .collect();

                    let autoclaves_data: Vec<_> = q.autoclaves
                        .iter()
                        .map(|(ent, auto, b)| {
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "layer": format!("{:?}", b.coord.layer),
                                "facing": format!("{:?}", auto.facing),
                                "auto_piston_enabled": auto.auto_piston_enabled,
                                "stored_power_kw": auto.stored_power_kw,
                                "total_catalysis_reactions": auto.total_catalysis_reactions,
                                "pending_outputs_count": auto.pending_outputs.len(),
                                "input_buffer_count": auto.input_buffer.len(),
                            })
                        })
                        .collect();

                    let bell_jars_data: Vec<_> = q.bell_jars
                        .iter()
                        .map(|(ent, jar, b)| {
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "layer": format!("{:?}", b.coord.layer),
                                "stored_charges": jar.stored_charges,
                                "max_charges": jar.max_charges,
                                "is_resonating": jar.is_resonating,
                            })
                        })
                        .collect();

                    let resonance_grove_data = resonance_grove.as_ref().map(|g| {
                        serde_json::json!({
                            "origin": [g.origin.x, g.origin.y],
                            "width": g.width,
                            "height": g.height,
                            "completed_blocks": g.completed_blocks,
                            "is_full_symphony": g.is_full_symphony,
                            "total_power_kw": g.total_power_kw,
                            "conflict_count": g.conflict_count,
                        })
                    });

                    let monoliths_data: Vec<_> = q.monoliths
                        .iter()
                        .map(|(ent, mono, b)| {
                            let tuning = q.tunings.iter().find(|(te, _, _)| *te == ent).map(|(_, t, _)| t);
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "baseline_power_kw": mono.baseline_power_kw,
                                "is_active": mono.is_active,
                                "frequency": tuning.as_ref().and_then(|t| t.frequency),
                                "is_locked": tuning.as_ref().map(|t| t.is_locked).unwrap_or(false),
                                "is_conflicted": tuning.as_ref().map(|t| t.is_conflicted).unwrap_or(false),
                                "in_harmony": tuning.as_ref().map(|t| t.in_harmony).unwrap_or(false),
                            })
                        })
                        .collect();

                    let tunings_data: Vec<_> = q.tunings
                        .iter()
                        .map(|(ent, tuning, b)| {
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [b.coord.x, b.coord.y],
                                "frequency": tuning.frequency,
                                "is_locked": tuning.is_locked,
                                "is_conflicted": tuning.is_conflicted,
                                "in_harmony": tuning.in_harmony,
                                "speed_multiplier": tuning.speed_multiplier(),
                            })
                        })
                        .collect();

                    let state_json = serde_json::json!({
                        "screen": current_screen,
                        "simulation": {
                            "paused": virtual_time.is_paused(),
                            "elapsed_seconds": virtual_time.elapsed_secs(),
                        },
                        "player": player_data,
                        "camera": camera_data,
                        "inventory": inventory_data,
                        "terrain": terrain_data,
                        "buildings": placed_buildings,
                        "compressors": compressors_data,
                        "autoclaves": autoclaves_data,
                        "bell_jars": bell_jars_data,
                        "resonance_grove": resonance_grove_data,
                        "monoliths": monoliths_data,
                        "tunings": tunings_data,
                        "transported_items_count": q.transported_items.iter().count(),
                        "tester_tasks": tester_progress.as_ref().map(|p| {
                            serde_json::json!({
                                "completed": p.completed_count(),
                                "moved": p.moved,
                                "rotated_camera": p.rotated_camera,
                                "prospected": p.prospected,
                                "flagged": p.flagged,
                                "mined": p.mined,
                                "crafted": p.crafted,
                                "selected_hotbar": p.selected_hotbar,
                                "placed_building": p.placed_building,
                                "dismantled_building": p.dismantled_building,
                            })
                        }),
                    });

                    serde_json::to_string(&state_json).unwrap_or_else(|_| "{}".to_string())
                } else if parts.len() >= 4 && parts[1].eq_ignore_ascii_case("visual") {
                    if let (Ok(x), Ok(y)) = (parts[2].parse::<i32>(), parts[3].parse::<i32>()) {
                        let layer = if parts.len() >= 5 {
                            match parts[4].to_ascii_lowercase().as_str() {
                                "viaduct" | "z1" | "1" => crate::game::Layer::Viaduct,
                                "skyway" | "z2" | "2" => crate::game::Layer::Skyway,
                                "subterranean" | "-1" => crate::game::Layer::Subterranean,
                                _ => crate::game::Layer::Ground,
                            }
                        } else {
                            crate::game::Layer::Ground
                        };

                        let target_coord = crate::game::GridCoord::new(x, y, layer);
                        let ground_coord = crate::game::GridCoord::ground(x, y);

                        // Tile visual data
                        let tile_info = island_grid.as_ref().and_then(|g| g.tiles.get(&ground_coord)).map(|t| {
                            let visual_tile = q.visual_tiles.iter().find(|(_, marker, _, _)| marker.coord == ground_coord);
                            serde_json::json!({
                                "coord": [x, y],
                                "prospect_state": format!("{:?}", t.prospect_state),
                                "has_hazard": t.has_overcharge_hazard,
                                "adjacent_hazards": t.adjacent_hazards,
                                "has_mesh": visual_tile.as_ref().map(|(_, _, m, _)| m.is_some()).unwrap_or(false),
                                "translation": visual_tile.as_ref().and_then(|(_, _, _, tr)| tr.map(|tr| [tr.translation.x, tr.translation.y, tr.translation.z])),
                            })
                        });

                        // Building visual data
                        let building_info = q.buildings.iter().find(|(_, b, _, _, _, _)| b.coord == target_coord).map(|(ent, b, mesh_opt, tr_opt, children_opt, col_opt)| {
                            let child_count = children_opt.map(|c| c.len()).unwrap_or(0);
                            let child_mesh_count = children_opt.map(|c| {
                                c.iter().filter(|child| q.all_meshes.get(*child).is_ok()).count()
                            }).unwrap_or(0);

                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "coord": [x, y],
                                "layer": format!("{:?}", layer),
                                "building_type": format!("{:?}", b.building_type),
                                "has_mesh": mesh_opt.is_some(),
                                "has_collider": col_opt.is_some(),
                                "translation": tr_opt.map(|tr| [tr.translation.x, tr.translation.y, tr.translation.z]),
                                "rotation": tr_opt.map(|tr| [tr.rotation.x, tr.rotation.y, tr.rotation.z, tr.rotation.w]),
                                "children_count": child_count,
                                "children_meshes_count": child_mesh_count,
                            })
                        });

                        // Transported items at this coordinate
                        let items_info: Vec<_> = q.transported_items.iter().filter(|(_, item, _, _)| item.current_coord == target_coord).map(|(ent, item, mesh_opt, tr_opt)| {
                            serde_json::json!({
                                "entity": ent.to_bits(),
                                "item_type": format!("{:?}", item.item_type),
                                "progress": item.progress,
                                "has_mesh": mesh_opt.is_some(),
                                "translation": tr_opt.map(|tr| [tr.translation.x, tr.translation.y, tr.translation.z]),
                            })
                        }).collect();

                        let visual_json = serde_json::json!({
                            "coord": [x, y],
                            "layer": format!("{:?}", layer),
                            "tile": tile_info,
                            "building": building_info,
                            "items": items_info,
                        });

                        serde_json::to_string(&visual_json).unwrap_or_else(|_| "{}".to_string())
                    } else {
                        "ERR_INVALID_COORDS".to_string()
                    }
                } else {
                    format!("ERR_UNKNOWN_QUERY {}", raw)
                }
            }
            "prospect" => {
                if parts.len() < 3 {
                    "ERR_USAGE prospect <x> <y> [unfold|flag]".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let mode = if parts.len() >= 4 && parts[3].eq_ignore_ascii_case("flag") {
                        crate::game::ProspectMode::Flag
                    } else {
                        crate::game::ProspectMode::Unfold
                    };

                    if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                        let coord = crate::game::GridCoord::ground(x, y);
                        commands.trigger(crate::game::ProspectEvent {
                            player_entity,
                            coord,
                            mode,
                        });
                        format!("ACK_PROSPECT {} {} {:?}", x, y, mode)
                    } else {
                        "ERR_NO_PLAYER".to_string()
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "mine" => {
                if parts.len() < 3 {
                    "ERR_USAGE mine <x> <y>".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                        let coord = crate::game::GridCoord::ground(x, y);
                        commands.trigger(crate::game::MineTileEvent {
                            player_entity,
                            coord,
                        });
                        format!("ACK_MINE {} {}", x, y)
                    } else {
                        "ERR_NO_PLAYER".to_string()
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "craft" => {
                if parts.len() < 2 {
                    "ERR_USAGE craft <recipe_id>".to_string()
                } else {
                    let recipe_id = parts[1].to_string();
                    if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                        commands.trigger(crate::game::StartCraftEvent {
                            player_entity,
                            recipe_id: recipe_id.clone(),
                        });
                        format!("ACK_CRAFT {}", recipe_id)
                    } else {
                        "ERR_NO_PLAYER".to_string()
                    }
                }
            }
            "build" => {
                if parts.len() < 4 {
                    "ERR_USAGE build <x> <y> <building_type> [layer]".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let btype = match parts[3].to_ascii_lowercase().as_str() {
                        "belt_n" | "belt" => Some(crate::game::BuildingType::Belt(crate::game::Direction::North)),
                        "belt_e" => Some(crate::game::BuildingType::Belt(crate::game::Direction::East)),
                        "belt_s" => Some(crate::game::BuildingType::Belt(crate::game::Direction::South)),
                        "belt_w" => Some(crate::game::BuildingType::Belt(crate::game::Direction::West)),
                        "viaduct_n" | "viaduct" => Some(crate::game::BuildingType::ViaductBelt(crate::game::Direction::North)),
                        "viaduct_e" => Some(crate::game::BuildingType::ViaductBelt(crate::game::Direction::East)),
                        "viaduct_s" => Some(crate::game::BuildingType::ViaductBelt(crate::game::Direction::South)),
                        "viaduct_w" => Some(crate::game::BuildingType::ViaductBelt(crate::game::Direction::West)),
                        "incline_up_n" | "incline_n" | "incline" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::North, upward: true }),
                        "incline_up_e" | "incline_e" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::East, upward: true }),
                        "incline_up_s" | "incline_s" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::South, upward: true }),
                        "incline_up_w" | "incline_w" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::West, upward: true }),
                        "incline_down_n" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::North, upward: false }),
                        "incline_down_e" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::East, upward: false }),
                        "incline_down_s" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::South, upward: false }),
                        "incline_down_w" => Some(crate::game::BuildingType::InclineRamp { direction: crate::game::Direction::West, upward: false }),
                        "turntable_n" | "turntable" => Some(crate::game::BuildingType::SwivelTurntable { rotation: crate::game::Direction::North }),
                        "turntable_e" => Some(crate::game::BuildingType::SwivelTurntable { rotation: crate::game::Direction::East }),
                        "turntable_s" => Some(crate::game::BuildingType::SwivelTurntable { rotation: crate::game::Direction::South }),
                        "turntable_w" => Some(crate::game::BuildingType::SwivelTurntable { rotation: crate::game::Direction::West }),
                        "sorter_n" | "sorter" => Some(crate::game::BuildingType::SortingGate {
                            filter_item: crate::game::ItemType::PolyominoSquare,
                            straight_dir: crate::game::Direction::North,
                            divert_dir: crate::game::Direction::East,
                        }),
                        "compressor_n" | "compressor" => Some(crate::game::BuildingType::CargoCompressor(crate::game::Direction::North)),
                        "compressor_e" => Some(crate::game::BuildingType::CargoCompressor(crate::game::Direction::East)),
                        "compressor_s" => Some(crate::game::BuildingType::CargoCompressor(crate::game::Direction::South)),
                        "compressor_w" => Some(crate::game::BuildingType::CargoCompressor(crate::game::Direction::West)),
                        "beacon" | "survey_beacon" => Some(crate::game::BuildingType::SurveyBeacon),
                        "diviner" | "logic_diviner" => Some(crate::game::BuildingType::LogicDiviner),
                        "wellhead" | "wellhead_n" | "wellhead_e" | "wellhead_s" | "wellhead_w" => Some(crate::game::BuildingType::Wellhead),
                        "chest" => Some(crate::game::BuildingType::StorageChest),
                        "hearth" => Some(crate::game::BuildingType::SmeltingHearth),
                        "autoclave_n" | "autoclave" => Some(crate::game::BuildingType::ChromaticAutoclave(crate::game::Direction::North)),
                        "autoclave_e" => Some(crate::game::BuildingType::ChromaticAutoclave(crate::game::Direction::East)),
                        "autoclave_s" => Some(crate::game::BuildingType::ChromaticAutoclave(crate::game::Direction::South)),
                        "autoclave_w" => Some(crate::game::BuildingType::ChromaticAutoclave(crate::game::Direction::West)),
                        "bell_jar" | "resonance_bell_jar" => Some(crate::game::BuildingType::ResonanceBellJar),
                        "monolith" | "acoustic_monolith" => Some(crate::game::BuildingType::AcousticMonolith),
                        "tuning_fork_pylon" | "pylon" => Some(crate::game::BuildingType::TuningForkPylon),
                        "harmonic_bridge" | "bridge" => Some(crate::game::BuildingType::HarmonicBridge(crate::game::Direction::North)),
                        "conductor" | "resonance_conductor" => Some(crate::game::BuildingType::ResonanceConductor),
                        _ => None,
                    };

                    if let Some(building_type) = btype {
                        let layer = if parts.len() >= 5 {
                            match parts[4].to_ascii_lowercase().as_str() {
                                "viaduct" | "z1" | "1" => crate::game::Layer::Viaduct,
                                "skyway" | "z2" | "2" => crate::game::Layer::Skyway,
                                "subterranean" | "-1" => crate::game::Layer::Subterranean,
                                _ => crate::game::Layer::Ground,
                            }
                        } else {
                            match building_type {
                                crate::game::BuildingType::ViaductBelt(_) => crate::game::Layer::Viaduct,
                                _ => crate::game::Layer::Ground,
                            }
                        };

                        if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                            let coord = crate::game::GridCoord::new(x, y, layer);
                            commands.trigger(crate::game::PlaceBuildingEvent {
                                player_entity,
                                coord,
                                building_type,
                            });
                            format!("ACK_BUILD {} {} {:?} {:?}", x, y, building_type, layer)
                        } else {
                            "ERR_NO_PLAYER".to_string()
                        }
                    } else {
                        format!("ERR_UNKNOWN_BUILDING_TYPE {}", parts[3])
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "dismantle" => {
                if parts.len() < 3 {
                    "ERR_USAGE dismantle <x> <y> [layer]".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let layer = if parts.len() >= 4 {
                        match parts[3].to_ascii_lowercase().as_str() {
                            "viaduct" | "z1" | "1" => crate::game::Layer::Viaduct,
                            "skyway" | "z2" | "2" => crate::game::Layer::Skyway,
                            "subterranean" | "-1" => crate::game::Layer::Subterranean,
                            _ => crate::game::Layer::Ground,
                        }
                    } else {
                        crate::game::Layer::Ground
                    };
                    if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                        let coord = crate::game::GridCoord::new(x, y, layer);
                        commands.trigger(crate::game::DismantleBuildingEvent {
                            player_entity,
                            coord,
                        });
                        format!("ACK_DISMANTLE {} {} {:?}", x, y, layer)
                    } else {
                        "ERR_NO_PLAYER".to_string()
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "slot" | "select_slot" => {
                if parts.len() < 2 {
                    "ERR_USAGE slot <0..9>".to_string()
                } else if let Ok(slot_idx) = parts[1].parse::<usize>() {
                    if slot_idx < 10 {
                        if let Some((_, _, _, Some(mut hotbar))) = q.player.iter_mut().next() {
                            hotbar.select_slot(slot_idx);
                            format!("ACK_SLOT {}", slot_idx)
                        } else {
                            "ERR_NO_HOTBAR".to_string()
                        }
                    } else {
                        "ERR_SLOT_OUT_OF_BOUNDS".to_string()
                    }
                } else {
                    "ERR_INVALID_SLOT".to_string()
                }
            }
            "tick" => {
                if parts.len() < 2 {
                    "ERR_USAGE tick <count>".to_string()
                } else {
                    match parts[1].to_ascii_lowercase().as_str() {
                        "resume" | "continue" | "unpause" => {
                            tick_control.enabled = false;
                            virtual_time.unpause();
                            "ACK_TICK RESUME".to_string()
                        }
                        "pause" => {
                            tick_control.enabled = true;
                            tick_control.remaining_ticks = 0;
                            virtual_time.pause();
                            "ACK_TICK 0 (paused)".to_string()
                        }
                        val => {
                            if let Ok(count) = val.parse::<u32>() {
                                if count == 0 {
                                    tick_control.enabled = true;
                                    tick_control.remaining_ticks = 0;
                                    virtual_time.pause();
                                    "ACK_TICK 0 (paused)".to_string()
                                } else {
                                    virtual_time.unpause();
                                    tick_control.enabled = true;
                                    tick_control.remaining_ticks += count;
                                    format!("ACK_TICK {}", count)
                                }
                            } else {
                                format!("ERR_INVALID_TICK_COUNT {}", val)
                            }
                        }
                    }
                }
            }
            "screen" => {
                if parts.len() < 2 {
                    "ERR_USAGE screen <name>".to_string()
                } else if let Some(ref mut next) = next_screen {
                    match parts[1].to_ascii_lowercase().as_str() {
                        "gameplay" | "game" => {
                            next.set(crate::screens::Screen::Gameplay);
                            "ACK_SCREEN Gameplay".to_string()
                        }
                        "title" => {
                            next.set(crate::screens::Screen::Title);
                            "ACK_SCREEN Title".to_string()
                        }
                        _ => format!("ERR_UNKNOWN_SCREEN {}", parts[1]),
                    }
                } else {
                    "ERR_NO_SCREEN_RESOURCE".to_string()
                }
            }
            "rotate_camera" | "rotate" => {
                let dir = if parts.len() >= 2 { parts[1].to_ascii_lowercase() } else { "cw".to_string() };
                if let Some(ref mut cfg) = camera_config {
                    match dir.as_str() {
                        "cw" | "right" | "e" => {
                            cfg.rotate_cw();
                            format!("ACK_ROTATE_CAMERA CW Step={} Yaw={:.3}", cfg.rotation_step, cfg.target_yaw)
                        }
                        "ccw" | "left" | "q" => {
                            cfg.rotate_ccw();
                            format!("ACK_ROTATE_CAMERA CCW Step={} Yaw={:.3}", cfg.rotation_step, cfg.target_yaw)
                        }
                        deg_str => {
                            if let Ok(deg) = deg_str.parse::<f32>() {
                                cfg.rotate_by_degrees(deg);
                                format!("ACK_ROTATE_CAMERA {:.1}deg Step={} Yaw={:.3}", deg, cfg.rotation_step, cfg.target_yaw)
                            } else {
                                format!("ERR_INVALID_ROTATION_DIR {}", dir)
                            }
                        }
                    }
                } else {
                    "ERR_NO_CAMERA_CONFIG".to_string()
                }
            }
            "autoclave" => {
                if parts.len() < 4 {
                    "ERR_USAGE autoclave <x> <y> <grid|solve|swap r1 c1 r2 c2>".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let subcmd = parts[3].to_ascii_lowercase();
                    let mut found = None;
                    for (ent, auto, b) in q.autoclaves.iter_mut() {
                        if b.coord.x == x && b.coord.y == y {
                            found = Some((ent, auto));
                            break;
                        }
                    }

                    if let Some((_ent, mut auto)) = found {
                        match subcmd.as_str() {
                            "grid" => {
                                format!("ACK_AUTOCLAVE_GRID\n{}", auto.grid.to_ascii())
                            }
                            "solve" => {
                                match auto.grid.auto_swap_best() {
                                    Ok(summary) => {
                                        auto.stored_power_kw += summary.power_generated_kw;
                                        auto.total_catalysis_reactions += 1;
                                        format!(
                                            "ACK_AUTOCLAVE_SOLVED matches={} cascades={} refined={} power={:.1}",
                                            summary.matches.len(),
                                            summary.cascade_count,
                                            summary.total_refined_ingots,
                                            summary.power_generated_kw
                                        )
                                    }
                                    Err(e) => format!("ERR_AUTOCLAVE_SOLVE {:?}", e),
                                }
                            }
                            "swap" => {
                                if parts.len() < 8 {
                                    "ERR_USAGE autoclave <x> <y> swap <r1> <c1> <r2> <c2>".to_string()
                                } else if let (Ok(r1), Ok(c1), Ok(r2), Ok(c2)) = (
                                    parts[4].parse::<usize>(),
                                    parts[5].parse::<usize>(),
                                    parts[6].parse::<usize>(),
                                    parts[7].parse::<usize>(),
                                ) {
                                    match auto.grid.swap(r1, c1, r2, c2) {
                                        Ok(summary) => {
                                            auto.stored_power_kw += summary.power_generated_kw;
                                            auto.total_catalysis_reactions += 1;
                                            format!(
                                                "ACK_AUTOCLAVE_SWAP matches={} cascades={} power={:.1}",
                                                summary.matches.len(),
                                                summary.cascade_count,
                                                summary.power_generated_kw
                                            )
                                        }
                                        Err(e) => format!("ERR_AUTOCLAVE_SWAP {:?}", e),
                                    }
                                } else {
                                    "ERR_INVALID_SWAP_COORDS".to_string()
                                }
                            }
                            _ => format!("ERR_UNKNOWN_AUTOCLAVE_SUBCMD {}", subcmd),
                        }
                    } else {
                        format!("ERR_NO_AUTOCLAVE_AT {} {}", x, y)
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "tune" => {
                if parts.len() < 4 {
                    "ERR_USAGE tune <x> <y> <1..9|none>".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let coord = crate::game::GridCoord::ground(x, y);
                    let mut reply_msg = format!("ERR_NO_BUILDING_AT {} {}", x, y);
                    for (ent, mut tuning, b) in q.tunings.iter_mut() {
                        if b.coord == coord {
                            if tuning.is_locked {
                                reply_msg = "ERR_TUNING_LOCKED".to_string();
                            } else {
                                let f_opt = match parts[3].to_ascii_lowercase().as_str() {
                                    "none" | "0" | "clear" => Some(None),
                                    num_str => {
                                        if let Ok(n) = num_str.parse::<u8>() {
                                            if (1..=9).contains(&n) {
                                                Some(Some(n))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                };

                                if let Some(target_freq) = f_opt {
                                    tuning.frequency = target_freq;
                                    commands.trigger(crate::game::FrequencyTunedEvent {
                                        entity: ent,
                                        coord,
                                        frequency: target_freq,
                                    });
                                    reply_msg = format!("ACK_TUNE {} {} freq={:?}", x, y, target_freq);
                                } else {
                                    reply_msg = format!("ERR_INVALID_FREQUENCY {}", parts[3]);
                                }
                            }
                            break;
                        }
                    }
                    reply_msg
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "grove" => {
                if parts.len() < 2 {
                    "ERR_USAGE grove <status|solve>".to_string()
                } else {
                    match parts[1].to_ascii_lowercase().as_str() {
                        "status" => {
                            if let Some(ref grove) = resonance_grove {
                                serde_json::json!({
                                    "origin": [grove.origin.x, grove.origin.y],
                                    "width": grove.width,
                                    "height": grove.height,
                                    "completed_blocks": grove.completed_blocks,
                                    "is_full_symphony": grove.is_full_symphony,
                                    "total_power_kw": grove.total_power_kw,
                                    "conflict_count": grove.conflict_count,
                                }).to_string()
                            } else {
                                "ERR_NO_RESONANCE_GROVE".to_string()
                            }
                        }
                        "solve" => {
                            if let Some(ref grove) = resonance_grove {
                                let mut board = [[None; 9]; 9];
                                for (_, tuning, b) in q.tunings.iter() {
                                    if let Some((r, c)) = grove.local_coords(b.coord) {
                                        board[r][c] = tuning.frequency;
                                    }
                                }
                                if crate::game::ResonanceGrove::solve_sudoku(&mut board) {
                                    let mut tuned_count = 0;
                                    for (ent, mut tuning, b) in q.tunings.iter_mut() {
                                        if !tuning.is_locked {
                                            if let Some((r, c)) = grove.local_coords(b.coord) {
                                                if let Some(f) = board[r][c] {
                                                    if tuning.frequency != Some(f) {
                                                        tuning.frequency = Some(f);
                                                        tuned_count += 1;
                                                        commands.trigger(crate::game::FrequencyTunedEvent {
                                                            entity: ent,
                                                            coord: b.coord,
                                                            frequency: Some(f),
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    format!("ACK_GROVE_SOLVED tuned_machines={}", tuned_count)
                                } else {
                                    "ERR_GROVE_UNSOLVABLE".to_string()
                                }
                            } else {
                                "ERR_NO_RESONANCE_GROVE".to_string()
                            }
                        }
                        _ => format!("ERR_UNKNOWN_GROVE_SUBCMD {}", parts[1]),
                    }
                }
            }
            "teleport" => {
                if parts.len() < 3 {
                    "ERR_USAGE teleport <x> <y>".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<f32>(), parts[2].parse::<f32>()) {
                    if let Some((player_entity, _, _, _)) = q.player.iter().next() {
                        let target_pos = Vec3::new(x * crate::game::GridCoord::TILE_SIZE, 0.5, y * crate::game::GridCoord::TILE_SIZE);
                        commands.entity(player_entity).insert((
                            Transform::from_translation(target_pos),
                            Position(target_pos),
                            LinearVelocity(Vec3::ZERO),
                        ));
                        format!("ACK_TELEPORT {:.1} {:.1}", x, y)
                    } else {
                        "ERR_NO_PLAYER".to_string()
                    }
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "spawn_item" => {
                if parts.len() < 3 {
                    "ERR_USAGE spawn_item <x> <y> [item_type] [layer]".to_string()
                } else if let (Ok(x), Ok(y)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>()) {
                    let itype = if parts.len() >= 4 {
                        match parts[3].to_ascii_lowercase().as_str() {
                            "iron_ore" => crate::game::ItemType::IronOre,
                            "copper_ore" => crate::game::ItemType::CopperOre,
                            "iron_plate" => crate::game::ItemType::IronPlate,
                            "copper_plate" => crate::game::ItemType::CopperPlate,
                            "ingot_cube" | "ingot" => crate::game::ItemType::IngotCube,
                            "domino" | "polyomino_domino" => crate::game::ItemType::PolyominoDomino,
                            "square" | "polyomino_square" => crate::game::ItemType::PolyominoSquare,
                            "ibar" | "polyomino_ibar" => crate::game::ItemType::PolyominoIBar,
                            "ruby" => crate::game::ItemType::Ruby,
                            "sapphire" => crate::game::ItemType::Sapphire,
                            "stone" => crate::game::ItemType::Stone,
                            "clay" => crate::game::ItemType::Clay,
                            _ => crate::game::ItemType::IronPlate,
                        }
                    } else {
                        crate::game::ItemType::IronPlate
                    };

                    let layer = if parts.len() >= 5 {
                        match parts[4].to_ascii_lowercase().as_str() {
                            "viaduct" | "z1" | "1" => crate::game::Layer::Viaduct,
                            "skyway" | "z2" | "2" => crate::game::Layer::Skyway,
                            "subterranean" | "-1" => crate::game::Layer::Subterranean,
                            _ => crate::game::Layer::Ground,
                        }
                    } else {
                        crate::game::Layer::Ground
                    };

                    let coord = crate::game::GridCoord::new(x, y, layer);
                    commands.spawn(crate::game::TransportedItem::new(itype, coord));
                    format!("ACK_SPAWN_ITEM {} {} {:?} {:?}", x, y, itype, layer)
                } else {
                    "ERR_INVALID_COORDS".to_string()
                }
            }
            "quit" | "exit" => {
                app_exit.write(AppExit::Success);
                "ACK_QUIT".to_string()
            }
            _ => format!("ERR_UNKNOWN_COMMAND {}", cmd),
        };

        let _ = req.responder.send(reply);
    }
}

/// Progresses or pauses simulation steps when step-timing control is active.
fn step_tick_control(mut tick_control: ResMut<AgentTickControl>, mut time: ResMut<Time<Virtual>>) {
    if tick_control.enabled {
        if tick_control.remaining_ticks > 0 {
            tick_control.remaining_ticks -= 1;
            if tick_control.remaining_ticks == 0 {
                time.pause();
            }
        } else {
            time.pause();
        }
    }
}

/// Maps ASCII key labels into Bevy KeyCodes.
pub fn parse_keycode(name: &str) -> Option<KeyCode> {
    match name.trim().to_ascii_uppercase().as_str() {
        "W" | "KEYW" => Some(KeyCode::KeyW),
        "A" | "KEYA" => Some(KeyCode::KeyA),
        "S" | "KEYS" => Some(KeyCode::KeyS),
        "D" | "KEYD" => Some(KeyCode::KeyD),
        "E" | "KEYE" => Some(KeyCode::KeyE),
        "Q" | "KEYQ" => Some(KeyCode::KeyQ),
        "R" | "KEYR" => Some(KeyCode::KeyR),
        "F" | "KEYF" => Some(KeyCode::KeyF),
        "C" | "KEYC" => Some(KeyCode::KeyC),
        "V" | "KEYV" => Some(KeyCode::KeyV),
        "Z" | "KEYZ" => Some(KeyCode::KeyZ),
        "X" | "KEYX" => Some(KeyCode::KeyX),
        "SPACE" => Some(KeyCode::Space),
        "ESCAPE" | "ESC" => Some(KeyCode::Escape),
        "SHIFT" | "LEFTSHIFT" | "SHIFTLEFT" => Some(KeyCode::ShiftLeft),
        "RIGHTSHIFT" | "SHIFTRIGHT" => Some(KeyCode::ShiftRight),
        "CONTROL" | "CTRL" | "LEFTCONTROL" | "CONTROLLEFT" => Some(KeyCode::ControlLeft),
        "ALT" | "LEFTALT" | "ALTLEFT" => Some(KeyCode::AltLeft),
        "ENTER" | "RETURN" => Some(KeyCode::Enter),
        "TAB" => Some(KeyCode::Tab),
        "BACKSPACE" => Some(KeyCode::Backspace),
        "UP" | "ARROWUP" => Some(KeyCode::ArrowUp),
        "DOWN" | "ARROWDOWN" => Some(KeyCode::ArrowDown),
        "LEFT" | "ARROWLEFT" => Some(KeyCode::ArrowLeft),
        "RIGHT" | "ARROWRIGHT" => Some(KeyCode::ArrowRight),
        "0" | "DIGIT0" => Some(KeyCode::Digit0),
        "1" | "DIGIT1" => Some(KeyCode::Digit1),
        "2" | "DIGIT2" => Some(KeyCode::Digit2),
        "3" | "DIGIT3" => Some(KeyCode::Digit3),
        "4" | "DIGIT4" => Some(KeyCode::Digit4),
        "5" | "DIGIT5" => Some(KeyCode::Digit5),
        "6" | "DIGIT6" => Some(KeyCode::Digit6),
        "7" | "DIGIT7" => Some(KeyCode::Digit7),
        "8" | "DIGIT8" => Some(KeyCode::Digit8),
        "9" | "DIGIT9" => Some(KeyCode::Digit9),
        _ => None,
    }
}
