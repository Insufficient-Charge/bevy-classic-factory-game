use bevy::prelude::*;
use avian3d::prelude::{Collider, LinearVelocity, Position, RigidBody};
use std::collections::{HashMap, HashSet};
use crate::screens::Screen;
use super::building::{Building, BuildingPlacedEvent, BuildingType};
use super::crafting::CraftingQueue;
use super::grid::{Direction, GridCoord};
use super::harmonic::{AcousticMonolith, HarmonicTuning, ResonanceGrove};
use super::inventory::{Hotbar, Inventory};
use super::items::ItemType;
use super::logistics::{ConveyorBelt, TransportedItem};
use super::mining::{MiningTool, ResourceDeposit};
use super::prospecting::TileStateChangedEvent;
use super::terrain::{IslandGrid, TileProspectState};

/// Resource containing all shared meshes and materials for the diorama world.
#[derive(Resource)]
pub struct DioramaAssets {
    pub tile_mesh: Handle<Mesh>,
    pub item_mesh: Handle<Mesh>,
    pub belt_mesh: Handle<Mesh>,
    pub viaduct_mesh: Handle<Mesh>,
    pub ramp_mesh: Handle<Mesh>,
    pub wellhead_mesh: Handle<Mesh>,
    pub chest_mesh: Handle<Mesh>,
    pub hearth_mesh: Handle<Mesh>,
    pub compressor_mesh: Handle<Mesh>,
    pub beacon_mesh: Handle<Mesh>,
    pub diviner_mesh: Handle<Mesh>,
    pub autoclave_mesh: Handle<Mesh>,
    pub bell_jar_mesh: Handle<Mesh>,
    pub monolith_mesh: Handle<Mesh>,
    pub pylon_mesh: Handle<Mesh>,
    pub conductor_mesh: Handle<Mesh>,

    pub grass_material: Handle<StandardMaterial>,
    pub revealed_material: Handle<StandardMaterial>,
    pub hazard_1_material: Handle<StandardMaterial>,
    pub hazard_2_material: Handle<StandardMaterial>,
    pub hazard_3_plus_material: Handle<StandardMaterial>,
    pub flagged_material: Handle<StandardMaterial>,
    pub chalk_material: Handle<StandardMaterial>,
    pub bridge_material: Handle<StandardMaterial>,
    pub iron_node_material: Handle<StandardMaterial>,
    pub copper_node_material: Handle<StandardMaterial>,
    pub crystal_node_material: Handle<StandardMaterial>,
    pub ruby_node_material: Handle<StandardMaterial>,
    pub belt_material: Handle<StandardMaterial>,
    pub viaduct_material: Handle<StandardMaterial>,
    pub wellhead_material: Handle<StandardMaterial>,
    pub chest_material: Handle<StandardMaterial>,
    pub hearth_material: Handle<StandardMaterial>,
    pub compressor_material: Handle<StandardMaterial>,
    pub beacon_material: Handle<StandardMaterial>,
    pub diviner_material: Handle<StandardMaterial>,
    pub autoclave_material: Handle<StandardMaterial>,
    pub bell_jar_material: Handle<StandardMaterial>,
    pub monolith_material: Handle<StandardMaterial>,
    pub pylon_material: Handle<StandardMaterial>,
    pub conductor_material: Handle<StandardMaterial>,
    pub harmonic_bridge_material: Handle<StandardMaterial>,

    pub iron_item_material: Handle<StandardMaterial>,
    pub copper_item_material: Handle<StandardMaterial>,
    pub plate_item_material: Handle<StandardMaterial>,
    pub polyomino_material: Handle<StandardMaterial>,
    pub ingot_cube_material: Handle<StandardMaterial>,
    pub sky_backdrop_mesh: Handle<Mesh>,
    pub sky_backdrop_material: Handle<StandardMaterial>,
}

