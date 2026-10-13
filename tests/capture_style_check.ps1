param(
    [string]$HostName = "127.0.0.1",
    [int]$Port = 6550
)

$ErrorActionPreference = "Stop"

function Send-HarnessCommand {
    param([string]$Cmd)
    Write-Host ">>> SENDING: $Cmd" -ForegroundColor Cyan
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command $Cmd
    Write-Host $res
    return $res
}

Write-Host "Connecting to port $Port..." -ForegroundColor Yellow
for ($i = 0; $i -lt 30; $i++) {
    $conn = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
    if ($conn) { break }
    Start-Sleep -Seconds 1
}

# 1. Wait for game assets to finish loading
Write-Host "Checking screen state..." -ForegroundColor Yellow
for ($i = 0; $i -lt 40; $i++) {
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "query state"
    if ($res -match '"screen":\s*"(Title|Gameplay)"') {
        Write-Host "Game assets loaded!" -ForegroundColor Green
        break
    }
    powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "tick 20"
    Start-Sleep -Milliseconds 300
}

# 2. Transition to Gameplay screen if on Title
$res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "query state"
if ($res -match '"screen":\s*"Title"') {
    Write-Host "Transitioning from Title to Gameplay..." -ForegroundColor Yellow
    Send-HarnessCommand "screen gameplay"
    Send-HarnessCommand "tick 30"
}

# 3. Wait for Player and diorama archipelago to spawn
Write-Host "Waiting for Player and diorama archipelago..." -ForegroundColor Yellow
for ($i = 0; $i -lt 30; $i++) {
    Send-HarnessCommand "tick 20"
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "query state"
    if ($res -match '"screen":\s*"Gameplay"' -and $res -match '"player":\s*\{') {
        Write-Host "Player and diorama archipelago fully loaded!" -ForegroundColor Green
        break
    }
    Start-Sleep -Milliseconds 200
}

Send-HarnessCommand "tick 60"

# 4. Unfold / flag tiles to demonstrate terrain interaction
Send-HarnessCommand "prospect 3 3 unfold"
Send-HarnessCommand "prospect 4 4 unfold"
Send-HarnessCommand "prospect 5 5 flag"
Send-HarnessCommand "tick 30"

# 5. Capture diorama spawn view with cloud-sea, golden lighting & field guide
Write-Host "Capturing initial diorama view..." -ForegroundColor Yellow
Send-HarnessCommand "screenshot target/screenshots/gameplay_diorama_improved.png"
Send-HarnessCommand "tick 30"
Start-Sleep -Seconds 1

# 6. Rotate camera to view cliff strata and cloud-sea depth
Write-Host "Rotating camera to view island cliff foundations..." -ForegroundColor Yellow
Send-HarnessCommand "rotate_camera cw"
Send-HarnessCommand "tick 30"
Send-HarnessCommand "screenshot target/screenshots/gameplay_rotated_improved.png"
Send-HarnessCommand "tick 30"
Start-Sleep -Seconds 1

# 7. Place factory buildings demonstrating toybox clockwork machinery
Write-Host "Placing multi-part toybox machines..." -ForegroundColor Yellow
Send-HarnessCommand "rotate_camera ccw"
Send-HarnessCommand "tick 20"
Send-HarnessCommand "teleport 4 3"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "prospect 4 2 unfold"
Send-HarnessCommand "prospect 4 3 unfold"
Send-HarnessCommand "prospect 4 4 unfold"
Send-HarnessCommand "prospect 3 3 unfold"
Send-HarnessCommand "tick 10"
Send-HarnessCommand "teleport 1 1"
Send-HarnessCommand "tick 10"
Send-HarnessCommand "build 2 2 wellhead ground"
Send-HarnessCommand "build 2 3 belt_n ground"
Send-HarnessCommand "build 2 4 belt_n ground"
Send-HarnessCommand "build 2 5 chest ground"
Send-HarnessCommand "build 1 3 autoclave_n ground"
Send-HarnessCommand "build 3 1 monolith ground"
Send-HarnessCommand "build 4 2 hearth ground"
Send-HarnessCommand "build 4 3 viaduct_n viaduct"
Send-HarnessCommand "build 4 4 viaduct_n viaduct"
Send-HarnessCommand "build 3 3 pylon ground"
Send-HarnessCommand "tick 60"

Write-Host "Capturing factory scene view..." -ForegroundColor Yellow
Send-HarnessCommand "screenshot target/screenshots/gameplay_factory_improved.png"
Send-HarnessCommand "tick 30"
Start-Sleep -Seconds 2

# 8. Clean shutdown
Send-HarnessCommand "quit"
Write-Host "Style check screenshots captured successfully!" -ForegroundColor Green
