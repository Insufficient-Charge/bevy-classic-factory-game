use bevy::prelude::*;
use super::autoclave::{ChromaticAutoclave, ResonanceBellJar};
use super::grid::{Direction, GridCoord};
use super::harmonic::{AcousticMonolith, HarmonicBridge, HarmonicTuning, ResonanceConductor, TuningForkPylon};
use super::inventory::Inventory;
use super::items::ItemType;
use super::logistics::{
    CargoCompressor, ClockworkWellhead, ConveyorBelt, InclineRamp, LogicDiviner,
    SmeltingHearth, SortingGate, StorageChest, SurveyBeacon, SwivelTurntable,
};
use super::mining::ResourceDeposit;
use super::terrain::{IslandGrid, TileProspectState};

/// Types of structures that can be placed on the factory grid (GDD §8.1 & §8.2).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum BuildingType {
    Belt(Direction),
    ViaductBelt(Direction),
    InclineRamp { direction: Direction, upward: bool },
    SwivelTurntable { rotation: Direction },
    SortingGate { filter_item: ItemType, straight_dir: Direction, divert_dir: Direction },
    CargoCompressor(Direction),
    SurveyBeacon,
    LogicDiviner,
    Wellhead,
    SmeltingHearth,
    StorageChest,
    ChromaticAutoclave(Direction),
    ResonanceBellJar,
    AcousticMonolith,
    TuningForkPylon,
    HarmonicBridge(Direction),
    ResonanceConductor,
}

impl BuildingType {
    /// The inventory item required to place this building.
    pub const fn required_item(&self) -> ItemType {
        match self {
            Self::Belt(_) => ItemType::WoodenRollerBelt,
            Self::ViaductBelt(_) => ItemType::ViaductBelt,
            Self::InclineRamp { .. } => ItemType::InclineRamp,
            Self::SwivelTurntable { .. } => ItemType::SwivelTurntable,
            Self::SortingGate { .. } => ItemType::SortingGate,
            Self::CargoCompressor(_) => ItemType::CargoCompressor,
            Self::SurveyBeacon => ItemType::SurveyBeacon,
            Self::LogicDiviner => ItemType::LogicDiviner,
            Self::Wellhead => ItemType::ClockworkWellhead,
            Self::SmeltingHearth => ItemType::SmeltingHearth,
            Self::StorageChest => ItemType::StorageChest,
            Self::ChromaticAutoclave(_) => ItemType::ChromaticAutoclave,
            Self::ResonanceBellJar => ItemType::ResonanceBellJar,
            Self::AcousticMonolith => ItemType::AcousticMonolith,
            Self::TuningForkPylon => ItemType::TuningForkPylon,
            Self::HarmonicBridge(_) => ItemType::HarmonicBridge,
            Self::ResonanceConductor => ItemType::ResonanceConductor,
        }
    }
}

/// A placed building entity on the diorama island.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub coord: GridCoord,
    pub building_type: BuildingType,
}

impl Building {
    pub const fn new(coord: GridCoord, building_type: BuildingType) -> Self {
        Self {
            coord,
            building_type,
        }
    }
}

/// Request to place a building on the grid.
#[derive(Event, Clone, Debug)]
pub struct PlaceBuildingEvent {
    pub player_entity: Entity,
    pub coord: GridCoord,
    pub building_type: BuildingType,
}

/// Request to dismantle a building on the grid.
#[derive(Event, Clone, Debug)]
pub struct DismantleBuildingEvent {
    pub player_entity: Entity,
    pub coord: GridCoord,
}

/// Emitted when a building is successfully placed.
#[derive(Event, Clone, Debug)]
pub struct BuildingPlacedEvent {
    pub entity: Entity,
    pub coord: GridCoord,
    pub building_type: BuildingType,
}

/// Emitted when a building is dismantled.
#[derive(Event, Clone, Debug)]
pub struct BuildingDismantledEvent {
    pub coord: GridCoord,
    pub building_type: BuildingType,
}

