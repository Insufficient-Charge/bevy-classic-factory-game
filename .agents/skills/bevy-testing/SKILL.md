---
name: bevy-testing
description: >-
  Expert guide for writing unit, integration, and system tests for Bevy 0.19
  games in Rust. Covers App-based test harnesses, ECS system testing, event
  assertions, resource validation, and CI integration. Activate when writing
  tests for Bevy systems, components, resources, or plugins.
risk: safe
source: project
date_added: "2026-08-06"
---

# Bevy Testing Expert

## Overview

A practical guide for testing Bevy 0.19 game logic without launching a window.
Bevy's ECS is fully testable in isolation: `App` can be constructed, systems
run, and state inspected — all in a plain `cargo test` run.

This project uses Bevy 0.19 with `default-features = false` and the `2d`, `ui`,
and `audio` features. Tests **must not** enable `bevy/dynamic_linking` (the
`dev` feature) to avoid linker issues in test builds.

---

## When to Use This Skill

- Writing unit tests for individual Bevy systems.
- Testing plugin initialization and resource defaults.
- Asserting that events are sent/received correctly.
- Validating component state after running a system.
- Setting up integration tests that span multiple systems.
- Debugging flaky or order-dependent system behavior.

---

## Step-by-Step Guide

### 1. Minimal Test App Harness

Never use `App::new()` with `DefaultPlugins` in tests — it opens a window and
requires a GPU. Use `MinimalPlugins` instead, and add only the plugins your
system under test needs.

```rust
#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    fn my_system(mut commands: Commands) {
        commands.spawn(Transform::default());
    }

    #[test]
    fn spawns_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, my_system);

        app.update(); // runs one tick

        let world = app.world();
        let count = world.query::<&Transform>().iter(world).count();
        assert_eq!(count, 1);
    }
}
```

**Key rules:**
- `app.update()` runs exactly one tick (all `Update` systems once).
- `app.world()` gives an immutable borrow for querying.
- `app.world_mut()` gives a mutable borrow for direct ECS manipulation.

---

### 2. Querying Components After a System Runs

```rust
#[derive(Component)]
struct Health(f32);

fn damage_system(mut query: Query<&mut Health>) {
    for mut hp in &mut query {
        hp.0 -= 10.0;
    }
}

#[test]
fn damage_reduces_health() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Update, damage_system);

    // Seed the world before the first update
    app.world_mut().spawn(Health(100.0));
    app.update();

    let world = app.world();
    let hp = world.query::<&Health>().single(world);
    assert!((hp.0 - 90.0).abs() < f32::EPSILON);
}
```

---

### 3. Testing Resources

```rust
#[derive(Resource, Default)]
struct Score(u32);

fn score_system(mut score: ResMut<Score>) {
    score.0 += 1;
}

#[test]
fn score_increments() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .init_resource::<Score>()
       .add_systems(Update, score_system);

    app.update();
    app.update();

    assert_eq!(app.world().resource::<Score>().0, 2);
}
```

---

### 4. Testing Events

Use `Events::<E>::send` / `EventReader` in a helper system, or inspect the
event queue directly via `app.world().resource::<Events<E>>()`.

```rust
#[derive(Event)]
struct PlayerDied;

fn kill_system(mut writer: EventWriter<PlayerDied>) {
    writer.send(PlayerDied);
}

fn count_deaths(
    mut reader: EventReader<PlayerDied>,
    mut count: ResMut<DeathCount>,
) {
    count.0 += reader.read().count();
}

#[derive(Resource, Default)]
struct DeathCount(usize);

#[test]
fn death_event_is_counted() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_event::<PlayerDied>()
       .init_resource::<DeathCount>()
       .add_systems(Update, (kill_system, count_deaths).chain());

    app.update();

    assert_eq!(app.world().resource::<DeathCount>().0, 1);
}
```

> **Note:** Events are double-buffered. They are only visible to readers on the
> same tick or the next tick. If you need to inspect raw events, call
> `app.world().resource::<Events<E>>()` after `app.update()`.

---

### 5. Testing Plugins

Test a plugin by adding it to a minimal app and asserting side-effects
(resources initialized, systems registered via their observable results).

```rust
struct MyPlugin;
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_systems(Update, score_system);
    }
}

#[test]
fn plugin_initializes_score() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, MyPlugin));
    // Resource should exist after plugin is built (before first update)
    assert!(app.world().contains_resource::<Score>());
}
```

---

### 6. Testing State Transitions

