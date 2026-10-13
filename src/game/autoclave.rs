use bevy::prelude::*;
use super::grid::{Direction, GridCoord};
use super::items::ItemType;
use std::collections::HashSet;

/// Gemstone colors for the 6x6 Chromatic Autoclave diorama tray (GDD §5.3 & §8.3).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum GemColor {
    Ruby,       // Red - Catalyzes Iron into Refined Iron & Heat
    Sapphire,   // Blue - Catalyzes Copper into Refined Copper & Chill
    Topaz,      // Yellow - Catalyzes Gold into Refined Gold & Lightning
    Emerald,    // Green - Catalyzes Logic Crystal into Refined Silicon & Logic
    Obsidian,   // Purple - Catalyzes Voidite into Void Alloys
    Prismatic,  // Opal/Rainbow - Wildcard that matches any color
}

impl GemColor {
    /// Checks if two gems match (handling Prismatic Opal wildcards).
    pub fn matches_with(&self, other: GemColor) -> bool {
        *self == other || *self == GemColor::Prismatic || other == GemColor::Prismatic
    }

    /// Convert from an inventory ItemType into a tray GemColor.
    pub fn from_item_type(item: ItemType) -> Option<Self> {
        match item {
            ItemType::Ruby => Some(Self::Ruby),
            ItemType::Sapphire => Some(Self::Sapphire),
            ItemType::Topaz => Some(Self::Topaz),
            ItemType::Emerald => Some(Self::Emerald),
            ItemType::Obsidian => Some(Self::Obsidian),
            ItemType::PrismaticGem => Some(Self::Prismatic),
            // Ingot cubes also dissolve into gemstone spheres
            ItemType::IronPlate | ItemType::IronOre => Some(Self::Ruby),
            ItemType::CopperPlate | ItemType::CopperOre => Some(Self::Sapphire),
            ItemType::GoldOre => Some(Self::Topaz),
            ItemType::LogicCrystal => Some(Self::Emerald),
            _ => None,
        }
    }

    /// Convert this gem to its corresponding catalytic ItemType.
    pub fn to_item_type(&self) -> ItemType {
        match self {
            Self::Ruby => ItemType::Ruby,
            Self::Sapphire => ItemType::Sapphire,
            Self::Topaz => ItemType::Topaz,
            Self::Emerald => ItemType::Emerald,
            Self::Obsidian => ItemType::Obsidian,
            Self::Prismatic => ItemType::PrismaticGem,
        }
    }

    pub fn char_code(&self) -> char {
        match self {
            Self::Ruby => 'R',
            Self::Sapphire => 'S',
            Self::Topaz => 'T',
            Self::Emerald => 'E',
            Self::Obsidian => 'O',
            Self::Prismatic => '*',
        }
    }
}

/// Types of match patterns detected in the Autoclave tray.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Reflect)]
pub enum MatchType {
    Match3,
    Match4,
    Match5,
    TOrLShape,
}

/// A matched group of gems on the 6x6 board.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct MatchGroup {
    pub color: GemColor,
    pub match_type: MatchType,
    pub cells: Vec<(usize, usize)>,
}

/// Comprehensive outcome of an Autoclave swap and any resulting cascades.
#[derive(Clone, Debug, Default, PartialEq, Reflect)]
pub struct MatchSummary {
    pub matches: Vec<MatchGroup>,
    pub total_refined_ingots: u32,
    pub total_alloy_ingots: u32,
    pub total_prismatic_cores: u32,
    pub power_generated_kw: f32,
    pub cascade_count: u32,
    pub max_multiplier: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SwapError {
    OutOfBounds,
    NotAdjacent,
    EmptyCell,
    NoMatch,
}

/// A 6x6 Bejeweled match-3 tray inside a Chromatic Autoclave (GDD §5.3).
#[derive(Clone, Debug, Reflect)]
pub struct AutoclaveGrid {
    pub cells: [[Option<GemColor>; Self::WIDTH]; Self::HEIGHT],
    pub total_swaps: u32,
    pub total_cascades: u32,
    // Random seed offset for refill generation
    seed: u64,
}

impl Default for AutoclaveGrid {
    fn default() -> Self {
        Self::new(42)
    }
}

impl AutoclaveGrid {
    pub const WIDTH: usize = 6;
    pub const HEIGHT: usize = 6;

    /// Creates a newly initialized 6x6 grid with no starting 3-in-a-row matches.
    pub fn new(seed: u64) -> Self {
        let mut grid = Self {
            cells: [[None; Self::WIDTH]; Self::HEIGHT],
            total_swaps: 0,
            total_cascades: 0,
            seed,
        };
        grid.fill_without_initial_matches();
        grid
    }

