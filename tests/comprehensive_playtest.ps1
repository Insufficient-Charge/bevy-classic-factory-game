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

Write-Host "Connecting to Agent Harness on port $Port..." -ForegroundColor Yellow
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

# 1. Query initial state & Verify Expanded Starting Platform
Write-Host "`n--- STEP 1: INITIAL STATE & EXPANDED STARTING PLATFORM ---" -ForegroundColor Yellow
$initState = Send-HarnessCommand "query state"

# Parse JSON to verify initial spawn position and expanded platform tile count
$initJson = $initState | ConvertFrom-Json
$playerX = $initJson.player.translation[0]
$playerY = $initJson.player.translation[1]
$playerZ = $initJson.player.translation[2]
$totalTiles = $initJson.terrain.total_tiles
Write-Host "Player starting pos: X=$playerX, Y=$playerY, Z=$playerZ" -ForegroundColor Green
Write-Host "Diorama archipelago total tiles: $totalTiles (Widened platform active)" -ForegroundColor Green

if ($totalTiles -lt 250) {
    Write-Error "FAILURE: Starting platform was not widened! Found only $totalTiles tiles."
    exit 1
}

# 2. Pause ticks for deterministic frame control
Send-HarnessCommand "tick pause"

# 3. Collision Testing (Ground Support & Solid Obstacle Impedance)
Write-Host "`n--- STEP 2: COLLISION TESTING (GROUND & OBSTACLES) ---" -ForegroundColor Yellow
# Assert player is supported by ground tile static cuboid colliders
if ($playerY -lt 0.0 -or $playerY -gt 1.5) {
    Write-Error "FAILURE: Ground collision error! Player Y=$playerY is not resting on ground collider."
    exit 1
}
Write-Host "SUCCESS: Player is securely resting on ground tile colliders at Y=$playerY" -ForegroundColor Green

# Place solid obstacle building at (0, 0) directly ahead along camera forward vector from spawn (1, 1)
Write-Host "Placing solid obstacle building (Storage Chest) at (0, 0)..." -ForegroundColor Yellow
Send-HarnessCommand "build 0 0 chest ground"
Send-HarnessCommand "tick 5"

# Attempt to walk forward into the solid obstacle
Write-Host "Attempting to walk forward into solid obstacle..." -ForegroundColor Yellow
Send-HarnessCommand "press W"
Send-HarnessCommand "tick 30"
Send-HarnessCommand "release W"

$blockedState = Send-HarnessCommand "query state"
$blockedJson = $blockedState | ConvertFrom-Json
$blockedX = $blockedJson.player.translation[0]
$blockedZ = $blockedJson.player.translation[2]
Write-Host "Player position against obstacle: X=$blockedX, Z=$blockedZ" -ForegroundColor Yellow

if ($blockedX -lt 0.5 -or $blockedZ -lt 0.5) {
    Write-Error "FAILURE: Player walked through the solid obstacle! Collider failed to block movement (X=$blockedX, Z=$blockedZ)."
    exit 1
}
Write-Host "SUCCESS: Obstacle collider successfully stopped player penetration! (X=$blockedX, Z=$blockedZ >= 0.5)" -ForegroundColor Green

# Dismantle obstacle to free the path
Write-Host "Dismantling obstacle at (0, 0) to clear path..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 0 0 ground"
Send-HarnessCommand "tick 5"

# Now walk forward through the cleared path
Write-Host "Walking through cleared path..." -ForegroundColor Yellow
Send-HarnessCommand "press W"
Send-HarnessCommand "tick 30"
Send-HarnessCommand "release W"

$unblockedState = Send-HarnessCommand "query state"
$unblockedJson = $unblockedState | ConvertFrom-Json
$unblockedX = $unblockedJson.player.translation[0]
$unblockedZ = $unblockedJson.player.translation[2]
Write-Host "Player position after clearing obstacle: X=$unblockedX, Z=$unblockedZ" -ForegroundColor Green

if ($unblockedX -gt 0.45 -or $unblockedZ -gt 0.45) {
    Write-Error "FAILURE: Player did not advance after path was cleared! X=$unblockedX, Z=$unblockedZ"
    exit 1
}
Write-Host "SUCCESS: Player freely advanced once obstacle was removed (X=$unblockedX, Z=$unblockedZ <= 0.45)" -ForegroundColor Green

# 4. 360-Degree Diorama Camera Rotation Back to Original Starting Point
Write-Host "`n--- STEP 3: 360-DEGREE CAMERA ROTATION BACK TO STARTING POINT ---" -ForegroundColor Yellow
$camInitJson = (Send-HarnessCommand "query state" | ConvertFrom-Json)
$initialYaw = $camInitJson.camera.angles.yaw
Write-Host "Initial Camera Yaw: $initialYaw" -ForegroundColor Yellow

# Rotate 4 x 90 degrees clockwise = 360 degrees full circle
Write-Host "Rotating 90 deg (Step 1 - 135 deg)..." -ForegroundColor Cyan
Send-HarnessCommand "rotate_camera cw"
Send-HarnessCommand "tick 15"

