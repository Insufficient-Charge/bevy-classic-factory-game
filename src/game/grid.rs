use bevy::prelude::*;

/// Vertical layer planes for multi-tier spatial factory routing (inspired by Shapez 2).
#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum Layer {
    /// Subterranean extraction basements (Z-1)
    Subterranean,
    /// Ground floor baseline terrain (Z0)
    #[default]
    Ground,
    /// Elevated viaduct overpass layer (Z1)
    Viaduct,
    /// High-altitude skyway suspension layer (Z2)
    Skyway,
}

impl Layer {
    /// Height offset in 3D world units for this layer.
    pub const fn height_offset(&self) -> f32 {
        match self {
            Self::Subterranean => -2.0,
            Self::Ground => 0.0,
            Self::Viaduct => 2.0,
            Self::Skyway => 4.0,
        }
    }

    /// Discrete integer level index.
    pub const fn level(&self) -> i8 {
        match self {
            Self::Subterranean => -1,
            Self::Ground => 0,
            Self::Viaduct => 1,
            Self::Skyway => 2,
        }
    }

    /// Try to convert from an integer level index.
    pub const fn from_level(level: i8) -> Option<Self> {
        match level {
            -1 => Some(Self::Subterranean),
            0 => Some(Self::Ground),
            1 => Some(Self::Viaduct),
            2 => Some(Self::Skyway),
            _ => None,
        }
    }

    /// The layer directly above this one, if it exists.
    pub const fn above(&self) -> Option<Self> {
        match self {
            Self::Subterranean => Some(Self::Ground),
            Self::Ground => Some(Self::Viaduct),
            Self::Viaduct => Some(Self::Skyway),
            Self::Skyway => None,
        }
    }

    /// The layer directly below this one, if it exists.
    pub const fn below(&self) -> Option<Self> {
        match self {
            Self::Subterranean => None,
            Self::Ground => Some(Self::Subterranean),
            Self::Viaduct => Some(Self::Ground),
            Self::Skyway => Some(Self::Viaduct),
        }
    }
}

/// Cardinal direction on the 2.5D isometric factory grid.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Returns the opposite direction.
    pub const fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    /// Rotates 90 degrees clockwise.
    pub const fn rotate_cw(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Rotates 90 degrees counter-clockwise.
    pub const fn rotate_ccw(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    /// Returns the (dx, dy) grid coordinate offset for this direction.
    pub const fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::East => (1, 0),
            Self::South => (0, -1),
            Self::West => (-1, 0),
        }
    }

    /// Returns unit vector in 3D world space (where North is -Z, East is +X).
    pub fn to_world_vector(&self) -> Vec3 {
        match self {
            Self::North => Vec3::new(0.0, 0.0, -1.0),
            Self::East => Vec3::new(1.0, 0.0, 0.0),
            Self::South => Vec3::new(0.0, 0.0, 1.0),
            Self::West => Vec3::new(-1.0, 0.0, 0.0),
        }
    }

    /// Converts a 3D forward vector to the closest cardinal grid direction.
    pub fn from_vector(v: Vec3) -> Self {
        if v.x.abs() > v.z.abs() {
            if v.x > 0.0 { Self::East } else { Self::West }
        } else if v.z < 0.0 {
            Self::North
        } else {
            Self::South
        }
    }
}

/// Discrete 3D spatial grid coordinate on the floating diorama.
#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct GridCoord {
    pub x: i32,
    pub y: i32,
    pub layer: Layer,
}

impl GridCoord {
    /// World size of one grid tile (meters/units).
    pub const TILE_SIZE: f32 = 1.0;

    /// Create a new coordinate on a specific layer.
    pub const fn new(x: i32, y: i32, layer: Layer) -> Self {
        Self { x, y, layer }
    }

    /// Create a new ground coordinate (Z0).
    pub const fn ground(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            layer: Layer::Ground,
        }
    }

    /// Converts this grid coordinate to a 3D world position (tile center).
    pub fn to_world_pos(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * Self::TILE_SIZE,
            self.layer.height_offset(),
            self.y as f32 * Self::TILE_SIZE,
        )
    }

    /// Converts a 3D world position into the nearest grid coordinate on the given layer.
    pub fn from_world_pos(pos: Vec3, layer: Layer) -> Self {
        let x = (pos.x / Self::TILE_SIZE).round() as i32;
        let y = (pos.z / Self::TILE_SIZE).round() as i32;
        Self { x, y, layer }
    }

    /// Get adjacent grid coordinate in a cardinal direction.
    pub fn neighbor(&self, dir: Direction) -> Self {
        let (dx, dy) = dir.to_offset();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            layer: self.layer,
        }
    }

    /// Returns the coordinate directly above on the next layer, if any.
    pub fn layer_above(&self) -> Option<Self> {
        self.layer.above().map(|l| Self {
            x: self.x,
            y: self.y,
            layer: l,
        })
    }

    /// Returns the coordinate directly below on the prior layer, if any.
    pub fn layer_below(&self) -> Option<Self> {
        self.layer.below().map(|l| Self {
            x: self.x,
            y: self.y,
            layer: l,
        })
    }

    /// Returns the 4 cardinal neighbors on the same layer (N, E, S, W).
    pub fn neighbors_cardinal(&self) -> [Self; 4] {
        [
            self.neighbor(Direction::North),
            self.neighbor(Direction::East),
            self.neighbor(Direction::South),
            self.neighbor(Direction::West),
        ]
    }

    /// Returns all 8 surrounding neighbors on the same layer (for Minesweeper adjacency counting).
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

    /// Manhattan distance on the 2D grid plane.
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Euclidean distance in 3D world space between tile centers.
    pub fn euclidean_distance(&self, other: &Self) -> f32 {
        self.to_world_pos().distance(other.to_world_pos())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(neighbors[0], GridCoord::ground(5, 6));
        assert_eq!(neighbors[1], GridCoord::ground(6, 5));
        assert_eq!(neighbors[2], GridCoord::ground(5, 4));
        assert_eq!(neighbors[3], GridCoord::ground(4, 5));
    }

    #[test]
    fn test_grid_neighbors_all_8_for_minesweeper() {
        let center = GridCoord::ground(10, 10);
        let all_8 = center.neighbors_all_8();
        assert_eq!(all_8.len(), 8);

        let mut unique = std::collections::HashSet::new();
        for n in all_8.iter() {
            assert!(unique.insert(*n), "Duplicate neighbor detected: {:?}", n);
            assert_eq!(center.manhattan_distance(n) <= 2, true);
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
}
