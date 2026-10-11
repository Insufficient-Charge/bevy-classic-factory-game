use bevy::prelude::*;
use std::collections::VecDeque;
use super::inventory::Inventory;
use super::items::ItemType;

/// A handcrafting recipe in the Field Workshop.
#[derive(Clone, Copy, Debug)]
pub struct CraftingRecipe {
    pub id: &'static str,
    pub name: &'static str,
    pub inputs: &'static [(ItemType, u32)],
    pub output_item: ItemType,
    pub output_count: u32,
    pub craft_time: f32,
}

/// Standard bootstrapping recipes as specified in GDD Section 14.3 & 14.4.
pub static RECIPES: &[CraftingRecipe] = &[
    CraftingRecipe {
        id: "smelt_iron_plate",
        name: "Smelt Iron Plate",
        inputs: &[(ItemType::IronOre, 2)],
        output_item: ItemType::IronPlate,
        output_count: 1,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "smelt_copper_plate",
        name: "Smelt Copper Plate",
        inputs: &[(ItemType::CopperOre, 2)],
        output_item: ItemType::CopperPlate,
        output_count: 1,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_brass_cog",
        name: "Craft Brass Clockwork Cog",
        inputs: &[(ItemType::IronPlate, 2), (ItemType::CopperPlate, 1)],
        output_item: ItemType::BrassCog,
        output_count: 1,
        craft_time: 2.0,
    },
    CraftingRecipe {
        id: "craft_wooden_plank",
        name: "Craft Wooden Planks",
        inputs: &[(ItemType::Clay, 1)],
        output_item: ItemType::WoodenPlank,
        output_count: 2,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "craft_roller_belt",
        name: "Assemble Wooden Roller Belts",
        inputs: &[(ItemType::IronPlate, 1), (ItemType::WoodenPlank, 1)],
        output_item: ItemType::WoodenRollerBelt,
        output_count: 4,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_wellhead",
        name: "Construct Clockwork Wellhead",
        inputs: &[(ItemType::IronPlate, 4), (ItemType::BrassCog, 2)],
        output_item: ItemType::ClockworkWellhead,
        output_count: 1,
        craft_time: 3.0,
    },
    CraftingRecipe {
        id: "craft_smelting_hearth",
        name: "Construct Smelting Hearth",
        inputs: &[(ItemType::Stone, 4), (ItemType::IronPlate, 2)],
        output_item: ItemType::SmeltingHearth,
        output_count: 1,
        craft_time: 3.0,
    },
    CraftingRecipe {
        id: "craft_storage_chest",
        name: "Assemble Parchment Storage Chest",
        inputs: &[(ItemType::WoodenPlank, 4), (ItemType::IronPlate, 2)],
        output_item: ItemType::StorageChest,
        output_count: 1,
        craft_time: 2.0,
    },
    // --- Phase 2 Multi-Layer & Automated Logistics ---
    CraftingRecipe {
        id: "craft_incline_ramp",
        name: "Assemble Roller Incline Ramp",
        inputs: &[(ItemType::WoodenPlank, 2), (ItemType::IronPlate, 2)],
        output_item: ItemType::InclineRamp,
        output_count: 2,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_viaduct_belt",
        name: "Construct Elevated Viaduct Belt",
        inputs: &[(ItemType::IronPlate, 2), (ItemType::WoodenPlank, 1)],
        output_item: ItemType::ViaductBelt,
        output_count: 4,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_swivel_turntable",
        name: "Assemble Swivel Turntable",
        inputs: &[(ItemType::BrassCog, 2), (ItemType::IronPlate, 1)],
        output_item: ItemType::SwivelTurntable,
        output_count: 1,
        craft_time: 2.0,
    },
    CraftingRecipe {
        id: "craft_sorting_gate",
        name: "Assemble Saloon Sorting Gate",
        inputs: &[(ItemType::BrassCog, 2), (ItemType::WoodenPlank, 2)],
        output_item: ItemType::SortingGate,
        output_count: 1,
        craft_time: 2.0,
    },
    CraftingRecipe {
        id: "craft_glass_pane",
        name: "Refine Glass Pane",
        inputs: &[(ItemType::Stone, 2)],
        output_item: ItemType::GlassPane,
        output_count: 1,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "craft_cargo_compressor",
        name: "Construct Vertical Cargo Compressor",
        inputs: &[
            (ItemType::IronPlate, 4),
            (ItemType::BrassCog, 2),
            (ItemType::GlassPane, 2),
        ],
        output_item: ItemType::CargoCompressor,
        output_count: 1,
        craft_time: 3.5,
    },
    CraftingRecipe {
        id: "craft_survey_beacon",
        name: "Construct Timber Survey Beacon",
        inputs: &[(ItemType::WoodenPlank, 4), (ItemType::BrassCog, 2)],
        output_item: ItemType::SurveyBeacon,
        output_count: 1,
        craft_time: 3.0,
    },
    CraftingRecipe {
        id: "craft_logic_diviner",
        name: "Sculpt Harmonic Logic Diviner",
        inputs: &[(ItemType::Stone, 4), (ItemType::BrassCog, 2)],
        output_item: ItemType::LogicDiviner,
        output_count: 1,
        craft_time: 3.0,
    },
    // --- Polyomino Shards (Tetris Crystals) ---
    CraftingRecipe {
        id: "crystallize_domino",
        name: "Crystallize Lattice Domino (1x2)",
        inputs: &[(ItemType::IronOre, 2)],
        output_item: ItemType::PolyominoDomino,
        output_count: 1,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "crystallize_ibar",
        name: "Crystallize Lattice I-Bar (1x3)",
        inputs: &[(ItemType::IronOre, 3)],
        output_item: ItemType::PolyominoIBar,
        output_count: 1,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "crystallize_square",
        name: "Crystallize Lattice Square (2x2)",
        inputs: &[(ItemType::IronOre, 4)],
        output_item: ItemType::PolyominoSquare,
        output_count: 1,
        craft_time: 2.0,
    },
    // --- Phase 3 Gemstones & Catalysis Recipes (GDD §5.3, §7.1, §8.3) ---
    CraftingRecipe {
        id: "craft_bronze_plate",
        name: "Smelt Bronze Plate",
        inputs: &[(ItemType::IronPlate, 1), (ItemType::CopperPlate, 1)],
        output_item: ItemType::BronzePlate,
        output_count: 1,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_acoustic_sine_coil",
        name: "Wind Acoustic Sine Coil",
        inputs: &[(ItemType::CopperPlate, 2), (ItemType::BrassCog, 1)],
        output_item: ItemType::AcousticSineCoil,
        output_count: 1,
        craft_time: 2.0,
    },
    CraftingRecipe {
        id: "craft_ingot_cube",
        name: "Compress Ingot Cube",
        inputs: &[(ItemType::IronPlate, 4), (ItemType::CopperPlate, 2)],
        output_item: ItemType::IngotCube,
        output_count: 1,
        craft_time: 2.5,
    },
    CraftingRecipe {
        id: "craft_chromatic_autoclave",
        name: "Construct Chromatic Autoclave",
        inputs: &[
            (ItemType::BronzePlate, 4),
            (ItemType::BrassCog, 4),
            (ItemType::GlassPane, 2),
            (ItemType::IngotCube, 2),
        ],
        output_item: ItemType::ChromaticAutoclave,
        output_count: 1,
        craft_time: 4.0,
    },
    CraftingRecipe {
        id: "craft_resonance_bell_jar",
        name: "Construct Resonance Bell Jar",
        inputs: &[
            (ItemType::BronzePlate, 2),
            (ItemType::GlassPane, 2),
            (ItemType::AcousticSineCoil, 1),
        ],
        output_item: ItemType::ResonanceBellJar,
        output_count: 1,
        craft_time: 3.0,
    },
    CraftingRecipe {
        id: "crack_geode_ruby",
        name: "Crack Geode for Ruby",
        inputs: &[(ItemType::Stone, 2), (ItemType::IronOre, 1)],
        output_item: ItemType::Ruby,
        output_count: 1,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "crack_geode_sapphire",
        name: "Crack Geode for Sapphire",
        inputs: &[(ItemType::Stone, 2), (ItemType::CopperOre, 1)],
        output_item: ItemType::Sapphire,
        output_count: 1,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "crack_geode_emerald",
        name: "Crack Geode for Emerald",
        inputs: &[(ItemType::Stone, 2), (ItemType::IronPlate, 1)],
        output_item: ItemType::Emerald,
        output_count: 1,
        craft_time: 1.0,
    },
    CraftingRecipe {
        id: "crack_geode_topaz",
        name: "Crack Geode for Topaz",
        inputs: &[(ItemType::Stone, 2), (ItemType::CopperPlate, 1)],
        output_item: ItemType::Topaz,
        output_count: 1,
        craft_time: 1.0,
    },
    // --- Phase 4 Sudoku Resonance & Harmonic Calibration Recipes (GDD §5.4, §8.4) ---
    CraftingRecipe {
        id: "craft_tuning_fork",
        name: "Forge Acoustic Tuning Fork",
        inputs: &[(ItemType::BronzePlate, 2), (ItemType::BrassCog, 1)],
        output_item: ItemType::TuningFork,
        output_count: 1,
        craft_time: 2.0,
    },
    CraftingRecipe {
        id: "craft_harmonic_rune",
        name: "Inscribe Harmonic Rune",
        inputs: &[(ItemType::Stone, 2), (ItemType::BronzePlate, 1)],
        output_item: ItemType::HarmonicRune,
        output_count: 1,
        craft_time: 1.5,
    },
    CraftingRecipe {
        id: "craft_acoustic_monolith",
        name: "Erect Acoustic Monolith",
        inputs: &[
            (ItemType::Stone, 6),
            (ItemType::BronzePlate, 2),
            (ItemType::AcousticSineCoil, 1),
        ],
        output_item: ItemType::AcousticMonolith,
        output_count: 1,
        craft_time: 4.0,
    },
    CraftingRecipe {
        id: "craft_tuning_fork_pylon",
        name: "Construct Tuning Fork Pylon",
        inputs: &[
            (ItemType::BronzePlate, 4),
            (ItemType::TuningFork, 1),
            (ItemType::BrassCog, 2),
        ],
        output_item: ItemType::TuningForkPylon,
        output_count: 1,
        craft_time: 3.5,
    },
    CraftingRecipe {
        id: "craft_harmonic_bridge",
        name: "Construct Harmonic Cloud Bridge",
        inputs: &[
            (ItemType::WoodenPlank, 4),
            (ItemType::BronzePlate, 2),
            (ItemType::AcousticSineCoil, 1),
        ],
        output_item: ItemType::HarmonicBridge,
        output_count: 2,
        craft_time: 2.5,
    },
    CraftingRecipe {
        id: "craft_resonance_conductor",
        name: "Assemble Resonance Conductor Owl",
        inputs: &[
            (ItemType::BrassCog, 4),
            (ItemType::TuningFork, 2),
            (ItemType::IngotCube, 1),
        ],
        output_item: ItemType::ResonanceConductor,
        output_count: 1,
        craft_time: 5.0,
    },
];

