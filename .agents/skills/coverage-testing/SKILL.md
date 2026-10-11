---
name: coverage-testing
description: >-
  Expert guide for generating, running, and reporting code coverage for Rust/Bevy 0.19
  projects using cargo-llvm-cov, cargo-tarpaulin, or native LLVM coverage instrumentation,
  ensuring 100% test coverage across all modules and puzzle systems.
risk: safe
source: project
date_added: "2026-08-06"
---

# 🎯 Code Coverage Testing Expert Guide

## Overview

A practical guide for collecting, analyzing, and demonstrating **100% test coverage** for Rust game logic and ECS systems in **GRIDLOCK** (`bevy-classic-factory-game`).

Code coverage measures the percentage of source code lines, functions, and execution branches executed during unit and integration test runs (`cargo test`). High-coverage testing ensures that factory algorithms, grid conversions, match engines, and puzzle solvers behave deterministically without untested edge cases.

---

## When to Use This Skill

- Measuring line, function, and branch coverage for newly added systems or puzzle modules.
- Identifying untested conditional branches, error paths, or unexecuted enum arms.
- Generating HTML coverage reports, LCOV files, or terminal summary metrics for review.
- Setting up continuous integration (CI) coverage assertions enforcing 100% test coverage thresholds.
- Configuring coverage exclusions for native window entry points, GPU render loops, or dev tools.

---

## Primary Coverage Tools in Rust

| Tool | Platform | Method | Recommendation |
|---|---|---|---|
| `cargo-llvm-cov` | Windows, macOS, Linux | LLVM Source-Based Coverage (`-C instrument-coverage`) | **Recommended for Bevy & Windows MSVC** |
| `cargo-tarpaulin` | Linux-only | Ptrace / Binary Instrumentation | Linux CI containers |
| `grcov` | Windows, macOS, Linux | LLVM GCOV Profiling | Legacy environments |

---

## Step-by-Step Guide to 100% Coverage Testing

### 1. Installation

Install `cargo-llvm-cov` via Cargo:

```powershell
cargo install cargo-llvm-cov
```

If LLVM tools are missing, install the `llvm-tools` rustup component:

```powershell
rustup component add llvm-tools
```

---

### 2. Running Coverage Commands for GRIDLOCK

Because Bevy native dev builds include `bevy/dynamic_linking` (which can cause linker issues during test instrumentation on Windows MSVC), **always run coverage tests with `--no-default-features`**.

#### Terminal Summary Report
```powershell
cargo llvm-cov --no-default-features --profile ci
```

#### Detailed Line-by-Line HTML Report
Generates interactive HTML coverage pages in `target/llvm-cov/html/index.html`:

```powershell
cargo llvm-cov --no-default-features --profile ci --html
```

#### LCOV File Output (for Codecov / CI upload)
Generates `lcov.info` for CI coverage upload:

```powershell
cargo llvm-cov --no-default-features --profile ci --lcov --output-path lcov.info
```

---

### 3. Enforcing 100% Test Coverage Thresholds

To verify and demonstrate 100% code coverage in local development or CI pipelines, use the `--fail-under-lines` flag:

```powershell
cargo llvm-cov --no-default-features --profile ci --fail-under-lines 100
```

If any non-excluded function or line is uncovered, `cargo llvm-cov` exits with a non-zero exit code (1), failing the build until test coverage reaches 100%.

---

## Coverage Exclusion Strategies

Some code regions (such as native OS window initialization, GPU event polling, or `main()` app runners) cannot execute in headless test environments. Use the following techniques to maintain clean 100% test coverage on actual game logic:

### 1. Gate Native Entry Points with `#[cfg(not(test))]`
Exclude GPU app launchers or native main loops from test builds:

```rust
#[cfg(not(test))]
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
```

### 2. Ignore Files via CLI Regex
Ignore non-logic boilerplate (e.g. dev tools, theme visual helpers, or asset loaders):

```powershell
cargo llvm-cov --no-default-features --profile ci --ignore-filename-regex "dev_tools|theme|menus|screens"
```

### 3. Ignore Unreachable Branches with Attributes
If a `panic!` or `unreachable!` arm is defensive:

```rust
#[cfg_attr(test, allow(dead_code))]
fn unreachable_branch() {
    unreachable!("Defensive check");
}
```

---