impl FromWorld for DioramaAssets {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let tile_mesh = meshes.add(Cuboid::new(0.96, 0.4, 0.96));
        let item_mesh = meshes.add(Sphere::new(0.18));
        let belt_mesh = meshes.add(Cuboid::new(0.90, 0.15, 0.90));
        let viaduct_mesh = meshes.add(Cuboid::new(0.90, 0.20, 0.90));
        let ramp_mesh = meshes.add(Cuboid::new(0.85, 0.25, 0.95));
        let wellhead_mesh = meshes.add(Cylinder::new(0.35, 0.8));
        let chest_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));
        let hearth_mesh = meshes.add(Cuboid::new(0.7, 0.5, 0.7));
        let compressor_mesh = meshes.add(Cuboid::new(0.8, 1.0, 0.8));
        let beacon_mesh = meshes.add(Cylinder::new(0.3, 1.2));
        let diviner_mesh = meshes.add(Cylinder::new(0.35, 0.9));
        let autoclave_mesh = meshes.add(Sphere::new(0.48));
        let bell_jar_mesh = meshes.add(Cylinder::new(0.38, 0.7));
        let monolith_mesh = meshes.add(Cuboid::new(0.7, 1.5, 0.7));
        let pylon_mesh = meshes.add(Cylinder::new(0.25, 1.4));
        let conductor_mesh = meshes.add(Sphere::new(0.4));
        let sky_backdrop_mesh = meshes.add(Cuboid::new(500.0, 0.1, 500.0));

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let grass_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.32, 0.68, 0.28),
            perceptual_roughness: 0.8,
            ..default()
        });
        let revealed_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.50, 0.55),
            perceptual_roughness: 0.6,
            ..default()
        });
        let hazard_1_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.52, 0.70),
            perceptual_roughness: 0.55,
            ..default()
        });
        let hazard_2_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.62, 0.45),
            perceptual_roughness: 0.55,
            ..default()
        });
        let hazard_3_plus_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.42, 0.30),
            perceptual_roughness: 0.5,
            ..default()
        });
        let flagged_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.65, 0.25),
            perceptual_roughness: 0.7,
            ..default()
        });
        let chalk_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.88, 0.90, 0.92),
            perceptual_roughness: 0.5,
            ..default()
        });
        let bridge_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.50, 0.35, 0.20),
            perceptual_roughness: 0.7,
            ..default()
        });
        let iron_node_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.78, 0.35, 0.25),
            perceptual_roughness: 0.4,
            metallic: 0.3,
            ..default()
        });
        let copper_node_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.72, 0.75),
            perceptual_roughness: 0.4,
            metallic: 0.4,
            ..default()
        });
        let crystal_node_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.15, 0.85, 0.65),
            emissive: LinearRgba::new(0.2, 0.8, 0.5, 1.0),
            perceptual_roughness: 0.2,
            ..default()
        });
        let belt_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.70, 0.55, 0.25),
            perceptual_roughness: 0.3,
            metallic: 0.5,
            ..default()
        });
        let viaduct_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.80, 0.75, 0.65),
            metallic: 0.7,
            perceptual_roughness: 0.25,
            ..default()
        });
        let wellhead_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.62, 0.22),
            perceptual_roughness: 0.3,
            metallic: 0.7,
            ..default()
        });
        let chest_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.58, 0.38, 0.22),
            perceptual_roughness: 0.8,
            ..default()
        });
        let hearth_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.32, 0.30),
            emissive: LinearRgba::new(0.6, 0.2, 0.05, 1.0),
            ..default()
        });
        let compressor_material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.85, 0.90, 0.95, 0.7),
            metallic: 0.5,
            perceptual_roughness: 0.1,
            ..default()
        });
        let beacon_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.65, 0.45, 0.25),
            perceptual_roughness: 0.6,
            ..default()
        });
        let diviner_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.45, 0.60),
            emissive: LinearRgba::new(0.1, 0.3, 0.6, 1.0),
            ..default()
        });

        let iron_item_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.38, 0.28),
            metallic: 0.6,
            ..default()
        });
        let copper_item_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.82, 0.85),
            metallic: 0.7,
            ..default()
        });
        let plate_item_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.78, 0.82),
            metallic: 0.9,
            ..default()
        });
        let polyomino_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.90, 0.70),
            emissive: LinearRgba::new(0.1, 0.5, 0.4, 1.0),
            perceptual_roughness: 0.2,
            ..default()
        });
        let ingot_cube_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.80, 0.30),
            metallic: 0.9,
            perceptual_roughness: 0.2,
            ..default()
        });
        let ruby_node_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.90, 0.15, 0.30),
            emissive: LinearRgba::new(0.3, 0.05, 0.1, 1.0),
            perceptual_roughness: 0.2,
            ..default()
        });
        let autoclave_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.35, 0.65),
            emissive: LinearRgba::new(0.4, 0.1, 0.3, 1.0),
            perceptual_roughness: 0.15,
            ..default()
        });
        let bell_jar_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.75, 0.25),
            emissive: LinearRgba::new(0.3, 0.2, 0.05, 1.0),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        });
        let monolith_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.88, 0.98),
            emissive: LinearRgba::new(0.3, 0.2, 0.5, 1.0),
            perceptual_roughness: 0.3,
            ..default()
        });
        let pylon_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.75, 0.20),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        });
        let conductor_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.85, 0.75),
            emissive: LinearRgba::new(0.1, 0.4, 0.3, 1.0),
            metallic: 0.5,
            perceptual_roughness: 0.3,
            ..default()
        });
        let harmonic_bridge_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.60, 0.75, 0.95),
            emissive: LinearRgba::new(0.2, 0.3, 0.6, 1.0),
            perceptual_roughness: 0.4,
            ..default()
        });

        Self {
            tile_mesh,
            item_mesh,
            belt_mesh,
            viaduct_mesh,
            ramp_mesh,
            wellhead_mesh,
            chest_mesh,
            hearth_mesh,
            compressor_mesh,
            beacon_mesh,
            diviner_mesh,
            autoclave_mesh,
            bell_jar_mesh,
            monolith_mesh,
            pylon_mesh,
            conductor_mesh,
            grass_material,
            revealed_material,
            hazard_1_material,
            hazard_2_material,
            hazard_3_plus_material,
            flagged_material,
            chalk_material,
            bridge_material,
            iron_node_material,
            copper_node_material,
            crystal_node_material,
            ruby_node_material,
            belt_material,
            viaduct_material,
            wellhead_material,
            chest_material,
            hearth_material,
            compressor_material,
            beacon_material,
            diviner_material,
            autoclave_material,
            bell_jar_material,
            monolith_material,
            pylon_material,
            conductor_material,
            harmonic_bridge_material,
            iron_item_material,
            copper_item_material,
            plate_item_material,
            polyomino_material,
            ingot_cube_material,
            sky_backdrop_mesh,
            sky_backdrop_material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.48, 0.65, 0.82),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
        }
    }
}

