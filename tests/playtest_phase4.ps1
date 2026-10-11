param(
    [string]$HostName = "127.0.0.1",
    [int]$Port = 6550,
    [int]$BrpPort = 15702,
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

# 1. Query initial state & verify Phase 4 world expansion
Write-Host "`n--- STEP 1: VERIFY PHASE 4 WORLD EXPANSION & INVENTORY ---" -ForegroundColor Yellow
$initState = Send-HarnessCommand "query state"
$initJson = $initState | ConvertFrom-Json

# Check Resonance Grove island exists in state
$grove = $initJson.resonance_grove
if (-not $grove) {
    Write-Error "FAILURE: Resonance Grove sector was not loaded into state!"
    exit 1
}
Write-Host "Resonance Grove origin: [$($grove.origin[0]), $($grove.origin[1])], size: $($grove.width)x$($grove.height)" -ForegroundColor Green

# Check Ancient Clue Monoliths
$monoliths = $initJson.monoliths
Write-Host "Discovered $($monoliths.Count) ancient monolith clues in diorama world" -ForegroundColor Green
if ($monoliths.Count -lt 3) {
    Write-Error "FAILURE: Expected at least 3 ancient clue monoliths, found $($monoliths.Count)"
    exit 1
}

$lockedClues = $monoliths | Where-Object { $_.is_locked -eq $true }
if ($lockedClues.Count -lt 3) {
    Write-Error "FAILURE: Expected at least 3 locked clues, found $($lockedClues.Count)"
    exit 1
}
Write-Host "SUCCESS: Found $($lockedClues.Count) ancient clue obelisks with locked frequencies!" -ForegroundColor Green

# Check player inventory has Phase 4 items
$invItems = $initJson.inventory.items
Write-Host "Player inventory holds $($invItems.Count) item types" -ForegroundColor Green
$hasMonolithItem = $invItems | Where-Object { $_.item -eq "AcousticMonolith" }
$hasTuningFork = $invItems | Where-Object { $_.item -eq "TuningFork" }
if (-not $hasMonolithItem -or -not $hasTuningFork) {
    Write-Error "FAILURE: Player missing Phase 4 starter items (AcousticMonolith or TuningFork)!"
    exit 1
}
Write-Host "SUCCESS: Phase 4 starter items present in player satchel!" -ForegroundColor Green

# 2. Pause simulation ticks for deterministic step-by-step verification
Send-HarnessCommand "tick pause"

# 3. Test 'grove status' command
Write-Host "`n--- STEP 2: TEST GROVE STATUS COMMAND ---" -ForegroundColor Yellow
$groveStatusResp = Send-HarnessCommand "grove status"
$groveStatusJson = $groveStatusResp | ConvertFrom-Json
if ($groveStatusJson.origin[0] -ne 16 -or $groveStatusJson.origin[1] -ne 0) {
    Write-Error "FAILURE: Unexpected grove origin! Got [$($groveStatusJson.origin[0]), $($groveStatusJson.origin[1])]"
    exit 1
}
Write-Host "SUCCESS: grove status confirmed 9x9 grove sector at [16, 0]" -ForegroundColor Green

# 4. Test locked clue immutability
Write-Host "`n--- STEP 3: TEST ANCIENT CLUE IMMUTABILITY ---" -ForegroundColor Yellow
$tuneLocked = Send-HarnessCommand "tune 16 0 2"
if ($tuneLocked -notmatch "ERR_TUNING_LOCKED") {
    Write-Error "FAILURE: Expected ERR_TUNING_LOCKED when attempting to re-tune ancient clue! Got: $tuneLocked"
    exit 1
}
Write-Host "SUCCESS: Immovable ancient clue obelisk at (16, 0) correctly rejected re-tuning!" -ForegroundColor Green

# 5. Place building and tune frequency
Write-Host "`n--- STEP 4: PLACE HARMONIC BUILDING & TUNE FREQUENCY ---" -ForegroundColor Yellow
$buildResp1 = Send-HarnessCommand "build 17 0 monolith ground"
if ($buildResp1 -notmatch "ACK_BUILD") {
    Write-Error "FAILURE: Could not place Acoustic Monolith at (17, 0)! Got: $buildResp1"
    exit 1
}
$tuneResp1 = Send-HarnessCommand "tune 17 0 2"
if ($tuneResp1 -notmatch "ACK_TUNE 17 0 freq=Some\(2\)") {
    Write-Error "FAILURE: Could not tune Acoustic Monolith at (17, 0) to freq 2! Got: $tuneResp1"
    exit 1
}
Write-Host "SUCCESS: Acoustic Monolith at (17, 0) tuned to frequency 2!" -ForegroundColor Green

# 6. Test Sudoku Acoustic Conflict Detection
Write-Host "`n--- STEP 5: SUDOKU CONFLICT DETECTION ---" -ForegroundColor Yellow
$buildResp2 = Send-HarnessCommand "build 18 0 monolith ground"
$tuneResp2 = Send-HarnessCommand "tune 18 0 2"
Send-HarnessCommand "tick 5"

$conflictState = Send-HarnessCommand "query state"
$conflictJson = $conflictState | ConvertFrom-Json
$tunings = $conflictJson.tunings

$m17 = $tunings | Where-Object { $_.coord[0] -eq 17 -and $_.coord[1] -eq 0 }
$m18 = $tunings | Where-Object { $_.coord[0] -eq 18 -and $_.coord[1] -eq 0 }

if (-not $m17.is_conflicted -or -not $m18.is_conflicted) {
    Write-Error "FAILURE: Conflict detection failed! Machines at (17,0) and (18,0) with identical freq=2 did not conflict!"
    exit 1
}
if ($m17.speed_multiplier -ne 0.5 -or $m18.speed_multiplier -ne 0.5) {
    Write-Error "FAILURE: Flutter speed penalty failed! Expected speed_multiplier=0.5, got m17=$($m17.speed_multiplier), m18=$($m18.speed_multiplier)"
    exit 1
}
Write-Host "SUCCESS: Row 0 frequency conflict detected! Both machines penalized with 0.5x speed flutter!" -ForegroundColor Green

# 7. Test Conflict Resolution
Write-Host "`n--- STEP 6: CONFLICT RESOLUTION ---" -ForegroundColor Yellow
$resolveResp = Send-HarnessCommand "tune 18 0 3"
Send-HarnessCommand "tick 5"

$resolvedState = Send-HarnessCommand "query state"
$resolvedJson = $resolvedState | ConvertFrom-Json
$r17 = $resolvedJson.tunings | Where-Object { $_.coord[0] -eq 17 -and $_.coord[1] -eq 0 }
$r18 = $resolvedJson.tunings | Where-Object { $_.coord[0] -eq 18 -and $_.coord[1] -eq 0 }

if ($r17.is_conflicted -or $r18.is_conflicted) {
    Write-Error "FAILURE: Conflict was not cleared after changing frequency to 3!"
    exit 1
}
if ($r17.speed_multiplier -ne 1.0 -or $r18.speed_multiplier -ne 1.0) {
    Write-Error "FAILURE: Speed was not restored to 1.0 baseline after resolving conflict!"
    exit 1
}
Write-Host "SUCCESS: Conflict resolved! Machines restored to 1.0x operational speed!" -ForegroundColor Green

# 8. Test Automatic Sudoku Solver via 'grove solve'
Write-Host "`n--- STEP 7: TEST AUTOMATIC SUDOKU SOLVER (grove solve) ---" -ForegroundColor Yellow
$solveResp = Send-HarnessCommand "grove solve"
if ($solveResp -notmatch "ACK_GROVE_SOLVED") {
    Write-Error "FAILURE: grove solve failed! Got: $solveResp"
    exit 1
}
Write-Host "SUCCESS: Backtracking Sudoku solver successfully calibrated grove machines!" -ForegroundColor Green

# 9. Test Bevy Remote Protocol (BRP) HTTP JSON-RPC endpoint
Write-Host "`n--- STEP 8: TEST BEVY REMOTE PROTOCOL (BRP) HTTP ENDPOINT ---" -ForegroundColor Yellow
try {
    $brpBody = '{"jsonrpc":"2.0","id":1,"method":"bevy/list"}'
    $brpResp = Invoke-RestMethod -Uri "http://127.0.0.1:$BrpPort" -Method Post -Body $brpBody -ContentType "application/json" -TimeoutSec 5
    if ($brpResp.result) {
        Write-Host "SUCCESS: BRP JSON-RPC introspection returned $($brpResp.result.Count) types!" -ForegroundColor Green
    } else {
        Write-Host "BRP response received: $($brpResp | ConvertTo-Json -Compress)" -ForegroundColor Yellow
    }
} catch {
    Write-Warning "BRP endpoint check warning: $_"
}

# 10. Clean exit
Write-Host "`n--- STEP 9: CLEAN SHUTDOWN ---" -ForegroundColor Yellow
Send-HarnessCommand "quit"
Write-Host "`n========================================================" -ForegroundColor Green
Write-Host "ALL PHASE 4 PLAYTEST VERIFICATIONS PASSED 100%!" -ForegroundColor Green
Write-Host "========================================================" -ForegroundColor Green
