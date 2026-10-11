# Agent Gameplay Testing & Headless Harness Runbook

This guide documents the headless test automation framework for autonomous execution, gameplay driving, vision-based rendering inspection, and structured telemetry analysis.

---

## 1. Booting the Game in Headless Mode

To compile and launch the game with the agent automation socket and telemetry active without opening a visible OS window:

```bash
cargo run --features agent_test -- --agent-headless
```

Alternatively, you can set the environment variable:
```bash
AGENT_HEADLESS=1 cargo run --features agent_test
```
(In PowerShell: `$env:AGENT_HEADLESS="1"; cargo run --features agent_test`)

The harness binds an ASCII TCP control socket to `127.0.0.1:6550` and streams structured JSONL logs to `target/agent_telemetry.jsonl`.

---

## 2. TCP Socket Command Protocol

The test server on `127.0.0.1:6550` accepts single-line ASCII commands terminated with `\n` and sends back single-line responses.

### Supported Commands

| Command | Description | Example Response |
|---------|-------------|------------------|
| `query state` | Dumps current gameplay metrics (screen state, player translation & Euler angles, camera transform, simulation status) as a JSON object | `{"screen":"Gameplay","simulation":{"elapsed_seconds":1.2,"paused":false},"player":{"translation":[...]},...}` |
| `screenshot <path>` | Requests Bevy's primary window renderer to save the current frame buffer to `<path>` | `ACK_SCREENSHOT target/test.png` |
| `press <Key>` | Injects a key press into Bevy's `ButtonInput<KeyCode>` (e.g., `W`, `A`, `S`, `D`, `Space`, `E`, `Escape`) | `ACK_KEY_PRESSED` |
| `release <Key>` | Clears the key from `ButtonInput<KeyCode>` | `ACK_KEY_RELEASED` |
| `tick <count>` | Advances simulation for `<count>` frames then pauses virtual time. Set `<count>` to `0` or `pause` to pause, or `resume` to resume continuous time. | `ACK_TICK 10` |
| `quit` / `exit` | Signals Bevy's `AppExit` to cleanly shut down the engine | `ACK_QUIT` |

---

## 3. Interacting via Terminal Tools

### PowerShell (Windows Native)

Send a command and read the response:

```powershell
$client = New-Object System.Net.Sockets.TcpClient('127.0.0.1', 6550)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

# Send command
$writer.WriteLine("query state")
$writer.Flush()

# Read response
$response = $reader.ReadLine()
Write-Output $response

$client.Close()
```

One-liner helper function in PowerShell:
```powershell
function Send-AgentCommand([string]$cmd) {
    $client = New-Object System.Net.Sockets.TcpClient('127.0.0.1', 6550)
    $stream = $client.GetStream()
    $writer = New-Object System.IO.StreamWriter($stream)
    $reader = New-Object System.IO.StreamReader($stream)
    $writer.WriteLine($cmd)
    $writer.Flush()
    $res = $reader.ReadLine()
    $client.Close()
    return $res
}

Send-AgentCommand "query state"
Send-AgentCommand "press W"
Send-AgentCommand "tick 60"
Send-AgentCommand "release W"
Send-AgentCommand "screenshot target/snapshot.png"
```

### Bash / Linux / Mac (`nc` / `netcat` or `/dev/tcp`)

```bash
# Using netcat
echo "query state" | nc -N 127.0.0.1 6550

# Using bash /dev/tcp
exec 3<>/dev/tcp/127.0.0.1/6550
echo "query state" >&3
head -n 1 <&3
exec 3<&-
```

---

## 4. Screenshot Verification Protocol

1. **Trigger Screenshot**:
   Send the screenshot command specifying the target PNG path:
   ```powershell
   Send-AgentCommand "screenshot target/agent_vision.png"
   ```
   Wait for response: `ACK_SCREENSHOT target/agent_vision.png`.

2. **Wait for Render & I/O Pipeline**:
   The screenshot is captured on the next render phase and asynchronously written to disk. Wait ~200-500ms or query state until the file exists on disk:
   ```powershell
   Test-Path target/agent_vision.png
   ```

3. **Multimodal Vision Verification**:
   Inspect the resulting image using your multimodal tool (`view_file` or visual analysis) to confirm:
   - Level geometry, lighting, and materials rendered as expected.
   - UI elements and HUD (pause/mute, health, dialogs) are present in expected positions.
   - No black screens, missing shader artifacts, or pink missing texture indicators.

---

## 5. Structured Log & Post-Mortem Evaluation

All logs are written as JSON lines to:
`target/agent_telemetry.jsonl`

### Checking for Errors, Warnings, and Panics

```powershell
# Read last 20 log entries
Get-Content target/agent_telemetry.jsonl -Tail 20 | ConvertFrom-Json

# Check for any ERROR level logs or panics
Get-Content target/agent_telemetry.jsonl | ConvertFrom-Json | Where-Object { $_.level -eq "ERROR" }
```

### Crash / Panic Protocol
The agent harness registers a custom panic hook. When a panic occurs, a structured entry is immediately flushed to `target/agent_telemetry.jsonl`:
```json
{
  "timestamp": 1740000000.123,
  "level": "ERROR",
  "target": "panic",
  "fields": {
    "message": "PANIC: <panic message>",
    "location": "src/player/control.rs:42:10"
  }
}
```
If the game process terminates unexpectedly, immediately inspect `target/agent_telemetry.jsonl` to pinpoint the failing line and stack trace.