/// Visual marker component attached to 3D voxel tile entities.
#[derive(Component)]
pub struct VisualTileMarker {
    pub coord: GridCoord,
}

/// Spawns the starter Cradle Meadows & Lattice Flats diorama islands when entering Gameplay.
pub fn setup_cradle_meadows(
    mut island_grid: ResMut<IslandGrid>,
    assets: Res<DioramaAssets>,
    mut camera_q: Query<(Entity, &mut Camera), With<crate::camera::SceneCamera>>,
    mut commands: Commands,
) {
    info!("Initializing Cradle Meadows & Lattice Flats archipelago diorama...");

    // Remove atmospheric skybox and fog from SceneCamera for clean diorama tabletop view
    // (Atmosphere LUTs create concentric distortion rings in orthographic projection)
    for (cam_entity, mut camera) in camera_q.iter_mut() {
        commands.entity(cam_entity).remove::<(
            bevy::pbr::AtmosphereSettings,
            bevy::light::AtmosphereEnvironmentMapLight,
            bevy::pbr::DistanceFog,
        )>();
        camera.clear_color = bevy::camera::ClearColorConfig::Default;
    }

    // Spawn serene pastoral sky backdrop below floating archipelago
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(assets.sky_backdrop_mesh.clone()),
        MeshMaterial3d(assets.sky_backdrop_material.clone()),
        Transform::from_xyz(4.0, -15.0, 4.0),
    ));

    // Spawn warm golden diorama sunlight with soft shadows
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.96, 0.88),
            illuminance: 80_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(16.0, 24.0, 16.0).looking_at(Vec3::new(4.0, 0.0, 6.0), Vec3::Y),
        DespawnOnExit(Screen::Gameplay),
    ));

    // Spawn cool sky fill light from opposite angle
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.75, 0.85, 1.0),
            illuminance: 25_000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(-12.0, 18.0, -8.0).looking_at(Vec3::new(4.0, 0.0, 6.0), Vec3::Y),
        DespawnOnExit(Screen::Gameplay),
    ));

    // Set pastoral sky clear color and ambient skylight bounce
    commands.insert_resource(ClearColor(Color::srgb(0.48, 0.65, 0.82)));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.82, 0.88, 0.98),
        brightness: 8000.0,
        ..default()
    });

    let mut hazards = HashSet::new();
    // Overcharge vent hazards in the Cradle Meadows prospecting zone
    hazards.insert((6, 6));
    hazards.insert((7, 6));

    // Generate Cradle Meadows (-5..13, -5..13) - widened by 5 tiles radius
    island_grid.generate_rectangular_island_bounds(-5, 13, -5, 13, &hazards);

    // Generate Sky-Bridge connecting Cradle Meadows to Lattice Flats (x: 3..5, y: 13..16)
    for x in 3..5 {
        for y in 13..16 {
            let coord = GridCoord::ground(x, y);
            island_grid.add_tile(coord, false);
            if let Some(t) = island_grid.tiles.get_mut(&coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
        }
    }

    // Generate Lattice Flats stepped chalk island (x: 0..8, y: 16..23)
    for x in 0..8 {
        for y in 16..23 {
            let coord = GridCoord::ground(x, y);
            let is_hazard = (x == 1 && y == 21) || (x == 6 && y == 20);
            island_grid.add_tile(coord, is_hazard);
        }
    }

    // Generate Sky-Bridge connecting Lattice Flats to Chromite Basin (x: 3..5, y: 23..26)
    for x in 3..5 {
        for y in 23..26 {
            let coord = GridCoord::ground(x, y);
            island_grid.add_tile(coord, false);
            if let Some(t) = island_grid.tiles.get_mut(&coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
        }
    }

    // Generate Chromite Basin gemstone quarry (x: 0..8, y: 26..33)
    for x in 0..8 {
        for y in 26..33 {
            let coord = GridCoord::ground(x, y);
            let is_hazard = (x == 2 && y == 31) || (x == 5 && y == 30);
            island_grid.add_tile(coord, is_hazard);
        }
    }

    // Generate Sky-Bridge connecting Cradle Meadows to Resonance Grove (x: 13..16, y: 3..5)
    for x in 13..16 {
        for y in 3..5 {
            let coord = GridCoord::ground(x, y);
            island_grid.add_tile(coord, false);
            if let Some(t) = island_grid.tiles.get_mut(&coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
        }
    }

    // Generate 9x9 Resonance Grove island (x: 16..25, y: 0..9) (GDD §5.4 & §8.4)
    for x in 16..25 {
        for y in 0..9 {
            let coord = GridCoord::ground(x, y);
            island_grid.add_tile(coord, false);
            if let Some(t) = island_grid.tiles.get_mut(&coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
        }
    }

    // Initialize 9x9 Resonance Grove resource origin at (16, 0)
    let grove_origin = GridCoord::ground(16, 0);
    commands.insert_resource(ResonanceGrove::new(grove_origin));

    // Spawn ancient clue monoliths with immovable harmonic runes (GDD §5.4)
    let clues = [
        (16, 0, 1u8), // Local (0, 0): Rune 1
        (20, 4, 5u8), // Local (4, 4): Rune 5
        (24, 8, 9u8), // Local (8, 8): Rune 9
    ];
    for (cx, cy, freq) in clues {
        let clue_coord = GridCoord::ground(cx, cy);
        let world_pos = clue_coord.to_world_pos();
        commands.spawn((
            DespawnOnExit(Screen::Gameplay),
            Building::new(clue_coord, BuildingType::AcousticMonolith),
            AcousticMonolith { baseline_power_kw: 250.0, is_active: false },
            HarmonicTuning::new(Some(freq), true), // locked ancient clue!
            Mesh3d(assets.monolith_mesh.clone()),
            MeshMaterial3d(assets.monolith_material.clone()),
            Transform::from_translation(world_pos + Vec3::new(0.0, 0.75, 0.0)),
            RigidBody::Static,
            Collider::cuboid(0.7, 1.5, 0.7),
        ));
    }

    // Add resource deposits
    let iron_coord = GridCoord::ground(2, 2);
    let copper_coord = GridCoord::ground(5, 5);
    let clay_coord = GridCoord::ground(1, 6);
    let stone_coord = GridCoord::ground(6, 1);
    let crystal_coord = GridCoord::ground(4, 19); // Logic crystal deposit on Lattice Flats
    let ruby_coord = GridCoord::ground(2, 29); // Ruby deposit on Chromite Basin
    let sapphire_coord = GridCoord::ground(5, 29); // Sapphire deposit on Chromite Basin

    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(iron_coord, ItemType::IronOre, 100),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(copper_coord, ItemType::CopperOre, 80),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(clay_coord, ItemType::Clay, 50),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(stone_coord, ItemType::Stone, 60),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(crystal_coord, ItemType::LogicCrystal, 120),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(ruby_coord, ItemType::Ruby, 150),
    ));
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ResourceDeposit::new(sapphire_coord, ItemType::Sapphire, 150),
    ));

    // Reveal starter area (0..4, 0..6) where the Constructor awakens and builds initial factory
    for x in 0..4 {
        for y in 0..6 {
            let coord = GridCoord::ground(x, y);
            if let Some(tile) = island_grid.tiles.get_mut(&coord) {
                tile.prospect_state = TileProspectState::Revealed;
            }
        }
    }

    // Spawn 3D visual tile entities
    for (&coord, tile) in island_grid.tiles.iter() {
        let world_pos = coord.to_world_pos();

        let material = if (coord.y >= 13 && coord.y < 16) || (coord.y >= 23 && coord.y < 26) {
            assets.bridge_material.clone()
        } else if coord.x >= 13 && coord.x < 16 && coord.y >= 3 && coord.y < 5 {
            assets.harmonic_bridge_material.clone()
        } else if coord.x >= 16 && coord.x < 25 && coord.y >= 0 && coord.y < 9 {
            assets.chalk_material.clone()
        } else if coord.y >= 26 {
            if coord == ruby_coord {
                assets.ruby_node_material.clone()
            } else {
                assets.revealed_material.clone()
            }
        } else if coord.y >= 16 {
            assets.chalk_material.clone()
        } else {
            match tile.prospect_state {
                TileProspectState::Covered => {
                    if coord == iron_coord {
                        assets.iron_node_material.clone()
                    } else if coord == copper_coord {
                        assets.copper_node_material.clone()
                    } else {
                        assets.grass_material.clone()
                    }
                }
                TileProspectState::Revealed | TileProspectState::Extracted => {
                    assets.revealed_material.clone()
                }
                TileProspectState::Flagged => assets.flagged_material.clone(),
            }
        };

        commands.spawn((
            DespawnOnExit(Screen::Gameplay),
            VisualTileMarker { coord },
            Mesh3d(assets.tile_mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos - Vec3::new(0.0, 0.2, 0.0)),
            RigidBody::Static,
            Collider::cuboid(1.0, 0.4, 1.0),
        ));
    }
    info!("Spawned {} visual tiles for diorama archipelago", island_grid.tiles.len());
}