pub fn find_recipe(id: &str) -> Option<&'static CraftingRecipe> {
    let canonical = match id {
        "craft_belt" => "craft_roller_belt",
        "craft_cog" => "craft_brass_cog",
        "craft_plank" => "craft_wooden_plank",
        "iron_plate" => "smelt_iron_plate",
        "copper_plate" => "smelt_copper_plate",
        "autoclave" => "craft_chromatic_autoclave",
        "bell_jar" => "craft_resonance_bell_jar",
        "tuning_fork" => "craft_tuning_fork",
        "harmonic_rune" => "craft_harmonic_rune",
        "monolith" | "acoustic_monolith" => "craft_acoustic_monolith",
        "tuning_fork_pylon" | "pylon" => "craft_tuning_fork_pylon",
        "harmonic_bridge" | "bridge" => "craft_harmonic_bridge",
        "conductor" | "resonance_conductor" => "craft_resonance_conductor",
        other => other,
    };
    RECIPES.iter().find(|r| r.id == canonical)
}

/// An in-progress craft item in the queue.
#[derive(Clone, Debug, Reflect)]
pub struct ActiveCraft {
    pub recipe_id: String,
    pub output_item: ItemType,
    pub output_count: u32,
    pub time_remaining: f32,
    pub total_time: f32,
}

