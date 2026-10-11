use bevy::prelude::*;
use std::collections::HashSet;
use super::grid::GridCoord;

/// Musical frequency runes and frequencies (1..=9) in GRIDLOCK (GDD §5.4).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum HarmonicFrequency {
    Rune1 = 1,
    Rune2 = 2,
    Rune3 = 3,
    Rune4 = 4,
    Rune5 = 5,
    Rune6 = 6,
    Rune7 = 7,
    Rune8 = 8,
    Rune9 = 9,
}

impl HarmonicFrequency {
    pub const ALL: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    pub const fn from_u8(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::Rune1),
            2 => Some(Self::Rune2),
            3 => Some(Self::Rune3),
            4 => Some(Self::Rune4),
            5 => Some(Self::Rune5),
            6 => Some(Self::Rune6),
            7 => Some(Self::Rune7),
            8 => Some(Self::Rune8),
            9 => Some(Self::Rune9),
            _ => None,
        }
    }

    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Musical rune glyph representation (GDD §5.4: $♩_1$ through $♬_9$).
    pub const fn rune(&self) -> &'static str {
        match self {
            Self::Rune1 => "♩1",
            Self::Rune2 => "♪2",
            Self::Rune3 => "♫3",
            Self::Rune4 => "♬4",
            Self::Rune5 => "𝅘𝅥𝅯5",
            Self::Rune6 => "𝅘𝅥𝅰6",
            Self::Rune7 => "𝅘𝅥𝅱7",
            Self::Rune8 => "𝅘𝅥𝅲8",
            Self::Rune9 => "𝄞9",
        }
    }

    /// Pastel rainbow color associated with this frequency.
    pub fn color(&self) -> Color {
        match self {
            Self::Rune1 => Color::srgb(0.95, 0.45, 0.50), // Coral Rose
            Self::Rune2 => Color::srgb(0.98, 0.65, 0.40), // Warm Peach
            Self::Rune3 => Color::srgb(0.95, 0.85, 0.35), // Golden Yellow
            Self::Rune4 => Color::srgb(0.40, 0.85, 0.55), // Spring Mint
            Self::Rune5 => Color::srgb(0.30, 0.85, 0.80), // Pastel Turquoise
            Self::Rune6 => Color::srgb(0.45, 0.70, 0.95), // Sky Blue
            Self::Rune7 => Color::srgb(0.60, 0.55, 0.95), // Soft Indigo
            Self::Rune8 => Color::srgb(0.80, 0.50, 0.90), // Radiant Violet
            Self::Rune9 => Color::srgb(0.95, 0.45, 0.80), // Celestial Magenta
        }
    }
}

/// Attached to machines and monoliths to track their Sudoku harmonic tuning (GDD §5.4).
#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct HarmonicTuning {
    /// Tuned frequency: 1..=9, or None for unassigned default.
    pub frequency: Option<u8>,
    /// True for ancient ruins and immovable clues that cannot be changed.
    pub is_locked: bool,
    /// True if another machine in the same row, column, or 3x3 block has the same frequency.
    pub is_conflicted: bool,
    /// True if machine operates in full non-conflicting harmony within a calibrated block.
    pub in_harmony: bool,
}

impl Default for HarmonicTuning {
    fn default() -> Self {
        Self {
            frequency: None,
            is_locked: false,
            is_conflicted: false,
            in_harmony: false,
        }
    }
}

impl HarmonicTuning {
    pub const fn new(frequency: Option<u8>, is_locked: bool) -> Self {
        Self {
            frequency,
            is_locked,
            is_conflicted: false,
            in_harmony: false,
        }
    }

    /// Speed multiplier governed by acoustic calibration (GDD §5.4):
    /// - Conflicted (flutter) or Unassigned: 0.5x speed penalty
    /// - Harmonious (Harmonic Symphony Bonus): 1.5x speed boost
    /// - Calibrated neutral: 1.0x baseline speed
    pub const fn speed_multiplier(&self) -> f32 {
        if self.is_conflicted || self.frequency.is_none() {
            0.5
        } else if self.in_harmony {
            1.5
        } else {
            1.0
        }
    }
}