/// Attaches inventory, hotbar, and tools to the Player upon spawning.
pub fn setup_player_inventory(
    mut players: Query<(Entity, &mut Transform, Option<&mut Position>, Option<&mut LinearVelocity>), Added<crate::player::Player>>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut pos_opt, mut vel_opt) in players.iter_mut() {
        let spawn_pos = GridCoord::ground(1, 1).to_world_pos() + Vec3::new(0.0, 0.5, 0.0);
        transform.translation = spawn_pos;
        if let Some(ref mut pos) = pos_opt {
            pos.0 = spawn_pos;
        }
        if let Some(ref mut vel) = vel_opt {
            vel.0 = Vec3::ZERO;
        }

        let mut inventory = Inventory::default();
        // Starter bootstrapping items (GDD Section 17.1 & 14.3)
        inventory.add_item(ItemType::WoodenPlank, 16);
        inventory.add_item(ItemType::IronPlate, 12);
        inventory.add_item(ItemType::BrassCog, 8);
        inventory.add_item(ItemType::WoodenRollerBelt, 12);
        inventory.add_item(ItemType::ClockworkWellhead, 1);
        inventory.add_item(ItemType::StorageChest, 2);

        // Phase 2 multi-layer and polyomino items (GDD §5.2 & §8.2)
        inventory.add_item(ItemType::ViaductBelt, 8);
        inventory.add_item(ItemType::InclineRamp, 4);
        inventory.add_item(ItemType::SwivelTurntable, 2);
        inventory.add_item(ItemType::SortingGate, 2);
        inventory.add_item(ItemType::CargoCompressor, 1);
        inventory.add_item(ItemType::PolyominoDomino, 4);
        inventory.add_item(ItemType::PolyominoIBar, 2);
        inventory.add_item(ItemType::PolyominoSquare, 2);
        inventory.add_item(ItemType::SurveyBeacon, 1);
        inventory.add_item(ItemType::LogicDiviner, 1);

        // Phase 3 Gemstone Catalysis & Bejeweled items (GDD §5.3 & §8.3)
        inventory.add_item(ItemType::ChromaticAutoclave, 2);
        inventory.add_item(ItemType::ResonanceBellJar, 2);
        inventory.add_item(ItemType::Ruby, 12);
        inventory.add_item(ItemType::Sapphire, 12);
        inventory.add_item(ItemType::IngotCube, 4);

        // Phase 4 Sudoku Resonance & Harmonic items (GDD §5.4 & §8.4)
        inventory.add_item(ItemType::AcousticMonolith, 2);
        inventory.add_item(ItemType::TuningForkPylon, 2);
        inventory.add_item(ItemType::TuningFork, 4);
        inventory.add_item(ItemType::HarmonicRune, 8);
        inventory.add_item(ItemType::ResonanceConductor, 1);
        inventory.add_item(ItemType::HarmonicBridge, 4);

        // Pre-equip hotbar slots
        let mut hotbar = Hotbar::default();
        hotbar.assign_slot(0, ItemType::WoodenRollerBelt);
        hotbar.assign_slot(1, ItemType::ViaductBelt);
        hotbar.assign_slot(2, ItemType::InclineRamp);
        hotbar.assign_slot(3, ItemType::ClockworkWellhead);
        hotbar.assign_slot(4, ItemType::CargoCompressor);
        hotbar.assign_slot(5, ItemType::StorageChest);
        hotbar.assign_slot(6, ItemType::SurveyBeacon);
        hotbar.assign_slot(7, ItemType::LogicDiviner);
        hotbar.assign_slot(8, ItemType::PolyominoDomino);
        hotbar.assign_slot(9, ItemType::PolyominoSquare);

        commands.entity(entity).insert((
            crate::camera::DioramaCameraTarget,
            inventory,
            hotbar,
            MiningTool::default(),
            CraftingQueue::default(),
        ));
    }
}

