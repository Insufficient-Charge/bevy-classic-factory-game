---
name: agent-harness
description: >-
  Expert guide for launching, controlling, inspecting, and mutating the Bevy game via
  the Agent Automation Harness, Bevy Remote Protocol (BRP), and bevy_brp_mcp. Covers
  dual-channel architecture (TCP port 6550 input/tick control + HTTP port 15702 BRP JSON-RPC),
  MCP server integration, command-line inspection via brp_cli.ps1, entity/component/resource
  querying & live mutation, and structured telemetry ingestion. Activate when inspecting ECS
  state, executing automated playtests, mutating live game parameters, or developing Bevy systems.
risk: safe
source: project
date_added: "2026-10-08"
---

# Agent Harness, Bevy Remote Protocol (BRP) & MCP Expert

## Overview & Architecture

The **Agent Automation Harness** provides a comprehensive, high-bandwidth inspection and control surface designed for autonomous AI agents and pair-programming developers.

It operates through **three complementary channels**:

```
                       +---------------------------------------+
                       |              AI AGENT                 |
                       | (Antigravity / Claude Code / Scripts) |
                       +-------------------+-------------------+
                                           |
             +-----------------------------+-----------------------------+
             |                             |                             |
      Channel 1 (TCP:6550)          Channel 2 (HTTP:15702)        Channel 3 (Disk)
      Low-Latency Control           Bevy Remote Protocol (BRP)    Telemetry & Panics
             |                             |                             |
             v                             v                             v
+-------------------------+   +-------------------------+   +-------------------------+
| Fast Input & Stepping   |   | Full ECS Introspection  |   | Non-blocking JSONL      |
| • press / release keys  |   | • bevy/query entities   |   | • tracing events        |
| • tick <N> frame steps  |   | • bevy/get & list comps |   | • panic payloads        |
| • screenshot capture    |   | • bevy/mutate live state|   | • target/agent_         |
| • quit / exit           |   | • bevy/get_resource     |   |   telemetry.jsonl       |
+-------------------------+   +-------------------------+   +-------------------------+
             |                             |                             |
             +----------------------+------+-----------------------------+
                                    |
                                    v
                     +-----------------------------+
                     |      BEVY 0.19 ENGINE       |
                     |  (--features agent_test)    |
                     +-----------------------------+
```