/// An ancient stepped stone obelisk generating radiant power when grove is harmonic (GDD §5.4 & §8.4).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct AcousticMonolith {
    pub baseline_power_kw: f32,
    pub is_active: bool,
}

impl Default for AcousticMonolith {
    fn default() -> Self {
        Self {
            baseline_power_kw: 250.0,
            is_active: false,
        }
    }
}

/// Oversized brass tuning fork broadcasting valid frequencies to nearby structures (GDD §5.4 & §8.4).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct TuningForkPylon {
    pub radius: i32,
    pub target_frequency: Option<u8>,
    pub timer: Timer,
}

impl Default for TuningForkPylon {
    fn default() -> Self {
        Self {
            radius: 3,
            target_frequency: None,
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

/// Cute clockwork owl automaton that automatically re-tunes machines (GDD §5.4 & §8.4).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ResonanceConductor {
    pub timer: Timer,
    pub auto_tune_enabled: bool,
    pub total_tunes: u32,
}

impl Default for ResonanceConductor {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            auto_tune_enabled: true,
            total_tunes: 0,
        }
    }
}

/// Elevated resonant bridge connecting sky islands with acoustic resonance conduits (GDD §5.4 & §8.4).
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HarmonicBridge;

/// Summary of a 9x9 grove validation pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GroveValidationSummary {
    pub conflicts: HashSet<(usize, usize)>,
    pub completed_blocks: [bool; 9],
    pub is_full_symphony: bool,
    pub total_harmonic_cells: usize,
    pub generated_power_kw: f32,
}

/// A 9x9 sector grid on the diorama island governed by ancient Sudoku acoustic laws (GDD §5.4).
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct ResonanceGrove {
    pub origin: GridCoord,
    pub width: usize,
    pub height: usize,
    pub completed_blocks: [bool; 9],
    pub is_full_symphony: bool,
    pub total_power_kw: f32,
    pub conflict_count: usize,
}

impl Default for ResonanceGrove {
    fn default() -> Self {
        Self::new(GridCoord::ground(16, 0))
    }
}

impl ResonanceGrove {
    pub const SIZE: usize = 9;

    pub const fn new(origin: GridCoord) -> Self {
        Self {
            origin,
            width: Self::SIZE,
            height: Self::SIZE,
            completed_blocks: [false; 9],
            is_full_symphony: false,
            total_power_kw: 0.0,
            conflict_count: 0,
        }
    }

    /// Converts a world GridCoord to grove-local `(row, col)` if within bounds.
    pub fn local_coords(&self, coord: GridCoord) -> Option<(usize, usize)> {
        if coord.layer != self.origin.layer {
            return None;
        }
        let dx = coord.x - self.origin.x;
        let dy = coord.y - self.origin.y;
        if dx >= 0 && dx < self.width as i32 && dy >= 0 && dy < self.height as i32 {
            Some((dy as usize, dx as usize))
        } else {
            None
        }
    }

    /// Converts grove-local `(row, col)` to a world GridCoord.
    pub fn world_coord(&self, row: usize, col: usize) -> GridCoord {
        GridCoord::new(
            self.origin.x + col as i32,
            self.origin.y + row as i32,
            self.origin.layer,
        )
    }

    /// Computes the 3x3 block index `0..9` for `(row, col)`.
    pub const fn block_index(row: usize, col: usize) -> usize {
        (row / 3) * 3 + (col / 3)
    }

