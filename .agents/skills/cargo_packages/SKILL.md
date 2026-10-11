---
name: cargo_packages
description: >-
  Expert guide for managing, adding, updating, and auditing Cargo packages
  and dependencies in Rust/Bevy 0.19 projects. Covers cargo CLI workflows,
  feature flag configurations, WASM target compatibility, profile settings,
  CI verification, and practical code usage examples for all project packages.
  Activate when inspecting, adding, removing, configuring, or using Cargo dependencies.
risk: safe
source: project
date_added: "2026-08-06"
---

# Cargo Packages & Dependency Management

## Overview

A dedicated guide for managing and using Rust dependencies in **GRIDLOCK** (`bevy-classic-factory-game`).
This project targets Bevy 0.19 on PC (Windows, macOS, Linux) and WebAssembly (`wasm32-unknown-unknown`).

Managing Cargo dependencies cleanly ensures fast compile times, clean WASM builds, reproducible CI builds, and optimal runtime performance without feature bloat.

---

## When to Use This Skill

- Adding new crate dependencies or dev-dependencies using `cargo add`.
- Removing or updating existing dependencies.
- Configuring Bevy feature flags (`2d`, `ui`, `audio`, `dev`, `dev_native`).
- Writing Rust code that uses installed dependencies (`serde`, `ron`, `strum`, `bitflags`, `noise`, etc.).
- Ensuring target-specific compatibility (e.g. `target.wasm32-unknown-unknown.dependencies`).
- Verifying builds and profiles (`cargo check --profile ci`, `cargo test`, `cargo bench`).
- Auditing dependency compilation overhead and performance implications.

---

## Core Principles

1. **Minimal Feature Sets**:
   - Keep `default-features = false` on heavy crates like `bevy` to avoid compiling unused engine modules (3D, animation, PBR, etc.).
   - Explicitly list required features (e.g., `features = ["2d", "ui", "audio"]`).

2. **WASM Compatibility**:
   - WebAssembly target (`wasm32-unknown-unknown`) requires JS-compatible random generators (`getrandom = { version = "0.3", features = ["wasm_js"] }`) and non-blocking I/O.
   - Avoid native-only filesystem or OS thread dependencies without `#[cfg(not(target_arch = "wasm32"))]` guards.

3. **CI & Dev Build Optimization**:
   - Use `dev` and `dev_native` features only in development profiles (`default = ["dev_native"]`).
   - Run verification via `cargo check --profile ci` for fast type-checking during development.

---

## Key Dependency Inventory & Usage Matrix

| Package | Purpose in GRIDLOCK | Feature Flags / Target Config |
|---|---|---|
| `bevy` | Core 2D Game Engine (v0.19) | `default-features = false`, `features = ["2d", "ui", "audio"]` |
| `rand` | Puzzle seeding & hazard distribution | `v0.9` |
| `serde` | Entity/Resource/Save serialization | `v1.0`, `features = ["derive"]` |
| `ron` | Human-readable asset/save format | `v0.12` |
| `serde_json` | Data-driven puzzle recipe configs | `v1.0` |
| `strum` / `strum_macros` | Enum iteration & string parsing | `v0.28` |
| `bitflags` | Sector cell states & crossword ray masks | `v2.13` |
| `noise` | Procedural map & biome generation | `v0.9` |
| `tracing` | Logging & diagnostic control | `v0.1`, max debug / release warn |
| `pretty_assertions` (dev) | Enhanced TDD assertion diffs | `v1.4` |
| `criterion` (dev) | High-precision benchmarking | `v0.8` |
| `getrandom` (wasm32) | WebAssembly random byte source | `v0.3`, `features = ["wasm_js"]` |

---

## Package Usage Examples (Code Patterns)

### 1. `serde` + `ron` & `serde_json` (Save/Load & Recipe Data)

Use `serde` to serialize game world state, player progress, and data-driven recipe definitions:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SaveData {
    pub player_insight: f64,
    pub active_sector: u32,
    pub discovered_alloys: Vec<String>,
}

// Human-readable RON format (for save files & configs)
let save = SaveData {
    player_insight: 250.0,
    active_sector: 1,
    discovered_alloys: vec!["Bronze".into(), "Electrum".into()],
};
let ron_string = ron::ser::to_string_pretty(&save, ron::ser::PrettyConfig::default()).unwrap();
let loaded_save: SaveData = ron::from_str(&ron_string).unwrap();

// JSON format (for data-driven recipe shipments)
let json_string = serde_json::to_string(&save).unwrap();
let loaded_json: SaveData = serde_json::from_str(&json_string).unwrap();
```

---

### 2. `strum` & `strum_macros` (Enum Iteration & Strings)

Use `strum` macros to iterate over the 10 puzzle system types, map building keywords, or parse strings to enums:

```rust
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, EnumString, Display)]
pub enum PuzzleSystemType {
    Minesweeper,
    Tetris,
    Bejeweled,
    Sudoku,
    Solitaire,
    Mahjong,
    Connections,
    Wordle,
    Pips,
    Crossword,
}

// 1. Iterate over all 10 puzzle variants:
for system in PuzzleSystemType::iter() {
    println!("Initializing system: {system}"); // Uses Display trait
}

