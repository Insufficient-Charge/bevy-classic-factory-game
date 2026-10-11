use bevy::prelude::*;
use super::items::ItemType;

/// 3D Polyomino shapes as defined in GDD Section 5.2 & Section 7.1.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum PolyominoShape {
    #[default]
    Domino,     // 1x2 (2 cells)
    IBar,       // 1x3 (3 cells)
    LTromino,   // 3 cells (corner)
    Square,     // 2x2 (4 cells)
    TTetromino, // 4 cells (T-shape)
    STetromino, // 4 cells (S-shape)
}

impl PolyominoShape {
    /// Returns the cell offsets `(x, y)` for this shape at rotation step 0 (0°).
    pub fn base_offsets(&self) -> &'static [(usize, usize)] {
        match self {
            Self::Domino => &[(0, 0), (1, 0)],
            Self::IBar => &[(0, 0), (1, 0), (2, 0)],
            Self::LTromino => &[(0, 0), (0, 1), (1, 0)],
            Self::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            Self::TTetromino => &[(0, 1), (1, 0), (1, 1), (2, 1)],
            Self::STetromino => &[(0, 0), (1, 0), (1, 1), (2, 1)],
        }
    }

    /// Converts this shape to its corresponding `ItemType`.
    pub const fn to_item_type(&self) -> ItemType {
        match self {
            Self::Domino => ItemType::PolyominoDomino,
            Self::IBar => ItemType::PolyominoIBar,
            Self::LTromino => ItemType::PolyominoLTromino,
            Self::Square => ItemType::PolyominoSquare,
            Self::TTetromino => ItemType::PolyominoTTetromino,
            Self::STetromino => ItemType::PolyominoSTetromino,
        }
    }

    /// Tries to convert from an `ItemType`.
    pub const fn from_item_type(item: ItemType) -> Option<Self> {
        match item {
            ItemType::PolyominoDomino => Some(Self::Domino),
            ItemType::PolyominoIBar => Some(Self::IBar),
            ItemType::PolyominoLTromino => Some(Self::LTromino),
            ItemType::PolyominoSquare => Some(Self::Square),
            ItemType::PolyominoTTetromino => Some(Self::TTetromino),
            ItemType::PolyominoSTetromino => Some(Self::STetromino),
            _ => None,
        }
    }

    /// Calculates rotated cell offsets normalized to (0, 0).
    /// `rotation_step` is 0 (0°), 1 (90° CW), 2 (180°), or 3 (270° CW).
    pub fn rotated_offsets(&self, rotation_step: u8) -> Vec<(usize, usize)> {
        let base = self.base_offsets();
        let step = rotation_step % 4;
        if step == 0 {
            return base.to_vec();
        }

        // Apply 90° clockwise rotation (x, y) -> (y, -x) repeatedly
        let mut points: Vec<(i32, i32)> = base.iter().map(|&(x, y)| (x as i32, y as i32)).collect();
        for _ in 0..step {
            points = points.iter().map(|&(x, y)| (y, -x)).collect();
        }

        // Normalize so min_x == 0 and min_y == 0
        let min_x = points.iter().map(|&(x, _)| x).min().unwrap_or(0);
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap_or(0);

        points
            .into_iter()
            .map(|(x, y)| ((x - min_x) as usize, (y - min_y) as usize))
            .collect()
    }

    /// Returns bounding width and height for this shape at the given rotation.
    pub fn dimensions(&self, rotation_step: u8) -> (usize, usize) {
        let offsets = self.rotated_offsets(rotation_step);
        let max_x = offsets.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let max_y = offsets.iter().map(|&(_, y)| y).max().unwrap_or(0);
        (max_x + 1, max_y + 1)
    }
}

/// Component attached to transported items that are polyomino shards.
#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub struct PolyominoShard {
    pub shape: PolyominoShape,
    pub rotation: u8, // 0..3 (90° steps)
}

impl PolyominoShard {
    pub const fn new(shape: PolyominoShape) -> Self {
        Self { shape, rotation: 0 }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }
}

/// 4x10 Transparent glass hopper grid matrix used in Cargo Compressors (GDD §5.2).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct CargoCompressorGrid {
    pub cells: [[bool; Self::WIDTH]; Self::HEIGHT],
    pub is_jammed: bool,
    pub total_squishes: u32,
    pub pending_ingots: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DropError {
    OutOfBounds,
    Overflow,
}

impl CargoCompressorGrid {
    pub const WIDTH: usize = 4;
    pub const HEIGHT: usize = 10;

    pub fn new() -> Self {
        Self {
            cells: [[false; Self::WIDTH]; Self::HEIGHT],
            is_jammed: false,
            total_squishes: 0,
            pending_ingots: 0,
        }
    }

    /// Checks if a cell is filled.
    pub fn is_filled(&self, col: usize, row: usize) -> bool {
        if col < Self::WIDTH && row < Self::HEIGHT {
            self.cells[row][col]
        } else {
            false
        }
    }