    /// Validates an entire 9x9 frequency grid against Sudoku constraints.
    pub fn validate_board(&mut self, grid: &[[Option<u8>; 9]; 9]) -> GroveValidationSummary {
        let mut conflicts = HashSet::new();

        // 1. Check rows
        for r in 0..9 {
            for c1 in 0..9 {
                if let Some(f1) = grid[r][c1] {
                    for c2 in (c1 + 1)..9 {
                        if let Some(f2) = grid[r][c2] {
                            if f1 == f2 {
                                conflicts.insert((r, c1));
                                conflicts.insert((r, c2));
                            }
                        }
                    }
                }
            }
        }

        // 2. Check columns
        for c in 0..9 {
            for r1 in 0..9 {
                if let Some(f1) = grid[r1][c] {
                    for r2 in (r1 + 1)..9 {
                        if let Some(f2) = grid[r2][c] {
                            if f1 == f2 {
                                conflicts.insert((r1, c));
                                conflicts.insert((r2, c));
                            }
                        }
                    }
                }
            }
        }

        // 3. Check 3x3 sub-blocks
        let mut completed_blocks = [false; 9];
        for b in 0..9 {
            let br = (b / 3) * 3;
            let bc = (b % 3) * 3;
            let mut seen = HashSet::new();
            let mut block_has_conflict = false;
            let mut filled_count = 0;

            for dr in 0..3 {
                for dc in 0..3 {
                    let r = br + dr;
                    let c = bc + dc;
                    if let Some(f) = grid[r][c] {
                        filled_count += 1;
                        if !seen.insert(f) {
                            block_has_conflict = true;
                            conflicts.insert((r, c));
                            // Also mark earlier instance in this block
                            for dr2 in 0..3 {
                                for dc2 in 0..3 {
                                    let r2 = br + dr2;
                                    let c2 = bc + dc2;
                                    if (r2, c2) != (r, c) && grid[r2][c2] == Some(f) {
                                        conflicts.insert((r2, c2));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Block is harmonious if fully filled with all 9 digits and zero conflicts
            if filled_count == 9 && !block_has_conflict {
                completed_blocks[b] = true;
            }
        }

        let is_full_symphony = completed_blocks.iter().all(|&b| b) && conflicts.is_empty();
        let mut total_harmonic_cells = 0;
        for r in 0..9 {
            for c in 0..9 {
                let b = Self::block_index(r, c);
                if grid[r][c].is_some() && !conflicts.contains(&(r, c)) && (completed_blocks[b] || is_full_symphony) {
                    total_harmonic_cells += 1;
                }
            }
        }

        let generated_power_kw = if is_full_symphony {
            500.0
        } else {
            completed_blocks.iter().filter(|&&b| b).count() as f32 * 50.0
        };

        self.completed_blocks = completed_blocks;
        self.is_full_symphony = is_full_symphony;
        self.total_power_kw = generated_power_kw;
        self.conflict_count = conflicts.len();

        GroveValidationSummary {
            conflicts,
            completed_blocks,
            is_full_symphony,
            total_harmonic_cells,
            generated_power_kw,
        }
    }

    /// Finds the lowest non-conflicting frequency (1..=9) for `(row, col)`, if one exists.
    pub fn find_valid_frequency(&self, grid: &[[Option<u8>; 9]; 9], row: usize, col: usize) -> Option<u8> {
        let mut used = [false; 10]; // 1-indexed

        // Row used
        for c in 0..9 {
            if c != col {
                if let Some(f) = grid[row][c] {
                    if (1..=9).contains(&f) {
                        used[f as usize] = true;
                    }
                }
            }
        }

        // Col used
        for r in 0..9 {
            if r != row {
                if let Some(f) = grid[r][col] {
                    if (1..=9).contains(&f) {
                        used[f as usize] = true;
                    }
                }
            }
        }

        // 3x3 block used
        let br = (row / 3) * 3;
        let bc = (col / 3) * 3;
        for dr in 0..3 {
            for dc in 0..3 {
                let r = br + dr;
                let c = bc + dc;
                if (r, c) != (row, col) {
                    if let Some(f) = grid[r][c] {
                        if (1..=9).contains(&f) {
                            used[f as usize] = true;
                        }
                    }
                }
            }
        }

        for f in 1..=9 {
            if !used[f] {
                return Some(f as u8);
            }
        }

        None
    }

    /// Solves an entire 9x9 grid via recursive backtracking Sudoku resolution.
    pub fn solve_sudoku(grid: &mut [[Option<u8>; 9]; 9]) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if grid[r][c].is_none() {
                    for f in 1..=9 {
                        let mut can_place = true;
                        // Row check
                        for col_idx in 0..9 {
                            if grid[r][col_idx] == Some(f) {
                                can_place = false;
                                break;
                            }
                        }
                        if !can_place { continue; }

                        // Col check
                        for row_idx in 0..9 {
                            if grid[row_idx][c] == Some(f) {
                                can_place = false;
                                break;
                            }
                        }
                        if !can_place { continue; }

                        // Block check
                        let br = (r / 3) * 3;
                        let bc = (c / 3) * 3;
                        for dr in 0..3 {
                            for dc in 0..3 {
                                if grid[br + dr][bc + dc] == Some(f) {
                                    can_place = false;
                                    break;
                                }
                            }
                            if !can_place { break; }
                        }
                        if !can_place { continue; }

                        grid[r][c] = Some(f);
                        if Self::solve_sudoku(grid) {
                            return true;
                        }
                        grid[r][c] = None;
                    }
                    return false;
                }
            }
        }
        true
    }
}

/// Event triggered when a 3x3 block or full 9x9 grove achieves harmonic calibration.
#[derive(Event, Clone, Debug)]
pub struct HarmonicSymphonyEvent {
    pub block_index: Option<usize>,
    pub is_full_grove: bool,
    pub bonus_power_kw: f32,
}

/// Event triggered when a machine's frequency is tuned or re-calibrated.
#[derive(Event, Clone, Debug)]
pub struct FrequencyTunedEvent {
    pub entity: Entity,
    pub coord: GridCoord,
    pub frequency: Option<u8>,
}

/// System that periodically validates the Resonance Grove, marking conflicts and harmony flags.
pub fn update_resonance_groves(
    mut grove: ResMut<ResonanceGrove>,
    mut tunings: Query<(Entity, &mut HarmonicTuning, &super::building::Building)>,
    mut monoliths: Query<(&mut AcousticMonolith, &super::building::Building)>,
    mut commands: Commands,
) {
    let mut board = [[None; 9]; 9];
    let mut entities_by_local = [[None; 9]; 9];

    for (ent, tuning, bld) in tunings.iter() {
        if let Some((r, c)) = grove.local_coords(bld.coord) {
            board[r][c] = tuning.frequency;
            entities_by_local[r][c] = Some(ent);
        }
    }

    let prev_symphony = grove.is_full_symphony;
    let summary = grove.validate_board(&board);

    // Apply conflict and harmony flags to all entities
    for (_ent, mut tuning, bld) in tunings.iter_mut() {
        if let Some((r, c)) = grove.local_coords(bld.coord) {
            let is_conflicted = summary.conflicts.contains(&(r, c));
            let b = ResonanceGrove::block_index(r, c);
            let in_harmony = tuning.frequency.is_some()
                && !is_conflicted
                && (summary.completed_blocks[b] || summary.is_full_symphony);

            tuning.is_conflicted = is_conflicted;
            tuning.in_harmony = in_harmony;
        }
    }

    // Activate monolith power if part of harmonious grove or block
    for (mut monolith, bld) in monoliths.iter_mut() {
        if let Some((r, c)) = grove.local_coords(bld.coord) {
            let b = ResonanceGrove::block_index(r, c);
            monolith.is_active = summary.completed_blocks[b] || summary.is_full_symphony;
        }
    }

    // Trigger celebration event when full grove achieves harmony
    if summary.is_full_symphony && !prev_symphony {
        commands.trigger(HarmonicSymphonyEvent {
            block_index: None,
            is_full_grove: true,
            bonus_power_kw: summary.generated_power_kw,
        });
    }
}

/// System that ticks Tuning Fork Pylons and broadcasts harmonic frequencies to neighbors.
pub fn update_tuning_fork_pylons(
    time: Res<Time>,
    mut pylons: Query<(&mut TuningForkPylon, &super::building::Building)>,
    grove: Option<Res<ResonanceGrove>>,
    mut tunings: Query<(Entity, &mut HarmonicTuning, &super::building::Building)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let Some(grove) = grove else { return; };

    let mut temp_board = [[None; 9]; 9];
    for (_, t, b) in tunings.iter() {
        if let Some((lr, lc)) = grove.local_coords(b.coord) {
            temp_board[lr][lc] = t.frequency;
        }
    }

    for (mut pylon, pylon_bld) in pylons.iter_mut() {
        pylon.timer.tick(dt);
        if !pylon.timer.just_finished() {
            continue;
        }

        let r = pylon.radius;
        for (ent, mut tuning, bld) in tunings.iter_mut() {
            if tuning.frequency.is_none() && !tuning.is_locked {
                let dx = (pylon_bld.coord.x - bld.coord.x).abs();
                let dy = (pylon_bld.coord.y - bld.coord.y).abs();
                if dx <= r && dy <= r {
                    if let Some(target) = pylon.target_frequency {
                        tuning.frequency = Some(target);
                        if let Some((lr, lc)) = grove.local_coords(bld.coord) {
                            temp_board[lr][lc] = Some(target);
                        }
                        commands.trigger(FrequencyTunedEvent {
                            entity: ent,
                            coord: bld.coord,
                            frequency: Some(target),
                        });
                        break;
                    } else if let Some((row, col)) = grove.local_coords(bld.coord) {
                        if let Some(valid_f) = grove.find_valid_frequency(&temp_board, row, col) {
                            tuning.frequency = Some(valid_f);
                            temp_board[row][col] = Some(valid_f);
                            commands.trigger(FrequencyTunedEvent {
                                entity: ent,
                                coord: bld.coord,
                                frequency: Some(valid_f),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// System that ticks Resonance Conductors and re-tunes untuned or conflicted machines.
pub fn update_resonance_conductors(
    time: Res<Time>,
    mut conductors: Query<&mut ResonanceConductor>,
    grove: Option<Res<ResonanceGrove>>,
    mut tunings: Query<(Entity, &mut HarmonicTuning, &super::building::Building)>,
    mut commands: Commands,
) {
    let dt = time.delta();
    let Some(grove) = grove else { return; };

    for mut conductor in conductors.iter_mut() {
        if !conductor.auto_tune_enabled {
            continue;
        }
        conductor.timer.tick(dt);
        if !conductor.timer.just_finished() {
            continue;
        }

        let mut temp_board = [[None; 9]; 9];
        for (_, t, b) in tunings.iter() {
            if let Some((lr, lc)) = grove.local_coords(b.coord) {
                temp_board[lr][lc] = t.frequency;
            }
        }

        // Find first conflicted or untuned non-locked machine
        for (ent, mut tuning, bld) in tunings.iter_mut() {
            if !tuning.is_locked && (tuning.frequency.is_none() || tuning.is_conflicted) {
                if let Some((row, col)) = grove.local_coords(bld.coord) {
                    if let Some(valid_f) = grove.find_valid_frequency(&temp_board, row, col) {
                        tuning.frequency = Some(valid_f);
                        conductor.total_tunes += 1;
                        commands.trigger(FrequencyTunedEvent {
                            entity: ent,
                            coord: bld.coord,
                            frequency: Some(valid_f),
                        });
                        break;
                    }
                }
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<HarmonicFrequency>()
        .register_type::<HarmonicTuning>()
        .register_type::<AcousticMonolith>()
        .register_type::<TuningForkPylon>()
        .register_type::<ResonanceConductor>()
        .register_type::<HarmonicBridge>()
        .register_type::<ResonanceGrove>()
        .init_resource::<ResonanceGrove>()
        .add_systems(
            Update,
            (
                update_resonance_groves,
                update_tuning_fork_pylons,
                update_resonance_conductors,
            ),
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_runes_and_colors() {
        for f in 1..=9 {
            let freq = HarmonicFrequency::from_u8(f).expect("Should parse frequency 1..=9");
            assert_eq!(freq.to_u8(), f);
            assert!(!freq.rune().is_empty());
            let _ = freq.color();
        }
        assert!(HarmonicFrequency::from_u8(0).is_none());
        assert!(HarmonicFrequency::from_u8(10).is_none());
    }

    #[test]
    fn test_sudoku_row_col_block_conflict_detection() {
        let mut grove = ResonanceGrove::new(GridCoord::ground(0, 0));
        let mut board = [[None; 9]; 9];

        // Place conflicting frequencies in row 0
        board[0][0] = Some(3);
        board[0][5] = Some(3); // Conflict in row 0!

        // Place conflicting frequencies in column 2
        board[1][2] = Some(7);
        board[8][2] = Some(7); // Conflict in column 2!

        // Place conflicting frequencies in 3x3 block 0 (rows 0..3, cols 0..3)
        board[1][1] = Some(5);
        board[2][2] = Some(5); // Conflict in block 0!

        let summary = grove.validate_board(&board);
        assert!(summary.conflicts.contains(&(0, 0)));
        assert!(summary.conflicts.contains(&(0, 5)));
        assert!(summary.conflicts.contains(&(1, 2)));
        assert!(summary.conflicts.contains(&(8, 2)));
        assert!(summary.conflicts.contains(&(1, 1)));
        assert!(summary.conflicts.contains(&(2, 2)));
        assert!(!summary.is_full_symphony);
    }

    #[test]
    fn test_harmonic_symphony_sub_block_completion() {
        let mut grove = ResonanceGrove::new(GridCoord::ground(0, 0));
        let mut board = [[None; 9]; 9];

        // Fill sub-block 0 (rows 0..3, cols 0..3) with all digits 1..=9
        let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut idx = 0;
        for dr in 0..3 {
            for dc in 0..3 {
                board[dr][dc] = Some(digits[idx]);
                idx += 1;
            }
        }

        let summary = grove.validate_board(&board);
        assert!(summary.conflicts.is_empty());
        assert!(summary.completed_blocks[0], "Block 0 should be completed and harmonious");
        assert_eq!(summary.generated_power_kw, 50.0);
    }

    #[test]
    fn test_full_9x9_sudoku_solver_and_harmony() {
        let mut grove = ResonanceGrove::new(GridCoord::ground(0, 0));
        let mut board = [[None; 9]; 9];

        // Known solvable Sudoku initial clues
        board[0][0] = Some(5);
        board[0][1] = Some(3);
        board[0][4] = Some(7);
        board[1][0] = Some(6);
        board[1][3] = Some(1);
        board[1][4] = Some(9);
        board[1][5] = Some(5);
        board[2][1] = Some(9);
        board[2][2] = Some(8);
        board[2][7] = Some(6);

        let solved = ResonanceGrove::solve_sudoku(&mut board);
        assert!(solved, "Backtracking solver should successfully solve the Sudoku board");

        let summary = grove.validate_board(&board);
        assert!(summary.conflicts.is_empty(), "Solved board must have zero conflicts");
        assert!(summary.is_full_symphony, "Solved board should trigger full Harmonic Symphony Bonus");
        assert_eq!(summary.generated_power_kw, 500.0);
    }

    #[test]
    fn test_tuning_speed_multiplier() {
        let unassigned = HarmonicTuning::default();
        assert_eq!(unassigned.speed_multiplier(), 0.5);

        let mut calibrated = HarmonicTuning::new(Some(4), false);
        assert_eq!(calibrated.speed_multiplier(), 1.0);

        calibrated.is_conflicted = true;
        assert_eq!(calibrated.speed_multiplier(), 0.5);

        calibrated.is_conflicted = false;
        calibrated.in_harmony = true;
        assert_eq!(calibrated.speed_multiplier(), 1.5);
    }

    #[test]
    fn test_tuning_fork_valid_frequency_lookup() {
        let grove = ResonanceGrove::new(GridCoord::ground(0, 0));
        let mut board = [[None; 9]; 9];

        // Fill row 0 with 1..=8 except column 8
        for c in 0..8 {
            board[0][c] = Some((c + 1) as u8);
        }

        let f = grove.find_valid_frequency(&board, 0, 8);
        assert_eq!(f, Some(9), "Only frequency 9 is valid for the remaining cell in row 0");
    }
}