Use `app.insert_state(MyState::Loading)` and assert state after update.

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState { #[default] Loading, Playing }

fn on_enter_playing(mut next: ResMut<NextState<AppState>>) {
    // example: auto-advance for test
    next.set(AppState::Playing);
}

#[test]
fn transitions_to_playing() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .init_state::<AppState>()
       .add_systems(OnEnter(AppState::Loading), on_enter_playing);

    app.update(); // triggers OnEnter(Loading) -> sets NextState
    app.update(); // applies transition

    let state = app.world().resource::<State<AppState>>();
    assert_eq!(*state.get(), AppState::Playing);
}
```

---

### 7. Multi-Tick Simulation

Run multiple ticks to simulate time-dependent behavior:

```rust
for _ in 0..10 {
    app.update();
}
```

To inject real `Time` advancement, use `app.world_mut().resource_mut::<Time>()` or
add `TimePlugin` and manually advance. For fixed-timestep systems, add
`bevy::time::TimePlugin` and use `Time::<Fixed>::from_seconds(1.0 / 60.0)` as a
resource.

---

### 8. Asserting Entity Structure

```rust
// Count entities with a specific component combination
let world = app.world();
let mut q = world.query_filtered::<Entity, (With<Enemy>, Without<Dead>)>();
let enemies: Vec<Entity> = q.iter(world).collect();
assert_eq!(enemies.len(), 3);
```

---

## Project-Specific Notes (bevy-classic-factory-game)

- **Bevy version:** 0.19 with `default-features = false`.
- **Active features in tests:** none — do NOT enable `dev` or `dev_native` in
  test configurations. Add `#[cfg(not(test))]` guards to `dev`-only system
  registrations if they would panic without a GPU.
- **CI profile:** Run tests with `cargo test --profile ci` for faster
  compilation (see `[profile.ci]` in Cargo.toml).
- **Existing modules to test:** `demo::movement`, `demo::player`,
  `demo::animation`, `screens`, `menus`, `audio`, `asset_tracking`.
- **Avoid in tests:** `DefaultPlugins`, `AssetPlugin` (requires filesystem
  access), `AudioPlugin` (requires audio device), `RenderPlugin`.

---

## Test Organization Patterns

### Inline unit tests (preferred for systems)

Place `#[cfg(test)] mod tests { ... }` at the bottom of the source file
containing the system. This keeps tests co-located with implementation.

### Integration tests (`tests/` folder)

Create `tests/integration_<name>.rs` for cross-plugin or cross-module tests.
These live outside `src/` and are compiled as separate crates by Cargo.

```
tests/
  integration_demo.rs    <- tests demo plugin end-to-end
  integration_menus.rs
```

### Shared test helpers

If multiple test modules share setup logic, create `src/testing.rs` gated
with `#[cfg(test)]`:

```rust
// src/testing.rs
#[cfg(test)]
pub fn make_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}
```

---

## Common Pitfalls & Fixes

| Problem | Fix |
|---------|-----|
| Test opens a window / panics on GPU | Replace `DefaultPlugins` with `MinimalPlugins` |
| System didn't run | Ensure `app.update()` was called after adding the system |
| Query returns 0 results | Spawn entities **before** `app.update()`, or via `app.world_mut().spawn(...)` |
| Event not received | Ensure `add_event::<E>()` was called; use `.chain()` to order writer before reader |
| State doesn't transition | Call `app.update()` twice: once to set `NextState`, once to apply it |
| Conflicting system access panic | Use `.chain()` or split across separate `app.update()` calls |
| `resource::<T>()` panics | Ensure `init_resource::<T>()` or `insert_resource(...)` was called first |
| Compile error with `dev` feature in tests | Do not add `features = ["dev"]` to test dependencies in Cargo.toml |

---

## Useful Bevy Testing APIs (0.19)

| API | Purpose |
|-----|---------|
| `App::new()` | Create a bare app (no plugins) |
| `app.add_plugins(MinimalPlugins)` | Add time + task pool, no window |
| `app.update()` | Run one tick |
| `app.world()` | Immutable world access |
| `app.world_mut()` | Mutable world access |
| `world.spawn(bundle)` | Spawn an entity directly |
| `world.query::<Q>().iter(world)` | Iterate query results |
| `world.query::<Q>().single(world)` | Assert exactly one match |
| `world.resource::<R>()` | Access a resource |
| `world.contains_resource::<R>()` | Check if a resource exists |
| `app.add_event::<E>()` | Register an event type |
| `app.init_state::<S>()` | Register a state machine |
| `app.insert_state(S::Variant)` | Set initial state value |

---

## Limitations

- Use this skill only when the task clearly matches the scope described above.
- Do not treat output as a substitute for environment-specific validation or
  expert review.
- Stop and ask for clarification if required inputs, permissions, safety
  boundaries, or success criteria are missing.
- This skill targets **Bevy 0.19**. Some APIs differ in earlier versions.
