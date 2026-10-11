use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::screens::Screen;
use super::building::{Building, BuildingType, DismantleBuildingEvent, PlaceBuildingEvent};
use super::crafting::{StartCraftEvent, find_recipe};
use super::grid::{Direction, GridCoord, Layer};
use super::inventory::{Hotbar, Inventory};
use super::items::ItemType;
use super::mining::{MineTileEvent, ResourceDeposit};
use super::prospecting::{ProspectEvent, ProspectMode};
use super::terrain::{IslandGrid, TileProspectState};

/// System that handles user keyboard and mouse input for interacting with the factory world:
/// - Left-Click / F / Space:
///   1. Mine deposit if targeted deposit is within reach.
///   2. Prospect / unfold tile if targeted tile is covered.
///   3. Place building if active hotbar slot has a structure and materials.
/// - Right-Click / R:
///   1. Dismantle building if targeted tile has a building.
///   2. Flag / unflag tile (Minesweeper) if tile is covered or flagged.
/// - Key X: Dismantle building at targeted tile.
/// - Key C / Tab: Quick craft available recipes in satchel.
pub fn handle_player_interactions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::camera::SceneCamera>>,
    players: Query<
        (Entity, &Transform, &Hotbar, &Inventory),
        With<crate::player::Player>,
    >,
    island_grid: Res<IslandGrid>,
    deposits: Query<&ResourceDeposit>,
    buildings: Query<&Building>,
    mut commands: Commands,
) {
    let Ok((player_entity, player_transform, hotbar, inventory)) = players.single() else {
        return;
    };

    let forward = player_transform.forward();
    let facing_dir = Direction::from_vector(Vec3::new(forward.x, 0.0, forward.z));

    // Determine targeted grid coordinate
    // Priority: mouse raycast on Y=0 plane if cursor is within reach, else front of player
    let mut target_coord = {
        let front_pos = player_transform.translation + Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero() * 1.2;
        GridCoord::from_world_pos(front_pos, Layer::Ground)
    };

    if let Ok(window) = windows.single() {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, cam_transform)) = camera_q.single() {
                if let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) {
                    if ray.direction.y.abs() > 0.0001 {
                        let t = -ray.origin.y / ray.direction.y;
                        if t > 0.0 {
                            let hit = ray.origin + ray.direction * t;
                            let cursor_coord = GridCoord::from_world_pos(hit, Layer::Ground);
                            if player_transform.translation.distance(cursor_coord.to_world_pos()) <= 3.5 {
                                target_coord = cursor_coord;
                            }
                        }
                    }
                }
            }
        }
    }

    let is_interact_pressed = mouse.just_pressed(MouseButton::Left)
        || keyboard.just_pressed(KeyCode::KeyF)
        || keyboard.just_pressed(KeyCode::Space);

    let is_secondary_pressed = mouse.just_pressed(MouseButton::Right)
        || keyboard.just_pressed(KeyCode::KeyR);

    let is_dismantle_pressed = keyboard.just_pressed(KeyCode::KeyX);

    let is_craft_pressed = keyboard.just_pressed(KeyCode::KeyC)
        || keyboard.just_pressed(KeyCode::Tab);

    // 1. Primary interaction: Mine -> Prospect -> Place
    if is_interact_pressed {
        // Check if there is an un-depleted resource deposit at target coord
        let has_deposit = deposits.iter().any(|d| d.coord == target_coord && !d.is_depleted());
        if has_deposit {
            commands.trigger(MineTileEvent {
                player_entity,
                coord: target_coord,
            });
            return;
        }

        // Check if target tile is covered
        if let Some(tile) = island_grid.tiles.get(&target_coord) {
            if tile.prospect_state == TileProspectState::Covered {
                commands.trigger(ProspectEvent {
                    player_entity,
                    coord: target_coord,
                    mode: ProspectMode::Unfold,
                });
                return;
            }
        }

        // Check if hotbar has a placeable building
        if hotbar.selected_slot < hotbar.slots.len() {
            if let Some(item_type) = hotbar.slots[hotbar.selected_slot] {
                if inventory.count_item(item_type) > 0 {
                    let building_type = match item_type {
                        ItemType::WoodenRollerBelt => Some(BuildingType::Belt(facing_dir)),
                        ItemType::ViaductBelt => Some(BuildingType::ViaductBelt(facing_dir)),
                        ItemType::InclineRamp => Some(BuildingType::InclineRamp {
                            direction: facing_dir,
                            upward: true,
                        }),
                        ItemType::SwivelTurntable => Some(BuildingType::SwivelTurntable {
                            rotation: facing_dir,
                        }),
                        ItemType::SortingGate => Some(BuildingType::SortingGate {
                            filter_item: ItemType::IronOre,
                            straight_dir: facing_dir,
                            divert_dir: facing_dir.rotate_cw(),
                        }),
                        ItemType::CargoCompressor => Some(BuildingType::CargoCompressor(facing_dir)),
                        ItemType::SurveyBeacon => Some(BuildingType::SurveyBeacon),
                        ItemType::LogicDiviner => Some(BuildingType::LogicDiviner),
                        ItemType::ClockworkWellhead => Some(BuildingType::Wellhead),
                        ItemType::SmeltingHearth => Some(BuildingType::SmeltingHearth),
                        ItemType::StorageChest => Some(BuildingType::StorageChest),
                        _ => None,
                    };

                    if let Some(btype) = building_type {
                        commands.trigger(PlaceBuildingEvent {
                            player_entity,
                            coord: target_coord,
                            building_type: btype,
                        });
                        return;
                    }
                }
            }
        }
    }

    // 2. Secondary interaction: Flag or Dismantle
    if is_secondary_pressed {
        // If building exists at coord, dismantle it
        let has_building = buildings.iter().any(|b| b.coord == target_coord);
        if has_building {
            commands.trigger(DismantleBuildingEvent {
                player_entity,
                coord: target_coord,
            });
            return;
        }

        // Else toggle flag on tile
        if let Some(tile) = island_grid.tiles.get(&target_coord) {
            if tile.prospect_state == TileProspectState::Covered
                || tile.prospect_state == TileProspectState::Flagged
            {
                commands.trigger(ProspectEvent {
                    player_entity,
                    coord: target_coord,
                    mode: ProspectMode::Flag,
                });
                return;
            }
        }
    }

    // 3. Dismantle Key (X)
    if is_dismantle_pressed {
        commands.trigger(DismantleBuildingEvent {
            player_entity,
            coord: target_coord,
        });
        return;
    }

    // 4. Quick Craft Key (C / Tab)
    if is_craft_pressed {
        let quick_recipes = [
            "smelt_iron_plate",
            "smelt_copper_plate",
            "craft_roller_belt",
            "craft_viaduct_belt",
            "craft_brass_cog",
            "craft_incline_ramp",
            "craft_wooden_plank",
        ];

        for recipe_id in quick_recipes {
            if let Some(recipe) = find_recipe(recipe_id) {
                if inventory.has_items(recipe.inputs) {
                    commands.trigger(StartCraftEvent {
                        player_entity,
                        recipe_id: recipe_id.to_string(),
                    });
                    break;
                }
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_player_interactions.run_if(in_state(Screen::Gameplay)),
    );
}