/// Crafting queue attached to player or workshop.
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CraftingQueue {
    pub queue: VecDeque<ActiveCraft>,
}

impl CraftingQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a recipe if the inventory has enough materials.
    /// Materials are consumed immediately upon queuing.
    pub fn queue_craft(&mut self, recipe_id: &str, inventory: &mut Inventory) -> bool {
        let Some(recipe) = find_recipe(recipe_id) else {
            return false;
        };

        if !inventory.consume_items(recipe.inputs) {
            return false;
        }

        self.queue.push_back(ActiveCraft {
            recipe_id: recipe.id.to_string(),
            output_item: recipe.output_item,
            output_count: recipe.output_count,
            time_remaining: recipe.craft_time,
            total_time: recipe.craft_time,
        });

        true
    }

    /// Ticks the crafting queue.
    /// Returns finished items as `(ItemType, u32)`.
    pub fn tick(&mut self, delta: f32) -> Vec<(ItemType, u32)> {
        let mut completed = Vec::new();
        let mut remaining_delta = delta;

        while remaining_delta > 0.0 {
            let Some(active) = self.queue.front_mut() else {
                break;
            };

            if active.time_remaining <= remaining_delta {
                remaining_delta -= active.time_remaining;
                let finished = self.queue.pop_front().unwrap();
                completed.push((finished.output_item, finished.output_count));
            } else {
                active.time_remaining -= remaining_delta;
                remaining_delta = 0.0;
            }
        }

        completed
    }
}

