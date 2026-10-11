use std::collections::{HashMap, HashSet};

// Minimal Vec2 and Vec3 for standalone mathematical testing
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y }
    pub fn length(&self) -> f32 { self.length_squared().sqrt() }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }
    pub fn length(&self) -> f32 { self.length_squared().sqrt() }
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.00001 {
            Self { x: self.x / len, y: self.y / len, z: self.z / len }
        } else {
            Self::ZERO
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn distance(&self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

// -------------------------------------------------------------
// GRID SYSTEM
// -------------------------------------------------------------
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Layer {
    Subterranean,
    Ground,
    Viaduct,
    Skyway,
}

impl Layer {
    pub const fn height_offset(&self) -> f32 {
        match self {
            Self::Subterranean => -2.0,
            Self::Ground => 0.0,
            Self::Viaduct => 2.0,
            Self::Skyway => 4.0,
        }
    }

    pub const fn level(&self) -> i8 {
        match self {
            Self::Subterranean => -1,
            Self::Ground => 0,
            Self::Viaduct => 1,
            Self::Skyway => 2,
        }
    }

    pub const fn from_level(level: i8) -> Option<Self> {
        match level {
            -1 => Some(Self::Subterranean),
            0 => Some(Self::Ground),
            1 => Some(Self::Viaduct),
            2 => Some(Self::Skyway),
            _ => None,
        }
    }

    pub const fn above(&self) -> Option<Self> {
        match self {
            Self::Subterranean => Some(Self::Ground),
            Self::Ground => Some(Self::Viaduct),
            Self::Viaduct => Some(Self::Skyway),
            Self::Skyway => None,
        }
    }

    pub const fn below(&self) -> Option<Self> {
        match self {
            Self::Subterranean => None,
            Self::Ground => Some(Self::Subterranean),
            Self::Viaduct => Some(Self::Ground),
            Self::Skyway => Some(Self::Viaduct),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub const fn rotate_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub const fn rotate_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub const fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::East => (1, 0),
            Self::South => (0, -1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
    pub layer: Layer,
}

impl GridCoord {
    pub const TILE_SIZE: f32 = 1.0;

    pub const fn new(x: i32, y: i32, layer: Layer) -> Self {
        Self { x, y, layer }
    }

    pub const fn ground(x: i32, y: i32) -> Self {
        Self { x, y, layer: Layer::Ground }
    }

    pub fn to_world_pos(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * Self::TILE_SIZE,
            self.layer.height_offset(),
            self.y as f32 * Self::TILE_SIZE,
        )
    }

    pub fn from_world_pos(pos: Vec3, layer: Layer) -> Self {
        let x = (pos.x / Self::TILE_SIZE).round() as i32;
        let y = (pos.z / Self::TILE_SIZE).round() as i32;
        Self { x, y, layer }
    }

    pub fn neighbor(&self, dir: Direction) -> Self {
        let (dx, dy) = dir.to_offset();
        Self { x: self.x + dx, y: self.y + dy, layer: self.layer }
    }

    pub fn neighbors_cardinal(&self) -> [Self; 4] {
        [
            self.neighbor(Direction::North),
            self.neighbor(Direction::East),
            self.neighbor(Direction::South),
            self.neighbor(Direction::West),
        ]
    }

    pub fn neighbors_all_8(&self) -> [Self; 8] {
        [
            Self::new(self.x, self.y + 1, self.layer),
            Self::new(self.x + 1, self.y + 1, self.layer),
            Self::new(self.x + 1, self.y, self.layer),
            Self::new(self.x + 1, self.y - 1, self.layer),
            Self::new(self.x, self.y - 1, self.layer),
            Self::new(self.x - 1, self.y - 1, self.layer),
            Self::new(self.x - 1, self.y, self.layer),
            Self::new(self.x - 1, self.y + 1, self.layer),
        ]
    }

    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        self.to_world_pos().distance(other.to_world_pos())
    }
}

// -------------------------------------------------------------
// DIORAMA CAMERA MATH
// -------------------------------------------------------------
pub struct DioramaCameraConfig {
    pub pitch_radians: f32,
    pub rotation_step: u8,
    pub distance: f32,
    pub vertical_size: f32,
    pub min_vertical_size: f32,
    pub max_vertical_size: f32,
}

impl Default for DioramaCameraConfig {
    fn default() -> Self {
        Self {
            pitch_radians: 35.264_f32.to_radians(),
            rotation_step: 0,
            distance: 50.0,
            vertical_size: 16.0,
            min_vertical_size: 6.0,
            max_vertical_size: 32.0,
        }
    }
}

impl DioramaCameraConfig {
    pub fn target_yaw_radians(&self) -> f32 {
        rotation_step_to_yaw(self.rotation_step)
    }

    pub fn rotate_cw(&mut self) {
        self.rotation_step = (self.rotation_step + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation_step = (self.rotation_step + 3) % 4;
    }

    pub fn adjust_zoom(&mut self, delta: f32) {
        self.vertical_size = (self.vertical_size + delta).clamp(self.min_vertical_size, self.max_vertical_size);
    }
}

pub fn rotation_step_to_yaw(step: u8) -> f32 {
    let degrees = 45.0 + (step % 4) as f32 * 90.0;
    degrees.to_radians()
}

pub fn compute_camera_offset(pitch: f32, yaw: f32, distance: f32) -> Vec3 {
    let x = distance * yaw.sin() * pitch.cos();
    let y = distance * pitch.sin();
    let z = distance * yaw.cos() * pitch.cos();
    Vec3::new(x, y, z)
}

pub fn iso_input_to_world_dir(input: Vec2, rotation_step: u8) -> Vec3 {
    if input.length_squared() < 0.001 {
        return Vec3::ZERO;
    }

    let yaw = rotation_step_to_yaw(rotation_step);
    let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos()).normalize();
    let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin()).normalize();

    (right * input.x + forward * input.y).normalize()
}

// -------------------------------------------------------------
// TERRAIN & MINESWEEPER PROSPECTING
// -------------------------------------------------------------
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TileProspectState {
    Covered,
    Revealed,
    Flagged,
}

#[derive(Clone, Debug)]
pub struct TerrainTile {
    pub coord: GridCoord,
    pub prospect_state: TileProspectState,
    pub has_overcharge_hazard: bool,
    pub adjacent_hazards: u8,
}

pub struct IslandGrid {
    pub tiles: HashMap<GridCoord, TerrainTile>,
}

impl IslandGrid {
    pub fn new() -> Self {
        Self { tiles: HashMap::new() }
    }

    pub fn insert_tile(&mut self, coord: GridCoord, has_hazard: bool) {
        self.tiles.insert(coord, TerrainTile {
            coord,
            prospect_state: TileProspectState::Covered,
            has_overcharge_hazard: has_hazard,
            adjacent_hazards: 0,
        });
    }

    pub fn recompute_hazards(&mut self) {
        let hazard_set: HashSet<GridCoord> = self.tiles.iter()
            .filter(|(_, t)| t.has_overcharge_hazard)
            .map(|(c, _)| *c)
            .collect();

        for (coord, tile) in self.tiles.iter_mut() {
            let mut count = 0;
            for n in coord.neighbors_all_8() {
                if hazard_set.contains(&n) {
                    count += 1;
                }
            }
            tile.adjacent_hazards = count;
        }
    }

    pub fn unfold_tile(&mut self, coord: &GridCoord) -> Result<bool, &'static str> {
        let tile = self.tiles.get(coord).ok_or("Tile not found")?;
        if tile.prospect_state == TileProspectState::Flagged {
            return Err("Flagged");
        }
        if tile.prospect_state == TileProspectState::Revealed {
            return Ok(false);
        }

        if tile.has_overcharge_hazard {
            if let Some(t) = self.tiles.get_mut(coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
            return Ok(true); // Pop!
        }

        let mut queue = vec![*coord];
        let mut visited = HashSet::new();

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }
            if let Some(t) = self.tiles.get_mut(&current) {
                if t.prospect_state == TileProspectState::Flagged || t.has_overcharge_hazard {
                    continue;
                }
                t.prospect_state = TileProspectState::Revealed;
                if t.adjacent_hazards == 0 {
                    for n in current.neighbors_all_8() {
                        if let Some(nt) = self.tiles.get(&n) {
                            if nt.prospect_state == TileProspectState::Covered {
                                queue.push(n);
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn toggle_flag(&mut self, coord: &GridCoord) -> Result<TileProspectState, &'static str> {
        let tile = self.tiles.get_mut(coord).ok_or("Tile not found")?;
        match tile.prospect_state {
            TileProspectState::Covered => {
                tile.prospect_state = TileProspectState::Flagged;
                Ok(TileProspectState::Flagged)
            }
            TileProspectState::Flagged => {
                tile.prospect_state = TileProspectState::Covered;
                Ok(TileProspectState::Covered)
            }
            TileProspectState::Revealed => Err("Already revealed"),
        }
    }
}

// -------------------------------------------------------------
// TESTS
// -------------------------------------------------------------
#[test]
fn test_layer_height_ordering() {
    assert!(Layer::Subterranean.height_offset() < Layer::Ground.height_offset());
    assert!(Layer::Ground.height_offset() < Layer::Viaduct.height_offset());
    assert!(Layer::Viaduct.height_offset() < Layer::Skyway.height_offset());
}

#[test]
fn test_layer_transitions() {
    assert_eq!(Layer::Ground.above(), Some(Layer::Viaduct));
    assert_eq!(Layer::Viaduct.above(), Some(Layer::Skyway));
    assert_eq!(Layer::Skyway.above(), None);

    assert_eq!(Layer::Skyway.below(), Some(Layer::Viaduct));
    assert_eq!(Layer::Viaduct.below(), Some(Layer::Ground));
    assert_eq!(Layer::Ground.below(), Some(Layer::Subterranean));
    assert_eq!(Layer::Subterranean.below(), None);
}

#[test]
fn test_layer_from_level() {
    assert_eq!(Layer::from_level(-1), Some(Layer::Subterranean));
    assert_eq!(Layer::from_level(0), Some(Layer::Ground));
    assert_eq!(Layer::from_level(1), Some(Layer::Viaduct));
    assert_eq!(Layer::from_level(2), Some(Layer::Skyway));
    assert_eq!(Layer::from_level(3), None);
}

#[test]
fn test_direction_rotations() {
    assert_eq!(Direction::North.rotate_cw(), Direction::East);
    assert_eq!(Direction::East.rotate_cw(), Direction::South);
    assert_eq!(Direction::South.rotate_cw(), Direction::West);
    assert_eq!(Direction::West.rotate_cw(), Direction::North);

    assert_eq!(Direction::North.rotate_ccw(), Direction::West);
    assert_eq!(Direction::West.rotate_ccw(), Direction::South);
    assert_eq!(Direction::South.rotate_ccw(), Direction::East);
    assert_eq!(Direction::East.rotate_ccw(), Direction::North);

    assert_eq!(Direction::North.opposite(), Direction::South);
    assert_eq!(Direction::East.opposite(), Direction::West);
}

#[test]
fn test_grid_neighbors_cardinal() {
    let origin = GridCoord::ground(5, 5);
    let neighbors = origin.neighbors_cardinal();

    assert_eq!(neighbors[0], GridCoord::ground(5, 6)); // North
    assert_eq!(neighbors[1], GridCoord::ground(6, 5)); // East
    assert_eq!(neighbors[2], GridCoord::ground(5, 4)); // South
    assert_eq!(neighbors[3], GridCoord::ground(4, 5)); // West
}

#[test]
fn test_grid_neighbors_all_8_for_minesweeper() {
    let center = GridCoord::ground(10, 10);
    let all_8 = center.neighbors_all_8();
    assert_eq!(all_8.len(), 8);

    let mut unique = HashSet::new();
    for n in all_8.iter() {
        assert!(unique.insert(*n), "Duplicate neighbor detected");
        assert!(center.manhattan_distance(n) <= 2);
    }
}

#[test]
fn test_world_pos_roundtrip() {
    let coord = GridCoord::new(7, -3, Layer::Viaduct);
    let world_pos = coord.to_world_pos();

    assert_eq!(world_pos.x, 7.0);
    assert_eq!(world_pos.y, 2.0);
    assert_eq!(world_pos.z, -3.0);

    let recovered = GridCoord::from_world_pos(world_pos, Layer::Viaduct);
    assert_eq!(coord, recovered);
}

#[test]
fn test_manhattan_distance() {
    let a = GridCoord::ground(2, 3);
    let b = GridCoord::ground(5, 7);
    assert_eq!(a.manhattan_distance(&b), 7);
}

#[test]
fn test_euclidean_distance_across_layers() {
    let ground = GridCoord::new(0, 0, Layer::Ground);
    let viaduct = GridCoord::new(0, 0, Layer::Viaduct);
    assert_eq!(ground.euclidean_distance(&viaduct), 2.0);
}

#[test]
fn test_rotation_step_cycling() {
    let mut config = DioramaCameraConfig::default();
    assert_eq!(config.rotation_step, 0);

    config.rotate_cw();
    assert_eq!(config.rotation_step, 1);
    config.rotate_cw();
    assert_eq!(config.rotation_step, 2);
    config.rotate_cw();
    assert_eq!(config.rotation_step, 3);
    config.rotate_cw();
    assert_eq!(config.rotation_step, 0);

    config.rotate_ccw();
    assert_eq!(config.rotation_step, 3);
    config.rotate_ccw();
    assert_eq!(config.rotation_step, 2);
    config.rotate_ccw();
    assert_eq!(config.rotation_step, 1);
    config.rotate_ccw();
    assert_eq!(config.rotation_step, 0);
}

#[test]
fn test_rotation_step_to_yaw_angles() {
    let yaw0 = rotation_step_to_yaw(0).to_degrees();
    assert!((yaw0 - 45.0).abs() < 0.001);

    let yaw1 = rotation_step_to_yaw(1).to_degrees();
    assert!((yaw1 - 135.0).abs() < 0.001);

    let yaw2 = rotation_step_to_yaw(2).to_degrees();
    assert!((yaw2 - 225.0).abs() < 0.001);

    let yaw3 = rotation_step_to_yaw(3).to_degrees();
    assert!((yaw3 - 315.0).abs() < 0.001);
}

#[test]
fn test_compute_camera_offset() {
    let pitch = 30.0_f32.to_radians();
    let yaw = 45.0_f32.to_radians();
    let distance = 10.0;
    let offset = compute_camera_offset(pitch, yaw, distance);

    assert!((offset.y - 5.0).abs() < 0.001);
    assert!(offset.x > 0.0);
    assert!(offset.z > 0.0);
    assert!((offset.length() - distance).abs() < 0.001);
}

#[test]
fn test_iso_input_direction_normalization() {
    let input_zero = Vec2::ZERO;
    let dir_zero = iso_input_to_world_dir(input_zero, 0);
    assert_eq!(dir_zero, Vec3::ZERO);

    let input_w = Vec2::new(0.0, 1.0);
    let dir_w = iso_input_to_world_dir(input_w, 0);
    assert!((dir_w.length() - 1.0).abs() < 0.001);
    assert_eq!(dir_w.y, 0.0);
}

#[test]
fn test_iso_input_direction_rotates_with_camera() {
    let input_forward = Vec2::new(0.0, 1.0);

    let dir_step0 = iso_input_to_world_dir(input_forward, 0);
    let dir_step1 = iso_input_to_world_dir(input_forward, 1);
    let dir_step2 = iso_input_to_world_dir(input_forward, 2);
    let dir_step3 = iso_input_to_world_dir(input_forward, 3);

    assert!(dir_step0.dot(dir_step1).abs() < 0.001);
    assert!(dir_step1.dot(dir_step2).abs() < 0.001);
    assert!((dir_step0.dot(dir_step2) + 1.0).abs() < 0.001);
    assert!((dir_step1.dot(dir_step3) + 1.0).abs() < 0.001);
}

#[test]
fn test_zoom_clamping() {
    let mut config = DioramaCameraConfig::default();
    config.min_vertical_size = 5.0;
    config.max_vertical_size = 20.0;
    config.vertical_size = 10.0;

    config.adjust_zoom(-100.0);
    assert_eq!(config.vertical_size, 5.0);

    config.adjust_zoom(100.0);
    assert_eq!(config.vertical_size, 20.0);
}

#[test]
fn test_adjacent_hazard_computation() {
    let mut grid = IslandGrid::new();
    for x in 0..3 {
        for y in 0..3 {
            grid.insert_tile(GridCoord::ground(x, y), x == 1 && y == 1);
        }
    }
    grid.recompute_hazards();

    let center = grid.tiles.get(&GridCoord::ground(1, 1)).unwrap();
    assert!(center.has_overcharge_hazard);

    for n in GridCoord::ground(1, 1).neighbors_all_8() {
        let nt = grid.tiles.get(&n).unwrap();
        assert_eq!(nt.adjacent_hazards, 1);
    }
}

#[test]
fn test_unfold_cascade_on_zero() {
    let mut grid = IslandGrid::new();
    for x in 0..5 {
        for y in 0..5 {
            grid.insert_tile(GridCoord::ground(x, y), x == 4 && y == 4);
        }
    }
    grid.recompute_hazards();

    let pop = grid.unfold_tile(&GridCoord::ground(0, 0)).expect("Unfold failed");
    assert_eq!(pop, false);

    let t00 = grid.tiles.get(&GridCoord::ground(0, 0)).unwrap();
    let t10 = grid.tiles.get(&GridCoord::ground(1, 0)).unwrap();
    let t01 = grid.tiles.get(&GridCoord::ground(0, 1)).unwrap();

    assert_eq!(t00.prospect_state, TileProspectState::Revealed);
    assert_eq!(t10.prospect_state, TileProspectState::Revealed);
    assert_eq!(t01.prospect_state, TileProspectState::Revealed);

    let hazard = grid.tiles.get(&GridCoord::ground(4, 4)).unwrap();
    assert_eq!(hazard.prospect_state, TileProspectState::Covered);
}

#[test]
fn test_flag_prevents_accidental_unfold() {
    let mut grid = IslandGrid::new();
    for x in 0..3 {
        for y in 0..3 {
            grid.insert_tile(GridCoord::ground(x, y), x == 1 && y == 1);
        }
    }
    grid.recompute_hazards();

    let target = GridCoord::ground(1, 1);
    let state = grid.toggle_flag(&target).unwrap();
    assert_eq!(state, TileProspectState::Flagged);

    let err = grid.unfold_tile(&target);
    assert!(err.is_err());

    let state2 = grid.toggle_flag(&target).unwrap();
    assert_eq!(state2, TileProspectState::Covered);

    let pop = grid.unfold_tile(&target).unwrap();
    assert_eq!(pop, true);
}
