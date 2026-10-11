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

# 3. Prospecting test: Unfold tile (4,4) and Flag tile (4,5)
Send-HarnessCommand "prospect 4 4 unfold"
Send-HarnessCommand "prospect 4 5 flag"

# 4. Mining test: Mine Iron Ore at (2,2)
Send-HarnessCommand "mine 2 2"

# 5. Crafting test: Craft an Iron Plate (requires 1 Iron Ore)
Send-HarnessCommand "craft smelt_iron_plate"

# 6. Step simulation by 90 ticks (1.5 seconds at 60Hz) to complete 1.0s crafting
Send-HarnessCommand "tick 90"

# 7. Building test: Place conveyor belt at (2,3) North
Send-HarnessCommand "build 2 3 belt_n"

# 8. Building test: Place clockwork wellhead at (2,2) North over the iron deposit
Send-HarnessCommand "build 2 2 wellhead_n"

# 9. Query state to verify all operations succeeded
Send-HarnessCommand "query state"

# 10. Test Bevy Remote Protocol (BRP) integration
Write-Host "`n>>> BRP HTTP QUERY: Checking Building components..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::building::Building"

Write-Host "`n>>> BRP HTTP QUERY: Reading DioramaCameraConfig resource..." -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 get-resource -Resource "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig"

# 11. Clean shutdown of the daemon
Send-HarnessCommand "quit"

Write-Host "`n=== PLAYTEST COMPLETE ===" -ForegroundColor Green