    /// Returns the gem at `(row, col)`. Row 0 is the bottom, Row 5 is the top.
    pub fn get(&self, row: usize, col: usize) -> Option<GemColor> {
        if row < Self::HEIGHT && col < Self::WIDTH {
            self.cells[row][col]
        } else {
            None
        }
    }

    /// Sets the gem at `(row, col)`.
    pub fn set(&mut self, row: usize, col: usize, gem: Option<GemColor>) {
        if row < Self::HEIGHT && col < Self::WIDTH {
            self.cells[row][col] = gem;
        }
    }

    fn next_gem(&mut self) -> GemColor {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        match (self.seed >> 32) % 5 {
            0 => GemColor::Ruby,
            1 => GemColor::Sapphire,
            2 => GemColor::Topaz,
            3 => GemColor::Emerald,
            _ => GemColor::Obsidian,
        }
    }

    /// Fills the grid ensuring no 3-in-a-row matches exist initially.
    pub fn fill_without_initial_matches(&mut self) {
        for r in 0..Self::HEIGHT {
            for c in 0..Self::WIDTH {
                let mut gem = self.next_gem();
                // Avoid horizontal match-3
                while (c >= 2
                    && self.cells[r][c - 1] == Some(gem)
                    && self.cells[r][c - 2] == Some(gem))
                    // Avoid vertical match-3
                    || (r >= 2
                        && self.cells[r - 1][c] == Some(gem)
                        && self.cells[r - 2][c] == Some(gem))
                {
                    gem = self.next_gem();
                }
                self.cells[r][c] = Some(gem);
            }
        }
    }

    /// Attempts to swap two adjacent gems at `(r1, c1)` and `(r2, c2)`.
    /// If the swap produces at least one match, clears matched gems, triggers gravity drops & refills,
    /// and continues cascading until stable. If no match is produced, rolls back the swap.
    pub fn swap(
        &mut self,
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize,
    ) -> Result<MatchSummary, SwapError> {
        if r1 >= Self::HEIGHT || c1 >= Self::WIDTH || r2 >= Self::HEIGHT || c2 >= Self::WIDTH {
            return Err(SwapError::OutOfBounds);
        }

        // Must be orthogonally adjacent
        let row_diff = (r1 as isize - r2 as isize).abs();
        let col_diff = (c1 as isize - c2 as isize).abs();
        if !((row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1)) {
            return Err(SwapError::NotAdjacent);
        }

        if self.cells[r1][c1].is_none() || self.cells[r2][c2].is_none() {
            return Err(SwapError::EmptyCell);
        }

        // Perform swap tentatively
        let temp = self.cells[r1][c1];
        self.cells[r1][c1] = self.cells[r2][c2];
        self.cells[r2][c2] = temp;

        let initial_matches = self.detect_matches();
        if initial_matches.is_empty() {
            // Roll back swap
            let temp = self.cells[r1][c1];
            self.cells[r1][c1] = self.cells[r2][c2];
            self.cells[r2][c2] = temp;
            return Err(SwapError::NoMatch);
        }

        self.total_swaps += 1;
        let mut summary = MatchSummary::default();
        let mut multiplier = 1u32;
        let mut current_matches = initial_matches;

        // Cascade resolve loop
        while !current_matches.is_empty() {
            if multiplier > 1 {
                self.total_cascades += 1;
            }

            for m in &current_matches {
                match m.match_type {
                    MatchType::Match3 => {
                        summary.total_refined_ingots += multiplier;
                        summary.power_generated_kw += 15.0 * multiplier as f32;
                    }
                    MatchType::Match4 => {
                        summary.total_alloy_ingots += multiplier;
                        summary.power_generated_kw += 60.0 * multiplier as f32;
                    }
                    MatchType::Match5 => {
                        summary.total_prismatic_cores += multiplier;
                        summary.power_generated_kw += 200.0 * multiplier as f32;
                    }
                    MatchType::TOrLShape => {
                        summary.total_alloy_ingots += multiplier;
                        summary.power_generated_kw += 120.0 * multiplier as f32;
                    }
                }
            }

            summary.matches.extend(current_matches.clone());
            summary.max_multiplier = summary.max_multiplier.max(multiplier);

            // Remove matched gems
            let mut cleared_cells = HashSet::new();
            for m in current_matches {
                for pos in m.cells {
                    cleared_cells.insert(pos);
                }
            }
            for (r, c) in cleared_cells {
                self.cells[r][c] = None;
            }

            // Gravity drop downwards towards row 0
            self.apply_gravity_drop_and_refill();

            multiplier += 1;
            summary.cascade_count += 1;

            // Check if falling gems triggered new cascade matches
            current_matches = self.detect_matches();
        }

        Ok(summary)
    }