/// Event triggered to initiate crafting.
#[derive(Event, Clone, Debug)]
pub struct StartCraftEvent {
    pub player_entity: Entity,
    pub recipe_id: String,
}

/// Event triggered when a craft finishes.
#[derive(Event, Clone, Debug)]
pub struct CraftCompletedEvent {
    pub player_entity: Entity,
    pub item: ItemType,
    pub count: u32,
}

/// System that processes crafting progress each frame.
pub fn process_crafting_queue(
    time: Res<Time>,
    mut players: Query<(Entity, &mut CraftingQueue, &mut Inventory)>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    for (entity, mut queue, mut inventory) in players.iter_mut() {
        let completed = queue.tick(dt);
        for (item, count) in completed {
            inventory.add_item(item, count);
            commands.trigger(CraftCompletedEvent {
                player_entity: entity,
                item,
                count,
            });
        }
    }
}

/// Observer that handles `StartCraftEvent`.
pub fn on_start_craft(
    on: On<StartCraftEvent>,
    mut players: Query<(&mut CraftingQueue, &mut Inventory)>,
) {
    let ev = on.event();
    if let Ok((mut queue, mut inventory)) = players.get_mut(ev.player_entity) {
        queue.queue_craft(&ev.recipe_id, &mut inventory);
    }
}

pub fn plugin(app: &mut App) {
    app.register_type::<CraftingQueue>()
        .add_observer(on_start_craft)
        .add_systems(Update, process_crafting_queue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crafting_flow() {
        let mut inv = Inventory::new(10);
        let mut queue = CraftingQueue::new();

        inv.add_item(ItemType::IronOre, 4);

        // Queue iron plate (needs 2 IronOre, craft time 1.5s)
        assert!(queue.queue_craft("smelt_iron_plate", &mut inv));
        assert_eq!(inv.count_item(ItemType::IronOre), 2);
        assert_eq!(queue.queue.len(), 1);

        // Tick 1.0s -> not finished yet
        let finished1 = queue.tick(1.0);
        assert!(finished1.is_empty());
        assert_eq!(queue.queue.len(), 1);

        // Tick another 0.6s -> completes!
        let finished2 = queue.tick(0.6);
        assert_eq!(finished2.len(), 1);
        assert_eq!(finished2[0], (ItemType::IronPlate, 1));
        assert!(queue.queue.is_empty());
    }

    #[test]
    fn test_insufficient_materials() {
        let mut inv = Inventory::new(10);
        let mut queue = CraftingQueue::new();

        inv.add_item(ItemType::IronOre, 1);
        // Needs 2 IronOre
        assert!(!queue.queue_craft("smelt_iron_plate", &mut inv));
        assert_eq!(inv.count_item(ItemType::IronOre), 1);
        assert!(queue.queue.is_empty());
    }

    #[test]
    fn test_phase3_recipes() {
        let mut inv = Inventory::new(20);
        let mut queue = CraftingQueue::new();

        inv.add_item(ItemType::BronzePlate, 4);
        inv.add_item(ItemType::BrassCog, 4);
        inv.add_item(ItemType::GlassPane, 2);
        inv.add_item(ItemType::IngotCube, 2);

        assert!(queue.queue_craft("craft_chromatic_autoclave", &mut inv));
        let finished = queue.tick(4.0);
        assert_eq!(finished, vec![(ItemType::ChromaticAutoclave, 1)]);

        // Geode cracking
        inv.add_item(ItemType::Stone, 2);
        inv.add_item(ItemType::IronOre, 1);
        assert!(queue.queue_craft("crack_geode_ruby", &mut inv));
        let finished_gem = queue.tick(1.0);
        assert_eq!(finished_gem, vec![(ItemType::Ruby, 1)]);
    }

    #[test]
    fn test_phase4_recipes() {
        let mut inv = Inventory::new(20);
        let mut queue = CraftingQueue::new();

        inv.add_item(ItemType::BronzePlate, 6);
        inv.add_item(ItemType::BrassCog, 3);
        inv.add_item(ItemType::Stone, 6);
        inv.add_item(ItemType::AcousticSineCoil, 1);

        // Craft tuning fork via alias
        assert!(queue.queue_craft("tuning_fork", &mut inv));
        let finished_fork = queue.tick(2.0);
        assert_eq!(finished_fork, vec![(ItemType::TuningFork, 1)]);

        // Craft acoustic monolith via alias
        assert!(queue.queue_craft("monolith", &mut inv));
        let finished_monolith = queue.tick(4.0);
        assert_eq!(finished_monolith, vec![(ItemType::AcousticMonolith, 1)]);
    }
}