    /// Tries to drop a polyomino shape into the grid starting at `col_offset`.
    /// The shape falls downwards under gravity until it collides with an existing block or the floor (row 0).
    pub fn drop_shape(
        &mut self,
        shape: PolyominoShape,
        rotation: u8,
        col_offset: usize,
    ) -> Result<usize, DropError> {
        if self.is_jammed {
            return Err(DropError::Overflow);
        }

        let offsets = shape.rotated_offsets(rotation);
        let (w, _h) = shape.dimensions(rotation);

        if col_offset + w > Self::WIDTH {
            return Err(DropError::OutOfBounds);
        }

        // A shape drops down under gravity from the top towards the floor (row 0).
        // Find the lowest row where all blocks fit without colliding with existing blocks.
        let mut resting_row = Self::HEIGHT;

        while resting_row > 0 {
            let next_row = resting_row - 1;
            let collides = offsets.iter().any(|&(dx, dy)| {
                let r = next_row + dy;
                let c = col_offset + dx;
                if r < Self::HEIGHT {
                    self.cells[r][c]
                } else {
                    false
                }
            });

            if collides {
                break;
            }
            resting_row = next_row;
        }

        // Check if any part of the resting shape protrudes past the chimney (row >= HEIGHT)
        let overflows = offsets.iter().any(|&(_, dy)| resting_row + dy >= Self::HEIGHT);
        if overflows {
            self.is_jammed = true;
            return Err(DropError::Overflow);
        }

        // Stamp the blocks into the grid
        for (dx, dy) in offsets {
            let r = resting_row + dy;
            let c = col_offset + dx;
            self.cells[r][c] = true;
        }

        Ok(resting_row)
    }

    /// Checks for full horizontal rows (all 4 columns filled), squishes them,
    /// shifts higher rows downwards, and yields 1 Ingot Cube per full row cleared.
    pub fn check_and_clear_rows(&mut self) -> u32 {
        let mut cleared_count = 0;
        let mut r = 0;

        while r < Self::HEIGHT {
            let row_full = (0..Self::WIDTH).all(|c| self.cells[r][c]);
            if row_full {
                cleared_count += 1;
                // Shift all rows above down by 1
                for above in r..(Self::HEIGHT - 1) {
                    self.cells[above] = self.cells[above + 1];
                }
                // Clear the top row
                self.cells[Self::HEIGHT - 1] = [false; Self::WIDTH];
                // Do not increment r, test the newly shifted row at index r
            } else {
                r += 1;
            }
        }

        if cleared_count > 0 {
            self.total_squishes += cleared_count;
            self.pending_ingots += cleared_count;
            self.is_jammed = false;
        }

        cleared_count
    }

    /// Clears the grid and resets jam status (e.g. maintenance reset).
    pub fn reset(&mut self) {
        self.cells = [[false; Self::WIDTH]; Self::HEIGHT];
        self.is_jammed = false;
    }
}

impl Default for CargoCompressorGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Fired whenever a Cargo Compressor triggers a Piston Squish row clear.
#[derive(Event, Clone, Debug)]
pub struct PistonSquishEvent {
    pub compressor_entity: Entity,
    pub cleared_rows: u32,
    pub produced_ingots: u32,
}

pub fn plugin(_app: &mut App) {
    // Registered in src/game/mod.rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyomino_dimensions_and_rotations() {
        let domino = PolyominoShape::Domino;
        assert_eq!(domino.dimensions(0), (2, 1)); // 1x2 horizontal
        assert_eq!(domino.dimensions(1), (1, 2)); // 2x1 vertical

        let square = PolyominoShape::Square;
        assert_eq!(square.dimensions(0), (2, 2));
        assert_eq!(square.dimensions(1), (2, 2));

        let ibar = PolyominoShape::IBar;
        assert_eq!(ibar.dimensions(0), (3, 1));
        assert_eq!(ibar.dimensions(1), (1, 3));
    }

    #[test]
    fn test_compressor_gravity_drop() {
        let mut grid = CargoCompressorGrid::new();

        // Drop a Domino (1x2 horizontal) at col 0
        let row1 = grid.drop_shape(PolyominoShape::Domino, 0, 0).unwrap();
        assert_eq!(row1, 0); // Rests on floor
        assert!(grid.cells[0][0]);
        assert!(grid.cells[0][1]);
        assert!(!grid.cells[0][2]);

        // Drop another Domino (1x2 horizontal) at col 0 directly on top of the first
        let row2 = grid.drop_shape(PolyominoShape::Domino, 0, 0).unwrap();
        assert_eq!(row2, 1); // Rests on row 1
        assert!(grid.cells[1][0]);
        assert!(grid.cells[1][1]);
    }

    #[test]
    fn test_compressor_row_clearing_squish() {
        let mut grid = CargoCompressorGrid::new();

        // Fill row 0 with two 1x2 Dominoes
        grid.drop_shape(PolyominoShape::Domino, 0, 0).unwrap();
        grid.drop_shape(PolyominoShape::Domino, 0, 2).unwrap();

        // All 4 columns of row 0 are now filled
        assert_eq!(grid.check_and_clear_rows(), 1);
        assert_eq!(grid.pending_ingots, 1);
        assert_eq!(grid.total_squishes, 1);

        // Row 0 should now be empty after clearing
        for col in 0..CargoCompressorGrid::WIDTH {
            assert!(!grid.cells[0][col]);
        }
    }

    #[test]
    fn test_compressor_out_of_bounds_and_overflow() {
        let mut grid = CargoCompressorGrid::new();

        // I-Bar width 3 dropped at col 2 exceeds width 4 (2 + 3 = 5 > 4)
        let err = grid.drop_shape(PolyominoShape::IBar, 0, 2);
        assert_eq!(err, Err(DropError::OutOfBounds));

        // Fill 5 2x2 squares at col 0 (height 10, filling 0..9)
        for _ in 0..5 {
            grid.drop_shape(PolyominoShape::Square, 0, 0).unwrap();
        }

        // Sixth square should overflow chimney
        let overflow = grid.drop_shape(PolyominoShape::Square, 0, 0);
        assert_eq!(overflow, Err(DropError::Overflow));
        assert!(grid.is_jammed);
    }
}
