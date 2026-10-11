use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::grid::{GridCoord, Layer};

/// Types of terrain voxels found on floating diorama islands.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum VoxelType {
    #[default]
    Grass,
    StoneCliff,
    Clay,
    RiverWater,
    RunicSlate,
    CrystalOutcrop,
}

/// State of a terrain tile on the prospecting grid.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum TileProspectState {
    /// Hidden under lush turf (Minesweeper covered)
    #[default]
    Covered,
    /// Unfolded to show runic slate and hazard count (Minesweeper revealed)
    Revealed,
    /// Marked with a wooden pin to prevent drilling (Minesweeper flagged)
    Flagged,
    /// Clockwork wellhead placed, extracting minerals
    Extracted,
}

/// A physical terrain tile entity component.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct TerrainTile {
    pub coord: GridCoord,
    pub voxel_type: VoxelType,
    pub prospect_state: TileProspectState,
    pub has_overcharge_hazard: bool,
    pub adjacent_hazards: u8,
}

impl TerrainTile {
    pub fn new(coord: GridCoord, voxel_type: VoxelType, has_hazard: bool) -> Self {
        Self {
            coord,
            voxel_type,
            prospect_state: TileProspectState::Covered,
            has_overcharge_hazard: has_hazard,
            adjacent_hazards: 0,
        }
    }
}

/// In-memory grid state for an island sector.
#[derive(Resource, Default, Debug)]
pub struct IslandGrid {
    pub tiles: HashMap<GridCoord, TerrainTile>,
}

impl IslandGrid {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    /// Insert or update a tile in the grid.
    pub fn insert_tile(&mut self, tile: TerrainTile) {
        self.tiles.insert(tile.coord, tile);
    }

    /// Helper to add a basic terrain tile with or without hazard.
    pub fn add_tile(&mut self, coord: GridCoord, has_hazard: bool) {
        self.insert_tile(TerrainTile::new(coord, VoxelType::Grass, has_hazard));
    }

    /// Recalculates adjacent hazard numbers across all tiles in the grid.
    pub fn recompute_all_adjacent_hazards(&mut self) {
        let hazard_positions: HashSet<GridCoord> = self
            .tiles
            .iter()
            .filter(|(_, t)| t.has_overcharge_hazard)
            .map(|(c, _)| *c)
            .collect();

        for (coord, tile) in self.tiles.iter_mut() {
            let mut count = 0;
            for neighbor in coord.neighbors_all_8() {
                if hazard_positions.contains(&neighbor) {
                    count += 1;
                }
            }
            tile.adjacent_hazards = count;
        }
    }

    /// Unfolds (reveals) a tile at the given coordinate.
    /// Unfolds (reveals) a tile at the given coordinate.
    /// Returns (hazard_struck, revealed_coords) where hazard_struck indicates if an overcharge vent
    /// was struck, and revealed_coords contains all tiles revealed (including cascade expansion).
    pub fn unfold_tile_with_revealed(
        &mut self,
        coord: &GridCoord,
    ) -> Result<(bool, Vec<GridCoord>), &'static str> {
        let tile = self.tiles.get(coord).ok_or("Tile not found")?;

        if tile.prospect_state == TileProspectState::Flagged {
            return Err("Cannot unfold flagged tile");
        }
        if tile.prospect_state == TileProspectState::Revealed {
            return Ok((false, Vec::new())); // Already revealed
        }

        let is_hazard = tile.has_overcharge_hazard;
        if is_hazard {
            if let Some(t) = self.tiles.get_mut(coord) {
                t.prospect_state = TileProspectState::Revealed;
            }
            return Ok((true, vec![*coord])); // Hazard struck!
        }

        // Perform reveal and cascade if zero hazards adjacent
        let mut queue = vec![*coord];
        let mut visited = HashSet::new();
        let mut revealed = Vec::new();

        while let Some(current) = queue.pop() {
            if !visited.insert(current) {
                continue;
            }

            if let Some(t) = self.tiles.get_mut(&current) {
                if t.prospect_state == TileProspectState::Flagged || t.has_overcharge_hazard {
                    continue;
                }
                t.prospect_state = TileProspectState::Revealed;
                revealed.push(current);

                // Cascade expand on zero
                if t.adjacent_hazards == 0 {
                    for n in current.neighbors_all_8() {
                        if let Some(neighbor_tile) = self.tiles.get(&n) {
                            if neighbor_tile.prospect_state == TileProspectState::Covered {
                                queue.push(n);
                            }
                        }
                    }
                }
            }
        }

