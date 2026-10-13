use bevy::prelude::*;
use std::collections::HashMap;
use super::autoclave::{CatalysisEvent, ChromaticAutoclave, ResonanceBellJar};
use super::grid::{Direction, GridCoord};
use super::inventory::Inventory;
use super::items::ItemType;
use super::mining::ResourceDeposit;
use super::polyomino::{CargoCompressorGrid, PistonSquishEvent, PolyominoShape, PolyominoShard};
use super::terrain::{IslandGrid, TileProspectState};
use super::prospecting::{OverchargePopEvent, TileStateChangedEvent};

/// A conveyor belt segment running on a single grid tile.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt {
    pub direction: Direction,
    pub speed: f32, // tiles per second
    pub held_item: Option<Entity>,
}

impl Default for ConveyorBelt {
    fn default() -> Self {
        Self {
            direction: Direction::North,
            speed: 1.0,
            held_item: None,
        }
    }
}

impl ConveyorBelt {
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            speed: 1.0,
            held_item: None,
        }
    }
}

/// Conveyor incline ramp that transfers items vertically between layers (GDD §5.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct InclineRamp {
    pub direction: Direction,
    pub upward: bool, // true: ascends (e.g. Z0 -> Z1), false: descends (Z1 -> Z0)
    pub speed: f32,
    pub held_item: Option<Entity>,
}

impl InclineRamp {
    pub const fn new(direction: Direction, upward: bool) -> Self {
        Self {
            direction,
            upward,
            speed: 1.0,
            held_item: None,
        }
    }
}

/// Swivel turntable plate that rotates passing polyomino crystals by 90° (GDD §5.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct SwivelTurntable {
    pub rotation: Direction,
    pub speed: f32,
    pub held_item: Option<Entity>,
}

impl SwivelTurntable {
    pub const fn new(rotation: Direction) -> Self {
        Self {
            rotation,
            speed: 1.0,
            held_item: None,
        }
    }
}

/// Saloon-door sorting gate that diverts items based on a filter condition (GDD §5.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct SortingGate {
    pub filter_item: ItemType,
    pub straight_dir: Direction,
    pub divert_dir: Direction,
    pub speed: f32,
    pub held_item: Option<Entity>,
}

impl SortingGate {
    pub const fn new(filter_item: ItemType, straight_dir: Direction, divert_dir: Direction) -> Self {
        Self {
            filter_item,
            straight_dir,
            divert_dir,
            speed: 1.0,
            held_item: None,
        }
    }
}

/// Vertical Cargo Compressor housing a 4x10 glass hopper (GDD §5.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CargoCompressor {
    pub facing: Direction,
    pub grid: CargoCompressorGrid,
    pub output_timer: Timer,
}

impl CargoCompressor {
    pub fn new(facing: Direction) -> Self {
        Self {
            facing,
            grid: CargoCompressorGrid::new(),
            output_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

/// Automated Timber Survey Beacon that auto-unfolds adjacent tiles (GDD §5.1).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct SurveyBeacon {
    pub timer: Timer,
    pub radius: i32,
}

impl Default for SurveyBeacon {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(8.0, TimerMode::Repeating),
            radius: 1,
        }
    }
}

/// Harmonic Logic Diviner that computes safe hazard deductions and flags vents (GDD §5.1).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct LogicDiviner {
    pub timer: Timer,
    pub radius: i32,
}

impl Default for LogicDiviner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            radius: 1,
        }
    }
}

/// An item physically being carried along conveyor belts.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct TransportedItem {
    pub item_type: ItemType,
    pub current_coord: GridCoord,
    pub progress: f32, // 0.0 to 1.0 along the current tile
}

impl TransportedItem {
    pub const fn new(item_type: ItemType, current_coord: GridCoord) -> Self {
        Self {
            item_type,
            current_coord,
            progress: 0.0,
        }
    }

    /// Computes the 3D world position of this item along a standard flat belt.
    pub fn world_pos(&self, belt_dir: Direction) -> Vec3 {
        let base = self.current_coord.to_world_pos();
        let forward = belt_dir.to_world_vector();
        let offset = forward * (self.progress - 0.5);
        base + offset + Vec3::new(0.0, 0.3, 0.0)
    }

    /// Computes the 3D world position of this item along an incline ramp.
    pub fn world_pos_ramp(&self, ramp_dir: Direction, upward: bool) -> Vec3 {
        let base = self.current_coord.to_world_pos();
        let forward = ramp_dir.to_world_vector();
        let offset = forward * (self.progress - 0.5);
        let height_delta = if upward { 2.0 } else { -2.0 } * self.progress;
        base + offset + Vec3::new(0.0, 0.3 + height_delta, 0.0)
    }
}

