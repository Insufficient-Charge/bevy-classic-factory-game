use bevy::prelude::*;

/// All item types available in GRIDLOCK.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum ItemType {
    // --- Raw Ores & Minerals ---
    #[default]
    IronOre,
    CopperOre,
    GoldOre,
    LogicCrystal,
    Stone,
    Clay,

    // --- Intermediate Materials ---
    IronPlate,
    CopperPlate,
    BronzePlate,
    BrassCog,
    WoodenPlank,
    AcousticSineCoil,
    GlassPane,
    IngotCube,

    // --- Polyomino Lattice Shards (Tetris GDD §5.2) ---
    PolyominoDomino,
    PolyominoIBar,
    PolyominoLTromino,
    PolyominoSquare,
    PolyominoTTetromino,
    PolyominoSTetromino,

    // --- Phase 1 Buildings & Machines ---
    WoodenRollerBelt,
    ClockworkWellhead,
    SmeltingHearth,
    StorageChest,

    // --- Phase 2 Multi-Layer & Automated Buildings (GDD §5.2, §8.1, §8.2) ---
    InclineRamp,
    ViaductBelt,
    SwivelTurntable,
    SortingGate,
    CargoCompressor,
    SurveyBeacon,
    LogicDiviner,

    // --- Phase 3 Gemstones & Catalysis (Bejeweled GDD §5.3, §7.1, §7.2, §8.3) ---
    Ruby,
    Sapphire,
    Topaz,
    Emerald,
    Obsidian,
    PrismaticGem,
    AlloyIngot,
    PrismaticCore,
    ChromaticElixir,
    ChromaticAutoclave,
    ResonanceBellJar,

    // --- Phase 4 Sudoku Harmonic Leylines & Power (GDD §5.4 & §8.4) ---
    AcousticMonolith,
    TuningForkPylon,
    HarmonicBridge,
    ResonanceConductor,
    HarmonicRune,
    TuningFork,
}

impl ItemType {
    /// Human-friendly display name.
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::IronOre => "Iron Ore Pebble",
            Self::CopperOre => "Copper Ore Nugget",
            Self::GoldOre => "Gold Ore Nugget",
            Self::LogicCrystal => "Logic Crystal",
            Self::Stone => "Stone Rubble",
            Self::Clay => "Soft River Clay",
            Self::IronPlate => "Iron Plate",
            Self::CopperPlate => "Copper Plate",
            Self::BronzePlate => "Bronze Plate",
            Self::BrassCog => "Brass Clockwork Cog",
            Self::WoodenPlank => "Wooden Plank",
            Self::AcousticSineCoil => "Acoustic Sine Coil",
            Self::GlassPane => "Glass Pane",
            Self::IngotCube => "Ingot Cube",
            Self::PolyominoDomino => "Lattice Domino (1x2)",
            Self::PolyominoIBar => "Lattice I-Bar (1x3)",
            Self::PolyominoLTromino => "Lattice L-Tromino",
            Self::PolyominoSquare => "Lattice Square (2x2)",
            Self::PolyominoTTetromino => "Lattice T-Tetromino",
            Self::PolyominoSTetromino => "Lattice S-Tetromino",
            Self::WoodenRollerBelt => "Wooden Roller Belt",
            Self::ClockworkWellhead => "Clockwork Wellhead",
            Self::SmeltingHearth => "Smelting Hearth",
            Self::StorageChest => "Parchment Storage Chest",
            Self::InclineRamp => "Roller Incline Ramp",
            Self::ViaductBelt => "Elevated Viaduct Belt",
            Self::SwivelTurntable => "Swivel Turntable",
            Self::SortingGate => "Saloon Sorting Gate",
            Self::CargoCompressor => "Vertical Cargo Compressor",
            Self::SurveyBeacon => "Timber Survey Beacon",
            Self::LogicDiviner => "Harmonic Logic Diviner",
            Self::Ruby => "Ruby Sphere",
            Self::Sapphire => "Sapphire Sphere",
            Self::Topaz => "Topaz Sphere",
            Self::Emerald => "Emerald Sphere",
            Self::Obsidian => "Obsidian Sphere",
            Self::PrismaticGem => "Prismatic Opal Sphere",
            Self::AlloyIngot => "Harmonic Alloy Ingot",
            Self::PrismaticCore => "Radiant Prismatic Core",
            Self::ChromaticElixir => "Chromatic Catalyst Elixir",
            Self::ChromaticAutoclave => "Chromatic Autoclave (6x6)",
            Self::ResonanceBellJar => "Resonance Bell Jar",
            Self::AcousticMonolith => "Acoustic Monolith (3x3)",
            Self::TuningForkPylon => "Tuning Fork Pylon",
            Self::HarmonicBridge => "Harmonic Cloud Bridge",
            Self::ResonanceConductor => "Resonance Conductor",
            Self::HarmonicRune => "Harmonic Acoustic Rune",
            Self::TuningFork => "Brass Tuning Fork",
        }
    }

    /// Max stack size as specified in GDD Section 16.5.
    pub const fn max_stack_size(&self) -> u32 {
        match self {
            Self::IronOre
            | Self::CopperOre
            | Self::GoldOre
            | Self::LogicCrystal
            | Self::Stone
            | Self::Clay
            | Self::PolyominoDomino
            | Self::PolyominoIBar
            | Self::PolyominoLTromino
            | Self::PolyominoSquare
            | Self::PolyominoTTetromino
            | Self::PolyominoSTetromino
            | Self::Ruby
            | Self::Sapphire
            | Self::Topaz
            | Self::Emerald
            | Self::Obsidian
            | Self::PrismaticGem
            | Self::PrismaticCore
            | Self::ChromaticElixir
            | Self::HarmonicRune
            | Self::TuningFork => 100, // Raw Ores, Shards, Gemstones & Cores: 100

            Self::IronPlate
            | Self::CopperPlate
            | Self::BronzePlate
            | Self::BrassCog
            | Self::WoodenPlank
            | Self::AcousticSineCoil
            | Self::GlassPane
            | Self::IngotCube
            | Self::AlloyIngot => 200, // Plates, Wire, Cogs, Ingot Cubes, Alloys: 200

            Self::WoodenRollerBelt
            | Self::ClockworkWellhead
            | Self::SmeltingHearth
            | Self::StorageChest
            | Self::InclineRamp
            | Self::ViaductBelt
            | Self::SwivelTurntable
            | Self::SortingGate
            | Self::CargoCompressor
            | Self::SurveyBeacon
            | Self::LogicDiviner
            | Self::ChromaticAutoclave
            | Self::ResonanceBellJar
            | Self::AcousticMonolith
            | Self::TuningForkPylon
            | Self::HarmonicBridge
            | Self::ResonanceConductor => 10, // Buildings & Machines: 10
        }
    }

    /// Returns true if this item is a placeable building.
    pub const fn is_building(&self) -> bool {
        matches!(
            self,
            Self::WoodenRollerBelt
                | Self::ClockworkWellhead
                | Self::SmeltingHearth
                | Self::StorageChest
                | Self::InclineRamp
                | Self::ViaductBelt
                | Self::SwivelTurntable
                | Self::SortingGate
                | Self::CargoCompressor
                | Self::SurveyBeacon
                | Self::LogicDiviner
                | Self::ChromaticAutoclave
                | Self::ResonanceBellJar
                | Self::AcousticMonolith
                | Self::TuningForkPylon
                | Self::HarmonicBridge
                | Self::ResonanceConductor
        )
    }

    /// Returns true if this item belongs to the Sudoku Harmonic power tier.
    pub const fn is_harmonic(&self) -> bool {
        matches!(
            self,
            Self::AcousticMonolith
                | Self::TuningForkPylon
                | Self::HarmonicBridge
                | Self::ResonanceConductor
                | Self::HarmonicRune
                | Self::TuningFork
        )
    }

    /// Returns true if this item is a Tetris polyomino shard.
    pub const fn is_polyomino(&self) -> bool {
        matches!(
            self,
            Self::PolyominoDomino
                | Self::PolyominoIBar
                | Self::PolyominoLTromino
                | Self::PolyominoSquare
                | Self::PolyominoTTetromino
                | Self::PolyominoSTetromino
        )
    }

    /// Returns true if this item is a Bejeweled catalytic gemstone.
    pub const fn is_gemstone(&self) -> bool {
        matches!(
            self,
            Self::Ruby
                | Self::Sapphire
                | Self::Topaz
                | Self::Emerald
                | Self::Obsidian
                | Self::PrismaticGem
        )
    }
}