        Ok((false, revealed))
    }

    /// Unfolds (reveals) a tile at the given coordinate.
    /// Returns true if a hazard was struck (playful steam pop!), false if safe.
    /// If safe and adjacent_hazards == 0, performs standard Minesweeper cascade unfold.
    pub fn unfold_tile(&mut self, coord: &GridCoord) -> Result<bool, &'static str> {
        let (hazard, _) = self.unfold_tile_with_revealed(coord)?;
        Ok(hazard)
    }

    /// Toggles a wooden safety flag on an unrevealed tile.
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
            TileProspectState::Revealed | TileProspectState::Extracted => {
                Err("Cannot flag an already revealed or extracted tile")
            }
        }
    }

    /// Helper to generate a rectangular island plateau with arbitrary bounds on Ground layer (Z0).
    pub fn generate_rectangular_island_bounds(
        &mut self,
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
        hazards: &HashSet<(i32, i32)>,
    ) {
        for x in min_x..max_x {
            for y in min_y..max_y {
                let coord = GridCoord::new(x, y, Layer::Ground);
                let has_hazard = hazards.contains(&(x, y));
                let tile = TerrainTile::new(coord, VoxelType::Grass, has_hazard);
                self.insert_tile(tile);
            }
        }
        self.recompute_all_adjacent_hazards();
    }

    /// Helper to generate a rectangular island plateau on Ground layer (Z0).
    pub fn generate_rectangular_island(
        &mut self,
        width: i32,
        height: i32,
        hazards: &HashSet<(i32, i32)>,
    ) {
        self.generate_rectangular_island_bounds(0, width, 0, height, hazards);
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<IslandGrid>()
        .register_type::<TerrainTile>()
        .register_type::<TileProspectState>()
        .register_type::<VoxelType>()
        .register_type::<GridCoord>()
        .register_type::<Layer>()
        .register_type::<super::grid::Direction>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_hazard_computation() {
        let mut grid = IslandGrid::new();
        let mut hazards = HashSet::new();
        // Place one hazard at (1, 1)
        hazards.insert((1, 1));
        grid.generate_rectangular_island(3, 3, &hazards);

        // Center is the hazard itself
        let center = grid.tiles.get(&GridCoord::ground(1, 1)).unwrap();
        assert!(center.has_overcharge_hazard);

        // All 8 surrounding tiles in a 3x3 grid must have adjacent_hazards = 1
        for n in GridCoord::ground(1, 1).neighbors_all_8() {
            let neighbor_tile = grid.tiles.get(&n).unwrap();
            assert_eq!(neighbor_tile.adjacent_hazards, 1);
        }
    }

    #[test]
    fn test_unfold_cascade_on_zero() {
        let mut grid = IslandGrid::new();
        let mut hazards = HashSet::new();
        // Place hazard far away at (4, 4)
        hazards.insert((4, 4));
        grid.generate_rectangular_island(5, 5, &hazards);

        // Unfold (0, 0) which has 0 adjacent hazards
        let pop = grid.unfold_tile(&GridCoord::ground(0, 0)).expect("Unfold failed");
        assert_eq!(pop, false, "Should not hit hazard");

        // (0, 0), (1, 0), (0, 1) should be revealed by cascade
        let t00 = grid.tiles.get(&GridCoord::ground(0, 0)).unwrap();
        let t10 = grid.tiles.get(&GridCoord::ground(1, 0)).unwrap();
        let t01 = grid.tiles.get(&GridCoord::ground(0, 1)).unwrap();

        assert_eq!(t00.prospect_state, TileProspectState::Revealed);
        assert_eq!(t10.prospect_state, TileProspectState::Revealed);
        assert_eq!(t01.prospect_state, TileProspectState::Revealed);

        // The hazard tile at (4, 4) must remain Covered!
        let hazard_tile = grid.tiles.get(&GridCoord::ground(4, 4)).unwrap();
        assert_eq!(hazard_tile.prospect_state, TileProspectState::Covered);
    }

    #[test]
    fn test_flag_prevents_accidental_unfold() {
        let mut grid = IslandGrid::new();
        let mut hazards = HashSet::new();
        hazards.insert((1, 1));
        grid.generate_rectangular_island(3, 3, &hazards);

        let target = GridCoord::ground(1, 1);
        // Flag the hazard
        let state = grid.toggle_flag(&target).unwrap();
        assert_eq!(state, TileProspectState::Flagged);

        // Attempting to unfold must return an error
        let result = grid.unfold_tile(&target);
        assert!(result.is_err());

        // Unflag
        let state2 = grid.toggle_flag(&target).unwrap();
        assert_eq!(state2, TileProspectState::Covered);

        // Now unfolding triggers the hazard!
        let pop = grid.unfold_tile(&target).unwrap();
        assert_eq!(pop, true);
    }

    #[test]
    fn test_rectangular_island_bounds() {
        let mut grid = IslandGrid::new();
        let hazards = HashSet::new();
        // Generate island spanning negative and positive coordinates (-5..13, -5..13)
        grid.generate_rectangular_island_bounds(-5, 13, -5, 13, &hazards);
        assert_eq!(grid.tiles.len(), 18 * 18);
        assert!(grid.tiles.contains_key(&GridCoord::ground(-5, -5)));
        assert!(grid.tiles.contains_key(&GridCoord::ground(12, 12)));
        assert!(!grid.tiles.contains_key(&GridCoord::ground(13, 13)));
    }
}
