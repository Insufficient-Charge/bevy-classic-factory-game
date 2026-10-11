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

# 1. Query initial state & Verify Phase 3 Inventory Items
Write-Host "`n--- STEP 1: INITIAL STATE & PHASE 3 INVENTORY ---" -ForegroundColor Yellow
$initState = Send-HarnessCommand "query state"
$initJson = $initState | ConvertFrom-Json

Write-Host "Screen: $($initJson.screen)" -ForegroundColor Green
Write-Host "Player starting pos: $($initJson.player.translation)" -ForegroundColor Green
Write-Host "Diorama archipelago total tiles: $($initJson.terrain.total_tiles)" -ForegroundColor Green

# 2. Pause ticks for deterministic frame control
Send-HarnessCommand "tick pause"

# 3. Prospect ground tile (1, 2) and (1, 3) to ensure revealed
Write-Host "`n--- STEP 2: PROSPECT TILES FOR AUTOCLAVE & BELT ---" -ForegroundColor Yellow
Send-HarnessCommand "prospect 1 2 unfold"
Send-HarnessCommand "prospect 1 3 unfold"
Send-HarnessCommand "prospect 3 2 unfold"
Send-HarnessCommand "tick 2"

# 4. Place Chromatic Autoclave and Resonance Bell Jar
Write-Host "`n--- STEP 3: PLACE CHROMATIC AUTOCLAVE & RESONANCE BELL JAR ---" -ForegroundColor Yellow
Send-HarnessCommand "build 1 2 autoclave_n ground"
Send-HarnessCommand "build 1 3 belt_n ground"
Send-HarnessCommand "build 3 2 bell_jar ground"
Send-HarnessCommand "tick 5"

$bldState = Send-HarnessCommand "query state"
$bldJson = $bldState | ConvertFrom-Json

Write-Host "Autoclaves count: $($bldJson.autoclaves.Count)" -ForegroundColor Green
Write-Host "Bell Jars count: $($bldJson.bell_jars.Count)" -ForegroundColor Green

if ($bldJson.autoclaves.Count -lt 1) {
    Write-Error "FAILURE: Chromatic Autoclave was not spawned!"
    exit 1
}
if ($bldJson.bell_jars.Count -lt 1) {
    Write-Error "FAILURE: Resonance Bell Jar was not spawned!"
    exit 1
}

# 5. Inspect Autoclave 6x6 Tray Grid via ASCII command
Write-Host "`n--- STEP 4: INSPECT 6x6 MATCH-3 TRAY GRID ---" -ForegroundColor Yellow
$gridAscii = Send-HarnessCommand "autoclave 1 2 grid"
if (-not ($gridAscii -match "ACK_AUTOCLAVE_GRID")) {
    Write-Error "FAILURE: Did not receive valid autoclave grid ASCII!"
    exit 1
}
Write-Host "SUCCESS: Autoclave grid rendered successfully." -ForegroundColor Green

# 6. Execute Match-3 Swap & Solve
Write-Host "`n--- STEP 5: EXECUTE CATALYSIS MATCH-3 SOLVE ---" -ForegroundColor Yellow
$solveRes = Send-HarnessCommand "autoclave 1 2 solve"
if (-not ($solveRes -match "ACK_AUTOCLAVE_SOLVED")) {
    Write-Error "FAILURE: Failed to execute autoclave match-3 solve: $solveRes"
    exit 1
}
Write-Host "SUCCESS: Match-3 catalysis reactions solved." -ForegroundColor Green

# 7. Tick simulation to process belt conveyance and power generation
Write-Host "`n--- STEP 6: TICK SIMULATION FOR REFINED INGOT DISPATCH ---" -ForegroundColor Yellow
Send-HarnessCommand "tick 30"

$postState = Send-HarnessCommand "query state"
$postJson = $postState | ConvertFrom-Json

$auto = $postJson.autoclaves[0]
Write-Host "Autoclave Stored Power: $($auto.stored_power_kw) kW" -ForegroundColor Green
Write-Host "Autoclave Total Reactions: $($auto.total_catalysis_reactions)" -ForegroundColor Green
Write-Host "Transported items active: $($postJson.transported_items_count)" -ForegroundColor Green

# 8. BRP HTTP JSON-RPC Inspection (port 15702)
Write-Host "`n--- STEP 7: BRP HTTP JSON-RPC INTROSPECTION ---" -ForegroundColor Yellow
try {
    $brpBody = @{
        jsonrpc = "2.0"
        method = "bevy/list"
        id = 1
    } | ConvertTo-Json

    $brpRes = Invoke-RestMethod -Uri "http://127.0.0.1:15702" -Method Post -Body $brpBody -ContentType "application/json" -TimeoutSec 3
    if ($brpRes -and $brpRes.result) {
        Write-Host "SUCCESS: BRP JSON-RPC endpoint responding with $($brpRes.result.Count) components." -ForegroundColor Green
    }
} catch {
    Write-Host "BRP introspection skipped or not running on port 15702: $_" -ForegroundColor DarkGray
}

# 9. Camera Orbit Rotation & Screenshot
Write-Host "`n--- STEP 8: CAMERA ROTATION & SCREENSHOT ---" -ForegroundColor Yellow
Send-HarnessCommand "rotate_camera 45"
Send-HarnessCommand "screenshot target/screenshots/phase3_playtest.png"
Send-HarnessCommand "tick 5"

Write-Host "`n=========================================" -ForegroundColor Green
Write-Host " PHASE 3 PLAYTEST SIMULATION PASSED! " -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green

Send-HarnessCommand "quit"