/// Observer that spawns 3D visuals and solid colliders when buildings are placed.
pub fn on_building_placed(
    on: On<BuildingPlacedEvent>,
    assets: Option<Res<DioramaAssets>>,
    mut commands: Commands,
) {
    let Some(assets) = assets.as_deref() else {
        return;
    };

    let ev = on.event();
    let world_pos = ev.coord.to_world_pos();
    let (mesh, material, offset, collider) = match ev.building_type {
        BuildingType::Belt(_) => (
            assets.belt_mesh.clone(),
            assets.belt_material.clone(),
            Vec3::new(0.0, 0.08, 0.0),
            None,
        ),
        BuildingType::ViaductBelt(_) => (
            assets.viaduct_mesh.clone(),
            assets.viaduct_material.clone(),
            Vec3::new(0.0, 0.08, 0.0),
            None,
        ),
        BuildingType::InclineRamp { .. } => (
            assets.ramp_mesh.clone(),
            assets.belt_material.clone(),
            Vec3::new(0.0, 0.1, 0.0),
            None,
        ),
        BuildingType::SwivelTurntable { .. } => (
            assets.belt_mesh.clone(),
            assets.wellhead_material.clone(),
            Vec3::new(0.0, 0.08, 0.0),
            None,
        ),
        BuildingType::SortingGate { .. } => (
            assets.chest_mesh.clone(),
            assets.belt_material.clone(),
            Vec3::new(0.0, 0.2, 0.0),
            None,
        ),
        BuildingType::CargoCompressor(_) => (
            assets.compressor_mesh.clone(),
            assets.compressor_material.clone(),
            Vec3::new(0.0, 0.5, 0.0),
            Some(Collider::cuboid(0.8, 1.0, 0.8)),
        ),
        BuildingType::SurveyBeacon => (
            assets.beacon_mesh.clone(),
            assets.beacon_material.clone(),
            Vec3::new(0.0, 0.6, 0.0),
            Some(Collider::cylinder(0.3, 1.2)),
        ),
        BuildingType::LogicDiviner => (
            assets.diviner_mesh.clone(),
            assets.diviner_material.clone(),
            Vec3::new(0.0, 0.5, 0.0),
            Some(Collider::cylinder(0.35, 0.9)),
        ),
        BuildingType::Wellhead => (
            assets.wellhead_mesh.clone(),
            assets.wellhead_material.clone(),
            Vec3::new(0.0, 0.4, 0.0),
            Some(Collider::cylinder(0.35, 0.8)),
        ),
        BuildingType::SmeltingHearth => (
            assets.hearth_mesh.clone(),
            assets.hearth_material.clone(),
            Vec3::new(0.0, 0.25, 0.0),
            Some(Collider::cuboid(0.7, 0.5, 0.7)),
        ),
        BuildingType::StorageChest => (
            assets.chest_mesh.clone(),
            assets.chest_material.clone(),
            Vec3::new(0.0, 0.3, 0.0),
            Some(Collider::cuboid(0.6, 0.6, 0.6)),
        ),
        BuildingType::ChromaticAutoclave(_) => (
            assets.autoclave_mesh.clone(),
            assets.autoclave_material.clone(),
            Vec3::new(0.0, 0.48, 0.0),
            Some(Collider::sphere(0.48)),
        ),
        BuildingType::ResonanceBellJar => (
            assets.bell_jar_mesh.clone(),
            assets.bell_jar_material.clone(),
            Vec3::new(0.0, 0.35, 0.0),
            Some(Collider::cylinder(0.38, 0.7)),
        ),
        BuildingType::AcousticMonolith => (
            assets.monolith_mesh.clone(),
            assets.monolith_material.clone(),
            Vec3::new(0.0, 0.75, 0.0),
            Some(Collider::cuboid(0.7, 1.5, 0.7)),
        ),
        BuildingType::TuningForkPylon => (
            assets.pylon_mesh.clone(),
            assets.pylon_material.clone(),
            Vec3::new(0.0, 0.7, 0.0),
            Some(Collider::cylinder(0.25, 1.4)),
        ),
        BuildingType::HarmonicBridge(_) => (
            assets.viaduct_mesh.clone(),
            assets.harmonic_bridge_material.clone(),
            Vec3::new(0.0, 0.08, 0.0),
            None,
        ),
        BuildingType::ResonanceConductor => (
            assets.conductor_mesh.clone(),
            assets.conductor_material.clone(),
            Vec3::new(0.0, 0.4, 0.0),
            Some(Collider::sphere(0.4)),
        ),
    };

    let mut entity_cmds = commands.entity(ev.entity);
    entity_cmds.insert((
        DespawnOnExit(Screen::Gameplay),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(world_pos + offset),
    ));

    if let Some(col) = collider {
        entity_cmds.insert((RigidBody::Static, col));
    }
}