Write-Host "Rotating 180 deg (Step 2 - 225 deg)..." -ForegroundColor Cyan
Send-HarnessCommand "rotate_camera cw"
Send-HarnessCommand "tick 15"

Write-Host "Rotating 270 deg (Step 3 - 315 deg)..." -ForegroundColor Cyan
Send-HarnessCommand "rotate_camera cw"
Send-HarnessCommand "tick 15"

Write-Host "Rotating 360 deg back to starting point (Step 0 - 45 deg)..." -ForegroundColor Cyan
Send-HarnessCommand "rotate_camera cw"
Send-HarnessCommand "tick 20"

$cam360Json = (Send-HarnessCommand "query state" | ConvertFrom-Json)
$finalYaw = $cam360Json.camera.angles.yaw
Write-Host "Final Camera Yaw after 360 deg rotation: $finalYaw" -ForegroundColor Green

$yawDiff = [Math]::Abs($finalYaw - $initialYaw)
if ($yawDiff -gt 0.2 -and [Math]::Abs($yawDiff - (2 * [Math]::PI)) -gt 0.2) {
    Write-Error "FAILURE: Camera did not return to original starting point! Diff=$yawDiff"
    exit 1
}
Write-Host "SUCCESS: Diorama camera rotated 360 degrees in full orbital sphere back to original starting point!" -ForegroundColor Green

# 5. Hotbar Selection
Write-Host "`n--- STEP 4: HOTBAR SLOT SELECTION ---" -ForegroundColor Yellow
Send-HarnessCommand "slot 1"
Send-HarnessCommand "slot 3"
Send-HarnessCommand "slot 0"

# 6. Minesweeper Prospecting: Unfold and Flag covered tiles on widened platform
Write-Host "`n--- STEP 5: PROSPECTING (UNFOLD & FLAG) ---" -ForegroundColor Yellow
Send-HarnessCommand "prospect 0 -1 flag"
Send-HarnessCommand "prospect -1 0 unfold"
Send-HarnessCommand "tick 5"

# 7. Hand Mining: Harvest Iron Ore deposit at (2,2)
Write-Host "`n--- STEP 6: RESOURCE MINING ---" -ForegroundColor Yellow
Send-HarnessCommand "mine 2 2"

# 8. Field Workshop Crafting: Smelt Iron Plate
Write-Host "`n--- STEP 7: WORKSHOP HANDCRAFTING ---" -ForegroundColor Yellow
Send-HarnessCommand "craft smelt_iron_plate"
Send-HarnessCommand "tick 90"

# 9. Multi-Layer Factory Construction
Write-Host "`n--- STEP 8: MULTI-LAYER FACTORY LOGISTICS ---" -ForegroundColor Yellow
# Place ground belt
Send-HarnessCommand "build 2 3 belt_n ground"
# Place wellhead extractor over deposit
Send-HarnessCommand "build 2 2 wellhead_n ground"
# Place Incline Ramp from Ground to Viaduct
Send-HarnessCommand "build 2 4 incline_up_n ground"
# Place elevated Viaduct Belt on Layer::Viaduct
Send-HarnessCommand "build 2 5 viaduct_n viaduct"

# Step simulation to allow factory systems to process
Send-HarnessCommand "tick 60"

# 10. Dismantling
Write-Host "`n--- STEP 9: DISMANTLING ---" -ForegroundColor Yellow
Send-HarnessCommand "dismantle 2 5 viaduct"
Send-HarnessCommand "tick 20"

# 11. Final Query State & Tester Guide Checklist Verification
Write-Host "`n--- STEP 10: VERIFYING SIMULATION & TESTER TASKS CHECKLIST ---" -ForegroundColor Yellow
$finalState = Send-HarnessCommand "query state"
$finalJson = $finalState | ConvertFrom-Json
Write-Host "Total buildings placed: $($finalJson.buildings.Count)" -ForegroundColor Green
Write-Host "Inventory item count: $($finalJson.inventory.items.Count)" -ForegroundColor Green
Write-Host "Terrain tiles: $($finalJson.terrain.total_tiles) (Revealed: $($finalJson.terrain.revealed), Flagged: $($finalJson.terrain.flagged))" -ForegroundColor Green

if ($finalJson.tester_tasks) {
    Write-Host "Tester tasks verified: $($finalJson.tester_tasks.completed) / 9 tasks completed!" -ForegroundColor Green
    if ($finalJson.tester_tasks.completed -lt 9) {
        Write-Error "FAILURE: Not all tester tasks were completed! Found ($($finalJson.tester_tasks.completed) / 9)"
        exit 1
    }
}

# 12. BRP Introspection
Write-Host "`n--- STEP 11: BRP HTTP JSON-RPC INTROSPECTION ---" -ForegroundColor Magenta
powershell -File .agents/skills/agent-harness/scripts/brp_cli.ps1 query -Components "bevy_classic_factory_game_2::game::building::Building"

# 13. Clean Shutdown
Send-HarnessCommand "quit"
Write-Host "`n=== COMPREHENSIVE PLAYTEST & SIMULATION SUCCEEDED! ===" -ForegroundColor Green