    /// Detects all collinear lines of 3 or more matching gems and L/T shapes.
    pub fn detect_matches(&self) -> Vec<MatchGroup> {
        let mut h_lines: Vec<(GemColor, Vec<(usize, usize)>)> = Vec::new();
        let mut v_lines: Vec<(GemColor, Vec<(usize, usize)>)> = Vec::new();

        // 1. Horizontal matches
        for (r, row) in self.cells.iter().enumerate().take(Self::HEIGHT) {
            let mut c = 0;
            while c < Self::WIDTH {
                if let Some(color) = row[c] {
                    let mut run = vec![(r, c)];
                    let mut resolved_color = if color == GemColor::Prismatic { None } else { Some(color) };
                    let mut next_c = c + 1;
                    while next_c < Self::WIDTH {
                        if let Some(next_color) = row[next_c] {
                            if next_color == GemColor::Prismatic {
                                run.push((r, next_c));
                                next_c += 1;
                            } else if let Some(rc) = resolved_color {
                                if rc == next_color {
                                    run.push((r, next_c));
                                    next_c += 1;
                                } else {
                                    break;
                                }
                            } else {
                                resolved_color = Some(next_color);
                                run.push((r, next_c));
                                next_c += 1;
                            }
                        } else {
                            break;
                        }
                    }

                    if run.len() >= 3 {
                        let match_color = resolved_color.unwrap_or(GemColor::Prismatic);
                        h_lines.push((match_color, run));
                        c = next_c;
                    } else {
                        c += 1;
                    }
                } else {
                    c += 1;
                }
            }
        }

        // 2. Vertical matches
        for c in 0..Self::WIDTH {
            let mut r = 0;
            while r < Self::HEIGHT {
                if let Some(color) = self.cells[r][c] {
                    let mut run = vec![(r, c)];
                    let mut resolved_color = if color == GemColor::Prismatic { None } else { Some(color) };
                    let mut next_r = r + 1;
                    while next_r < Self::HEIGHT {
                        if let Some(next_color) = self.cells[next_r][c] {
                            if next_color == GemColor::Prismatic {
                                run.push((next_r, c));
                                next_r += 1;
                            } else if let Some(rc) = resolved_color {
                                if rc == next_color {
                                    run.push((next_r, c));
                                    next_r += 1;
                                } else {
                                    break;
                                }
                            } else {
                                resolved_color = Some(next_color);
                                run.push((next_r, c));
                                next_r += 1;
                            }
                        } else {
                            break;
                        }
                    }

                    if run.len() >= 3 {
                        let match_color = resolved_color.unwrap_or(GemColor::Prismatic);
                        v_lines.push((match_color, run));
                        r = next_r;
                    } else {
                        r += 1;
                    }
                } else {
                    r += 1;
                }
            }
        }

        // 3. Combine matches and check for T/L intersections
        let mut results = Vec::new();
        let mut consumed_h = vec![false; h_lines.len()];
        let mut consumed_v = vec![false; v_lines.len()];

        for (h_idx, (h_color, h_cells)) in h_lines.iter().enumerate() {
            for (v_idx, (v_color, v_cells)) in v_lines.iter().enumerate() {
                if h_color.matches_with(*v_color) {
                    // Check for intersection
                    let has_intersection = h_cells.iter().any(|h| v_cells.contains(h));
                    if has_intersection {
                        let mut combined = h_cells.clone();
                        for v in v_cells {
                            if !combined.contains(v) {
                                combined.push(*v);
                            }
                        }
                        consumed_h[h_idx] = true;
                        consumed_v[v_idx] = true;
                        results.push(MatchGroup {
                            color: *h_color,
                            match_type: MatchType::TOrLShape,
                            cells: combined,
                        });
                    }
                }
            }
        }

        // Remaining uncombined horizontal lines
        for (idx, (color, cells)) in h_lines.into_iter().enumerate() {
            if !consumed_h[idx] {
                let match_type = match cells.len() {
                    3 => MatchType::Match3,
                    4 => MatchType::Match4,
                    _ => MatchType::Match5,
                };
                results.push(MatchGroup {
                    color,
                    match_type,
                    cells,
                });
            }
        }

        // Remaining uncombined vertical lines
        for (idx, (color, cells)) in v_lines.into_iter().enumerate() {
            if !consumed_v[idx] {
                let match_type = match cells.len() {
                    3 => MatchType::Match3,
                    4 => MatchType::Match4,
                    _ => MatchType::Match5,
                };
                results.push(MatchGroup {
                    color,
                    match_type,
                    cells,
                });
            }
        }

        results
    }