/// Automated clockwork wellhead that pumps ore from an underlying resource deposit.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ClockworkWellhead {
    pub timer: Timer,
    pub extraction_interval: f32,
}

impl Default for ClockworkWellhead {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            extraction_interval: 2.0,
        }
    }
}

/// Smelting hearth that automatically refines raw ore into metal plates.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct SmeltingHearth {
    pub timer: Timer,
    pub input_ore: Option<ItemType>,
    pub output_plate: Option<ItemType>,
}

impl Default for SmeltingHearth {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            input_ore: None,
            output_plate: None,
        }
    }
}

/// Storage chest that accepts items from conveyors and players.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct StorageChest {
    pub inventory: Inventory,
}

impl Default for StorageChest {
    fn default() -> Self {
        Self {
            inventory: Inventory::new(100),
        }
    }
}

/// Event emitted when an item is transferred between tiles.
#[derive(Event, Clone, Debug)]
pub struct ItemMovedEvent {
    pub item_entity: Entity,
    pub from: GridCoord,
    pub to: GridCoord,
}

/// System that advances transported items along conveyor belts, incline ramps, and gates.
pub fn update_conveyor_items(
    time: Res<Time>,
    mut item_query: Query<(Entity, &mut TransportedItem, Option<&mut PolyominoShard>)>,
    mut belt_query: Query<(Entity, &mut ConveyorBelt, &super::building::Building)>,
    mut ramp_query: Query<(Entity, &mut InclineRamp, &super::building::Building)>,
    mut turntable_query: Query<(Entity, &mut SwivelTurntable, &super::building::Building)>,
    mut gate_query: Query<(Entity, &mut SortingGate, &super::building::Building)>,
    mut compressor_query: Query<(Entity, &mut CargoCompressor, &super::building::Building)>,
    mut chest_query: Query<(&mut StorageChest, &super::building::Building)>,
    mut hearth_query: Query<(&mut SmeltingHearth, &super::building::Building)>,
    mut autoclave_query: Query<(Entity, &mut ChromaticAutoclave, &super::building::Building)>,
    tunings: Query<&super::harmonic::HarmonicTuning>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();

    // Fast coordinate lookup maps
    let mut belts_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in belt_query.iter() {
        belts_by_coord.insert(building.coord, entity);
    }

    let mut ramps_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in ramp_query.iter() {
        ramps_by_coord.insert(building.coord, entity);
    }

    let mut turntables_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in turntable_query.iter() {
        turntables_by_coord.insert(building.coord, entity);
    }

    let mut gates_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in gate_query.iter() {
        gates_by_coord.insert(building.coord, entity);
    }

    let mut compressors_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in compressor_query.iter() {
        compressors_by_coord.insert(building.coord, entity);
    }

    let mut chests_by_coord: HashMap<GridCoord, usize> = HashMap::new();
    for (i, (_, building)) in chest_query.iter().enumerate() {
        chests_by_coord.insert(building.coord, i);
    }

    let mut hearths_by_coord: HashMap<GridCoord, usize> = HashMap::new();
    for (i, (_, building)) in hearth_query.iter().enumerate() {
        hearths_by_coord.insert(building.coord, i);
    }

    let mut autoclaves_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in autoclave_query.iter() {
        autoclaves_by_coord.insert(building.coord, entity);
    }

    for (item_entity, mut item, mut polyomino_shard) in item_query.iter_mut() {
        let current_coord = item.current_coord;

        // Determine current conveyance properties: direction, speed, and optional vertical transition
        let (speed, next_coord) = if let Some(&belt_ent) = belts_by_coord.get(&current_coord) {
            let Ok((_, b, _)) = belt_query.get(belt_ent) else { continue };
            let mult = tunings.get(belt_ent).map(|t| t.speed_multiplier()).unwrap_or(1.0);
            (b.speed * mult, current_coord.neighbor(b.direction))
        } else if let Some(&ramp_ent) = ramps_by_coord.get(&current_coord) {
            let Ok((_, ramp, _)) = ramp_query.get(ramp_ent) else { continue };
            let mult = tunings.get(ramp_ent).map(|t| t.speed_multiplier()).unwrap_or(1.0);
            let target = if ramp.upward {
                current_coord.neighbor(ramp.direction).layer_above().unwrap_or(current_coord.neighbor(ramp.direction))
            } else {
                current_coord.neighbor(ramp.direction).layer_below().unwrap_or(current_coord.neighbor(ramp.direction))
            };
            (ramp.speed * mult, target)
        } else if let Some(&turn_ent) = turntables_by_coord.get(&current_coord) {
            let Ok((_, turn, _)) = turntable_query.get(turn_ent) else { continue };
            let mult = tunings.get(turn_ent).map(|t| t.speed_multiplier()).unwrap_or(1.0);
            (turn.speed * mult, current_coord.neighbor(turn.rotation))
        } else if let Some(&gate_ent) = gates_by_coord.get(&current_coord) {
            let Ok((_, gate, _)) = gate_query.get(gate_ent) else { continue };
            let mult = tunings.get(gate_ent).map(|t| t.speed_multiplier()).unwrap_or(1.0);
            let dir = if item.item_type == gate.filter_item { gate.divert_dir } else { gate.straight_dir };
            (gate.speed * mult, current_coord.neighbor(dir))
        } else {
            continue;
        };

        if item.progress < 1.0 {
            item.progress = (item.progress + speed * dt).min(1.0);
        }

        // When item reaches the tile boundary (progress >= 1.0), attempt to move to neighbor
        if item.progress >= 1.0 {
            // Helper closure to clear item from current tile
            let clear_current_holder = |belts: &mut Query<(Entity, &mut ConveyorBelt, &super::building::Building)>,
                                        ramps: &mut Query<(Entity, &mut InclineRamp, &super::building::Building)>,
                                        turntables: &mut Query<(Entity, &mut SwivelTurntable, &super::building::Building)>,
                                        gates: &mut Query<(Entity, &mut SortingGate, &super::building::Building)>| {
                if let Some(&b_ent) = belts_by_coord.get(&current_coord) {
                    if let Ok((_, mut b, _)) = belts.get_mut(b_ent) { b.held_item = None; }
                } else if let Some(&r_ent) = ramps_by_coord.get(&current_coord) {
                    if let Ok((_, mut r, _)) = ramps.get_mut(r_ent) { r.held_item = None; }
                } else if let Some(&t_ent) = turntables_by_coord.get(&current_coord) {
                    if let Ok((_, mut t, _)) = turntables.get_mut(t_ent) { t.held_item = None; }
                } else if let Some(&g_ent) = gates_by_coord.get(&current_coord) {
                    if let Ok((_, mut g, _)) = gates.get_mut(g_ent) { g.held_item = None; }
                }
            };

            let is_leaving_turntable = turntables_by_coord.contains_key(&current_coord);

            // 1. Next tile is a Conveyor Belt
            if let Some(&next_belt_ent) = belts_by_coord.get(&next_coord) {
                let next_free = belt_query.get(next_belt_ent).map(|(_, b, _)| b.held_item.is_none()).unwrap_or(false);
                if next_free {
                    clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                    if let Ok((_, mut nxt_b, _)) = belt_query.get_mut(next_belt_ent) {
                        nxt_b.held_item = Some(item_entity);
                    }
                    if is_leaving_turntable {
                        if let Some(ref mut shard) = polyomino_shard {
                            shard.rotate_cw();
                        }
                    }
                    item.current_coord = next_coord;
                    item.progress = 0.0;
                    commands.trigger(ItemMovedEvent { item_entity, from: current_coord, to: next_coord });
                }
            }
            // 2. Next tile is an Incline Ramp
            else if let Some(&next_ramp_ent) = ramps_by_coord.get(&next_coord) {
                let next_free = ramp_query.get(next_ramp_ent).map(|(_, r, _)| r.held_item.is_none()).unwrap_or(false);
                if next_free {
                    clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                    if let Ok((_, mut nxt_r, _)) = ramp_query.get_mut(next_ramp_ent) {
                        nxt_r.held_item = Some(item_entity);
                    }
                    if is_leaving_turntable {
                        if let Some(ref mut shard) = polyomino_shard {
                            shard.rotate_cw();
                        }
                    }
                    item.current_coord = next_coord;
                    item.progress = 0.0;
                    commands.trigger(ItemMovedEvent { item_entity, from: current_coord, to: next_coord });
                }
            }
            // 3. Next tile is a Swivel Turntable
            else if let Some(&next_turn_ent) = turntables_by_coord.get(&next_coord) {
                let next_free = turntable_query.get(next_turn_ent).map(|(_, t, _)| t.held_item.is_none()).unwrap_or(false);
                if next_free {
                    clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                    if let Ok((_, mut nxt_t, _)) = turntable_query.get_mut(next_turn_ent) {
                        nxt_t.held_item = Some(item_entity);
                    }
                    if is_leaving_turntable {
                        if let Some(ref mut shard) = polyomino_shard {
                            shard.rotate_cw();
                        }
                    }
                    item.current_coord = next_coord;
                    item.progress = 0.0;
                    commands.trigger(ItemMovedEvent { item_entity, from: current_coord, to: next_coord });
                }
            }
            // 4. Next tile is a Sorting Gate
            else if let Some(&next_gate_ent) = gates_by_coord.get(&next_coord) {
                let next_free = gate_query.get(next_gate_ent).map(|(_, g, _)| g.held_item.is_none()).unwrap_or(false);
                if next_free {
                    clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                    if let Ok((_, mut nxt_g, _)) = gate_query.get_mut(next_gate_ent) {
                        nxt_g.held_item = Some(item_entity);
                    }
                    if is_leaving_turntable {
                        if let Some(ref mut shard) = polyomino_shard {
                            shard.rotate_cw();
                        }
                    }
                    item.current_coord = next_coord;
                    item.progress = 0.0;
                    commands.trigger(ItemMovedEvent { item_entity, from: current_coord, to: next_coord });
                }
            }
            // 5. Next tile is a Vertical Cargo Compressor (Tetris Squish)
            else if let Some(&next_comp_ent) = compressors_by_coord.get(&next_coord) {
                if let Ok((comp_entity, mut comp, _)) = compressor_query.get_mut(next_comp_ent) {
                    if item.item_type.is_polyomino() {
                        if is_leaving_turntable {
                            if let Some(ref mut shard) = polyomino_shard {
                                shard.rotate_cw();
                            }
                        }
                        let shape = PolyominoShape::from_item_type(item.item_type).unwrap_or(PolyominoShape::Domino);
                        let rot = polyomino_shard.as_ref().map(|s| s.rotation).unwrap_or(0);

                        if let Ok(_row) = comp.grid.drop_shape(shape, rot, 0) {
                            let cleared = comp.grid.check_and_clear_rows();
                            if cleared > 0 {
                                commands.trigger(PistonSquishEvent {
                                    compressor_entity: comp_entity,
                                    cleared_rows: cleared,
                                    produced_ingots: cleared,
                                });
                            }
                            clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                            commands.entity(item_entity).despawn();
                        }
                    }
                }
            }
            // 6. Next tile is a Storage Chest
            else if chests_by_coord.contains_key(&next_coord) {
                for (mut chest, building) in chest_query.iter_mut() {
                    if building.coord == next_coord {
                        if chest.inventory.add_item(item.item_type, 1) > 0 {
                            clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                            commands.entity(item_entity).despawn();
                        }
                        break;
                    }
                }
            }
            // 7. Next tile is a Smelting Hearth
            else if hearths_by_coord.contains_key(&next_coord) {
                for (mut hearth, building) in hearth_query.iter_mut() {
                    if building.coord == next_coord && hearth.input_ore.is_none() {
                        if matches!(item.item_type, ItemType::IronOre | ItemType::CopperOre) {
                            hearth.input_ore = Some(item.item_type);
                            clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                            commands.entity(item_entity).despawn();
                        }
                        break;
                    }
                }
            }
            // 8. Next tile is a Chromatic Autoclave (Bejeweled Catalysis)
            else if let Some(&auto_ent) = autoclaves_by_coord.get(&next_coord) {
                if let Ok((_, mut auto, _)) = autoclave_query.get_mut(auto_ent) {
                    if item.item_type.is_gemstone() || item.item_type == ItemType::IngotCube {
                        auto.input_buffer.push(item.item_type);
                        clear_current_holder(&mut belt_query, &mut ramp_query, &mut turntable_query, &mut gate_query);
                        commands.entity(item_entity).despawn();
                    }
                }
            }
        }
    }
}