/// A discrete stack of items in an inventory slot.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Reflect)]
pub struct ItemStack {
    pub item: ItemType,
    pub count: u32,
}

impl ItemStack {
    pub const fn new(item: ItemType, count: u32) -> Self {
        Self { item, count }
    }

    /// Returns remaining capacity in this stack.
    pub fn remaining_capacity(&self) -> u32 {
        self.item.max_stack_size().saturating_sub(self.count)
    }

    /// Returns true if the stack is at or above max capacity.
    pub fn is_full(&self) -> bool {
        self.count >= self.item.max_stack_size()
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<ItemType>()
        .register_type::<ItemStack>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_limits() {
        assert_eq!(ItemType::IronOre.max_stack_size(), 100);
        assert_eq!(ItemType::IronPlate.max_stack_size(), 200);
        assert_eq!(ItemType::WoodenRollerBelt.max_stack_size(), 10);
    }

    #[test]
    fn test_stack_capacity() {
        let mut stack = ItemStack::new(ItemType::IronOre, 60);
        assert_eq!(stack.remaining_capacity(), 40);
        assert!(!stack.is_full());

        stack.count = 100;
        assert_eq!(stack.remaining_capacity(), 0);
        assert!(stack.is_full());
    }

    #[test]
    fn test_is_building() {
        assert!(ItemType::WoodenRollerBelt.is_building());
        assert!(ItemType::ClockworkWellhead.is_building());
        assert!(ItemType::ChromaticAutoclave.is_building());
        assert!(ItemType::ResonanceBellJar.is_building());
        assert!(ItemType::AcousticMonolith.is_building());
        assert!(ItemType::TuningForkPylon.is_building());
        assert!(!ItemType::IronOre.is_building());
        assert!(!ItemType::IronPlate.is_building());
    }

    #[test]
    fn test_gemstones() {
        assert!(ItemType::Ruby.is_gemstone());
        assert!(ItemType::Sapphire.is_gemstone());
        assert!(ItemType::Topaz.is_gemstone());
        assert!(ItemType::Emerald.is_gemstone());
        assert!(ItemType::Obsidian.is_gemstone());
        assert!(ItemType::PrismaticGem.is_gemstone());
        assert!(!ItemType::IronOre.is_gemstone());
        assert!(!ItemType::AlloyIngot.is_gemstone());
    }

    #[test]
    fn test_harmonic_items() {
        assert!(ItemType::AcousticMonolith.is_harmonic());
        assert!(ItemType::TuningForkPylon.is_harmonic());
        assert!(ItemType::HarmonicBridge.is_harmonic());
        assert!(ItemType::ResonanceConductor.is_harmonic());
        assert!(ItemType::HarmonicRune.is_harmonic());
        assert!(ItemType::TuningFork.is_harmonic());
        assert!(!ItemType::IronOre.is_harmonic());
        assert!(!ItemType::Ruby.is_harmonic());
    }
}