1. **Interactive Control Socket (`127.0.0.1:6550`):** Fast, line-delimited ASCII protocol for injecting keystrokes, rotating cameras, pausing/unpausing, and advancing simulation frame-by-frame.
2. **Bevy Remote Protocol & MCP (`http://127.0.0.1:15702`):** Official Bevy JSON-RPC 2.0 interface enabling full ECS reflection, entity queries, component inspection/mutation, resource updates, and type guides via [`bevy_brp_mcp`](https://crates.io/crates/bevy_brp_mcp).
3. **Structured Telemetry Stream (`target/agent_telemetry.jsonl`):** Non-blocking JSON log stream capturing engine diagnostics, subsystem logs, and panic backtraces.

---

## Launching the Game with Full Harness & BRP

Both Channel 1 (port 6550) and Channel 2 (port 15702) are initialized together whenever the game runs with `--features agent_test`.

### 1. Interactive Visual Mode
Runs the game with the window visible while exposing both ports:
```powershell
cargo run --features agent_test
```

### 2. Headless Mode (Recommended for AI Background Runs)
Runs the engine with window visibility disabled (`visible: false`), ideal for test automation:
```powershell
# Using CLI flag:
cargo run --features agent_test -- --agent-headless

# Or using environment variable:
$env:AGENT_HEADLESS = "1"
cargo run --features agent_test
```

### 3. Background Daemon in Antigravity
When launching as an agent background task:
* Command: `cargo run --features agent_test -- --agent-headless`
* `IsDaemon: true`
* `WaitMsBeforeAsync: 3000` (allows Bevy to compile/initialize and bind ports `6550` and `15702`).

---

## Channel 1: Control Socket Protocol (`127.0.0.1:6550`)

Used for fast inputs and deterministic frame stepping.

### Command Reference

| Command | Arguments | Example | Response | Description |
|---|---|---|---|---|
| `query` | `state` | `query state` | `<JSON_STRING>` | High-level summary of player position, camera yaw, screen state. |
| `tick` | `<count>` | `tick 10` | `ACK_TICK 10` | Advances simulation by $N$ frames, then freezes time. |
| `tick` | `0` or `pause` | `tick pause` | `ACK_TICK 0 (paused)` | Pauses virtual simulation immediately. |
| `tick` | `resume` | `tick resume` | `ACK_TICK RESUME` | Unpauses virtual simulation for continuous execution. |
| `press` | `<Key>` | `press W` | `ACK_KEY_PRESSED` | Simulates pressing a key down. |
| `release` | `<Key>` | `release W` | `ACK_KEY_RELEASED` | Simulates releasing a key. |
| `screenshot` | `<path>` | `screenshot target/screens/test.png` | `ACK_SCREENSHOT <path>` | Captures primary window and saves to disk. |
| `quit` | *(none)* | `quit` | `ACK_QUIT` | Gracefully terminates Bevy with `AppExit::Success`. |

### Helper Script: `send_command.ps1`
```powershell
# Step 60 frames and pause
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "tick 60"

# Inject movement
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "press W"
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "tick 30"
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "release W"

# Rotate diorama camera 90° clockwise
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "press E"
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "tick 1"
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "release E"
```

---

## Channel 2: Bevy Remote Protocol (BRP) CLI (`http://127.0.0.1:15702`)

The Bevy Remote Protocol allows an LLM to query, inspect, and mutate **any** entity, component, or resource registered in Bevy's reflection registry.

Use the built-in PowerShell CLI tool [`scripts/brp_cli.ps1`](./scripts/brp_cli.ps1) to interact with BRP from the terminal without compiling external binaries.

### 1. Schema & Method Discovery
Discover all available remote procedures supported by the running engine:
```powershell
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 discover
```

### 2. Querying Entities (`bevy/query`)
Search for entities having specific components and extract their values:
```powershell
# Query all entities with Transform and GridCoord
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query `
    -Components "bevy_transform::components::transform::Transform,bevy_classic_factory_game_2::game::grid::GridCoord"

# Query entities with TerrainTile, filtering out any without specific markers
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query `
    -Components "bevy_classic_factory_game_2::game::terrain::TerrainTile" `
    -FilterWith "bevy_classic_factory_game_2::game::terrain::TerrainTile"
```

### 3. Inspecting Entities (`bevy/get` & `bevy/list`)
Inspect components on a specific entity:
```powershell
# List all component type paths on an entity
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 list -Entity 4294967301

# Get specific component data on an entity
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 get `
    -Entity 4294967301 `
    -Components "bevy_transform::components::transform::Transform"
```

### 4. Mutating Components Live (`bevy/mutate`)
Modify live game state in real time without recompiling or restarting:
```powershell
# Change camera rotation step live
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 mutate `
    -Entity 4294967302 `
    -Component "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig" `
    -ValueJson '{"rotation_step": 2, "vertical_size": 22.0}'

# Teleport an entity to a new position
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 mutate `
    -Entity 4294967301 `
    -Component "bevy_transform::components::transform::Transform" `
    -ValueJson '{"translation": [10.0, 0.0, 15.0]}'
```

### 5. Inspecting & Mutating Resources (`bevy/get_resource` & `bevy/mutate_resource`)
Interact with global Bevy resources:
```powershell
# List all registered resources in the World
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 list-resources

# Read the global DioramaCameraConfig resource
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 get-resource `
    -Resource "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig"

# Read the IslandGrid resource
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 get-resource `
    -Resource "bevy_classic_factory_game_2::game::terrain::IslandGrid"
```

### 6. Spawning & Despawning Entities (`bevy/spawn` & `bevy/destroy`)
```powershell
# Spawn a new entity with components
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 spawn `
    -ValueJson '{"components": {"bevy_transform::components::transform::Transform": {"translation": [5.0, 0.0, 5.0]}}}'

# Despawn an entity by ID
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 destroy -Entity 4294967305
```

### 7. Raw JSON-RPC 2.0 Queries
Send any custom JSON-RPC request body directly:
```powershell
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 raw `
    -RawJson '{"jsonrpc": "2.0", "method": "bevy/list_resources", "id": 1}'
```

---

## Channel 2B: `bevy_brp_mcp` Integration

[`bevy_brp_mcp`](https://crates.io/crates/bevy_brp_mcp) is a Model Context Protocol (MCP) server that provides AI coding assistants with high-level agent tools bridging to Bevy Remote Protocol.

### 1. Installing `bevy_brp_mcp`
```powershell
cargo install bevy_brp_mcp
```

### 2. MCP Configuration
Both workspace and global configurations are pre-registered in:
* **Workspace:** [`.agents/mcp_config.json`](../../.agents/mcp_config.json)
* **Global:** `~/.gemini/config/mcp_config.json`

```json
{
  "mcpServers": {
    "bevy-brp": {
      "command": "bevy_brp_mcp",
      "args": [],
      "env": {}
    }
  }
}
```

### 3. Tools Exposed by `bevy_brp_mcp`
When connected via MCP, the AI assistant has access to these native tools:

| MCP Tool | Purpose | Key Parameters |
|---|---|---|
| `world_query` | Query entities matching component filters | `data.components`, `filter.with`, `filter.without` |
| `world_get_components` | Get component values on an entity | `entity`, `components` |
| `world_list_components` | List all component types on an entity | `entity` |
| `world_mutate_component` | Mutate specific component fields | `entity`, `component`, `value` |
| `world_insert_component` | Attach a new component to an entity | `entity`, `component`, `value` |
| `world_remove_component` | Detach a component from an entity | `entity`, `component` |
| `world_spawn_entity` | Spawn an entity with given components | `components` |
| `world_despawn_entity` | Despawn an entity | `entity` |
| `world_find_entities_by_name` | Look up entities by Name component | `name`, `match_type` (`Exact`, `Prefix`, `Contains`) |
| `world_get_resource` | Fetch global resource state | `resource` |
| `world_mutate_resource` | Mutate global resource fields | `resource`, `value` |
| `world_list_resources` | List all available resources | *(none)* |
| `brp_type_guide` | Retrieve JSON schema examples for any type | `type_name` |
| `app_run` | Launch a Bevy application target | `target`, `args` |
| `app_status` | Check if app process is running | *(none)* |

---

## Registered Reflection Types in GRIDLOCK

The following types are registered with Bevy's reflection registry and are immediately queryable and mutable via BRP and MCP:

| Type Path | Kind | Key Fields |
|---|---|---|
| `bevy_classic_factory_game_2::game::grid::GridCoord` | Component | `x: i32`, `y: i32`, `layer: Layer` |
| `bevy_classic_factory_game_2::game::grid::Layer` | Enum / Comp | `Subterranean`, `Ground`, `Viaduct`, `Skyway` |
| `bevy_classic_factory_game_2::game::grid::Direction` | Enum | `North`, `East`, `South`, `West` |
| `bevy_classic_factory_game_2::game::terrain::TerrainTile` | Component | `coord`, `voxel_type`, `prospect_state`, `has_overcharge_hazard`, `adjacent_hazards` |
| `bevy_classic_factory_game_2::game::terrain::TileProspectState` | Enum | `Covered`, `Revealed`, `Flagged`, `Extracted` |
| `bevy_classic_factory_game_2::game::terrain::VoxelType` | Enum | `Grass`, `StoneCliff`, `Clay`, `RiverWater`, `RunicSlate`, `CrystalOutcrop` |
| `bevy_classic_factory_game_2::game::terrain::IslandGrid` | Resource | `tiles: HashMap<GridCoord, TerrainTile>` |
| `bevy_classic_factory_game_2::camera::diorama::DioramaCamera` | Component | Marker |
| `bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig` | Resource / Comp | `pitch_radians`, `rotation_step`, `distance`, `vertical_size`, `smooth_factor` |
| `bevy_classic_factory_game_2::camera::diorama::DioramaCameraTarget` | Component | Marker |
| `bevy_transform::components::transform::Transform` | Component | `translation: [x,y,z]`, `rotation: [x,y,z,w]`, `scale: [x,y,z]` |

---

## "Total Signal" Autonomous Development Workflow

When an LLM agent develops a new feature (e.g. polyomino conveyors, new building types, or Minesweeper tile flipping), follow this verified workflow:

### Step 1: Launch Background Headless Instance
```powershell
cargo run --features agent_test -- --agent-headless
```
*(Wait 3 seconds for ports `6550` and `15702` to bind).*

### Step 2: Freeze Time for Deterministic Inspection
```powershell
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "tick pause"
```

### Step 3: Inspect World State via BRP
Query the current entities and components to verify system setup:
```powershell
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query `
    -Components "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig"
```

### Step 4: Live-Tune or Mutate Components
Verify how the game responds to state mutations without recompiling:
```powershell
# Change camera zoom and rotation step live
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 mutate `
    -Entity <CAMERA_ENTITY_ID> `
    -Component "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig" `
    -ValueJson '{"rotation_step": 1, "vertical_size": 12.0}'
```

### Step 5: Advance Ticks and Observe Reaction
```powershell
# Step 10 frames to allow systems to process the change
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "tick 10"

# Re-query transform to assert that position/rotation updated correctly
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 get `
    -Entity <CAMERA_ENTITY_ID> `
    -Components "bevy_transform::components::transform::Transform"
```

### Step 6: Visual Artifact Capture
Capture a screenshot to disk and verify the rendered result:
```powershell
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "screenshot target/verification.png"
```

### Step 7: Teardown & Clean Log Check
```powershell
# Terminate the game cleanly
powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 "quit"

# Verify that no errors or panics occurred
powershell -File .agents/skills/agent-harness/scripts/inspect_telemetry.ps1 -ErrorsOnly
```