// 2. Parse string name to enum:
let parsed: PuzzleSystemType = "Bejeweled".parse().expect("Invalid puzzle system name");
assert_eq!(parsed, PuzzleSystemType::Bejeweled);
```

---

### 3. `bitflags` (Sector Grid & Cell State Masks)

Use `bitflags` for memory-efficient cell states (Minesweeper flags, Crossword ray directions):

```rust
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct CellStateFlags: u8 {
        const COVERED   = 0b0000_0001;
        const REVEALED  = 0b0000_0010;
        const FLAGGED   = 0b0000_0100;
        const HAZARD    = 0b0000_1000;
        const EXTRACTED = 0b0001_0000;
    }
}

let mut cell = CellStateFlags::COVERED | CellStateFlags::HAZARD;

// Reveal cell and uncover
cell.remove(CellStateFlags::COVERED);
cell.insert(CellStateFlags::REVEALED);

assert!(cell.contains(CellStateFlags::REVEALED));
assert!(!cell.contains(CellStateFlags::FLAGGED));
```

---

### 4. `noise` (Procedural Map & Biome Generation)

Use `noise` to generate procedural terrain elevation and seed biomes across 64x64 sector chunks:

```rust
use noise::{NoiseFn, Perlin};

let perlin = Perlin::new(1337); // Seeded per world
let grid_x = 12.5;
let grid_y = 45.2;

// Generates a continuous value in [-1.0, 1.0]
let elevation = perlin.get([grid_x, grid_y]);

if elevation > 0.4 {
    // Generate Vault Highlands biome
} else if elevation < -0.2 {
    // Generate Chromite Basin biome
}
```

---

### 5. `rand` (Seeding & Hazard Distribution)

Use `rand` for procedural hazard density, polyomino shard generation, and random drops:

```rust
use rand::prelude::*;

let mut rng = rand::rng();

// Random range selection
let hazard_roll: f32 = rng.random_range(0.0..1.0);
let shard_type_index: usize = rng.random_range(0..7);
```

---

### 6. `tracing` (Structured Diagnostics & Logging)

Use `tracing` macros instead of `println!` for logging in Bevy systems:

```rust
use tracing::{debug, error, info, warn};

info!(sector_id = 3, "Sector Resonance Bonus activated (+50% output)");
warn!(building_id = 42, "Resonance interference detected on Frequency Band 5");
error!("Compressor grid jammed!");
```

---

### 7. `pretty_assertions` (TDD Unit Testing)

Use `pretty_assertions::assert_eq!` in `#[cfg(test)]` modules for clear, colorized diff output when tests fail:

```rust
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn test_grid_clear() {
        let expected = vec![1, 2, 3, 4];
        let actual = vec![1, 2, 3, 4];
        assert_eq!(expected, actual); // Clear diff display on failure
    }
}
```

---

### 8. `criterion` (Benchmarking Heavy Systems)

Use `criterion` to benchmark sparse ray projections or match-3 vat cascade logic in `benches/`:

```rust
// benches/puzzle_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_bejeweled_cascade_scan(c: &mut Criterion) {
    c.bench_function("bejeweled_6x6_cascade_scan", |b| {
        b.iter(|| {
            // Benchmark vat match scanning algorithm
        });
    });
}

criterion_group!(benches, bench_bejeweled_cascade_scan);
criterion_main!(benches);
```

---

## Standard Cargo CLI Workflows

### 1. Adding Dependencies
```powershell
# Add standard dependency
cargo add serde --features derive

# Add multiple utility crates
cargo add ron serde_json strum strum_macros bitflags noise

# Add dev-only testing/benchmarking dependencies
cargo add --dev pretty_assertions criterion

# Add target-specific dependency for WebAssembly
cargo add getrandom --target wasm32-unknown-unknown --features wasm_js
```

### 2. Removing / Updating Dependencies
```powershell
# Remove a crate
cargo rm crate_name

# Update dependency lockfile
cargo update
```

### 3. Verification Commands
```powershell
# Fast check using the fast CI profile
cargo check --profile ci

# Run test suite
cargo test --profile ci

# Check WASM target compatibility
cargo check --target wasm32-unknown-unknown
```

---

## Cargo Profile Configurations (`Cargo.toml`)

- **Development (`[profile.dev]`)**: `opt-level = 1` for fast iteration; dependencies compiled at `opt-level = 3` for smooth framerates.
- **CI Build (`[profile.ci]`)**: Inherits `dev` with `opt-level = 0` and `codegen-units = 4` for rapid compilation in continuous integration.
- **Release (`[profile.release]`)**: `codegen-units = 1`, `lto = "thin"` for max performance.
- **Web Release (`[profile.web-release]`)**: `opt-level = "s"`, `strip = "debuginfo"` for minimal WASM payload size.

---

## Troubleshooting Common Cargo Issues

| Issue | Root Cause | Solution |
|---|---|---|
| `getrandom` compilation error on WASM | Missing `wasm_js` feature flag | Ensure `[target.wasm32-unknown-unknown.dependencies]` includes `getrandom = { version = "0.3", features = ["wasm_js"] }` |
| Slow dev build compile times | Dynamic linking disabled or unused Bevy sub-crates compiled | Ensure `dev` feature includes `bevy/dynamic_linking` and `bevy` uses `default-features = false` |
| Test suite fails linking on Windows | `bevy/dynamic_linking` active during `cargo test` | Do not pass `features = ["dev"]` when running tests (`cargo test --profile ci`) |
| Unused dependency lint / warning | Unreferenced crate in `Cargo.toml` | Run `cargo remove <crate>` or wrap imports in appropriate `#[cfg]` flags |