/// System that ticks Cargo Compressors and dispenses finished Ingot Cubes onto exit belts.
pub fn update_cargo_compressors(
    time: Res<Time>,
    mut compressors: Query<(&mut CargoCompressor, &super::building::Building)>,
    mut belts: Query<(Entity, &mut ConveyorBelt, &super::building::Building)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let mut belts_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in belts.iter() {
        belts_by_coord.insert(building.coord, entity);
    }

    for (mut comp, building) in compressors.iter_mut() {
        comp.output_timer.tick(dt);
        if comp.output_timer.just_finished() && comp.grid.pending_ingots > 0 {
            let exit_coord = building.coord.neighbor(comp.facing);
            if let Some(&belt_ent) = belts_by_coord.get(&exit_coord) {
                if let Ok((_, mut belt, _)) = belts.get_mut(belt_ent) {
                    if belt.held_item.is_none() {
                        let item_entity = commands
                            .spawn(TransportedItem::new(ItemType::IngotCube, exit_coord))
                            .id();
                        belt.held_item = Some(item_entity);
                        comp.grid.pending_ingots -= 1;
                    }
                }
            }
        }
    }
}

/// System that ticks automated Survey Beacons and unfolds adjacent covered tiles (GDD §5.1).
pub fn update_survey_beacons(
    time: Res<Time>,
    mut beacons: Query<(&mut SurveyBeacon, &super::building::Building)>,
    mut island_grid: ResMut<IslandGrid>,
    mut commands: Commands,
) {
    let dt = time.delta();
    for (mut beacon, building) in beacons.iter_mut() {
        beacon.timer.tick(dt);
        if !beacon.timer.just_finished() {
            continue;
        }

        let r = beacon.radius;
        for dx in -r..=r {
            for dy in -r..=r {
                let target_coord = GridCoord::new(building.coord.x + dx, building.coord.y + dy, building.coord.layer);
                if let Some(tile) = island_grid.tiles.get(&target_coord) {
                    if tile.prospect_state == TileProspectState::Covered {
                        if let Ok(hazard_struck) = island_grid.unfold_tile(&target_coord) {
                            if hazard_struck {
                                commands.trigger(OverchargePopEvent { coord: target_coord });
                            } else {
                                let adj = island_grid.tiles.get(&target_coord).map(|t| t.adjacent_hazards).unwrap_or(0);
                                commands.trigger(TileStateChangedEvent {
                                    coord: target_coord,
                                    new_state: TileProspectState::Revealed,
                                    adjacent_hazards: adj,
                                });
                            }
                            return; // Unfolds one tile per pulse for satisfying pacing
                        }
                    }
                }
            }
        }
    }
}