/// Spawns or updates 3D visual representations of transported items on belts.
pub fn sync_transported_item_visuals(
    items: Query<(Entity, &TransportedItem), Without<Mesh3d>>,
    mut moving_items: Query<(&TransportedItem, &mut Transform), With<Mesh3d>>,
    belts: Query<(&ConveyorBelt, &Building)>,
    assets: Option<Res<DioramaAssets>>,
    mut commands: Commands,
) {
    let Some(assets) = assets.as_deref() else {
        return;
    };

    let mut belts_by_coord = HashMap::new();
    for (belt, building) in belts.iter() {
        belts_by_coord.insert(building.coord, belt.direction);
    }

    // Attach mesh & material to newly spawned items
    for (entity, item) in items.iter() {
        let dir = belts_by_coord.get(&item.current_coord).copied().unwrap_or(Direction::North);
        let pos = item.world_pos(dir);

        let mat = match item.item_type {
            ItemType::IronOre => assets.iron_item_material.clone(),
            ItemType::CopperOre => assets.copper_item_material.clone(),
            ItemType::IngotCube => assets.ingot_cube_material.clone(),
            _ if item.item_type.is_polyomino() => assets.polyomino_material.clone(),
            _ => assets.plate_item_material.clone(),
        };

        commands.entity(entity).insert((
            DespawnOnExit(Screen::Gameplay),
            Mesh3d(assets.item_mesh.clone()),
            MeshMaterial3d(mat),
            Transform::from_translation(pos),
        ));
    }

    // Update position of existing items
    for (item, mut transform) in moving_items.iter_mut() {
        let dir = belts_by_coord.get(&item.current_coord).copied().unwrap_or(Direction::North);
        transform.translation = item.world_pos(dir);
    }
}