    /// Drops existing gems downward towards row 0, filling empty cells at the top.
    pub fn apply_gravity_drop_and_refill(&mut self) {
        for c in 0..Self::WIDTH {
            // Compact existing gems down
            let mut write_r = 0;
            for r in 0..Self::HEIGHT {
                if let Some(gem) = self.cells[r][c] {
                    if r != write_r {
                        self.cells[write_r][c] = Some(gem);
                        self.cells[r][c] = None;
                    }
                    write_r += 1;
                }
            }

            // Fill empty rows at the top (from write_r up to HEIGHT)
            for r in write_r..Self::HEIGHT {
                self.cells[r][c] = Some(self.next_gem());
            }
        }
    }

    /// Scans the entire 6x6 board for any valid adjacent swaps that would trigger a match.
    /// Returns a list of `((r1, c1), (r2, c2), match_score)`.
    pub fn find_possible_swaps(&self) -> Vec<((usize, usize), (usize, usize), usize)> {
        let mut possible = Vec::new();

        let deltas = [(0, 1), (1, 0)];
        for r in 0..Self::HEIGHT {
            for c in 0..Self::WIDTH {
                for (dr, dc) in deltas {
                    let r2 = r + dr;
                    let c2 = c + dc;
                    if r2 < Self::HEIGHT && c2 < Self::WIDTH {
                        let mut test_grid = self.clone();
                        let temp = test_grid.cells[r][c];
                        test_grid.cells[r][c] = test_grid.cells[r2][c2];
                        test_grid.cells[r2][c2] = temp;

                        let matches = test_grid.detect_matches();
                        if !matches.is_empty() {
                            let score: usize = matches.iter().map(|m| m.cells.len()).sum();
                            possible.push(((r, c), (r2, c2), score));
                        }
                    }
                }
            }
        }

        possible
    }

    /// Automatically executes the best valid swap on the board (used by the Brass Swap-Piston).
    pub fn auto_swap_best(&mut self) -> Result<MatchSummary, SwapError> {
        let mut possible = self.find_possible_swaps();
        if possible.is_empty() {
            // Board has no valid moves; reshuffle board
            self.fill_without_initial_matches();
            possible = self.find_possible_swaps();
            if possible.is_empty() {
                return Err(SwapError::NoMatch);
            }
        }

        // Sort by highest match score
        possible.sort_by_key(|a| std::cmp::Reverse(a.2));
        let ((r1, c1), (r2, c2), _) = possible[0];
        self.swap(r1, c1, r2, c2)
    }

    /// Renders ASCII representation of the 6x6 grid.
    pub fn to_ascii(&self) -> String {
        let mut s = String::new();
        // Display top to bottom (row 5 down to 0)
        for row in self.cells.iter().rev() {
            for cell in row {
                let ch = cell.map(|g| g.char_code()).unwrap_or('.');
                s.push(ch);
                s.push(' ');
            }
            s.push('\n');
        }
        s
    }
}

/// A placed Chromatic Autoclave structure (GDD §5.3 & §8.3).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ChromaticAutoclave {
    pub facing: Direction,
    pub grid: AutoclaveGrid,
    pub auto_piston_enabled: bool,
    pub auto_piston_timer: Timer,
    pub stored_power_kw: f32,
    pub total_catalysis_reactions: u32,
    pub pending_outputs: Vec<ItemType>,
    pub input_buffer: Vec<ItemType>,
}

impl ChromaticAutoclave {
    pub fn new(facing: Direction) -> Self {
        Self {
            facing,
            grid: AutoclaveGrid::new(1337),
            auto_piston_enabled: true,
            auto_piston_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            stored_power_kw: 0.0,
            total_catalysis_reactions: 0,
            pending_outputs: Vec::new(),
            input_buffer: Vec::new(),
        }
    }
}

/// A Resonance Bell Jar acoustic buffer building (GDD §5.3 & §8.3).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ResonanceBellJar {
    pub stored_charges: u32,
    pub max_charges: u32,
    pub is_resonating: bool,
    pub discharge_timer: Timer,
}

