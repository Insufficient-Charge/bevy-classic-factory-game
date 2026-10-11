param(
    [string]$HostName = "127.0.0.1",
    [int]$Port = 6550,
    [int]$TimeoutSeconds = 10
)

$ErrorActionPreference = "Stop"

function Send-HarnessCommand {
    param([string]$Cmd)
    Write-Host "`n>>> SENDING: $Cmd" -ForegroundColor Cyan
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command $Cmd -HostName $HostName -Port $Port -TimeoutSeconds $TimeoutSeconds
    Write-Host $res
    return $res
}

Write-Host "Waiting for Harness port $Port to open..." -ForegroundColor Yellow
$connected = $false
for ($i = 0; $i -lt 30; $i++) {
    $conn = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
    if ($conn) {
        $connected = $true
        Write-Host "Connected to Agent Harness on port $Port!" -ForegroundColor Green
        break
    }
    Start-Sleep -Seconds 1
}

if (-not $connected) {
    Write-Error "Could not connect to Agent Harness on port $Port within 30 seconds."
    exit 1
}

# 1. Query initial state
Send-HarnessCommand "query state"

# 2. Pause ticks for deterministic testing
Send-HarnessCommand "tick pause"

# 3. Test hotbar slot selection
Send-HarnessCommand "slot 1"
Send-HarnessCommand "slot 2"
Send-HarnessCommand "slot 4"

# 4. Multi-layer Building placement tests:
# Ground level incline ramp at (2, 2)
Send-HarnessCommand "build 2 2 incline_up_n ground"

# Elevated viaduct overpass belt at (2, 3) on Layer::Viaduct
Send-HarnessCommand "build 2 3 viaduct_n viaduct"

# Tetris Cargo Compressor at (4, 4)
Send-HarnessCommand "build 4 4 compressor_n ground"

# Swivel Turntable at (4, 3)
Send-HarnessCommand "build 4 3 turntable_n ground"

# Survey Beacon at (1, 1)
Send-HarnessCommand "build 1 1 beacon ground"

# Logic Diviner at (3, 3)
Send-HarnessCommand "build 3 3 diviner ground"

# 5. Advance simulation ticks
Send-HarnessCommand "tick 60"

# 6. Query state to verify multi-layer buildings, compressor status, and tiles
Send-HarnessCommand "query state"

# 7. Crafting test for Phase 2: craft Viaduct Belt
Send-HarnessCommand "craft craft_viaduct_belt"
Send-HarnessCommand "tick 90"

# 8. Dismantle elevated viaduct layer at (2, 3)
Send-HarnessCommand "dismantle 2 3 viaduct"
Send-HarnessCommand "tick 30"

# 9. Query state post-dismantle
Send-HarnessCommand "query state"

# 10. BRP HTTP JSON-RPC introspection
Write-Host "`n>>> BRP HTTP QUERY: Checking Building components..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::building::Building"

Write-Host "`n>>> BRP HTTP QUERY: Checking CargoCompressor components..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::logistics::CargoCompressor"

Write-Host "`n>>> BRP HTTP QUERY: Checking InclineRamp components..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::logistics::InclineRamp"

Write-Host "`n>>> BRP HTTP QUERY: Checking SwivelTurntable components..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::logistics::SwivelTurntable"

# 11. Clean shutdown of the game daemon
Send-HarnessCommand "quit"

Write-Host "`n=== PHASE 2 PLAYTEST COMPLETE ===" -ForegroundColor Green