/// System that ticks automated Logic Diviners and flags unflagged hazard tiles (GDD §5.1).
pub fn update_logic_diviners(
    time: Res<Time>,
    mut diviners: Query<(&mut LogicDiviner, &super::building::Building)>,
    mut island_grid: ResMut<IslandGrid>,
    mut commands: Commands,
) {
    let dt = time.delta();
    for (mut diviner, building) in diviners.iter_mut() {
        diviner.timer.tick(dt);
        if !diviner.timer.just_finished() {
            continue;
        }

        let r = diviner.radius;
        for dx in -r..=r {
            for dy in -r..=r {
                let target_coord = GridCoord::new(building.coord.x + dx, building.coord.y + dy, building.coord.layer);
                let should_flag = if let Some(tile) = island_grid.tiles.get(&target_coord) {
                    if tile.has_overcharge_hazard && tile.prospect_state == TileProspectState::Covered {
                        Some(tile.adjacent_hazards)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(adjacent_hazards) = should_flag {
                    let _ = island_grid.toggle_flag(&target_coord);
                    commands.trigger(TileStateChangedEvent {
                        coord: target_coord,
                        new_state: TileProspectState::Flagged,
                        adjacent_hazards,
                    });
                    return; // Flags one hazard per pulse
                }
            }
        }
    }
}

/// System that ticks wellheads and pumps ore onto adjacent conveyor belts.
pub fn update_wellhead_extraction(
    time: Res<Time>,
    mut wellheads: Query<(&mut ClockworkWellhead, &super::building::Building, Option<&super::harmonic::HarmonicTuning>)>,
    mut deposits: Query<&mut ResourceDeposit>,
    mut belts: Query<(Entity, &mut ConveyorBelt, &super::building::Building)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let mut belts_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in belts.iter() {
        belts_by_coord.insert(building.coord, entity);
    }

    for (mut wellhead, building, maybe_tuning) in wellheads.iter_mut() {
        let speed_mult = maybe_tuning.map(|t| t.speed_multiplier()).unwrap_or(1.0);
        wellhead.timer.tick(dt.mul_f32(speed_mult));
        if !wellhead.timer.just_finished() {
            continue;
        }

        let Some(mut deposit) = deposits.iter_mut().find(|d| d.coord == building.coord && !d.is_depleted()) else {
            continue;
        };

        let neighbors = building.coord.neighbors_cardinal();
        for neighbor in neighbors {
            if let Some(&belt_entity) = belts_by_coord.get(&neighbor) {
                if let Ok((_, mut belt, _)) = belts.get_mut(belt_entity) {
                    if belt.held_item.is_none() {
                        let ore = deposit.item_type;
                        deposit.extract(1);

                        let item_entity = commands
                            .spawn(TransportedItem::new(ore, neighbor))
                            .id();

                        belt.held_item = Some(item_entity);
                        break;
                    }
                }
            }
        }
    }
}

/// System that ticks smelting hearths and refines ores into plates.
pub fn update_smelting_hearth(
    time: Res<Time>,
    mut hearths: Query<(&mut SmeltingHearth, &super::building::Building, Option<&super::harmonic::HarmonicTuning>)>,
    mut belts: Query<(Entity, &mut ConveyorBelt, &super::building::Building)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let mut belts_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in belts.iter() {
        belts_by_coord.insert(building.coord, entity);
    }

    for (mut hearth, building, maybe_tuning) in hearths.iter_mut() {
        let speed_mult = maybe_tuning.map(|t| t.speed_multiplier()).unwrap_or(1.0);
        if let Some(ore) = hearth.input_ore {
            hearth.timer.tick(dt.mul_f32(speed_mult));
            if hearth.timer.just_finished() {
                let plate = match ore {
                    ItemType::IronOre => ItemType::IronPlate,
                    ItemType::CopperOre => ItemType::CopperPlate,
                    _ => ItemType::IronPlate,
                };
                hearth.input_ore = None;
                hearth.output_plate = Some(plate);
            }
        }

        if let Some(plate) = hearth.output_plate {
            let neighbors = building.coord.neighbors_cardinal();
            for neighbor in neighbors {
                if let Some(&belt_entity) = belts_by_coord.get(&neighbor) {
                    if let Ok((_, mut belt, _)) = belts.get_mut(belt_entity) {
                        if belt.held_item.is_none() {
                            let item_entity = commands
                                .spawn(TransportedItem::new(plate, neighbor))
                                .id();

                            belt.held_item = Some(item_entity);
                            hearth.output_plate = None;
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// System that ticks Chromatic Autoclaves: executes automated Match-3 catalysis swaps,
/// generates radiant power, charges nearby Resonance Bell Jars on cascades,
/// and outputs refined ingots onto exit conveyor belts.
pub fn update_chromatic_autoclaves(
    time: Res<Time>,
    mut autoclaves: Query<(Entity, &mut ChromaticAutoclave, &super::building::Building)>,
    mut bell_jars: Query<(&mut ResonanceBellJar, &super::building::Building), Without<ChromaticAutoclave>>,
    mut belts: Query<(Entity, &mut ConveyorBelt, &super::building::Building), (Without<ChromaticAutoclave>, Without<ResonanceBellJar>)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let mut belts_by_coord: HashMap<GridCoord, Entity> = HashMap::new();
    for (entity, _, building) in belts.iter() {
        belts_by_coord.insert(building.coord, entity);
    }

    for (auto_entity, mut auto, building) in autoclaves.iter_mut() {
        // 1. Process any input items in input_buffer (e.g. dissolve IngotCube or Gemstone)
        while let Some(input_item) = auto.input_buffer.pop() {
            if input_item == ItemType::IngotCube {
                auto.stored_power_kw += 20.0;
            } else if let Some(color) = super::autoclave::GemColor::from_item_type(input_item) {
                auto.grid.cells[5][0] = Some(color);
            }
        }

        // 2. Automated Brass Swap-Piston
        if auto.auto_piston_enabled {
            auto.auto_piston_timer.tick(dt);
            if auto.auto_piston_timer.just_finished() {
                if let Ok(summary) = auto.grid.auto_swap_best() {
                    auto.stored_power_kw += summary.power_generated_kw;
                    auto.total_catalysis_reactions += 1;

                    for _ in 0..summary.total_refined_ingots {
                        auto.pending_outputs.push(ItemType::AlloyIngot);
                    }
                    for _ in 0..summary.total_alloy_ingots {
                        auto.pending_outputs.push(ItemType::AlloyIngot);
                    }
                    for _ in 0..summary.total_prismatic_cores {
                        auto.pending_outputs.push(ItemType::PrismaticCore);
                    }

                    // Charge nearby Resonance Bell Jars (within 6 tiles) on cascades
                    if summary.max_multiplier >= 2 {
                        let auto_coord = building.coord;
                        for (mut bell_jar, jar_bld) in bell_jars.iter_mut() {
                            let dx = (auto_coord.x - jar_bld.coord.x) as f32;
                            let dy = (auto_coord.y - jar_bld.coord.y) as f32;
                            let dist_sq = dx * dx + dy * dy;
                            if dist_sq <= 36.0 {
                                bell_jar.stored_charges = (bell_jar.stored_charges + 1).min(bell_jar.max_charges);
                                bell_jar.is_resonating = true;
                            }
                        }
                    }

                    commands.trigger(CatalysisEvent {
                        entity: auto_entity,
                        coord: building.coord,
                        summary,
                    });
                }
            }
        }

        // 3. Dispense pending outputs onto facing conveyor belt
        if !auto.pending_outputs.is_empty() {
            let exit_coord = building.coord.neighbor(auto.facing);
            if let Some(&belt_ent) = belts_by_coord.get(&exit_coord) {
                if let Ok((_, mut belt, _)) = belts.get_mut(belt_ent) {
                    if belt.held_item.is_none() {
                        let out_item = auto.pending_outputs.remove(0);
                        let item_entity = commands
                            .spawn(TransportedItem::new(out_item, exit_coord))
                            .id();
                        belt.held_item = Some(item_entity);
                    }
                }
            }
        }
    }
}

/// System that ticks Resonance Bell Jars: maintains resonance, discharges stored surge power.
pub fn update_resonance_bell_jars(
    time: Res<Time>,
    mut bell_jars: Query<&mut ResonanceBellJar>,
) {
    let dt = time.delta();
    for mut bell_jar in bell_jars.iter_mut() {
        if bell_jar.stored_charges > 0 {
            bell_jar.discharge_timer.tick(dt);
            if bell_jar.discharge_timer.just_finished() {
                bell_jar.stored_charges = bell_jar.stored_charges.saturating_sub(1);
                if bell_jar.stored_charges == 0 {
                    bell_jar.is_resonating = false;
                }
            }
        } else {
            bell_jar.is_resonating = false;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ConveyorBelt>()
        .register_type::<InclineRamp>()
        .register_type::<SwivelTurntable>()
        .register_type::<SortingGate>()
        .register_type::<CargoCompressor>()
        .register_type::<SurveyBeacon>()
        .register_type::<LogicDiviner>()
        .register_type::<TransportedItem>()
        .register_type::<ClockworkWellhead>()
        .register_type::<SmeltingHearth>()
        .register_type::<StorageChest>()
        .add_systems(
            Update,
            (
                update_conveyor_items,
                update_cargo_compressors,
                update_survey_beacons,
                update_logic_diviners,
                update_wellhead_extraction,
                update_smelting_hearth,
                update_chromatic_autoclaves,
                update_resonance_bell_jars,
            ),
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::game::building::{Building, BuildingType};
    use crate::game::grid::Layer;

    fn make_test_app() -> App {
        let mut app = App::new();
        app.init_resource::<Time>();
        app.add_systems(
            Update,
            (
                update_conveyor_items,
                update_cargo_compressors,
                update_wellhead_extraction,
                update_smelting_hearth,
                update_chromatic_autoclaves,
                update_resonance_bell_jars,
            ),
        );
        app
    }

    #[test]
    fn test_incline_ramp_layer_transition() {
        let mut app = make_test_app();

        // Place an upward incline ramp at (0, 0, Ground) facing North
        let ramp_coord = GridCoord::new(0, 0, Layer::Ground);
        let ramp_entity = app
            .world_mut()
            .spawn((
                InclineRamp::new(Direction::North, true),
                Building::new(ramp_coord, BuildingType::InclineRamp { direction: Direction::North, upward: true }),
            ))
            .id();

        // Place a receiving viaduct belt at (0, 1, Viaduct) facing North
        let viaduct_coord = GridCoord::new(0, 1, Layer::Viaduct);
        let viaduct_entity = app
            .world_mut()
            .spawn((
                ConveyorBelt::new(Direction::North),
                Building::new(viaduct_coord, BuildingType::ViaductBelt(Direction::North)),
            ))
            .id();

        // Spawn item on ramp with progress = 0.5
        let item_entity = app
            .world_mut()
            .spawn(TransportedItem {
                item_type: ItemType::IronOre,
                current_coord: ramp_coord,
                progress: 0.5,
            })
            .id();

        app.world_mut()
            .get_mut::<InclineRamp>(ramp_entity)
            .unwrap()
            .held_item = Some(item_entity);

        // Advance time by 0.6 seconds (item reaches 1.0 and transfers to viaduct)
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(600));
        app.update();

        // Item should now be on Viaduct layer!
        let item = app.world().get::<TransportedItem>(item_entity).unwrap();
        assert_eq!(item.current_coord, viaduct_coord);
        assert_eq!(item.current_coord.layer, Layer::Viaduct);

        let viaduct = app.world().get::<ConveyorBelt>(viaduct_entity).unwrap();
        assert_eq!(viaduct.held_item, Some(item_entity));
    }

    #[test]
    fn test_turntable_rotates_polyomino_shard() {
        let mut app = make_test_app();

        let turn_coord = GridCoord::ground(0, 0);
        let turn_entity = app
            .world_mut()
            .spawn((
                SwivelTurntable::new(Direction::North),
                Building::new(turn_coord, BuildingType::SwivelTurntable { rotation: Direction::North }),
            ))
            .id();

        let belt_coord = GridCoord::ground(0, 1);
        let belt_entity = app
            .world_mut()
            .spawn((
                ConveyorBelt::new(Direction::North),
                Building::new(belt_coord, BuildingType::Belt(Direction::North)),
            ))
            .id();

        let item_entity = app
            .world_mut()
            .spawn((
                TransportedItem {
                    item_type: ItemType::PolyominoDomino,
                    current_coord: turn_coord,
                    progress: 0.8,
                },
                PolyominoShard::new(PolyominoShape::Domino),
            ))
            .id();

        app.world_mut()
            .get_mut::<SwivelTurntable>(turn_entity)
            .unwrap()
            .held_item = Some(item_entity);

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(300));
        app.update();

        let shard = app.world().get::<PolyominoShard>(item_entity).unwrap();
        assert_eq!(shard.rotation, 1); // Rotated 90° clockwise!

        let belt = app.world().get::<ConveyorBelt>(belt_entity).unwrap();
        assert_eq!(belt.held_item, Some(item_entity));
    }

    #[test]
    fn test_sorting_gate_diverts_matching_items() {
        let mut app = make_test_app();

        let gate_coord = GridCoord::ground(0, 0);
        let gate_entity = app
            .world_mut()
            .spawn((
                SortingGate::new(ItemType::IronPlate, Direction::North, Direction::East),
                Building::new(
                    gate_coord,
                    BuildingType::SortingGate {
                        filter_item: ItemType::IronPlate,
                        straight_dir: Direction::North,
                        divert_dir: Direction::East,
                    },
                ),
            ))
            .id();

        let east_belt_coord = GridCoord::ground(1, 0);
        let _east_belt = app
            .world_mut()
            .spawn((
                ConveyorBelt::new(Direction::East),
                Building::new(east_belt_coord, BuildingType::Belt(Direction::East)),
            ))
            .id();

        // Spawn IronPlate (matches filter -> diverts East)
        let item_entity = app
            .world_mut()
            .spawn(TransportedItem {
                item_type: ItemType::IronPlate,
                current_coord: gate_coord,
                progress: 0.9,
            })
            .id();

        app.world_mut()
            .get_mut::<SortingGate>(gate_entity)
            .unwrap()
            .held_item = Some(item_entity);

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(200));
        app.update();

        let item = app.world().get::<TransportedItem>(item_entity).unwrap();
        assert_eq!(item.current_coord, east_belt_coord);
    }

    #[test]
    fn test_autoclave_simulation_and_belt_output() {
        use crate::game::autoclave::GemColor;

        let mut app = make_test_app();

        let auto_coord = GridCoord::ground(0, 0);
        let mut autoclave = ChromaticAutoclave::new(Direction::East);
        // Force a guaranteed match move
        autoclave.grid.cells[0][0] = Some(GemColor::Ruby);
        autoclave.grid.cells[0][1] = Some(GemColor::Ruby);
        autoclave.grid.cells[0][2] = Some(GemColor::Topaz);
        autoclave.grid.cells[1][2] = Some(GemColor::Ruby);

        let auto_entity = app
            .world_mut()
            .spawn((
                autoclave,
                Building::new(auto_coord, BuildingType::ChromaticAutoclave(Direction::East)),
            ))
            .id();

        let exit_coord = GridCoord::ground(1, 0);
        let belt_entity = app
            .world_mut()
            .spawn((
                ConveyorBelt::new(Direction::East),
                Building::new(exit_coord, BuildingType::Belt(Direction::East)),
            ))
            .id();

        // Advance time past auto_piston_timer (1.5s)
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(1600));
        app.update();

        let auto = app.world().get::<ChromaticAutoclave>(auto_entity).unwrap();
        assert!(auto.stored_power_kw > 0.0);
        assert!(auto.total_catalysis_reactions >= 1);

        let belt = app.world().get::<ConveyorBelt>(belt_entity).unwrap();
        assert!(belt.held_item.is_some(), "Autoclave should output refined alloy ingot onto facing belt");

        let out_item = app.world().get::<TransportedItem>(belt.held_item.unwrap()).unwrap();
        assert_eq!(out_item.item_type, ItemType::AlloyIngot);
    }

    #[test]
    fn test_autoclave_item_ingestion_from_belt() {
        let mut app = make_test_app();

        let belt_coord = GridCoord::ground(0, 0);
        let auto_coord = GridCoord::ground(0, 1);

        let belt_entity = app
            .world_mut()
            .spawn((
                ConveyorBelt::new(Direction::North),
                Building::new(belt_coord, BuildingType::Belt(Direction::North)),
            ))
            .id();

        let auto_entity = app
            .world_mut()
            .spawn((
                ChromaticAutoclave::new(Direction::North),
                Building::new(auto_coord, BuildingType::ChromaticAutoclave(Direction::North)),
            ))
            .id();

        let item_entity = app
            .world_mut()
            .spawn(TransportedItem {
                item_type: ItemType::IngotCube,
                current_coord: belt_coord,
                progress: 0.9,
            })
            .id();

        app.world_mut()
            .get_mut::<ConveyorBelt>(belt_entity)
            .unwrap()
            .held_item = Some(item_entity);

        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_millis(200));
        app.update();

        let belt = app.world().get::<ConveyorBelt>(belt_entity).unwrap();
        assert!(belt.held_item.is_none(), "Belt should be cleared after item enters autoclave");

        let auto = app.world().get::<ChromaticAutoclave>(auto_entity).unwrap();
        // Either in input_buffer or dissolved by update_chromatic_autoclaves
        assert!(auto.stored_power_kw >= 20.0 || auto.input_buffer.contains(&ItemType::IngotCube));
    }
}