impl Default for ResonanceBellJar {
    fn default() -> Self {
        Self {
            stored_charges: 0,
            max_charges: 5,
            is_resonating: false,
            discharge_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

/// Event triggered when an autoclave completes a catalytic match reaction.
#[derive(Event, Clone, Debug)]
pub struct CatalysisEvent {
    pub entity: Entity,
    pub coord: GridCoord,
    pub summary: MatchSummary,
}

pub fn plugin(app: &mut App) {
    app.register_type::<GemColor>()
        .register_type::<MatchType>()
        .register_type::<MatchGroup>()
        .register_type::<MatchSummary>()
        .register_type::<AutoclaveGrid>()
        .register_type::<ChromaticAutoclave>()
        .register_type::<ResonanceBellJar>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_initialization_no_matches() {
        let grid = AutoclaveGrid::new(100);
        assert!(grid.detect_matches().is_empty(), "Initial grid should have no pre-existing matches");
    }

    #[test]
    fn test_detect_horizontal_match_3() {
        let mut grid = AutoclaveGrid::new(101);
        grid.cells[0][0] = Some(GemColor::Ruby);
        grid.cells[0][1] = Some(GemColor::Ruby);
        grid.cells[0][2] = Some(GemColor::Ruby);
        grid.cells[0][3] = Some(GemColor::Sapphire);
        grid.cells[0][4] = Some(GemColor::Topaz);
        grid.cells[0][5] = Some(GemColor::Emerald);

        let matches = grid.detect_matches();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].match_type, MatchType::Match3);
        assert_eq!(matches[0].cells.len(), 3);
    }

    #[test]
    fn test_detect_match_4_and_match_5() {
        let mut grid = AutoclaveGrid::new(102);
        for cell in grid.cells[1].iter_mut().take(4) {
            *cell = Some(GemColor::Topaz);
        }
        grid.cells[1][4] = Some(GemColor::Ruby);
        grid.cells[1][5] = Some(GemColor::Sapphire);

        let matches = grid.detect_matches();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].match_type, MatchType::Match4);

        for cell in grid.cells[2].iter_mut().take(5) {
            *cell = Some(GemColor::Sapphire);
        }
        grid.cells[2][5] = Some(GemColor::Ruby);
        let matches2 = grid.detect_matches();
        assert!(matches2.iter().any(|m| m.match_type == MatchType::Match5));
    }

    #[test]
    fn test_valid_swap_and_rollback() {
        let mut grid = AutoclaveGrid::new(103);
        // Setup row 0: Ruby, Ruby, Topaz
        // Setup row 1: Emerald, Emerald, Ruby
        // Swapping (0, 2) [Topaz] with (1, 2) [Ruby] should produce 3 Rubies on row 0!
        grid.cells[0][0] = Some(GemColor::Ruby);
        grid.cells[0][1] = Some(GemColor::Ruby);
        grid.cells[0][2] = Some(GemColor::Topaz);

        grid.cells[1][0] = Some(GemColor::Emerald);
        grid.cells[1][1] = Some(GemColor::Emerald);
        grid.cells[1][2] = Some(GemColor::Ruby);

        // Invalid swap first: (0, 0) with (1, 0)
        let invalid = grid.swap(0, 0, 1, 0);
        assert_eq!(invalid, Err(SwapError::NoMatch));
        assert_eq!(grid.cells[0][0], Some(GemColor::Ruby), "Swap should roll back");

        // Valid swap: (0, 2) with (1, 2)
        let valid = grid.swap(0, 2, 1, 2);
        assert!(valid.is_ok());
        let summary = valid.unwrap();
        assert!(summary.total_refined_ingots >= 1);
        assert!(summary.power_generated_kw >= 15.0);
    }

    #[test]
    fn test_prismatic_wildcard_match() {
        let mut grid = AutoclaveGrid::new(104);
        grid.cells[0][0] = Some(GemColor::Ruby);
        grid.cells[0][1] = Some(GemColor::Prismatic);
        grid.cells[0][2] = Some(GemColor::Ruby);
        // Break any matches in rows 1, 2 and columns 0, 1, 2
        grid.cells[1][0] = Some(GemColor::Sapphire);
        grid.cells[1][1] = Some(GemColor::Topaz);
        grid.cells[1][2] = Some(GemColor::Emerald);
        grid.cells[2][0] = Some(GemColor::Topaz);
        grid.cells[2][1] = Some(GemColor::Emerald);
        grid.cells[2][2] = Some(GemColor::Sapphire);

        let matches = grid.detect_matches();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].match_type, MatchType::Match3);
    }

    #[test]
    fn test_auto_swap_best() {
        let mut grid = AutoclaveGrid::new(105);
        // Force a possible swap
        grid.cells[0][0] = Some(GemColor::Sapphire);
        grid.cells[0][1] = Some(GemColor::Sapphire);
        grid.cells[0][2] = Some(GemColor::Topaz);
        grid.cells[1][2] = Some(GemColor::Sapphire);

        let res = grid.auto_swap_best();
        assert!(res.is_ok(), "auto_swap_best should find and execute the match");
    }
}
