use bevy::prelude::*;
use crate::screens::Screen;
use super::inventory::{Hotbar, Inventory};

/// Marker component for the hotbar root container.
#[derive(Component)]
pub struct HotbarRoot;

/// Marker component for an individual slot UI node.
#[derive(Component)]
pub struct HotbarSlotUi {
    pub slot_index: usize,
}

/// Text label marker inside a hotbar slot node.
#[derive(Component)]
pub struct HotbarSlotText {
    pub slot_index: usize,
}

/// Spawns the tactile wooden hotbar HUD at the bottom of the screen.
pub fn spawn_hotbar_hud(mut commands: Commands) {
    commands
        .spawn((
            HotbarRoot,
            DespawnOnExit(Screen::Gameplay),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(16.0),
                left: Val::Percent(50.0),
                margin: UiRect {
                    left: Val::Px(-270.0), // center 540px container
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                width: Val::Px(540.0),
                height: Val::Px(56.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                border: UiRect::all(Val::Px(3.0)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.18, 0.12, 0.08, 0.90)),
            BorderColor::all(Color::srgb(0.55, 0.38, 0.20)),
        ))
        .with_children(|parent| {
            for slot_index in 0..10 {
                let key_label = if slot_index == 9 { 0 } else { slot_index + 1 };
                parent
                    .spawn((
                        HotbarSlotUi { slot_index },
                        Node {
                            width: Val::Px(48.0),
                            height: Val::Px(46.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(2.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.28, 0.20, 0.14, 0.85)),
                        BorderColor::all(Color::srgb(0.40, 0.28, 0.18)),
                    ))
                    .with_children(|slot_parent| {
                        // Key index header
                        slot_parent.spawn((
                            Text::new(key_label.to_string()),
                            TextFont::from_font_size(10.0),
                            TextColor(Color::srgb(0.75, 0.65, 0.50)),
                        ));

                        // Item label / count
                        slot_parent.spawn((
                            HotbarSlotText { slot_index },
                            Text::new("-"),
                            TextFont::from_font_size(11.0),
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

/// Handles number keys 1 through 0 to select the active hotbar slot.
pub fn handle_hotbar_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Hotbar, With<crate::player::Player>>,
) {
    let key_slots = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
        (KeyCode::Digit8, 7),
        (KeyCode::Digit9, 8),
        (KeyCode::Digit0, 9),
    ];

    for (key, slot) in key_slots {
        if keyboard.just_pressed(key) {
            for mut hotbar in players.iter_mut() {
                hotbar.selected_slot = slot;
            }
            break;
        }
    }
}

/// Syncs the HUD visual borders, highlight colors, and item labels with player state.
pub fn sync_hotbar_hud(
    players: Query<(&Hotbar, &Inventory), With<crate::player::Player>>,
    mut slot_nodes: Query<(&HotbarSlotUi, &mut BorderColor, &mut BackgroundColor)>,
    mut text_nodes: Query<(&HotbarSlotText, &mut Text, &mut TextColor)>,
) {
    let Ok((hotbar, inventory)) = players.single() else {
        return;
    };

    // Update slot node borders (highlight active slot with golden glow)
    for (slot_ui, mut border, mut bg) in slot_nodes.iter_mut() {
        if slot_ui.slot_index == hotbar.selected_slot {
            *border = BorderColor::all(Color::srgb(0.95, 0.82, 0.22)); // Golden border
            bg.0 = Color::srgba(0.42, 0.30, 0.18, 0.95);
        } else {
            *border = BorderColor::all(Color::srgb(0.40, 0.28, 0.18)); // Wooden brown
            bg.0 = Color::srgba(0.28, 0.20, 0.14, 0.85);
        }
    }

    // Update item text in each slot
    for (slot_text, mut text, mut color) in text_nodes.iter_mut() {
        let i = slot_text.slot_index;
        if i < hotbar.slots.len() {
            if let Some(item_type) = hotbar.slots[i] {
                let count = inventory.count_item(item_type);
                // Abbreviate name for small slot display
                let abbrev = match item_type {
                    super::items::ItemType::WoodenRollerBelt => "Belt",
                    super::items::ItemType::ViaductBelt => "Viad",
                    super::items::ItemType::InclineRamp => "Ramp",
                    super::items::ItemType::ClockworkWellhead => "Well",
                    super::items::ItemType::SmeltingHearth => "Hrth",
                    super::items::ItemType::StorageChest => "Chst",
                    super::items::ItemType::CargoCompressor => "Comp",
                    super::items::ItemType::SwivelTurntable => "Turn",
                    super::items::ItemType::SortingGate => "Gate",
                    super::items::ItemType::SurveyBeacon => "Beac",
                    super::items::ItemType::LogicDiviner => "Divn",
                    super::items::ItemType::IronPlate => "FePl",
                    super::items::ItemType::BrassCog => "Cog",
                    super::items::ItemType::WoodenPlank => "Plnk",
                    super::items::ItemType::IngotCube => "Cube",
                    super::items::ItemType::PolyominoDomino => "Dom",
                    super::items::ItemType::PolyominoIBar => "I-Bar",
                    super::items::ItemType::PolyominoSquare => "Squr",
                    _ => "Item",
                };
                text.0 = format!("{}\n{}", abbrev, count);
                color.0 = if count > 0 { Color::WHITE } else { Color::srgb(0.6, 0.5, 0.5) };
            } else {
                text.0 = "-".to_string();
                color.0 = Color::srgb(0.4, 0.35, 0.3);
            }
        }
    }
}

/// Tracks tester task completions during gameplay verification.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct TesterTasksProgress {
    pub moved: bool,
    pub rotated_camera: bool,
    pub prospected: bool,
    pub flagged: bool,
    pub mined: bool,
    pub crafted: bool,
    pub selected_hotbar: bool,
    pub placed_building: bool,
    pub dismantled_building: bool,
}

impl TesterTasksProgress {
    pub fn completed_count(&self) -> usize {
        let mut count = 0;
        if self.moved { count += 1; }
        if self.rotated_camera { count += 1; }
        if self.prospected { count += 1; }
        if self.flagged { count += 1; }
        if self.mined { count += 1; }
        if self.crafted { count += 1; }
        if self.selected_hotbar { count += 1; }
        if self.placed_building { count += 1; }
        if self.dismantled_building { count += 1; }
        count
    }
}

/// Marker component for individual task items in the tester guide box.
#[derive(Component)]
pub struct TesterTaskItemText {
    pub task_id: u8,
}

/// Marker component for the summary footer text in the tester guide box.
#[derive(Component)]
pub struct TesterGuideSummaryText;

/// Spawns the non-obtrusive tester checklist panel on the right side of the screen.
pub fn spawn_tester_guide_hud(mut commands: Commands) {
    let task_labels = [
        "Walk & Sprint: WASD / Shift",
        "Orbit Camera: Q / E / MMB (360°)",
        "Prospect Turf: LMB / F (Unfold)",
        "Flag Hazard: RMB / R",
        "Mine Deposit: LMB on Ore Node",
        "Handcraft: C / Tab (Iron Plate)",
        "Select Hotbar: Keys 1–0",
        "Place Factory: LMB (Belt/Ramp)",
        "Dismantle: RMB / X",
    ];

    commands
        .spawn((
            DespawnOnExit(Screen::Gameplay),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(24.0),
                right: Val::Px(20.0),
                width: Val::Px(290.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.14, 0.20, 0.82)),
            BorderColor::all(Color::srgba(0.40, 0.55, 0.70, 0.50)),
        ))
        .with_children(|parent| {
            // Header title
            parent.spawn((
                Text::new("TESTER CHECKLIST"),
                TextFont::from_font_size(13.0),
                TextColor(Color::srgb(0.95, 0.82, 0.35)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Introductory verification tasks:"),
                TextFont::from_font_size(10.5),
                TextColor(Color::srgb(0.65, 0.75, 0.85)),
            ));

            // Task item rows
            for (i, label) in task_labels.iter().enumerate() {
                parent.spawn((
                    TesterTaskItemText { task_id: i as u8 },
                    Text::new(format!("[ ] {}", label)),
                    TextFont::from_font_size(11.0),
                    TextColor(Color::srgb(0.85, 0.88, 0.92)),
                ));
            }

            // Summary progress footer
            parent.spawn((
                TesterGuideSummaryText,
                Text::new("Tasks Verified: 0 / 9"),
                TextFont::from_font_size(11.5),
                TextColor(Color::srgb(0.50, 0.85, 0.95)),
            ));
        });
}

/// Continuous system monitoring tester progress for movement, camera, and hotbar interactions.
pub fn update_tester_tasks_continuous(
    mut progress: ResMut<TesterTasksProgress>,
    players: Query<(&Transform, &Hotbar), With<crate::player::Player>>,
    cam_config: Option<Res<crate::camera::DioramaCameraConfig>>,
) {
    if let Ok((transform, hotbar)) = players.single() {
        let spawn_pos = super::grid::GridCoord::ground(1, 1).to_world_pos() + Vec3::new(0.0, 0.5, 0.0);
        if transform.translation.distance(spawn_pos) > 0.4 {
            progress.moved = true;
        }
        if hotbar.selected_slot > 0 {
            progress.selected_hotbar = true;
        }
    }

    if let Some(config) = cam_config {
        let initial_yaw = 45.0_f32.to_radians();
        if (config.current_yaw - initial_yaw).abs() > 0.1 {
            progress.rotated_camera = true;
        }
    }
}

/// Observers tracking events for prospecting, mining, handcrafting, and building placement.
pub fn on_prospect_task(on: On<super::prospecting::TileStateChangedEvent>, mut progress: ResMut<TesterTasksProgress>) {
    let ev = on.event();
    match ev.new_state {
        super::terrain::TileProspectState::Revealed | super::terrain::TileProspectState::Extracted => {
            progress.prospected = true;
        }
        super::terrain::TileProspectState::Flagged => {
            progress.flagged = true;
        }
        _ => {}
    }
}

pub fn on_mine_task(_on: On<super::mining::MineTileEvent>, mut progress: ResMut<TesterTasksProgress>) {
    progress.mined = true;
}

pub fn on_craft_task(_on: On<super::crafting::StartCraftEvent>, mut progress: ResMut<TesterTasksProgress>) {
    progress.crafted = true;
}

pub fn on_place_task(_on: On<super::building::BuildingPlacedEvent>, mut progress: ResMut<TesterTasksProgress>) {
    progress.placed_building = true;
}

pub fn on_dismantle_task(_on: On<super::building::DismantleBuildingEvent>, mut progress: ResMut<TesterTasksProgress>) {
    progress.dismantled_building = true;
}

/// Syncs the tester guide box texts and checkboxes with current completion progress.
pub fn sync_tester_guide_hud(
    progress: Res<TesterTasksProgress>,
    mut task_texts: Query<(&TesterTaskItemText, &mut Text, &mut TextColor), Without<TesterGuideSummaryText>>,
    mut summary_text: Query<(&mut Text, &mut TextColor), With<TesterGuideSummaryText>>,
) {
    let task_defs = [
        (0, progress.moved, "Walk & Sprint: WASD / Shift"),
        (1, progress.rotated_camera, "Orbit Camera: Q / E / MMB (360°)"),
        (2, progress.prospected, "Prospect Turf: LMB / F (Unfold)"),
        (3, progress.flagged, "Flag Hazard: RMB / R"),
        (4, progress.mined, "Mine Deposit: LMB on Ore Node"),
        (5, progress.crafted, "Handcraft: C / Tab (Iron Plate)"),
        (6, progress.selected_hotbar, "Select Hotbar: Keys 1–0"),
        (7, progress.placed_building, "Place Factory: LMB (Belt/Ramp)"),
        (8, progress.dismantled_building, "Dismantle: RMB / X"),
    ];

    for (item, mut text, mut color) in task_texts.iter_mut() {
        if let Some(&(_, done, label)) = task_defs.iter().find(|(id, _, _)| *id == item.task_id) {
            if done {
                text.0 = format!("[✓] {}", label);
                color.0 = Color::srgb(0.40, 0.90, 0.50); // Vibrant mint green
            } else {
                text.0 = format!("[ ] {}", label);
                color.0 = Color::srgb(0.85, 0.88, 0.92); // Clean off-white
            }
        }
    }

    if let Ok((mut summary, mut summary_color)) = summary_text.single_mut() {
        let count = progress.completed_count();
        if count == 9 {
            summary.0 = "Tasks Verified: 9 / 9 (All Systems Operational!)".to_string();
            summary_color.0 = Color::srgb(0.40, 0.95, 0.55);
        } else {
            summary.0 = format!("Tasks Verified: {} / 9", count);
            summary_color.0 = Color::srgb(0.50, 0.85, 0.95);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<TesterTasksProgress>()
        .register_type::<TesterTasksProgress>()
        .add_systems(OnEnter(Screen::Gameplay), (spawn_hotbar_hud, spawn_tester_guide_hud))
        .add_observer(on_prospect_task)
        .add_observer(on_mine_task)
        .add_observer(on_craft_task)
        .add_observer(on_place_task)
        .add_observer(on_dismantle_task)
        .add_systems(
            Update,
            (
                handle_hotbar_keys,
                sync_hotbar_hud,
                update_tester_tasks_continuous,
                sync_tester_guide_hud,
            )
                .run_if(in_state(Screen::Gameplay)),
        );
}