/// Observer that handles building placement validation and spawning.
pub fn on_place_building(
    on: On<PlaceBuildingEvent>,
    mut players: Query<(&Transform, &mut Inventory)>,
    mut island_grid: ResMut<IslandGrid>,
    buildings: Query<&Building>,
    deposits: Query<&ResourceDeposit>,
    mut commands: Commands,
) {
    let ev = on.event();
    let Ok((_player_transform, mut inventory)) = players.get_mut(ev.player_entity) else {
        return;
    };

    // 1. Tile must exist on ground level in IslandGrid
    let ground_coord = GridCoord::ground(ev.coord.x, ev.coord.y);
    let Some(tile) = island_grid.tiles.get(&ground_coord) else {
        warn!("Cannot place building: tile {:?} does not exist", ev.coord);
        return;
    };

    // 2. Ground tile must be revealed or extracted (cannot build above covered/flagged hazards)
    if tile.prospect_state != TileProspectState::Revealed
        && tile.prospect_state != TileProspectState::Extracted
    {
        warn!("Cannot place building: ground tile {:?} is not revealed ({:?})", ground_coord, tile.prospect_state);
        return;
    }

    // 3. Tile coordinate (including Layer) must not already have a building
    if buildings.iter().any(|b| b.coord == ev.coord) {
        warn!("Cannot place building: tile {:?} is already occupied", ev.coord);
        return;
    }

    // 4. Wellhead requires a ResourceDeposit on the tile
    if ev.building_type == BuildingType::Wellhead {
        let has_deposit = deposits.iter().any(|d| d.coord == ground_coord && !d.is_depleted());
        if !has_deposit {
            warn!("Cannot place Wellhead: tile {:?} has no resource deposit", ground_coord);
            return;
        }
    }

    // 5. Player must have the required building item
    let needed_item = ev.building_type.required_item();
    if !inventory.remove_item(needed_item, 1) {
        warn!("Cannot place building: player lacks item {:?}", needed_item);
        return;
    }

    // 6. Spawn building entity with specific mechanics components
    let mut entity_cmds = commands.spawn((
        Building::new(ev.coord, ev.building_type),
        HarmonicTuning::default(),
    ));

    match ev.building_type {
        BuildingType::Belt(dir) | BuildingType::ViaductBelt(dir) => {
            entity_cmds.insert(ConveyorBelt::new(dir));
        }
        BuildingType::InclineRamp { direction, upward } => {
            entity_cmds.insert(InclineRamp::new(direction, upward));
        }
        BuildingType::SwivelTurntable { rotation } => {
            entity_cmds.insert(SwivelTurntable::new(rotation));
        }
        BuildingType::SortingGate { filter_item, straight_dir, divert_dir } => {
            entity_cmds.insert(SortingGate::new(filter_item, straight_dir, divert_dir));
        }
        BuildingType::CargoCompressor(dir) => {
            entity_cmds.insert(CargoCompressor::new(dir));
        }
        BuildingType::SurveyBeacon => {
            entity_cmds.insert(SurveyBeacon::default());
        }
        BuildingType::LogicDiviner => {
            entity_cmds.insert(LogicDiviner::default());
        }
        BuildingType::Wellhead => {
            entity_cmds.insert(ClockworkWellhead::default());
            if let Some(t) = island_grid.tiles.get_mut(&ground_coord) {
                t.prospect_state = TileProspectState::Extracted;
            }
        }
        BuildingType::SmeltingHearth => {
            entity_cmds.insert(SmeltingHearth::default());
        }
        BuildingType::StorageChest => {
            entity_cmds.insert(StorageChest::default());
        }
        BuildingType::ChromaticAutoclave(dir) => {
            entity_cmds.insert(ChromaticAutoclave::new(dir));
        }
        BuildingType::ResonanceBellJar => {
            entity_cmds.insert(ResonanceBellJar::default());
        }
        BuildingType::AcousticMonolith => {
            entity_cmds.insert(AcousticMonolith::default());
        }
        BuildingType::TuningForkPylon => {
            entity_cmds.insert(TuningForkPylon::default());
        }
        BuildingType::HarmonicBridge(_) => {
            entity_cmds.insert(HarmonicBridge);
        }
        BuildingType::ResonanceConductor => {
            entity_cmds.insert(ResonanceConductor::default());
        }
    }

    let entity = entity_cmds.id();
    commands.trigger(BuildingPlacedEvent {
        entity,
        coord: ev.coord,
        building_type: ev.building_type,
    });
}

/// Observer that handles building dismantling and material refund.
pub fn on_dismantle_building(
    on: On<DismantleBuildingEvent>,
    mut players: Query<&mut Inventory>,
    buildings: Query<(Entity, &Building, Option<&ConveyorBelt>)>,
    mut island_grid: ResMut<IslandGrid>,
    mut commands: Commands,
) {
    let ev = on.event();
    let Ok(mut inventory) = players.get_mut(ev.player_entity) else {
        return;
    };

    let Some((entity, building, maybe_belt)) = buildings.iter().find(|(_, b, _)| b.coord == ev.coord) else {
        return;
    };

    let refunded_item = building.building_type.required_item();
    let btype = building.building_type;

    // Refund building item to player satchel
    inventory.add_item(refunded_item, 1);

    // Despawn any item held on belt
    if let Some(belt) = maybe_belt {
        if let Some(held) = belt.held_item {
            commands.entity(held).despawn();
        }
    }

    // If wellhead was dismantled, restore tile to Revealed
    if btype == BuildingType::Wellhead {
        let ground_coord = GridCoord::ground(ev.coord.x, ev.coord.y);
        if let Some(t) = island_grid.tiles.get_mut(&ground_coord) {
            t.prospect_state = TileProspectState::Revealed;
        }
    }

    commands.entity(entity).despawn();
    commands.trigger(BuildingDismantledEvent {
        coord: ev.coord,
        building_type: btype,
    });
}