## Native LLVM Coverage Without Additional CLI Tools

If `cargo-llvm-cov` is not installed, raw coverage data can be produced using native `rustc` flags:

```powershell
# 1. Enable LLVM coverage instrumentation
$env:RUSTFLAGS = "-C instrument-coverage"

# 2. Run unit test suite
cargo test --no-default-features --profile ci

# 3. Reset RUSTFLAGS after run
$env:RUSTFLAGS = ""
```

---

## Target Modules Coverage Matrix

Ensure unit test suites cover all core systems and puzzle modules:

| Module | Core Responsibility | Required Test Assertions |
|---|---|---|
| `src/testing.rs` | Headless App Harness | App instantiation & entity count |
| `src/core/state.rs` | Game & Overlay State Machine | Initial state values (`MainMenu`, `None`) |
| `src/core/camera.rs` | 2D Camera & Cursor Translation | Cursor resource default initialization |
| `src/factory/grid.rs` | Chunk & Coordinate Math | Bidirectional World ↔ Grid conversion, tile placement |
| `src/factory/building.rs` | Building ECS & Footprints | Footprint tile math, orientation rotation, placement collision |
| `src/factory/conveyor.rs` | Item Shard Routing | `LatticeShard` component properties & shapes |
| `src/factory/power.rs` | Power Grid Economy | Net power calculation & deficit triggers |
| `src/puzzles/minesweeper` | Terrain Prospecting | 8-neighbor hazard count, flag protection, reveal states |
| `src/puzzles/tetris` | Routing & Compression | 90°/180°/270° rotation matrices, line clear drop & ingot yields |
| `src/puzzles/bejeweled` | Chromatic Refinery | Match-3/4/5 scanning, cascade gravity drops, power surge MW math |
| `src/puzzles/sudoku` | Grid Calibration | Row/Col/3x3 box collision checks, efficiency penalties, sector bonus |
| `src/puzzles/solitaire` | Assembly Line Cascades | Rank 1–13 & polarity (+/-) attachment rules, 13-to-1 completion sweep |
| `src/puzzles/mahjong` | Vault Deconstruction | 3D layer coverage, unblocked side accessibility, pair extraction |
| `src/puzzles/connections` | Material Classification | 4x4 group submission, decoy rejection, recipe discovery |
| `src/puzzles/wordle` | Parameter Cipher Locks | Green/Yellow/Gray slot feedback, guess limit lock |
| `src/puzzles/pips` | Cargo Pier Valuation | 3x4 container grid row/col/diag sum arithmetic |
| `src/puzzles/crossword` | Keyword Synergy Grid | Axis keyword beam projections, harmonic nodes, conflict null nodes |
| `src/meta/research.rs` | Insight Point Economy | Research point generation, spending, affordability checks |

---

## Best Practices for 100% Coverage

- ✅ **Do**: Use `pretty_assertions::assert_eq!` in `#[cfg(test)] mod tests` for clear diffs when coverage assertions fail.
- ✅ **Do**: Test both happy paths and edge cases (e.g. empty grids, full grids, out-of-bounds inputs, zero power).
- ✅ **Do**: Run `cargo llvm-cov --no-default-features --profile ci --fail-under-lines 100` before committing PRs or code updates.
- ❌ **Don't**: Rely on manual window execution for testing game logic; keep logic testable in windowless `MinimalPlugins` headless harnesses.
- ❌ **Don't**: Enable `bevy/dynamic_linking` (`dev_native`) in test coverage commands on Windows MSVC.

---

## Troubleshooting Common Coverage Issues

| Issue | Root Cause | Solution |
|---|---|---|
| `LNK1189: library limit of 65535 objects exceeded` on Windows | `bevy/dynamic_linking` feature enabled during test build | Pass `--no-default-features` to `cargo llvm-cov` / `cargo test` |
| `StateTransition schedule is missing` panic | `StatesPlugin` omitted in headless app harness | Add `StatesPlugin` or `app.add_plugins(StatesPlugin)` before `init_state` |
| Low coverage on `main.rs` | Window initialization & GPU setup not run in headless unit tests | Annotate window launcher functions with `#[cfg(not(test))]` |
| `llvm-cov` command not found | Crate binary missing from cargo path | Run `cargo install cargo-llvm-cov` and `rustup component add llvm-tools` |
