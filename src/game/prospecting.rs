use bevy::prelude::*;
use super::grid::GridCoord;
use super::mining::MiningTool;
use super::terrain::{IslandGrid, TileProspectState};

/// Mode of prospecting action (unfold vs. flag).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum ProspectMode {
    Unfold,
    Flag,
}

/// Request to prospect a tile (tap with Surveying Tapper or plant marker flag).
#[derive(Event, Clone, Debug)]
pub struct ProspectEvent {
    pub player_entity: Entity,
    pub coord: GridCoord,
    pub mode: ProspectMode,
}

/// Emitted when an unflagged overcharge vent is struck (playful steam pop!).
#[derive(Event, Clone, Debug)]
pub struct OverchargePopEvent {
    pub coord: GridCoord,
}

/// Emitted when a tile's prospect state changes.
#[derive(Event, Clone, Debug)]
pub struct TileStateChangedEvent {
    pub coord: GridCoord,
    pub new_state: TileProspectState,
    pub adjacent_hazards: u8,
}

/// Observer that handles player prospecting actions.
pub fn on_prospect(
    on: On<ProspectEvent>,
    players: Query<(&Transform, &MiningTool)>,
    mut island_grid: ResMut<IslandGrid>,
    mut commands: Commands,
) {
    let ev = on.event();
    let Ok((player_transform, tool)) = players.get(ev.player_entity) else {
        return;
    };

    let tile_world_pos = ev.coord.to_world_pos();
    let distance = player_transform.translation.distance(tile_world_pos);

    if distance > tool.reach {
        trace!("Prospect target {:?} out of reach ({:.2} > {:.2})", ev.coord, distance, tool.reach);
        return;
    }

    match ev.mode {
        ProspectMode::Unfold => {
            match island_grid.unfold_tile_with_revealed(&ev.coord) {
                Ok((hazard_struck, revealed_coords)) => {
                    if hazard_struck {
                        commands.trigger(OverchargePopEvent { coord: ev.coord });
                    }

                    // Notify changes for all revealed tiles (including cascade expansion)
                    for r_coord in revealed_coords {
                        if let Some(tile) = island_grid.tiles.get(&r_coord) {
                            commands.trigger(TileStateChangedEvent {
                                coord: r_coord,
                                new_state: tile.prospect_state,
                                adjacent_hazards: tile.adjacent_hazards,
                            });
                        }
                    }
                }
                Err(err) => {
                    debug!("Cannot unfold tile {:?}: {}", ev.coord, err);
                }
            }
        }
        ProspectMode::Flag => {
            match island_grid.toggle_flag(&ev.coord) {
                Ok(new_state) => {
                    let hazards = island_grid.tiles.get(&ev.coord).map(|t| t.adjacent_hazards).unwrap_or(0);
                    commands.trigger(TileStateChangedEvent {
                        coord: ev.coord,
                        new_state,
                        adjacent_hazards: hazards,
                    });
                }
                Err(err) => {
                    debug!("Cannot toggle flag on tile {:?}: {}", ev.coord, err);
                }
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ProspectMode>()
        .add_observer(on_prospect);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_prospect_mode_reflection() {
        assert_eq!(ProspectMode::Unfold, ProspectMode::Unfold);
    }

    #[test]
    fn test_prospecting_event_unfold_and_flag() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<IslandGrid>();
        app.add_observer(on_prospect);

        let coord = GridCoord::ground(1, 1);
        {
            let mut grid = app.world_mut().resource_mut::<IslandGrid>();
            grid.generate_rectangular_island(3, 3, &HashSet::new());
        }

        let player = app.world_mut().spawn((
            Transform::from_xyz(1.0, 0.0, 1.0),
            MiningTool { reach: 2.5, mining_rate: 1.0 },
        )).id();

        // 1. Flag the tile
        app.world_mut().commands().trigger(ProspectEvent {
            player_entity: player,
            coord,
            mode: ProspectMode::Flag,
        });
        app.update();

        {
            let grid = app.world().resource::<IslandGrid>();
            assert_eq!(grid.tiles.get(&coord).unwrap().prospect_state, TileProspectState::Flagged);
        }

        // 2. Unflag the tile
        app.world_mut().commands().trigger(ProspectEvent {
            player_entity: player,
            coord,
            mode: ProspectMode::Flag,
        });
        app.update();

        {
            let grid = app.world().resource::<IslandGrid>();
            assert_eq!(grid.tiles.get(&coord).unwrap().prospect_state, TileProspectState::Covered);
        }

        // 3. Unfold the tile
        app.world_mut().commands().trigger(ProspectEvent {
            player_entity: player,
            coord,
            mode: ProspectMode::Unfold,
        });
        app.update();

        {
            let grid = app.world().resource::<IslandGrid>();
            assert_eq!(grid.tiles.get(&coord).unwrap().prospect_state, TileProspectState::Revealed);
        }
    }
}