pub fn plugin(app: &mut App) {
    app.register_type::<Building>()
        .register_type::<BuildingType>()
        .add_observer(on_place_building)
        .add_observer(on_dismantle_building);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::grid::Layer;

    #[test]
    fn test_required_item_mappings() {
        assert_eq!(BuildingType::Belt(Direction::North).required_item(), ItemType::WoodenRollerBelt);
        assert_eq!(BuildingType::ViaductBelt(Direction::East).required_item(), ItemType::ViaductBelt);
        assert_eq!(BuildingType::InclineRamp { direction: Direction::North, upward: true }.required_item(), ItemType::InclineRamp);
        assert_eq!(BuildingType::SwivelTurntable { rotation: Direction::North }.required_item(), ItemType::SwivelTurntable);
        assert_eq!(BuildingType::CargoCompressor(Direction::North).required_item(), ItemType::CargoCompressor);
        assert_eq!(BuildingType::SurveyBeacon.required_item(), ItemType::SurveyBeacon);
        assert_eq!(BuildingType::LogicDiviner.required_item(), ItemType::LogicDiviner);
        assert_eq!(BuildingType::Wellhead.required_item(), ItemType::ClockworkWellhead);
        assert_eq!(BuildingType::SmeltingHearth.required_item(), ItemType::SmeltingHearth);
        assert_eq!(BuildingType::StorageChest.required_item(), ItemType::StorageChest);
        assert_eq!(BuildingType::ChromaticAutoclave(Direction::North).required_item(), ItemType::ChromaticAutoclave);
        assert_eq!(BuildingType::ResonanceBellJar.required_item(), ItemType::ResonanceBellJar);
        assert_eq!(BuildingType::AcousticMonolith.required_item(), ItemType::AcousticMonolith);
        assert_eq!(BuildingType::TuningForkPylon.required_item(), ItemType::TuningForkPylon);
        assert_eq!(BuildingType::HarmonicBridge(Direction::North).required_item(), ItemType::HarmonicBridge);
        assert_eq!(BuildingType::ResonanceConductor.required_item(), ItemType::ResonanceConductor);
    }

    #[test]
    fn test_multi_layer_building_coexistence() {
        let mut app = App::new();
        app.add_observer(on_place_building);

        let mut grid = IslandGrid::new();
        let ground = GridCoord::ground(2, 2);
        grid.add_tile(ground, false);
        grid.tiles.get_mut(&ground).unwrap().prospect_state = TileProspectState::Revealed;
        app.insert_resource(grid);

        let player_entity = app
            .world_mut()
            .spawn((
                Transform::default(),
                Inventory::new(10),
            ))
            .id();

        // Give player 1 ground belt and 1 viaduct belt
        {
            let mut inv = app.world_mut().get_mut::<Inventory>(player_entity).unwrap();
            inv.add_item(ItemType::WoodenRollerBelt, 1);
            inv.add_item(ItemType::ViaductBelt, 1);
        }

        // Place ground belt at (2, 2, Ground)
        app.world_mut().commands().trigger(PlaceBuildingEvent {
            player_entity,
            coord: ground,
            building_type: BuildingType::Belt(Direction::North),
        });
        app.update();

        // Place viaduct belt at (2, 2, Viaduct) directly above ground belt
        let viaduct_coord = GridCoord::new(2, 2, Layer::Viaduct);
        app.world_mut().commands().trigger(PlaceBuildingEvent {
            player_entity,
            coord: viaduct_coord,
            building_type: BuildingType::ViaductBelt(Direction::East),
        });
        app.update();

        // Both buildings should successfully exist!
        let buildings: Vec<Building> = app.world_mut().query::<&Building>().iter(app.world()).copied().collect();
        assert_eq!(buildings.len(), 2);
        assert!(buildings.iter().any(|b| b.coord == ground));
        assert!(buildings.iter().any(|b| b.coord == viaduct_coord));
    }
}