/// Observer that updates tile visuals when prospect state changes.
pub fn on_tile_state_changed(
    on: On<TileStateChangedEvent>,
    mut visual_tiles: Query<(&VisualTileMarker, &mut MeshMaterial3d<StandardMaterial>)>,
    assets: Option<Res<DioramaAssets>>,
) {
    let Some(assets) = assets.as_deref() else {
        return;
    };

    let ev = on.event();
    for (tile_marker, mut mat) in visual_tiles.iter_mut() {
        if tile_marker.coord == ev.coord {
            mat.0 = match ev.new_state {
                TileProspectState::Covered => assets.grass_material.clone(),
                TileProspectState::Revealed | TileProspectState::Extracted => {
                    match ev.adjacent_hazards {
                        0 => assets.revealed_material.clone(),
                        1 => assets.hazard_1_material.clone(),
                        2 => assets.hazard_2_material.clone(),
                        _ => assets.hazard_3_plus_material.clone(),
                    }
                }
                TileProspectState::Flagged => assets.flagged_material.clone(),
            };
            break;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<DioramaAssets>()
        .add_systems(OnEnter(Screen::Gameplay), setup_cradle_meadows)
        .add_observer(on_building_placed)
        .add_observer(on_tile_state_changed)
        .add_systems(
            Update,
            (setup_player_inventory, sync_transported_item_visuals)
                .run_if(in_state(Screen::Gameplay)),
        );
}
