# 🏭 GRIDLOCK — Game Design Document

> *"The machines don't just build. They play — in a miniature clockwork world of moss, brass, and pastel dreams."*

**Version:** 1.0 — 2.5D Isometric Diorama & Spatial Factory Overhaul  
**Engine:** Bevy (Rust)  
**Genre:** 2.5D Isometric Factory / Puzzle Hybrid  
**Perspective:** 2.5D Axonometric / Isometric Diorama with Tilt-Shift & Multi-Layer Z-Planes (Ground $Z_0$, Overpass $Z_1$, Skyway $Z_2$)  
**Visual Style:** Cutesy Toybox-Diorama with Hand-Illustrated Manuals (inspired by the lush geometric aesthetic of **TUNIC**) meets Multi-Level Modular Factory Architecture (inspired by **Shapez 2**)  
**Target Platform:** PC (Windows, macOS, Linux, Steam Deck Verified)  

---

## Table of Contents

1. [Vision & High Concept](#1-vision--high-concept)
2. [Lore & World](#2-lore--world)
   - 2.1 [Setting — The Floating Archipelago of The Patchwork](#21-setting--the-floating-archipelago-of-the-patchwork)
   - 2.2 [The Substrate and Its Moods](#22-the-substrate-and-its-moods)
   - 2.3 [Factions](#23-factions)
3. [Core Design Pillars](#3-core-design-pillars)
4. [The Core Loop](#4-the-core-loop)
   - [Session Flow](#session-flow)
5. [Puzzle Mechanics](#5-puzzle-mechanics)
   - 5.1 [Minesweeper — Terrain Prospecting & Voxel Tile Unfolding](#51-minesweeper--terrain-prospecting--voxel-tile-unfolding)
   - 5.2 [Tetris — Multi-Level Conveyor Routing & 3D Cargo Compression](#52-tetris--multi-level-conveyor-routing--3d-cargo-compression)
   - 5.3 [Bejeweled — Chromatic Autoclaves & Gemstone Catalysis](#53-bejeweled--chromatic-autoclaves--gemstone-catalysis)
   - 5.4 [Sudoku — Resonant Monoliths & Harmonic Leyline Calibration](#54-sudoku--resonant-monoliths--harmonic-leyline-calibration)
   - 5.5 [Solitaire — Step-Tiered Marble Cascades & Sequential Assembly](#55-solitaire--step-tiered-marble-cascades--sequential-assembly)
   - 5.6 [Mahjong Solitaire — Ancient Stepped Vault Deconstruction](#56-mahjong-solitaire--ancient-stepped-vault-deconstruction)
   - 5.7 [Connections — The Specimen Carousel & Material Classification Lab](#57-connections--the-specimen-carousel--material-classification-lab)
   - 5.8 [Wordle — Clockwork Tumbler Ciphers & Ancient Pillar Calibration](#58-wordle--clockwork-tumbler-ciphers--ancient-pillar-calibration)
   - 5.9 [Pips — Dice-Faceted Bento Crates & Sky-Barge Trade Manifests](#59-pips--dice-faceted-bento-crates--sky-barge-trade-manifests)
   - 5.10 [Crossword — Luminous Leyline Intersections & Blueprint Synergy Grids](#510-crossword--luminous-leyline-intersections--blueprint-synergy-grids)
6. [World Structure & Progression](#6-world-structure--progression)
   - 6.1 [Floating Diorama Map Layout](#61-floating-diorama-map-layout)
   - 6.2 [Progression Gates & Archipelago Unlocks](#62-progression-gates--archipelago-unlocks)
7. [Resources & Materials](#7-resources--materials)
   - 7.1 [Raw Resources (Extracted via Tactile Prospecting)](#71-raw-resources-extracted-via-tactile-prospecting)
   - 7.2 [Intermediate Materials & 3D Polyomino Shards](#72-intermediate-materials--3d-polyomino-shards)
   - 7.3 [Finished Goods & The Grand Resonance Engine](#73-finished-goods--the-grand-resonance-engine)
8. [Buildings & Machines](#8-buildings--machines)
   - 8.1 [Extraction Tier ($Z_0$ Ground Anchors)](#81-extraction-tier-z_0-ground-anchors)
   - 8.2 [Transport Tier (Multi-Layer $Z_0/Z_1/Z_2$ Logistics)](#82-transport-tier-multi-layer-z_0z_1z_2-logistics)
   - 8.3 [Refining Tier (Glass Terrarium Autoclaves)](#83-refining-tier-glass-terrarium-autoclaves)
   - 8.4 [Power Tier (Harmonic Crystal Pillars)](#84-power-tier-harmonic-crystal-pillars)
   - 8.5 [Assembly Tier (Cascading Stepped Chutes)](#85-assembly-tier-cascading-stepped-chutes)
   - 8.6 [Deconstruction Tier (Miniature Gantry Cranes)](#86-deconstruction-tier-miniature-gantry-cranes)
   - 8.7 [Classification Tier (Pneumatic Carousel Pavilions)](#87-classification-tier-pneumatic-carousel-pavilions)
   - 8.8 [Cipher Tier (Clockwork Monolith Dials)](#88-cipher-tier-clockwork-monolith-dials)
   - 8.9 [Trade Tier (Floating Wooden Wharves & Sky-Docks)](#89-trade-tier-floating-wooden-wharves--sky-docks)
   - 8.10 [Synergy Tier (Resonance Beacon Emitters)](#810-synergy-tier-resonance-beacon-emitters)
9. [Automation & Research Tree](#9-automation--research-tree)
   - 9.1 [Philosophy: From Tactile Hands to Clockwork Symphony](#91-philosophy-from-tactile-hands-to-clockwork-symphony)
   - 9.2 [The Illustrated Field-Guide Tech Tree](#92-the-illustrated-field-guide-tech-tree)
   - 9.3 [Insight Sparks & Harmonic Research Currency](#93-insight-sparks--harmonic-research-currency)
10. [Visual & Audio Design](#10-visual--audio-design)
    - 10.1 [Visual Style — The Tunic × Shapez 2 Fusion](#101-visual-style--the-tunic--shapez-2-fusion)
    - 10.2 [Audio Design — Generative Toybox Acoustic & Chiptune Symphony](#102-audio-design--generative-toybox-acoustic--chiptune-symphony)
    - 10.3 [UI / UX Aesthetic — The Golden Inscribed Explorer's Manual](#103-ui--ux-aesthetic--the-golden-inscribed-explorers-manual)
11. [UI / UX Philosophy & Diorama Controls](#11-ui--ux-philosophy--diorama-controls)
    - 11.1 [Seamless Diorama Inspection & Isometric Lens Overlays](#111-seamless-diorama-inspection--isometric-lens-overlays)
    - 11.2 [Tactile Pause-to-Plan Mode](#112-tactile-pause-to-plan-mode)
    - 11.3 [Charming Physical Alerts & Postcard Toast System](#113-charming-physical-alerts--postcard-toast-system)
12. [Open Questions & Design Risks](#12-open-questions--design-risks)
    - 12.1 [Spatial & 2.5D Balance Considerations](#121-spatial--25d-balance-considerations)
    - 12.2 [Planned Expansion Modules](#122-planned-expansion-modules)
    - 12.3 [Technical Architecture (Bevy 0.14+ 3D/Isometric Rust Engine)](#123-technical-architecture-bevy-014-3disometric-rust-engine)
13. [The Constructor — The Miniature Clockwork Engineer](#13-the-constructor--the-miniature-clockwork-engineer)
    - 13.1 [Character Overview & Whimsical Form](#131-character-overview--whimsical-form)
    - 13.2 [Character Stats & Movement Physics](#132-character-stats--movement-physics)
    - 13.3 [Equipment Slots & Wearable Trinkets](#133-equipment-slots--wearable-trinkets)
    - 13.4 [Faction Badges & Personal Traits](#134-faction-badges--personal-traits)
14. [Handcrafting & Material Recipes](#14-handcrafting--material-recipes)
    - 14.1 [Philosophy: Tactile Craftsmanship Precedes Automation](#141-philosophy-tactile-craftsmanship-precedes-automation)
    - 14.2 [The Cozy Pocket Workshop (Crafting Panel)](#142-the-cozy-pocket-workshop-crafting-panel)
    - 14.3 [Building Material Costs & Multi-Tier Footprints](#143-building-material-costs--multi-tier-footprints)
    - 14.4 [Intermediate Component Recipes](#144-intermediate-component-recipes)
    - 14.5 [The 3D Stamp Assembler](#145-the-3d-stamp-assembler)
    - 14.6 [Luminous Ghost Holograms & Snapping Guides](#146-luminous-ghost-holograms--snapping-guides)
15. [World Traversal & Diorama Camera](#15-world-traversal--diorama-camera)
    - 15.1 [Isometric Movement, Waddling Physics & Climbing](#151-isometric-movement-waddling-physics--climbing)
    - 15.2 [The Tilt-Shift Axonometric Camera System](#152-the-tilt-shift-axonometric-camera-system)
    - 15.3 [Atmospheric Cloud Strata & Fog of Wonder](#153-atmospheric-cloud-strata--fog-of-wonder)
    - 15.4 [3D Stepped Terrain, Ramps, & Sky Bridges](#154-3d-stepped-terrain-ramps--sky-bridges)
    - 15.5 [Whimsical Fast Travel — Gliders, Trams, & Pneumatic Canisters](#155-whimsical-fast-travel--gliders-trams--pneumatic-canisters)
16. [Personal Inventory & Toybox Hotbar](#16-personal-inventory--toybox-hotbar)
    - 16.1 [Backpack Layout (The Canvas Satchel)](#161-backpack-layout-the-canvas-satchel)
    - 16.2 [The Tactile Wooden Hotbar](#162-the-tactile-wooden-hotbar)
    - 16.3 [Trinket & Garb Panel](#163-trinket--garb-panel)
    - 16.4 [Adorable Delivery Sparrows (Late-Game Personal Logistics)](#164-adorable-delivery-sparrows-late-game-personal-logistics)
    - 16.5 [Item Stack Sizes & Miniature Physics Voxels](#165-item-stack-sizes--miniature-physics-voxels)
17. [Hand Mining & The Cozy Bootstrapping Phase](#17-hand-mining--the-cozy-bootstrapping-phase)
    - 17.1 [The First Hour: Solitary Awakening on Mossy Stones](#171-the-first-hour-solitary-awakening-on-mossy-stones)
    - 17.2 [The Brass Winding-Key Drill](#172-the-brass-winding-key-drill)
    - 17.3 [Bootstrapping Flowchart](#173-bootstrapping-flowchart)
    - 17.4 [Harvesting Crystalline Outcrops](#174-harvesting-crystalline-outcrops)
    - 17.5 [Soft Earth, River Pebbles, and Natural Fluff](#175-soft-earth-river-pebbles-and-natural-fluff)
18. [Day / Night Cycle & Whimsical Environmental Curiosities](#18-day--night-cycle--whimsical-environmental-curiosities)
    - 18.1 [The Diurnal Rhythm of The Patchwork](#181-the-diurnal-rhythm-of-the-patchwork)
    - 18.2 [Sunbeam Bonuses & Dewdrop Mornings](#182-sunbeam-bonuses--dewdrop-mornings)
    - 18.3 [Twilight Curiosities & Starfall Rhythms](#183-twilight-curiosities--starfall-rhythms)
    - 18.4 [Warm Lantern Lighting & Bioluminescent Flora](#184-warm-lantern-lighting--bioluminescent-flora)
    - 18.5 [Ruin Sprites — Shy Ancient Guardians](#185-ruin-sprites--shy-ancient-guardians)
    - 18.6 [The Cozy Cottage Hub (Personal Camp & Anchor)](#186-the-cozy-cottage-hub-personal-camp--anchor)
- [Appendix A — Illustrated Glossary](#appendix-a--illustrated-glossary)

---

## 1. Vision & High Concept

**GRIDLOCK** is a 2.5D isometric factory-puzzle hybrid. Set in a breathtaking diorama world resembling a living miniature tabletop set, the world marries the warm, mysterious, storybook visual elegance of **TUNIC** with the clean, multi-layered spatial engineering and endless conveyor satisfaction of **Shapez 2**.

The laws of automation are not governed by cold industrial spreadsheets; they are powered by ancient, tactile mechanical puzzles embedded within the machines themselves. The conveyor network does not merely transport goods across flat planes — it weaves across multi-tiered elevations ($Z_0$ ground paths, $Z_1$ elevated viaducts, $Z_2$ soaring suspension skyways) packing **Tetris** polyomino crystals. The refinery is a glowing, brass-and-glass **Bejeweled** autoclave. The electrical grid balances harmonic frequency rings along ancient **Sudoku** obelisks. Research is decoded through whimsical **Connections** specimen carousels, and sealed ruin portals unlock via chunky clockwork **Wordle** tumblers.

```
          [ Z2 Skyway Conveyor: Polyomino Shards ]
                   /=====================\
                  /                       \
   +-------------+                         +-------------+
   | Chromatic   |                         | 3D Stepped  |
   | Autoclave   | ===[ Z1 Viaduct ]======>| Solitaire   |
   | (Bejeweled) |                         | Cascade     |
   +-------------+                         +-------------+
         |                                        |
         | [ Z0 Ground Belts ]                    |
         v                                        v
   +-------------+                         +-------------+
   | Sudoku      |                         | Floating    |
   | Resonant    |                         | Trade Pier  |
   | Obelisk     |                         | (Pips Bento)|
   +-------------+                         +-------------+
```

The core tension balances **manual tactile wonder vs. sprawling multi-layer automation**:
1. **Early Game:** The player explores on foot as a tiny, cloaked clockwork automaton, flipping puzzle tiles with hand-cranked brass tools, carrying candy-like crystal shards in their satchel, and listening to the soft clatter of running water and singing birds.
2. **Mid Game:** The player blueprints elevated conveyor bridges, vertical spiral elevators, sorting arms, and pneumatic filters that automate puzzle solutions, turning their quiet diorama into a humming, interconnected clockwork wonderland.
3. **Late Game:** The player oversees monumental multi-level industrial complexes spanning floating cloud-islands, where cascading combos trigger celestial music and golden resonance beams light up the sky.

---

## 2. Lore & World

### 2.1 Setting — The Floating Archipelago of The Patchwork

The world of GRIDLOCK is **The Patchwork** — an archipelago of floating, stepped diorama islands suspended in a dreamy expanse of pastel sea mist and golden cloud banks. Ancient cliffs of beveled sandstone and slate drop into endless sky, carpeted with lush jade moss, tiny white daisies, and sparkling crystal streams.

Thousands of years ago, a mysterious race known as the **Architects** constructed a world made of **Logic Substrate** — a crystalline, geo-computational mineral that hums with living mathematical energy. When their era ended, their golden clockwork monoliths and brass watermills fell silent, sleeping beneath blankets of clover and ivy.

The player is **The Constructor** (fondly known as *The Gearling*) — a miniature, cloaked automaton with glowing spherical eyes, a cozy woolen scarf, and an oversized brass winding-key on their back. Washing ashore on a tiny, sunlit floating islet, the Constructor must awaken the sleeping machinery, harmonize the Logic Substrate, and construct the legendary **Resonance Engine** to unite the fractured sky islands.

### 2.2 The Substrate and Its Moods

Logic Substrate behaves like a playful, living mineral that crystallizes into tangible geometric forms:

- **Volatile (Minesweeper):** Dormant terrain voxels that hide pressurized steam vents. Stepping or drilling heedlessly flips the tile with a startled *poof*, releasing playful shockwaves that bounce nearby machines like toy blocks.
- **Crystalline (Tetris):** Raw ore forms beveled, translucent candy-like polyomino clusters (dominoes, trominoes, and tetrominoes) that slide smoothly along brass conveyor rollers.
- **Chromatic (Bejeweled):** Refined substrate sparkles in five jewel-tones (Ruby Red, Sapphire Blue, Topaz Yellow, Emerald Green, and Obsidian Voidite), reacting when grouped in harmonic trios inside glass autoclaves.
- **Harmonic (Sudoku):** Substrate crystals hum at distinct acoustic frequencies (1 through 9), casting luminous pastel halo rings that must not clash in adjacent sky quadrants.
- **Sequential (Solitaire):** Multi-layered alloys demand stepped assembly, sliding down marble-run stairs in strict descending rank and alternating polarity.
- **Stratified (Mahjong Solitaire):** High-grade relic caches reside inside ancient stepped stone pagodas, where tiles must be plucked free from outer edges before the deeper treasures can be reached.
- **Categorical (Connections):** Mixed shipments of odd curiosities that arrive in pneumatic brass tubes, waiting to be sorted into themed specimen jars.
- **Encoded (Wordle):** Massive overgrown archways sealed by five-ring brass tumbler dials with engraved runic glyphs.
- **Quantified (Pips):** Export cargo stamped with glowing gold domino-style pips, packed into compartmentalized wooden bento boxes for visiting airships.
- **Spatial (Crossword):** Buildings emit delicate, luminous thread-like leylines across the grass. Where matching keyword threads intersect, a glowing golden lily blossoms, showering the factory in passive harmony.

### 2.3 Factions

The scattered inhabitants and spirits of The Patchwork view the Substrate through distinct philosophical lenses:

| Faction | Emblem | Philosophy | Gameplay Specialization |
|---|---|---|---|
| **The Purists** | Carved Wooden Acorn | Reverence for the handmade; machines should assist, not replace the artisan. | Immense hand-crafting bonuses; enhanced manual puzzle yields; cozy wooden building variants. |
| **The Mechanists** | Polished Brass Gear | Perfect, frictionless clockwork; automation is the ultimate song of nature. | Unlocks high-speed multi-layer $Z$-belts, pneumatic sorters, and fast logic auto-solvers. |
| **The Resonants** | Prismatic Crystal Bell | Seeking acoustic and emotional harmony with the Substrate's song. | Generates cascading combo energy; amplifies Sudoku and Bejeweled harmonic resonance. |
| **The Salvagers** | Weathered Iron Key | Curators of antiquity; peeling back ivy to reveal ancient Architect genius. | Specializes in Mahjong vault excavation; enhanced recycling yields; ruin restoration discounts. |

---

## 3. Core Design Pillars

| # | Pillar | 2.5D Diorama Manifestation |
|---|---|---|
| 1 | **Puzzles Are the Living Machines** | Puzzles are physical mechanisms built into the 3D structures — gears turn, glass globes bubble, and marble chutes rattle as you solve or automate them. |
| 2 | **Multi-Layered Spatial Elegance** | Belts climb ramps, cross over viaducts on $Z_1$, and soar along suspension bridges on $Z_2$. Clean vertical logistics (Shapez 2 style) eliminates flat-grid belt spaghetti. |
| 3 | **Juicy, Tactile Toybox Feedback** | Every action feels like interacting with a handcrafted luxury wooden-and-brass toy: pieces click with solid *thwips*, steam gently puffs in pastel clouds, and tiles bounce with subtle squash-and-stretch. |
| 4 | **The Illustrated Storybook Aesthetic** | Menus, blueprints, and tech trees look like hand-drawn pages from a cozy, gilded explorer’s notebook filled with watercolor diagrams, mysterious runic marginalia, and helpful arrows. |
| 5 | **Gentle Failure & Playful Recovery** | Jams and shockwaves never cause game-ending rage. Overflowing belts stack into cute leaning piles, and overcharge detonations merely pop buildings into repairable bounce-bubbles. |

---

## 4. The Core Loop

```
               [ 1. PROSPECT & UNFOLD ]
         (Minesweeper: Reveal voxel tiles, tap
          soft earth, discover crystalline veins)
                         |
                         v
               [ 2. HARVEST & TRANSPORT ]
         (Tetris: Route candy-like polyominoes
          across multi-tier Z0/Z1/Z2 conveyor belts)
                         |
                         v
               [ 3. REFINE & CATALYZE ]
         (Bejeweled: Tumble gem nodes in glass
          terrarium vats to generate power & alloys)
                         |
     +-------------------+-------------------+-------------------+
     |                   |                   |                   |
     v                   v                   v                   v
[ 4A. ASSEMBLE ]   [ 4B. HARMONIZE ]   [ 4C. EXCAVATE ]    [ 4D. PACK & TRADE ]
(Solitaire: Step-   (Sudoku: Tune       (Mahjong: Pluck     (Pips: Fill bento
 tiered marble       monolith halos      ceramic tiles       crates for cute
 cascades on Z1)     across islands)     from ancient ruins) sky-barges)
     |                   |                   |                   |
     +-------------------+-------------------+-------------------+
                         |
                         v
               [ 5. UNLOCK & EXPAND ]
         (Wordle: Spin brass ruin tumblers to access new islands;
          Crossword: Weave glowing grass leylines for massive synergies)
                         |
                         v
               [ 6. HARMONIC CONVERGENCE ]
         (Power the Grand Resonance Engine across all cloud isles)
```

### Session Flow

| Phase | Timeframe | Player Activity & World Evolution |
|---|---|---|
| **The Mossy Awakening** | 0–30 min | Hand-crank small ore nodes, solve friendly 4×4 Minesweeper grass tiles, place cozy wooden rollers along the stream. |
| **Pneumatic Sorting** | 30–60 min | Build the first Specimen Carousel (Connections), unlock bronze gears, place glass-domed $Z_0$ conveyors. |
| **The Glass Autoclave** | 1–2 hrs | Build the first Bejeweled refinery, watch gem matches illuminate the evening sky, unlock the first automated brass swap-piston. |
| **Multi-Level Skyways** | 2–4 hrs | Route $Z_1$ overpass viaducts over $Z_0$ belts, pack polyominoes into 3D Compressors, calibrate the first 4×4 Sudoku crystal beacon. |
| **Archipelago Mastery** | 4–10 hrs | Build zip-lines and gliders between floating islands, crack 5-ring ruin tumblers (Wordle), dismantle towering Mahjong pagodas. |
| **The Great Clockwork Sky** | 10+ hrs | Vast, multi-tier automated factories weaving across cloud islands, all 10 puzzle systems humming in harmonic generative music, assembling the Resonance Engine. |

---

## 5. Puzzle Mechanics

### 5.1 Minesweeper — Terrain Prospecting & Voxel Tile Unfolding

**Factory Role:** World Exploration, Island Expansion, and Subterranean Extraction  

```
        [ Covered Voxel ]              [ Unfolded Voxel ]
            +-------+                      +-------+
           /       / \                    /   (2) / \
          +-------+   +    --[ Tap ]-->  +-------+   +
          | Grass |  /                   | Slate |  /
          | Block | /                    | Runic | /
          +-------+                      +-------+
```

#### Concept
Unexplored terrain on each floating island is rendered as pristine, beveled geometric voxel blocks covered in emerald turf, wildflowers, and clover. Placing a **Survey Beacon** or walking up with a Hand-Tapper causes covered blocks to pop open with a delightful wooden tile-flip animation, unfolding to show engraved golden numerals indicating adjacent **Overcharge Vents**.

#### Rules
- Numbers indicate the exact count of hazardous Overcharge Vents lurking in the 8 adjacent 2.5D grid tiles.
- Tiles exist in four visual states: **Lush Turf (Covered)**, **Engraved Slate (Revealed)**, **Wooden Marker (Flagged)**, and **Brass Well (Extracted)**.
- Marking an unrevealed tile with a wooden pin flags it, preventing worker automatons and conveyor builders from puncturing the vent.
- Safe revealed tiles can host **Clockwork Wellheads** to passively harvest ore deposits.
- Inadvertently puncturing an unflagged Overcharge Vent triggers a **Playful Pop**: a geyser of pastel steam and harmless shockwaves that topples adjacent machines into wobbly, bouncing repair-bubbles (easily cleared with a wrench tap).

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Survey Beacon** | $Z_0$ | 2×2 | Cozy timber lookout tower with a spinning brass brass compass. | Auto-unfolds adjacent 3×3 tiles every 10 seconds. |
| **Logic Diviner** | $Z_0$ | 2×2 | Brass pendulum hanging from a sculpted stone archway. | Computes safe deductions and automatically plants wooden hazard flags. |
| **Vapor Vent Tapper**| $Z_0$ | 2×2 | Domed brass kettle with a whistling chimney. | Safely releases overcharge steam, converting hazard pressure into continuous pneumatic power. |
| **Bounce Damper** | $Z_0$ | 1×1 | Chunky spring-loaded rubber mushroom cushion. | Absorbs shockwaves from accidental pops within a 5-tile radius. |

---

### 5.2 Tetris — Multi-Level Conveyor Routing & 3D Cargo Compression

**Factory Role:** Multi-Tier Logistics, Belt Routing, and Cargo Density  

```
   [ Z1 Skyway Belt ]
        \
         v
    +---------+  <-- Polyomino Crystal Drops In
    | Glass   |
    | Hopper  |  [ * ][ * ][   ][ * ]  <-- Line Fills & Flashes
    | (4x10)  |  ====================
    +---------+  ===> Outputs Compact Ingot Cube onto [ Z0 Belt ]
```

#### Concept
Mined ore crystallizes into translucent, jewel-like **Lattice Shards** — 3D polyominoes (dominoes, trominoes, and tetrominoes) that travel along multi-level conveyor belts. Like in **Shapez 2**, conveyors run cleanly across distinct vertical planes: $Z_0$ ground level, $Z_1$ elevated viaducts, and $Z_2$ skyway suspension lines, allowing players to hop belts over one another without tangled spaghetti.

To convert bulky shards into dense ingots, belts route polyominoes into **Vertical Cargo Compressors** — 4×10 transparent glass hopper buildings where pieces drop under gentle toybox gravity.

#### Rules
- Shards ride along physical conveyor rollers, never overlapping.
- When routed into the top hopper of a Compressor, shards tumble down. Filling an entire horizontal row triggers a **Piston Squish**: a brass stamper presses the row with a satisfying *thump*, outputting a sleek, beveled **Ingot Cube** onto the exit belt.
- Gaps cause the glass hopper to fill up. If pieces spill over the chimney, the machine jams and puffs polite warning steam until cleared.
- Conveyor **Rotator Plates** can swivel shards in 90° increments; **Incline Ramps** lift shards smoothly from $Z_0 \to Z_1 \to Z_2$.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Roller Incline Ramp** | $Z_0 \to Z_1$ | 1×3 | Gentle wooden-and-brass conveyor ramp with turning gears. | Elevates shards seamlessly between vertical layers. |
| **Spiral Elevator** | $Z_0 \to Z_2$ | 2×2 | Helical glass tube with a corkscrew cork lifter inside. | Compact 2-level vertical ascent for dense factory footprints. |
| **Swivel Turntable** | $Z_0/Z_1$ | 1×1 | Circular brass plate with painted curved directional arrows. | Rotates passing polyominoes by 90°, 180°, or 270°. |
| **Sorting Gate** | $Z_0/Z_1$ | 2×1 | Dual-rail wooden gate with cute swinging saloon doors. | Reads shard shape or color and diverts matching pieces. |
| **Clockwork Auto-Packer** | $Z_0$ | 2×2 | Articulated wooden arm with padded rubber suction cups. | Pre-arranges queued shards into perfect row-clearing sequences. |

---

### 5.3 Bejeweled — Chromatic Autoclaves & Gemstone Catalysis

**Factory Role:** Chemical Refining, Material Transformation & Radiant Power  

```
        +-----------------------------+
        |   Chromatic Glass Dome      |
        |   [R] [B] [Y] [G] [R] [B]   |
        |   [B] [R] [R] [R] [Y] [G]   | <-- Match-3 Triggers Chime!
        |   [Y] [G] [B] [Y] [G] [R]   |
        +-----------------------------+
                     |
                     v
        Sparks Sparkle & Cascade Multipliers Rise!
```

#### Concept
Compressed Ingot Cubes are transported into **Chromatic Autoclaves** — gorgeous, round glass terrarium vats framed in polished brass. Inside, ingots dissolve into glowing, round gemstone spheres that roll gently across a tilted 6×6 or 9×9 diorama tray under warm internal lighting. Swapping adjacent gems triggers match-3 chemical catalysis, causing gems to burst into sparkling confetti stars and output refined metal ingots.

#### Match Rules

| Match Type | Visual & Particle Effect | Output | Power Surge |
|---|---|---|---|
| **Match-3** | Crisp chime; soft pastel flash. | 1 Refined Metal Ingot | +15 kW sustained |
| **Match-4** | Golden starburst; creates a ticking **Spark Orb**. | 1 Alloy Ingot + Line-Clearing Orb | +60 kW burst |
| **Match-5** | Prismatic rainbow beam shoot upward through dome. | 1 Prismatic Core (rare) | +200 kW radiant pulse |
| **L/T-Shape** | Swirling vortex; creates a **Color Bomb**. | 1 Alloy Ingot + Radial Clear Bomb | +120 kW burst |
| **Cascade (Chain)** | Ascending musical harp arpeggios; screen glows gently. | Multiplied outputs ($2\times, 3\times, 4\times$) | Cascading power multiplier |
| **Full Clear** | Celestial music box fanfare; floating glitter rain. | Grand Resonance Cache | Full island grid supercharge |

Gemstone Palettes: **Ruby** (Iron), **Sapphire** (Copper), **Topaz** (Gold), **Emerald** (Logic Crystal), **Obsidian** (Voidite), and **Opal** (Wildcard/Prismatic).

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Brass Swap-Piston** | Internal | 1×1 | Spring-loaded clockwork rod with soft velvet tips. | Automatically swaps one valid match per second. |
| **Predictive Periscope** | $Z_1$ mount | 1×1 | Cute brass telescope focused downward into the dome. | Identifies optimal cascade chain swaps for swap-pistons. |
| **Overhead Hopper Chute**| $Z_2 \to Z_1$| 1×1 | Funnel dropping gems into specific columns from skyways. | Injects specific colors to set up massive match-5 combos. |
| **Resonance Bell Jar** | $Z_0$ | 2×2 | Acoustic glass bell that vibrates with warm amber light. | Stores up to 5 cascade multipliers, discharging during peak load. |

---

### 5.4 Sudoku — Resonant Monoliths & Harmonic Leyline Calibration

**Factory Role:** Island Power Distribution, Frequency Alignment & Circuit Tuning  

```
        Island Sector (9x9 Resonance Grove)
       +-----------+-----------+-----------+
       | (1) (2) . | .   .   . | .   (9) . |   Each machine wears a
       |  .  (4) . | .  (5)  . | .   .   . |   floating pastel halo ring
       |  .   .  . | (6) .   . | .   .  (3)|   representing its frequency (1-9).
       +-----------+-----------+-----------+
       |     No two matching rings may share the same row, column,     |
       |     or 3x3 stone grove without creating buzzing discord!      |
       +---------------------------------------------------------------+
```

#### Concept
Power in GRIDLOCK does not travel through tangled black wires. Each floating island sector is an ancient **Resonance Grove** organized as a 9×9 grid of stone pedestals. Every machine placed in that grove emits a soft, floating **Harmonic Halo** — a gently hovering, luminous pastel ring showing a musical frequency from 1 to 9 (rendered in musical runes: $♩_1$ through $♬_9$).

According to ancient Architect acoustic laws, no two machines in the same row, column, or 3×3 grove can share the same frequency without causing **Resonance Flutter** (a soft, out-of-tune humming that reduces machine speed).

#### Rules
- Freshly constructed machines default to an unassigned frequency (halo flickers gently; machine runs at 50% baseline speed).
- Assigning unique frequencies 1 through 9 resolves local acoustic interference.
- Achieving a fully harmonious 9×9 grove triggers the **Harmonic Symphony Bonus**: the entire grove glows with warm golden light, wildflowers bloom instantly across the grass, and all machines gain +50% speed.
- Ancient mossy monoliths found across ruins possess pre-set, locked frequencies, serving as delightful puzzle clues.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Acoustic Tuning Fork** | $Z_0$ | 1×1 | Oversized polished brass tuning fork stuck into moss. | Broadcasts valid non-conflicting frequencies to nearby machines. |
| **Prismatic Monolith** | $Z_0$ | 3×3 | Ancient stepped stone obelisk with glowing runic grooves. | Generates baseline island power (+250 kW) when grove is harmonic. |
| **Harmonic Cloud Bridge**| $Z_1$ | 1×4 | Floating crystal walkway linking two separate sky islands. | Bridges power networks between floating islands without cross-interference. |
| **Resonance Conductor** | $Z_0$ | 2×2 | Cute clockwork owl automaton perched on a music stand. | Automatically re-tunes machines when new structures are placed. |

---

### 5.5 Solitaire — Step-Tiered Marble Cascades & Sequential Assembly

**Factory Role:** High-Tier Component Assembly & Step-by-Step Fabrication  

```
    [ Inflow Belt ] ---> [ Top Hopper ]
                               |
             Lane 1   Lane 2   Lane 3   Lane 4   Lane 5   Lane 6   Lane 7
             [13 +]   [11 -]   [ 9 +]   [ 7 -]   [ 5 +]   [ 3 -]   [ 1 +]
             [12 -]   [10 +]   [ 8 -]   [ 6 +]   [ 4 -]   [ 2 +]     |
             [11 +]   [ 9 -]   [ 7 +]   [ 5 -]   [ 3 +]     |        v
               |        |        |        |        |        |    [ Output Chute ]
               v        v        v        v        v        v   (Finished Engine!)
```

#### Concept
Advanced clockwork parts cannot be hammered out in simple furnaces. They are crafted on **Step-Tiered Assembly Cascades** — magnificent stepped wooden-and-brass ramps cascading gracefully down from elevation $Z_2$ to $Z_0$, evoking the satisfying motion of a marble run or desktop clockwork toy.

Parts arrive as chunky wooden-and-brass tokens stamped with a **Rank** (1 through 13, styled after ancient playing card ranks) and a **Polarity** (Sun Polarity ☼ / Moon Polarity ☽). Tokens slide down seven stepped chutes, obeying the eternal laws of Klondike Solitaire.

#### Rules
- A token may only latch onto a stack if its Rank is exactly 1 lower than the current top, and its Polarity is opposite (e.g., a Rank 7 ☽ Moon token onto a Rank 8 ☼ Sun token).
- When a chute completes an unbroken sequence from Rank 13 down to Rank 1, a wooden gate opens with a cheerful *click-clack*, and the entire sequence rolls into the packing hopper, emerging as a **Masterwork Clockwork Core**.
- Empty chutes can only be started with a Rank 13 "Crown" piece.
- Unplaceable tokens slide into a rotating **Waiting Carousel**; if the carousel overfills, the assembly line pauses politely.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Carousel Diverter** | $Z_2$ feed | 2×1 | Brass flipper gate with painted Sun and Moon icons. | Directs tokens to the first valid matching cascade chute. |
| **Token Buffer Tower**| $Z_1$ | 2×2 | Spiral wooden marble tower with glass observation windows. | Stores up to 6 waiting tokens, releasing them as openings appear. |
| **Mechanical Card Sweeper**| $Z_0$ | 4×1 | Articulated wooden sweeper rake with rubber rollers. | Instantly clears completed 13-to-1 sequences onto exit belts. |
| **Polarity Inverter** | $Z_1$ | 1×1 | Rotating magnetic copper ring through which tokens roll. | Flips a token's polarity from Sun ☼ to Moon ☽ for 5 kW power. |

---

### 5.6 Mahjong Solitaire — Ancient Stepped Vault Deconstruction

**Factory Role:** Ancient Ruin Excavation, Rare Mineral Recovery & Salvage  

```
                 [ Top Tile ]
                   +-----+
                  /  ●  /|
                 +-----+ |
           +-----+  ●  +-----+      <- Multi-Tier 3D Pyramid
          /  ▲  /|     /  ▲  /|        Pluck matching free
         +-----+ |    +-----+ |        pairs from outer edges!
         |  ▲  |/     |  ▲  |/
         +-----+      +-----+
```

#### Concept
The richest caches of ancient Architect components are entombed within **Stepped Vault Pagodas** — exquisite 3D multi-layered monuments of polished ceramic and jade tiles arranged in classic Mahjong formations (The Turtle, The Pagoda, The Dragon). These structures sit majestically in overgrown courtyards, glistening under soft sunlight.

To harvest these rare treasures, the player constructs a **Miniature Gantry Crane** above the vault. The crane can only lift tiles that are completely unburdened from above and free on either their left or right edge.

#### Rules
- Plucking two accessible tiles with matching rune symbols matches the pair, sending both into the factory's conveyor chutes.
- Removing pairs carelessly can isolate single tiles deep beneath the pyramid, locking them away as "Stranded Tiles".
- Rare **Architect Bloom Tiles** (wild seasonal tiles) can match with any other bloom tile, releasing bursts of ancient research dust.
- If no valid moves remain, the remaining tiles become brittle stone, requiring an acoustic resonator to safely disassemble without loss.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Miniature Gantry Crane**| $Z_2$ frame| 4×4 | Delicate brass lattice frame with an automated winch. | Lifts and matches exposed identical tile pairs every 3 seconds. |
| **Glint Scanner** | $Z_1$ mount| 1×1 | Whimsical glowing lantern hanging from a wooden post. | Projects soft golden highlights onto all currently free, matchable tiles. |
| **Crane Tile-Shuffler** | $Z_2$ frame| 2×2 | Articulated grabber arm with velvet-cushioned tongs. | Rearranges 2 blocked tiles into free positions using acoustic power. |
| **Relic Cleansing Vat** | $Z_0$ | 2×2 | Water basin filled with pink lotus flowers and bubbling water. | Cleanses salvaged tiles, converting stone cruft into pure sand and clay. |

---

### 5.7 Connections — The Specimen Carousel & Material Classification Lab

**Factory Role:** Alloy Discovery, Tech Tree Breakthroughs & Quality Control  

```
        +-----------------------------------------------+
        |        The Specimen Carousel (16 Pots)        |
        |  [Jar 01]   [Jar 02]   [Jar 03]   [Jar 04]    |
        |  [Jar 05]   [Jar 06]   [Jar 07]   [Jar 08]    |  <- Select 4 Jars
        |  [Jar 09]   [Jar 10]   [Jar 11]   [Jar 12]    |     sharing a hidden
        |  [Jar 13]   [Jar 14]   [Jar 15]   [Jar 16]    |     botanical/mineral
        +-----------------------------------------------+     affinity!
                                 |
                                 v
        Correct group lifts up with golden sparkles & unlocks new recipe!
```

#### Concept
Mined geological samples often emerge with unknown molecular affinities. Mixed shipments arrive at the **Classification Laboratory** — a cozy wooden pavilion featuring a revolving brass carousel of 16 glass specimen jars containing curious ores, seeds, and clockwork cogs.

The Constructor inspects the 16 items and deduces **four groups of four** connected by a subtle, clever underlying property. Solving a group triggers a shower of golden confetti, filing a beautifully illustrated new recipe page directly into the Constructor's Field Handbook.

#### Example Specimen Groupings

| Secret Category Label | Specimen Jar Contents | Illustrated Handbook Reward |
|---|---|---|
| *"Gleams like Golden Honey"* | Amber Resin, Sun-Topaz, Brass Filings, Pyrite Acorn | Bronze Alloy Recipe |
| *"Born from High Pressure"* | Compressed Slate, Void Pebble, Diamond Spore, Dense Obsidian | Diamond-Tipped Drill Bit |
| *"Rings with Musical Chimes"*| Resonite Shard, Brass Bell, Quartz Tuning Fork, Harmonic Glass | Acoustic Power Conductor |
| *"Architect Artifacts"* | Engraved Runic Cog, Ceramic Lens, Whistling Whistle, Sun Dial | Ancient Automaton Blueprint |

#### Rules
- A shipment presents a 4×4 array of 16 specimen jars on the pavilion table.
- Selecting 4 jars submits an inspection guess.
- Correct guesses burst with colorful ribbon particles and remove the 4 jars, leaving fewer choices.
- Guessing incorrectly wobbles the carousel and burns one of 4 **Tea Candles** burning on the desk. If all 4 candles burn out, the remaining samples are safely routed to generic smelters as raw scrap.
- Flawless categorization (solving all 4 groups with 0 mistakes) awards a **Golden Wax Seal**, granting bonus tech insight.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Specimen Carousel** | $Z_0$ | 4×4 | Octagonal wooden gazebo with revolving brass lazy-susan. | Houses the active classification puzzle for manual or automated study. |
| **Curator's Automaton** | $Z_0$ | 2×2 | Little clockwork scribe wearing tiny spectacles. | Cross-references past catalog entries to auto-solve known groups. |
| **Heuristic Lens** | $Z_1$ mount| 1×1 | Magnifying glass mounted on a brass articulated swivel arm. | Highlights subtle visual cues and shared material traits on jars. |
| **Sample Recycler Chute**| $Z_0$ | 1×2 | Polished brass slide leading to an underground bin. | Recycles failed shipments into baseline iron and copper ingots. |

---

### 5.8 Wordle — Clockwork Tumbler Ciphers & Ancient Pillar Calibration

**Factory Role:** Ruin Portals, Island Gates & Advanced Machine Tuning  

```
        Ancient Archway (5-Ring Clockwork Dial)
       +---------------------------------------------+
       |   [ ᚛ ]     [ ᚜ ]     [ ᚠ ]     [ ᚱ ]     [ ᚦ ]   |  Guess 1: 2 Green, 1 Yellow
       |  (Green)   (Gray)   (Yellow)   (Gray)   (Green) |
       +---------------------------------------------+
       Spin the brass dials to test runic combinations!
```

#### Concept
Ancient stone gates leading to untouched floating islands, as well as high-tier Architect super-machines, are sealed by **Clockwork Tumbler Portals** — monumental moss-covered stone archways featuring five massive cylindrical brass dials engraved with glowing ancient runic glyphs.

The player approaches the console, spinning the five tumblers to input a 5-glyph sequence. Upon pulling the wooden activation lever, the gate locks into place with hefty, reverberating mechanical clunks, illuminating each ring with glowing watercolor halos:
- **Soft Jade Green:** Correct glyph in the correct tumbler ring.
- **Warm Amber Yellow:** Correct glyph, but belongs in a different tumbler ring.
- **Cool Slate Grey:** Glyph does not appear in this portal's lock code.

#### Rules
- The player has 6 calibration attempts before the portal enters a gentle 5-minute twilight slumber.
- Codes are procedurally generated per world seed and recorded automatically into the player’s **Field Journal**.
- Portals within the same archipelago biome often share regional root glyphs, rewarding players who pay attention to architectural motifs.
- Cracking a cipher on the first or second guess earns a **Luminous Sun-Chest**, containing rare ancient blue fabric and golden gears.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Cipher Scribe Desk** | $Z_0$ | 2×2 | Wooden writing bureau with rolled parchment and quill. | Automatically records all guess results into the Field Journal. |
| **Acoustic Stethoscope**| $Z_0$ | 1×1 | Brass horn pressed against the stone ruin pillar. | Narrows down glyph choices by listening to internal tumbler clicks. |
| **Clockwork Key-Turner**| $Z_0$ | 2×2 | Automaton winding-engine that mounts directly to portal dials. | Systematically tests non-eliminated combinations over time. |
| **Glyph Decryption Lens**| $Z_1$ mount| 1×1 | Stained glass prism that casts colored beams onto dials. | Pre-solves one guaranteed glyph for 50 kW power. |

---

### 5.9 Pips — Dice-Faceted Bento Crates & Sky-Barge Trade Manifests

**Factory Role:** Export Economy, Island Commerce & Rare Material Import  

```
        Sky-Barge Dock (3x4 Bento Crate)
       +--------+--------+--------+  Target Row Sums:
       | [:::]  | [ : ]  | [::]   |  Row 1: [ 6 + 2 + 4 = 12 ] (Check!)
       | [ · ]  | [:::]  | [ : ]  |  Row 2: [ 1 + 6 + 2 =  9 ] (Check!)
       | [::]   | [ · ]  | [:::]  |  Row 3: [ 4 + 1 + 6 = 11 ] (Check!)
       | [ : ]  | [::]   | [ · ]  |  Row 4: [ 2 + 4 + 1 =  7 ] (Check!)
       +--------+--------+--------+
         Col 1    Col 2    Col 3     <-- Target Column Sums
         (13)     (13)     (13)
```

#### Concept
The Constructor is not alone in the skies. Adorable wooden **Sky-Barges** and whimsical paddle-steamer airships with cheerful billowing sails dock at **Floating Trade Wharves** along the island’s edge. They carry contracts from distant sky realms requesting exports packed into compartmentalized wooden **Bento Cargo Crates** (a 3×4 grid of item slots).

Finished clockwork parts coming off the Solitaire assembly lines are stamped with glowing golden **Pips** (1 through 6 dots, like dice or dominoes), reflecting their craftsmanship tier. Fulfilling a contract requires packing goods so that the sum of pips across each row, column, and diagonal matches the merchant's manifest!

#### Rules
- A Bento Crate holds 12 items arranged in 3 columns and 4 rows.
- The merchant manifest specifies required numerical sums for each row and column, plus optional diagonal challenges.
- Achieving exact sums on all axes earns a **Five-Star Stamp**, rewarding rare **Sky-Silk**, exotic seeds, and off-world brass components.
- Shipping partially-filled crates still earns coins and baseline goods, ensuring new players are never blocked.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Wharf Gantry Crane** | $Z_1 \to Z_0$| 4×6 | Wooden pier crane with striped canvas awning and ropes. | Loads stamped crates onto docked sky-barges automatically. |
| **Pip Stamper** | $Z_0$ | 2×2 | Heavy brass stamper that imprints glowing dots on passing goods. | Can upgrade an item's pip count by +1 using gold leaf foil. |
| **Manifest Sorting Abacus**| $Z_0$ | 2×2 | Giant wooden abacus with sliding colored beads. | Diverts goods with needed pip values into waiting crate slots. |
| **Pneumatic Crate Packer** | $Z_0$ | 3×3 | Overhead suction arm with soft leather bellows. | Instantly packs crates when all target sums are satisfied. |

---

### 5.10 Crossword — Luminous Leyline Intersections & Blueprint Synergy Grids

**Factory Role:** Sprawling Spatial Optimization & Passive Efficiency Leylines  

```
   [ Chromatic Autoclave ] -----( REFINE Thread )----->
             |
             | ( HEAT Thread )
             v
   +---------------------+  <-- Intersection Point:
   |   GOLDEN SYNERGY    |      A glowing golden flower blooms!
   |   LILY BLOSSOMS!    |      +25% Speed & Radiant Cascade Bonus!
   +---------------------+
             ^
             | ( HEAT Thread )
             |
   [ Smelting Hearth ] --------( HEAT Thread )-------->
```

#### Concept
The island floor is an ancient tapestry. Every building placed in the world carries two or three **Functional Keywords** carved into its foundation plaque (e.g., the Chromatic Autoclave carries `REFINE`, `HEAT`, `GEM`; the Smelting Hearth carries `HEAT`, `SMELT`, `METAL`).

When active, buildings project delicate, luminous pastel **Leyline Threads** across the grass along their cardinal axes. When two matching threads intersect at an open ground tile, a **Golden Synergy Lily** blossoms from the moss. Nearby machines within the lily’s fragrant golden glow gain tremendous passive production buffs!

#### Rules
- Leyline threads project in straight cardinal directions across $Z_0$ ground until blocked by elevated cliffs or opposing leylines.
- Two identical threads (e.g., `HEAT` crossing `HEAT`) sprout a **Synergy Lily**, granting +20% production speed to both origin structures.
- Contradictory threads (e.g., `HEAT` crossing `CHILL`) create a **Frost Knot**, suppressing bonuses until resolved.
- Players can toggle the **Tapestry Lens** (Keybind: `X`) to view all glowing colored threads traversing the landscape.

#### Automation

| Building | Elevation | Size | Visual Treatment | Function |
|---|---|---|---|---|
| **Brass Leyline Prism** | $Z_0$ | 1×1 | Small multifaceted glass crystal on a carved stone birdbath. | Bends and redirects passing leylines by 90°. |
| **Resonance Thread Weaver**| $Z_0$ | 2×2 | Loom-like machine with spinning wooden shuttles. | Amplifies the range of all leylines passing through its frame. |
| **Lotus Harmonizer** | $Z_0$ | 3×3 | Serene stone pond with glowing water lilies. | Converts conflicting Frost Knots into peaceful neutral harmony. |
| **Architect Beacon Pillar**| $Z_1$ | 2×2 | Tall slender marble pillar that broadcasts keywords over cliffs. | Allows leylines to project across $Z_1$ elevations without ground obstruction. |

---

## 6. World Structure & Progression

### 6.1 Floating Diorama Map Layout

The world consists of discrete, procedurally scattered floating diorama islands linked by wooden suspension bridges, stone arches, and cloud-skiffs:

```
                          [ THE DEEPROOT CITADEL ]
                         (Endgame Floating Mountain)
                                    /     \
                                   /       \
          [ CIPHER PEAKS ]                  [ TRADE ATOLL ]
       (Wordle Ruin Monoliths)           (Pips Sky-Barge Wharves)
                |                                  |
          [ VAULT HIGHLANDS ]               [ SORTING GROVES ]
       (Mahjong Stepped Pagodas)         (Connections Carousels)
                \                                  /
                 +----------+          +----------+
                            |          |
                      [ CHROMITE BASIN ]
                   (Bejeweled Autoclaves)
                            |
                      [ LATTICE FLATS ]
                   (Tetris Multi-Layer Belts)
                            |
                     [ CRADLE MEADOWS ]
                 (Cozy Starting Diorama)
```

| Biome Island | Visual Aesthetic | Puzzle Emphasis | Unique Flora & Mineral |
|---|---|---|---|
| **Cradle Meadows** | Bright emerald clover, rolling grassy knolls, sunny daisies, mossy stone walls. | Minesweeper (Easy 4×4), Basic Belts. | Soft Clay, Iron Pebbles, Clover Blossoms. |
| **Lattice Flats** | Stepped white chalk terraces, shallow turquoise reflecting pools, geometric quartz. | Tetris Multi-Level Conveyors ($Z_0/Z_1/Z_2$). | Quartz Crystals, Copper Sand, Paper Birch Trees. |
| **Chromite Basin** | Iridescent opal-colored mudflats, giant glowing toadstools, bubbling hot springs. | Bejeweled Chromatic Autoclaves. | Ruby Nodules, Prismatic Geodes, Glowing Spores. |
| **Resonant Grove** | Deep lavender meadows, ancient acoustic pine forests, wind chimes in the boughs. | Sudoku 9×9 Monolith Halos. | Resonite Spar, Violet Bellflowers, Ancient Slate. |
| **Cascade Rifts** | Towering stepped sandstone cliffs, cascading waterfalls with wooden watermills. | Solitaire Step-Tiered Marble Cascades. | Terracotta Clay, Hardened Pine Resin, Gold Flakes. |
| **Vault Highlands** | Misty bamboo gardens, weathered granite pagodas, serene raked gravel paths. | Mahjong Stepped Pagodas. | Jade Tiles, White Marble, Ancient Ceramic Shards. |
| **Sorting Groves** | Quaint orchard terraces with vintage wooden packing sheds and brass weather vanes. | Connections Specimen Carousels. | Curious Seeds, Amber Resins, Botanical Samples. |
| **Cipher Peaks** | Windswept crags bathed in twilight, colossal ancient brass archways wrapped in ivy. | Wordle Clockwork Tumblers. | Runic Slate, Star-Metal Ore, Twilight Ferns. |
| **Trade Atoll** | Coastal wooden boardwalks, colorful striped canvas canopies, floating docks on cloud-sea. | Pips Dice-Faceted Bento Crates. | Driftwood, Sky-Silk, Merchant Gold Leaf. |
| **The Deeproot Citadel**| Colossal floating mountain island wrapped in golden Architect machinery and clouds. | All 10 Puzzle Systems Unified. | Pure Logic Substrate, Star Core Fragments. |

### 6.2 Progression Gates & Archipelago Unlocks

Progress unfolds like turning the pages of an illustrated storybook:

```
[ Awakening in Cradle Meadows ]
             |
             +---> Craft Wooden Drill & Survey Tapper
             |
             v
[ The First Sky-Bridge ] ---> Solve 4x4 Minesweeper Meadow
             |
             v
[ Reaching Lattice Flats ] ---> Unlock Multi-Tier Z1 Conveyor Viaducts
             |
             v
[ The Chromite Awakening ] ---> Construct First Glass Autoclave (Bejeweled)
             |
             v
[ Unlocking the Portals ] ---> Crack 5-Ring Wordle Archway to Vault Highlands
             |
             v
[ The Grand Skyways ] ---> Unify All 10 Systems to Awaken the Resonance Engine!
```

---

## 7. Resources & Materials

### 7.1 Raw Resources (Extracted via Tactile Prospecting)

All raw resources appear as tactile, chunky 3D objects with distinct toybox shapes and colors:

| Raw Mineral | Visual Representation | Palette | Common Polyomino Shapes |
|---|---|---|---|
| **Iron Ore** | Smooth beveled hematite pebbles with painted orange seams. | Coral Red / Terracotta | I-Bar (1×3), Domino (1×2) |
| **Copper Ore** | Chunky hexagonal nuggets with bright cyan patina. | Sky Cyan / Turquoise | L-Triomino, S-Tetromino |
| **Gold Ore** | Sparkling cubic nuggets that glint in the sun. | Buttercup Yellow / Gold | Single Voxel, Square (2×2) |
| **Logic Crystal** | Translucent prism crystals with soft internal green light. | Mint Green / Emerald | T-Tetromino, Z-Tetromino |
| **Voidite** | Mysterious smooth obsidian marbles that cast purple shadows. | Deep Blackberry / Violet | Rare Corner Voxel (3-cell) |

### 7.2 Intermediate Materials & 3D Polyomino Shards

| Intermediate Item | Form & Crafting Origin | Primary Application |
|---|---|---|
| **Ingot Cube** | Beveled metallic cube produced by squishing Tetris rows in Compressors. | Feedstock for Chromatic Autoclaves. |
| **Refined Plate** | Polished flat plate with rounded edges (Iron, Copper, Bronze). | Base material for conveyor tracks and machine frames. |
| **Clockwork Cog** | Delicate brass gear wheel with intricate cutouts. | Primary component in arms, rotators, and cranes. |
| **Acoustic Sine Coil**| Rolled copper wire spool with floating musical note particles. | Power grid conduit and tuning fork calibration. |
| **Alloy Ingot** | Shimmering two-tone ingot from Bejeweled Match-4 reactions. | Advanced tier buildings, elevators, and gantry cranes. |
| **Prismatic Core** | Radiant rainbow-faceted gemstone from Match-5 reactions. | High-tier portal keys and the Resonance Engine core. |
| **Ranked Assemblage**| Stamped wooden-and-brass tokens (Ranks 1–13) from Solitaire cascades. | Modular parts for engines, airships, and advanced robots. |
| **Sealed Bento Crate**| Wooden compartmentalized crate stamped with glowing gold pips. | Export currency for rare off-world components. |

### 7.3 Finished Goods & The Grand Resonance Engine

```
 [ Refined Iron ] + [ Clockwork Cog ] -----> [ Mechanical Automaton Arm ]
 [ Refined Copper ] + [ Sine Coil ] -------> [ Harmonic Acoustic Resonator ]
 [ Jade Tile ] + [ Prismatic Core ] -------> [ Celestial Focusing Lens ]
 [ Voidite Ingot ] + [ Star-Metal ] -------> [ Gravity Suspension Core ]
                                                    |
                                                    v
                                    [ THE GRAND RESONANCE ENGINE ]
                                    (Harmonizes the Sky Archipelago)
```

---

## 8. Buildings & Machines

All structures feature clean, toy-like footprints adhering to a strict 2.5D isometric tile grid. Machines exhibit charming physical animations: spinning brass gears, bubbling glass globes, and rocking counterweights.

### 8.1 Extraction Tier ($Z_0$ Ground Anchors)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Wooden Survey Tower** | 2×2 | 5 kW | Timber tower with an animated brass weathervane and lookout deck. |
| **Clockwork Wellhead** | 1×1 | 8 kW | Brass water-pump mechanism with a rocking wooden seesaw arm. |
| **Logic Diviner** | 2×2 | 15 kW | Carved stone torii arch suspending a swinging pendulum. |
| **Vapor Vent Tapper** | 2×2 | Generates | Copper kettle boiling gently, emitting pleasant lavender steam. |

### 8.2 Transport Tier (Multi-Layer $Z_0/Z_1/Z_2$ Logistics)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Conveyor Roller (10)** | 1×1 | 0.5 kW/tile | Brass roller tracks with miniature spinning cogs along the rim. |
| **Viaduct Pylon ($Z_1$)** | 1×1 | — | Beveled stone pillar supporting elevated overpass tracks. |
| **Skyway Suspension ($Z_2$)**| 1×1 | — | Elegant white wooden trestle with brass tension cables. |
| **Spiral Lifter** | 2×2 | 10 kW | Glass cylindrical tower with an internal rotating corkscrew ramp. |
| **3D Cargo Compressor** | 3×4 | 25 kW | Tall glass hopper building with a heavy brass drop-hammer stamper. |

### 8.3 Refining Tier (Glass Terrarium Autoclaves)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Chromatic Autoclave (6×6)**| 4×4 | 30 kW | Domed glass terrarium with bubbling gem trays and brass valve wheels. |
| **Grand Autoclave (9×9)** | 6×6 | 75 kW | Colossal greenhouse dome with glowing crystal chandelier. |
| **Brass Swap-Piston** | Internal | 10 kW | Spring-loaded clockwork armature operating inside the glass dome. |
| **Resonance Bell Jar** | 2×2 | 15 kW | Fluted crystal bell that emits warm amber rings during cascades. |

### 8.4 Power Tier (Harmonic Crystal Pillars)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Acoustic Monolith** | 3×3 | Generates | Stepped granite obelisk carved with glowing, musical runic channels. |
| **Tuning Fork Pylon** | 1×1 | 5 kW | Slender brass fork humming with visible soundwave rings. |
| **Cloud-Bridge Conductor**| 1×4 | 15 kW | Luminous floating stepping-stones linking island power grids. |

### 8.5 Assembly Tier (Cascading Stepped Chutes)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Marble-Run Cascade** | 4×8 | 40 kW | Tiered wooden ramp cascading from $Z_2 \to Z_0$ with 7 marble tracks. |
| **Carousel Diverter** | 2×1 | 10 kW | Clockwork paddle wheel sorting pieces by Sun and Moon polarity. |
| **Token Buffer Tower** | 2×2 | 12 kW | Spiral wooden storage tower holding waiting assembly tokens. |

### 8.6 Deconstruction Tier (Miniature Gantry Cranes)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Miniature Gantry Crane**| 4×4 | 25 kW | Delicate brass gantry riding on wooden timber rails over the vault. |
| **Glint Lantern Post** | 1×1 | 3 kW | Curved wrought-iron streetlamp projecting warm golden spotlights. |
| **Relic Cleansing Pool** | 2×2 | 15 kW | Carved stone birdbath filled with pink water lilies and bubbling water. |

### 8.7 Classification Tier (Pneumatic Carousel Pavilions)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Classification Pavilion**| 4×4 | 25 kW | Octagonal timber gazebo with a striped canvas roof and lazy-susan. |
| **Curator's Desk** | 2×2 | 10 kW | Cozy mahogany roll-top desk with a ticking clockwork typewriter. |
| **Pneumatic Tube Depot** | 2×1 | 15 kW | Polished copper pipes swooshing glass sample canisters in and out. |

### 8.8 Cipher Tier (Clockwork Monolith Dials)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Cipher Scribe Bureau** | 2×2 | 15 kW | Wooden lectern with an illuminated magnifier and parchment rolls. |
| **Tumbler Winding Engine**| 2×2 | 30 kW | Heavy cast-iron flywheel mechanism that slowly turns stone dials. |
| **Acoustic Stethoscope** | 1×1 | 5 kW | Oversized brass listening horn angled against ancient ruin stone. |

### 8.9 Trade Tier (Floating Wooden Wharves & Sky-Docks)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Sky-Barge Wharf** | 4×8 | — | Wooden boardwalk pier with mooring bollards, rope coils, and awnings. |
| **Wharf Gantry Crane** | 3×3 | 20 kW | Wooden boom crane with heavy hemp ropes and a hanging cargo net. |
| **Pip Stamper** | 2×2 | 35 kW | Pneumatic brass press stamping glowing gold leaf dots on boxes. |

### 8.10 Synergy Tier (Resonance Beacon Emitters)

| Building | Footprint | Power | Aesthetic Treatment |
|---|---|---|---|
| **Leyline Prism Beacon** | 1×1 | 5 kW | Cut glass crystal floating above a carved marble basin. |
| **Tapestry Loom** | 2×2 | 20 kW | Wooden frame weaving glowing pastel thread lines along the ground. |
| **Lotus Harmonizer Pool**| 3×3 | 15 kW | Quiet koi pond covered in glowing water lilies that soothe bad vibes. |

---

## 9. Automation & Research Tree

### 9.1 Philosophy: From Tactile Hands to Clockwork Symphony

Research in GRIDLOCK is not an abstract progress bar. The tech tree is presented as the **Architect’s Illustrated Field Handbook** — a charming, cloth-bound adventurer's notebook with hand-inked drawings, watercolor annotations, and gold-foil stamping.

Progression follows a satisfying philosophical arc:
1. **The Artisan Phase:** You operate machines by hand — tapping tiles, turning dials, sorting specimens.
2. **The Clockwork Phase:** You build miniature mechanical helpers — spring-wound arms, gravity hoppers, and pneumatic flippers.
3. **The Symphony Phase:** Your multi-layer factories hum autonomously, orchestrating complex puzzle solutions while you sit back and admire the diorama.

### 9.2 The Illustrated Field-Guide Tech Tree

```
[ PAGE 1: WOOD & STONE ]
   |
   +---> [ Timber Surveying ] ----> Survey Tower -> Logic Diviner -> Steam Tapper
   |
   +---> [ Multi-Layer Logistics ] -> Viaduct Pylons (Z1) -> Spiral Lifters -> 3D Compressors
   |
   +---> [ Glass Autoclaves ] ------> Brass Swap-Piston -> Bell Jar -> Match-5 Catalysis
   |
[ PAGE 2: BRASS & HARMONY ]
   |
   +---> [ Acoustic Monoliths ] ---> Resonance Halos -> Harmonic Bridges -> Symphony Grove
   |
   +---> [ Stepped Cascades ] -----> Marble-Run Ramps -> Polarity Inverters -> Clockwork Sweepers
   |
   +---> [ Pagoda Excavation ] ----> Miniature Gantry -> Glint Lanterns -> Lotus Cleansers
   |
[ PAGE 3: SECRETS OF THE SKY ]
   |
   +---> [ Specimen Curation ] ----> Carousel Pavilion -> Curator Automaton -> Golden Seals
   |
   +---> [ Runic Decryption ] -----> Scribe Bureau -> Winding Engine -> Master Glyph Prisms
   |
   +---> [ Sky-Barge Commerce ] ---> Wharf Gantry -> Pip Stampers -> Bento Packing Abacus
   |
   +---> [ Leyline Tapestry ] -----> Prism Beacons -> Tapestry Looms -> Lotus Harmonizers
                                              |
                                              v
                              [ THE GRAND RESONANCE ENGINE ]
```

### 9.3 Insight Sparks & Harmonic Research Currency

Research is funded by **Insight Sparks** — glowing, golden firefly-like motes earned whenever puzzle systems operate cleanly and harmoniously:
- **Minesweeper:** Earned by uncovering safe terrain without triggering overcharge steam.
- **Tetris:** Earned each time a 3D Compressor stamps an unbroken row of polyominoes.
- **Bejeweled:** Earned exponentially during multi-step gem cascades.
- **Sudoku:** Earned per minute as long as an island grove maintains perfect harmonic balance.
- **Solitaire:** Earned upon completing a full 13-to-1 descending sequence.
- **Mahjong:** Earned whenever matching tile pairs are plucked from ancient pagodas.
- **Connections:** Earned by correctly grouping 4 specimen jars on the carousel.
- **Wordle:** Earned when cracking ancient ruin gates (with hefty bonuses for $\le 3$ guesses).
- **Pips:** Earned whenever an export Bento Crate hits all target row and column sums.
- **Crossword:** Earned continuously by active Golden Synergy Lilies blooming at leyline junctions.

---

## 10. Visual & Audio Design

### 10.1 Visual Style — The Tunic × Shapez 2 Fusion

```
  +-----------------------------------------------------------------+
  |                  THE DIORAMA AESTHETIC                          |
  |                                                                 |
  |   • Isometric Axonometric Camera (Soft Tilt-Shift Blur at Edges)|
  |   • Warm Amber Sunbeams & Volumetric God Rays                   |
  |   • Beveled Low-Poly Surfaces with Tactile Clay/Wood Matte Finish|
  |   • Floating Cloud-Sea Horizon with Pastel Mauve & Peach Mist   |
  |   • Multi-Level Spatial Belts (Z0 Ground, Z1 Overpass, Z2 Sky)  |
  +-----------------------------------------------------------------+
```

- **World Rendering:** The world is treated as a handcrafted miniature diorama or tabletop architectural model. Cliffs have soft, beveled edges showing layers of sandstone and ancient terracotta brickwork. Soft green moss and patches of clover drape over stone retaining walls.
- **Lighting & Atmosphere:** Warm, golden-hour sunlight streams across the islands, casting soft diffuse shadows. Volumetric god rays filter through the branches of stylized geometric pine trees. A subtle tilt-shift depth-of-field lens blur softens the upper and lower edges of the screen, creating an irresistible sense of miniature scale.
- **Materials & Textures:** Avoid harsh, noisy textures. Everything feels pleasant to touch: satin-finish painted wood, polished porcelain tiles, clear bubbly hand-blown glass, brushed brass with soft patina, and candy-translucent mineral crystals.
- **Factory Architecture (Shapez 2 Influence):** Conveyors are pristine, clean, and modular. Instead of tangled visual chaos, belts travel along clear vertical layers ($Z_0, Z_1, Z_2$) with elegant stone pillars and brass brackets. Polyomino shards glide smoothly, snapping into place with crisp mechanical elegance.

### 10.2 Audio Design — Generative Toybox Acoustic & Chiptune Symphony

GRIDLOCK's audio combines the warm, nostalgic, acoustic intimacy of **TUNIC** (finger-picked classical guitar, felted upright piano, wooden kalimba, warm cello) with the crisp, rhythmic, mechanical satisfaction of a living clockwork factory.

Every puzzle system functions as an instrument in an **emergent generative soundtrack**:

| Puzzle System | Tactile Sound Effect | Musical Voice in the Factory Symphony |
|---|---|---|
| **Minesweeper** | Crisp wooden tile flips; hollow stone clacks. | Deep wooden marimba & warm acoustic bass. |
| **Tetris** | Smooth marble rolling; satisfying heavy rubber stamper *thud*. | Crisp brush snare & wooden woodblock percussion. |
| **Bejeweled** | Sparkling glass marble clinks; ascending windchimes. | Delicate music box & celesta melody lead. |
| **Sudoku** | Resonant singing bowls; vibrating brass tuning forks. | Warm, sustained cello & harmonium chords. |
| **Solitaire** | Wooden tiles sliding down ramps; mechanical click-clack gates. | Playful pizzicato strings & fingerpicked guitar. |
| **Mahjong** | Heavy ceramic tiles tapping together; soft rope winch creaks. | Resonant bamboo clappers & gentle zither accents. |
| **Connections** | Pneumatic glass tube *whump*; brass desk bell ding. | Staccato vibraphone & whimsical clarinet phrases. |
| **Wordle** | Heavy brass tumbler clicks; reverberating stone door rumble. | Suspenseful bowed double-bass & swelling harp arpeggios. |
| **Pips** | Wooden dice rolling in a felt bowl; brass cash-register chime. | Jaunty acoustic accordion & gentle tambourine. |
| **Crossword** | Soft harp glissando; delicate crystal bell tone. | Radiant choir pad & soaring flute harmony. |

A small, quiet bootstrapping workshop sounds like a peaceful meadow with sporadic wooden clicks and chirping crickets. A sprawling late-game factory sounds like a breathtaking, lush acoustic symphony!

### 10.3 UI / UX Aesthetic — The Golden Inscribed Explorer's Manual

All user interfaces eschew cold, industrial sci-fi HUDs in favor of an **in-world illustrated adventure booklet**:
- Menus appear as open pages of a leather-bound, gilt-edged pocket journal.
- Blueprints look like watercolor architectural schematics with hand-drawn ink arrows and cute marginalia doodles.
- Buttons look like embossed wax seals, wooden toggles, and polished mother-of-pearl tabs.
- Fonts use an elegant, friendly serif for titles and a crisp, readable hand-drafted monospace for engineering metrics.

---

## 11. UI / UX Philosophy & Diorama Controls

### 11.1 Seamless Diorama Inspection & Isometric Lens Overlays

Rather than jarring full-screen menu takeovers, inspecting any machine tilts the camera gently closer while keeping the charming 3D world alive in the background.

Players can instantly toggle specialized **Inspection Lenses** to reveal puzzle data overlaid directly onto the 3D diorama:

| Keybind | Lens Name | Visual Diorama Overlay |
|---|---|---|
| `M` | **Surveyor's Lens** | Highlights Minesweeper numbers and hazard radii on ground voxels. |
| `T` | **Logistics Lens** | Color-codes conveyor layers ($Z_0$ Green, $Z_1$ Blue, $Z_2$ Purple) and shows shard shapes. |
| `R` | **Alchemist's Lens** | Renders internal gem grids inside all Chromatic Autoclaves at a glance. |
| `P` | **Harmonic Lens** | Displays floating pastel frequency halos ($1–9$) and warns of acoustic discord. |
| `A` | **Artisan's Lens** | Shows sequence progress and token ranks along all Marble Cascades. |
| `V` | **Antiquarian's Lens**| Spotlights free, unblocked tile pairs on all ancient Mahjong pagodas. |
| `C` | **Botanist's Lens** | Opens the Specimen Carousel overlay with active classification clues. |
| `W` | **Cipher Lens** | Highlights nearby ruin archways, guess history, and discovered runic roots. |
| `$` | **Merchant's Lens** | Displays sky-barge arrival timers and pending Bento Crate sum requirements. |
| `X` | **Tapestry Lens** | Visualizes glowing pastel leyline threads and blossoming Synergy Lilies. |

### 11.2 Tactile Pause-to-Plan Mode

Tapping the `Spacebar` engages **Pause-to-Plan Mode**:
- The world freezes in a dreamy, golden twilight amber.
- Falling polyominoes, bubbling gems, and rolling marbles hang peacefully in mid-air.
- The player can freely pan the isometric camera, inspect intricate puzzles, place holographic ghost blueprints, and re-route multi-tier conveyors at their own relaxed pace.
- Tapping `Spacebar` again unfreezes time with a soft chime, and all clockwork machinery resumes in harmony.

### 11.3 Charming Physical Alerts & Postcard Toast System

Instead of abrasive red flashing sirens, notifications arrive as delightful **In-World Postcards** that slide neatly into the bottom corner of the screen:
- *"A curious steam bubble has popped in Cradle Meadows! (Tap with wrench to soothe)."*
- *"A sky-barge from the Opal Isles has docked at Wharf 2 requesting a 12-Pip crate!"*
- *"Hooray! Grove 4 has achieved perfect 9×9 harmonic resonance! Wildflowers are blooming!"*

---

## 12. Open Questions & Design Risks

### 12.1 Spatial & 2.5D Balance Considerations

| Potential Risk | Design Mitigation |
|---|---|
| **Isometric visual occlusion** (tall buildings hiding conveyors behind them). | Buildings feature semi-transparent glass domes and cutaway frames; pressing `Alt` renders foreground structures as cute translucent wireframes. |
| **Sudoku arithmetic fatigue**. | Frequency bands are represented by 9 distinct pastel colors and musical symbols, not just raw math; auto-conductor totems eliminate manual re-tuning. |
| **Multi-layer belt confusion ($Z_0/Z_1/Z_2$)**. | Each layer snaps to distinct vertical heights with clear shadow casting and color-coded pylon supports (Shapez 2 style). |
| **Overwhelming puzzle variety**. | Puzzles unlock sequentially across distinct biomes over 10+ hours; early game focuses purely on intuitive Minesweeper and Tetris. |

### 12.2 Planned Expansion Modules

- [ ] **Co-Op Sky Islands:** Two little cloaked automata running a shared factory, tossing tokens across floating chasms!
- [ ] **Custom Ruin Pagoda Editor:** Players design and share custom Mahjong puzzle layouts through the Steam Workshop.
- [ ] **Photo & Diorama Mode:** Free-flight tilt-shift camera with depth-of-field sliders, time-of-day controls, and sticker frames.

### 12.3 Technical Architecture (Bevy 0.14+ 3D/Isometric Rust Engine)

The transition from a flat 2D engine to a rich 2.5D diorama leverages Bevy's modern 3D rendering pipeline:

```rust
// Core Perspective Setup in Bevy 0.14+
pub fn setup_diorama_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(14.0),
                scale: 1.0,
                ..default()
            }.into(),
            transform: Transform::from_xyz(20.0, 30.0, 20.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DioramaCameraMarker,
        DepthOfFieldSettings {
            focal_distance: 35.0,
            linear_variance: 0.1,
            ..default()
        },
    ));
}
```

- **Renderer:** Bevy 3D PBR with custom stylized toon-lit shaders, soft ambient occlusion (GTAO), and tilt-shift depth-of-field post-processing.
- **Isometric Projection:** Orthographic projection angled at $35.264^\circ$ pitch and $45^\circ$ yaw (true isometric), ensuring 2.5D tile grids align with pixel-perfect precision while supporting full 3D models and multi-level verticality.
- **Layered Spatial Grid:** Factory coordinates utilize discrete 3D integer coords: `GridPos { x: i32, y: i32, z: Layer }` where `Layer` represents `Ground (0)`, `Viaduct (1)`, or `Skyway (2)`.
- **ECS Plugin Isolation:** Each puzzle system is implemented as an independent Bevy plugin (`MinesweeperPlugin`, `TetrisConveyorPlugin`, `BejeweledRefineryPlugin`, etc.) communicating through decoupled Bevy event channels.

---

## 13. The Constructor — The Miniature Clockwork Engineer

### 13.1 Character Overview & Whimsical Form

The player embodies **The Constructor** — an adorable, diminutive clockwork automaton:
- **Appearance:** A small, pear-shaped brass-and-ceramic body standing roughly one tile tall. Wears a cozy knitted hood and hooded poncho (dyed in customizable pastel earth tones) and a tiny leather satchel.
- **The Winding Key:** A prominent, polished brass winding-key sits on the Constructor’s back, spinning merrily as they waddle along.
- **Animations:** Endearing waddling run cycle with gentle footstep *patter-patters*; cute idle animations such as pulling out a pocket compass, brushing clover seeds from their cloak, or sitting down to sip warm tea from a tiny canteen.

```
       [ Cute Knitted Hood ]
              ( o.o )   <-- Luminous Circular Visor Eyes
             /| === |\  <-- Cozy Woolen Poncho
            ( |     | )
             /       \  <-- Tiny Waddling Boots
             [ Winding Key Spins on Back! ]
```

### 13.2 Character Stats & Movement Physics

| Stat | Base Value | Upgrade Progression | Notes |
|---|---|---|---|
| **Waddle Speed** | 5 tiles/sec | +0.5 per Leather Boot upgrade | Smooth, responsive movement with gentle deceleration. |
| **Sprint Speed** | 8.5 tiles/sec | +0.5 per Brass Spring upgrade | Drains Winding Energy at 1.5×/sec; kicks up tiny dust puffs. |
| **Winding Energy** | 100 Key Turns | +25 per Mainspring upgrade | Recharges automatically when standing still or walking. |
| **Satchel Space** | 40 item slots | +10 per Expandable Satchel | Items stack neatly; overloaded state triggers a cute panting waddle. |
| **Tool Reach** | 2.0 tiles | +0.5 per Telescopic Arm upgrade| Distance to reach nodes, flip tiles, and interact with machines. |
| **Inspection Radius**| 4.0 tiles | +1.0 per Monocle upgrade | Range to view internal machine puzzle states. |
| **Shell Integrity** | 100 HP | +25 per Ceramic Plating upgrade| Absorbs accidental shockwave pops; regenerates in sunlight. |

### 13.3 Equipment Slots & Wearable Trinkets

The Constructor can equip six charming personal items:

| Slot | Category | Example Trinkets | In-World Effect |
|---|---|---|---|
| **Headwear** | Hood & Visor | *Surveyor's Monocle* | Permanently highlights Minesweeper numbers within 5 tiles. |
| **Garb** | Cloak & Poncho | *Deeproot Knit Poncho* | Grants immunity to thorn scratches and cold morning dew. |
| **Footwear** | Boots | *Magnetic Brass Galoshes* | Allows walking directly along active conveyor tracks without sliding. |
| **Held Tool** | Handheld | *Brass Winding Drill* | Used for manual mining, tile flipping, and puzzle calibration. |
| **Curio A** | Trinket | *Humming Chime Charm* | Dampens nearby acoustic interference in Sudoku groves. |
| **Curio B** | Trinket | *Feather Glider* | Allows gliding gracefully from high $Z_2$ cliffs down to lower islands! |

### 13.4 Faction Badges & Personal Traits

| Faction Badge | Visual Pin | Passive Engineer Trait |
|---|---|---|
| **Purist Acorn** | Carved oak acorn pin. | Hand-mining yield +25%; hand-crafting speed doubled. |
| **Mechanist Cog** | Polished brass mini-gear. | Machines within 4 tiles operate +15% faster. |
| **Resonant Bell** | Tiny silver bell that jingles. | Cascade combos trigger +20% extra power and sparks. |
| **Salvager Key** | Antique skeleton key. | Deconstructing machines refunds 100% of materials. |

---

## 14. Handcrafting & Material Recipes

### 14.1 Philosophy: Tactile Craftsmanship Precedes Automation

In GRIDLOCK, every magnificent factory begins with a single handcrafted brass screw. The Constructor carries a personal crafting satchel, allowing them to fashion basic tools and initial machine frames on the go before delegating production to automated Stamp Assemblers.

### 14.2 The Cozy Pocket Workshop (Crafting Panel)

Pressing `E` opens the **Field Workshop**:
- Displays a grid of available recipes illustrated like hand-drawn botanical sketches.
- Hovering shows required materials, fabrication time, and an animated 3D preview of the finished toy-like structure.
- Queued items appear as tiny progress circles around the Constructor's winding key, crafting one by one with pleasant rhythmic hammer clicks.

### 14.3 Building Material Costs & Multi-Tier Footprints

#### Tier 1 — Cozy Bootstrapping Foundations
| Structure | Footprint | Elevation | Recipe Cost |
|---|---|---|---|
| **Timber Survey Tower** | 2×2 | $Z_0$ | 8× Iron Plate, 4× Copper Plate, 2× Wooden Plank |
| **Clockwork Wellhead** | 1×1 | $Z_0$ | 6× Iron Plate, 4× Brass Cog, 1× Wooden Rod |
| **Wooden Roller Belt (10)**| 1×1 | $Z_0$ | 4× Iron Plate, 2× Brass Cog, 2× Wooden Plank |
| **Turntable Swivel** | 1×1 | $Z_0$ | 4× Iron Plate, 2× Brass Cog |
| **Parchment Storage Chest** | 1×1 | $Z_0$ | 8× Wooden Plank, 2× Brass Nails |
| **Acoustic Power Pole** | 1×1 | $Z_0$ | 3× Iron Plate, 2× Copper Wire |

#### Tier 2 — Multi-Layer Factory Essentials
| Structure | Footprint | Elevation | Recipe Cost |
|---|---|---|---|
| **Viaduct Incline Ramp** | 1×3 | $Z_0 \to Z_1$ | 10× Iron Plate, 6× Brass Cog, 4× Stone Brick |
| **Spiral Lifter** | 2×2 | $Z_0 \to Z_2$ | 16× Iron Plate, 10× Brass Cog, 4× Glass Tube |
| **3D Cargo Compressor** | 3×4 | $Z_0$ | 24× Iron Plate, 12× Copper Plate, 6× Brass Cog, 4× Glass Pane |
| **Chromatic Autoclave (6×6)**| 4×4 | $Z_0$ | 32× Iron Plate, 16× Bronze Plate, 8× Crystal Prism, 6× Glass Pane |
| **Marble-Run Cascade** | 4×8 | $Z_2 \to Z_0$ | 28× Wooden Plank, 16× Bronze Plate, 10× Brass Cog, 4× Sine Coil |
| **Acoustic Monolith** | 3×3 | $Z_0$ | 24× Stone Brick, 12× Refined Copper, 4× Resonite Crystal |

#### Tier 3 — Monumental Clockwork Wonders
| Structure | Footprint | Elevation | Recipe Cost |
|---|---|---|---|
| **Classification Pavilion**| 4×4 | $Z_0$ | 24× Carved Timber, 12× Bronze Plate, 8× Specimen Jar, 4× Glass Lens |
| **Miniature Gantry Crane** | 4×4 | $Z_2$ frame | 20× Steel Beam, 10× Alloy Ingot, 6× Brass Winch, 4× Rope Spool |
| **Sky-Barge Trade Wharf** | 4×8 | $Z_0$ coastal | 40× Boardwalk Timber, 18× Alloy Ingot, 8× Rope Spool, 4× Canvas Sheet |
| **Grand Resonance Engine** | 8×8 | Center Isle | 50× Star-Metal Beam, 30× Prismatic Core, 20× Celestial Lens, 16× Resonant Bell |

### 14.4 Intermediate Component Recipes

| Item | Inputs | Machine Origin | Craft Time |
|---|---|---|---|
| **Iron Plate** | 2× Iron Ore | Smelting Hearth | 1.5s |
| **Copper Plate** | 2× Copper Ore | Smelting Hearth | 1.5s |
| **Brass Cog** | 2× Iron Plate + 1× Copper Plate | Stamp Assembler | 2.0s |
| **Copper Wire** | 1× Copper Plate | Wire Drawer | 1.0s |
| **Acoustic Sine Coil** | 3× Copper Wire + 1× Resin | Stamp Assembler | 2.5s |
| **Bronze Plate** | 2× Copper Plate + 1× Iron Plate | Alloy Hearth | 3.0s |
| **Glass Pane** | 2× Clean River Sand | Smelting Hearth | 2.0s |
| **Polished Jade Tile** | 2× Raw Jade + 1× Spring Water | Lapidary Wheel | 3.5s |
| **Prismatic Core** | 5-Gem Cascade Clear | Chromatic Autoclave | Instant |
| **Bento Crate Frame** | 4× Wooden Plank + 2× Brass Nails | Stamp Assembler | 2.0s |

### 14.5 The 3D Stamp Assembler

Once unlocked, the **3D Stamp Assembler** automates any handcraft recipe:
- Visualized as an adorable open-sided wooden workbench featuring an articulated brass stamper arm that punches down with a cheerful puff of steam.
- Accepts inputs from $Z_0$ or $Z_1$ conveyor belts and outputs finished goods cleanly onto designated exit belts.

### 14.6 Luminous Ghost Holograms & Snapping Guides

Placing buildings displays a **Luminous Ghost Hologram** — a glowing, translucent pastel miniature model floating above the grid:
- Snaps crisply to the 2.5D isometric tile grid.
- Shows clear vertical snapping guides when placing elevated $Z_1$ and $Z_2$ structures.
- Right-click rotates the hologram in 90° increments with a delightful clockwork clicking sound.

---

## 15. World Traversal & Diorama Camera

### 15.1 Isometric Movement, Waddling Physics & Climbing

Movement is smooth and expressive:
- **Controls:** `WASD` moves the Constructor along the isometric diagonals.
- **Elevation Traversal:** The Constructor automatically walks up short stone stairs, ramps, and gentle hillside slopes.
- **Jumping & Gliding:** Tapping `Space` performs a tiny hop; holding `Space` while descending from high cliffs deploys the **Feather Glider**, allowing the player to float gently over factory belts.

### 15.2 The Tilt-Shift Axonometric Camera System

- **Default Tracking:** The camera tracks the Constructor with smooth, cushioned spring interpolation.
- **Stepped 90° Rotation:** Pressing `Q` or `E` rotates the camera view by $90^\circ$ around the focal center, revealing hidden pathways behind tall cliffs and structures.
- **Zoom Range:** Mouse wheel zooms seamlessly from an intimate close-up (admiring tiny turning gears) to a panoramic bird's-eye view of the entire floating island.
- **Tilt-Shift Post-Processing:** Configurable miniature lens blur softly washes the top and bottom third of the screen, accentuating the toybox tabletop feel.

### 15.3 Atmospheric Cloud Strata & Fog of Wonder

Unexplored regions are shrouded in **Atmospheric Cloud Strata**:
- Plump, fluffy pastel clouds hover softly over uncharted islands.
- As the Constructor walks forward or builds Survey Towers, clouds part with a whimsical swirl, revealing the mossy world below.

### 15.4 3D Stepped Terrain, Ramps, & Sky Bridges

Islands feature vertical variety:
- **Ground Floor ($Z_0$):** Flat grassy meadows, riverbeds, and coastal beaches.
- **Elevated Terraces ($Z_1$):** Chalk bluffs and ancient stone ruins reachable via stone ramps.
- **High Spires ($Z_2$):** Mountain peaks and ancient celestial observatory platforms.

### 15.5 Whimsical Fast Travel — Gliders, Trams, & Pneumatic Canisters

- **Wooden Sky-Trams:** Charming rope-pulled aerial gondolas linking distant floating islands.
- **Pneumatic Canister Tubes:** Late-game transit tubes that gently suck the Constructor in with a *shwoop* and pop them out across the archipelago!

---

## 16. Personal Inventory & Toybox Hotbar

### 16.1 Backpack Layout (The Canvas Satchel)

Pressing `Tab` or `I` opens the **Canvas Satchel** — a 10×5 grid of wooden-framed slots where items sit as adorable miniature 3D icons that tilt slightly when hovered.

### 16.2 The Tactile Wooden Hotbar

The bottom 10 slots of the satchel remain permanently pinned along the lower screen as the **Wooden Hotbar**:
- Mapped to number keys `1` through `0`.
- Holds tools, building blueprints, and quick-use items.
- Selecting a building highlights its blueprint ghost at the cursor with instant cost tooltips.

### 16.3 Trinket & Garb Panel

A cozy sidebar panel attached to the satchel displays the Constructor’s paper-doll silhouette, allowing quick equipping of hoods, boots, cloaks, and trinket charms.

### 16.4 Adorable Delivery Sparrows (Late-Game Personal Logistics)

Upon researching **Avian Logistics**, the player gains personal delivery helpers:
- Tiny, plump mechanical **Brass Sparrows** with clockwork wings flutter out from nearby storage depots.
- Sparrows carry requested building materials in tiny wooden baskets, hovering over the Constructor's head to refill their satchel!

### 16.5 Item Stack Sizes & Miniature Physics Voxels

| Category | Max Stack | Drop Behavior |
|---|---|---|
| Raw Ores & Shards | 100 | Drops as cute bouncing candy pebbles. |
| Plates, Wire, Cogs | 200 | Drops as tidy miniature tied bundles. |
| Buildings & Machines | 10 | Drops as charming wooden toy models. |
| Relics & Prismatic Cores| 5 | Emits soft glowing sparkles on the ground. |

---

## 17. Hand Mining & The Cozy Bootstrapping Phase

### 17.1 The First Hour: Solitary Awakening on Mossy Stones

The game begins with the Constructor waking up on a quiet, sunlit floating island beside a burbling freshwater brook. Scattered nearby are:
- A trusty **Brass Winding Drill** (starter tool).
- A **Surveying Tapper** for revealing safe ground voxels.
- A **Stone Smelting Hearth** nestled under an ancient weeping willow.

### 17.2 The Brass Winding-Key Drill

The drill is operated by approaching an exposed ore vein and holding `Left Click`:
- The Constructor winds up the drill and plants it against the crystal.
- A delightful tactile vibration occurs as tiny gem shards pop free and hop directly into the player's satchel.

| Drill Tier | Mining Rate | Reach | Visual Motif |
|---|---|---|---|
| **Brass Winding Drill** | 1.0 ore/sec | 2.0 tiles | Hand-cranked wooden drill with a spinning brass screw bit. |
| **Bronze Gear Drill** | 2.5 ore/sec | 2.5 tiles | Polished bronze chassis with dual counter-rotating heads. |
| **Prismatic Core Drill** | 6.0 ore/sec | 3.0 tiles | Floating crystal drill bit that hums with gentle musical notes. |

### 17.3 Bootstrapping Flowchart

```
[ Wake up in Cradle Meadows ]
             |
             v
[ Hand-mine Iron Pebble Node ] ---> [ Smelt into Iron Plates at Hearth ]
             |
             v
[ Craft Wooden Conveyor Rollers & Turntables ]
             |
             v
[ Build First Clockwork Wellhead over Iron Vein ]
             |
             v
[ Unfold 4x4 Meadow via Survey Tapper ] ---> [ Reveal Safe Veins ]
             |
             v
[ Route Polyomino Shards into First 3D Compressor ]
             |
             v
[ AUTOMATION AWAKENS! ]
```

### 17.4 Harvesting Crystalline Outcrops

Ore nodes are not flat circles; they are beautiful crystal clusters jutting from the moss. As they are harvested, individual crystal facets shrink with satisfying chip sounds until the node is spent.

### 17.5 Soft Earth, River Pebbles, and Natural Fluff

Non-puzzle resources are gathered easily:
- **Soft Clay:** Dug from riverbanks for brickmaking.
- **Timber Planks:** Gathered by gently tapping fallen logs (living trees are preserved for beauty!).
- **River Water:** Scooped into wooden buckets for cooling and lapidary polishing.

---

## 18. Day / Night Cycle & Whimsical Environmental Curiosities

### 18.1 The Diurnal Rhythm of The Patchwork

The world turns on a gentle **24-minute real-time day/night cycle**:

```
      [ DAWN: 3 min ]          [ DAY: 10 min ]         [ DUSK: 3 min ]        [ NIGHT: 8 min ]
     Soft pink skies,        Warm golden sunbeams,    Purple-amber sunset,   Twinkling stars,
     dewdrop sparkles.       full solar efficiency.   long cozy shadows.     fireflies, lantern glow.
```

### 18.2 Sunbeam Bonuses & Dewdrop Mornings

- **Morning Dew (Dawn):** Conveyor rollers run 10% faster as morning dew lubricates gears.
- **Solar Radiance (Day):** Solar crystal collectors generate +50 kW bonus power.
- **Twilight Glow (Dusk):** Chromite gems in autoclaves shimmer brighter, making match combos easier to spot.

### 18.3 Twilight Curiosities & Starfall Rhythms

Night in GRIDLOCK is never terrifying or punishing — it is magical and cozy. Logic Substrate glows with gentle bioluminescent pastel colors, turning the factory into a sparkling fairy-tale village.

### 18.4 Warm Lantern Lighting & Bioluminescent Flora

Placing **Brass Lantern Posts** illuminates the night with warm, buttery circles of light. The Constructor carries a tiny hip lantern that swings gently as they waddle, casting dancing shadows across the moss.

### 18.5 Ruin Sprites — Shy Ancient Guardians

Instead of violent monsters, night brings **Ruin Sprites**:
- Adorable, glowing floating wisps shaped like tiny flying teapots or luminous moths.
- They wander curiously around ancient ruins.
- If the Constructor approaches gently, sprites dance in circles, dropping rare **Ancient Dust** before floating away into the stars!

### 18.6 The Cozy Cottage Hub (Personal Camp & Anchor)

The Constructor can establish a **Cottage Hub** on any island:
- A storybook stone-and-timber cottage with smoking chimney, flower boxes, and a porch swing.
- Serves as the primary save point, wardrobe dressing room, and research library.
- Stepping inside triggers a cozy interior camera view, playing a soft acoustic lullaby while you review your blueprints by candlelight.

---

## Appendix A — Illustrated Glossary

| Term | Definition |
|---|---|
| **The Patchwork** | The floating cloud archipelago where the game takes place. |
| **The Constructor** | The adorable cloaked clockwork engineer played by the user. |
| **Logic Substrate** | The ancient, living mathematical crystal mineral that powers the world. |
| **Voxel Unfolding** | The physical tile-flip mechanism used to prospect Minesweeper terrain. |
| **Lattice Shard** | A 3D candy-like polyomino crystal travelling along conveyors (Tetris). |
| **3D Compressor** | A glass-walled hopper building where polyominoes are squished into ingots. |
| **Chromatic Autoclave**| A domed glass terrarium vat where gem matches refine metals (Bejeweled). |
| **Resonance Grove** | A 9×9 island sector calibrated with harmonic frequency halos (Sudoku). |
| **Step-Tiered Cascade**| A cascading marble-run assembly chute sorted by rank and polarity (Solitaire).|
| **Stepped Vault Pagoda**| An ancient multi-layer 3D monument dismantled pair by pair (Mahjong). |
| **Specimen Carousel** | A revolving 16-jar table used to classify mystery alloys (Connections). |
| **Clockwork Tumbler** | A 5-ring brass dial mechanism sealing ancient ruin archways (Wordle). |
| **Bento Cargo Crate** | A 3×4 compartmentalized box packed to hit target pip sums (Pips). |
| **Synergy Lily** | A glowing golden flower that blossoms where matching leylines cross (Crossword).|
| **$Z$-Planes** | The three discrete vertical conveyor tiers: $Z_0$ Ground, $Z_1$ Viaduct, $Z_2$ Skyway.|
| **Insight Sparks** | Golden firefly-like currency earned by running puzzle systems harmoniously. |
| **The Grand Resonance Engine**| The monumental endgame wonder that unites all floating sky islands. |

---

*Document Status: Approved 2.5D Concept Master v1.0 — Architecture for Bevy Isometric Implementation*  
*Artistic Alignment: Tunic (Visual Warmth, Diorama Framing & Illustrated Manual) × Shapez 2 (Multi-Layer Spatial Conveyor Logistics & Modular Mastery)*
