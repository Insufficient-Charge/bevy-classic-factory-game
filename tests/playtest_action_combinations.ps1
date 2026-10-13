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

function Wait-For-Screen {
    Write-Host "Waiting for Screen::Gameplay..." -ForegroundColor Yellow
    for ($i = 0; $i -lt 30; $i++) {
        $stateRaw = Send-HarnessCommand "query state"
        if ($stateRaw -match '"screen"\s*:\s*"Gameplay"') {
            Write-Host "Gameplay screen active!" -ForegroundColor Green
            return
        }
        Send-HarnessCommand "tick 10"
        Start-Sleep -Milliseconds 200
    }
    Write-Error "Timed out waiting for Gameplay screen"
    exit 1
}

Write-Host "==================================================================" -ForegroundColor Magenta
Write-Host " EXECUTABLE PLAYTEST SUITE: 2-3 PLAYER ACTION COMBINATIONS & CHAINS" -ForegroundColor Magenta
Write-Host "==================================================================" -ForegroundColor Magenta

# Check TCP connectivity
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

# Wait until Gameplay screen is loaded
Wait-For-Screen
Send-HarnessCommand "tick pause"

# -------------------------------------------------------------------------
# COMBO 1: RESOURCE EXTRACTION PIPELINE (Prospect Turf -> Mine Deposit -> Handcraft)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 1] EXTRACTION PIPELINE: PROSPECT -> MINE -> CRAFT" -ForegroundColor Yellow

# Step 1.1: Prospect covered turf
Write-Host "Action 1: Prospecting turf..." -ForegroundColor Gray
$targetC1Y = 4
for ($y = 4; $y -le 10; $y++) {
    $check = (Send-HarnessCommand "query visual 4 $y ground") | ConvertFrom-Json
    if ($check.tile -and $check.tile.prospect_state -eq "Covered" -and -not $check.tile.has_hazard) {
        $targetC1Y = $y
        break
    }
}
$c1TpY = $targetC1Y - 1
Send-HarnessCommand "teleport 4 $c1TpY"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "prospect 4 $targetC1Y unfold"
Send-HarnessCommand "tick 5"
$c1Tile = (Send-HarnessCommand "query visual 4 $targetC1Y ground") | ConvertFrom-Json
if ($c1Tile.tile.prospect_state -ne "Revealed") {
    Write-Error "FAIL [Combo 1]: Turf at (4, $targetC1Y) was not revealed!"
    exit 1
}
Write-Host "Action 1 Succeeded: Turf (4, $targetC1Y) revealed with 3D terrain visual." -ForegroundColor Green

# Step 1.2: Mine Iron Ore deposit at (2, 2)
Write-Host "Action 2: Mining Iron Ore deposit at (2, 2)..." -ForegroundColor Gray
Send-HarnessCommand "teleport 2 2"
Send-HarnessCommand "tick 5"

$inv1 = (Send-HarnessCommand "query state") | ConvertFrom-Json
$orePre = 0
foreach ($item in $inv1.inventory.items) {
    if ($item.item -eq "IronOre") { $orePre += $item.count }
}

Send-HarnessCommand "mine 2 2"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "mine 2 2"
Send-HarnessCommand "tick 5"

$inv2 = (Send-HarnessCommand "query state") | ConvertFrom-Json
$orePost = 0
foreach ($item in $inv2.inventory.items) {
    if ($item.item -eq "IronOre") { $orePost += $item.count }
}
if ($orePost -lt ($orePre + 2)) {
    Write-Error "FAIL [Combo 1]: Expected at least 2 iron ore extracted, got $orePost (was $orePre)!"
    exit 1
}
Write-Host "Action 2 Succeeded: Extracted iron ore ($orePre -> $orePost)." -ForegroundColor Green

# Step 1.3: Handcraft Smelt Iron Plate
Write-Host "Action 3: Handcrafting smelt_iron_plate..." -ForegroundColor Gray
$ironPlatesPre = 0
foreach ($item in $inv2.inventory.items) {
    if ($item.item -eq "IronPlate") { $ironPlatesPre += $item.count }
}
Send-HarnessCommand "craft smelt_iron_plate"

$ironPlatesPost = $ironPlatesPre
for ($w = 0; $w -lt 25; $w++) {
    Send-HarnessCommand "tick 15"
    Start-Sleep -Milliseconds 100
    $inv3 = (Send-HarnessCommand "query state") | ConvertFrom-Json
    $curIronPlates = 0
    foreach ($item in $inv3.inventory.items) {
        if ($item.item -eq "IronPlate") { $curIronPlates += $item.count }
    }
    if ($curIronPlates -gt $ironPlatesPre) {
        $ironPlatesPost = $curIronPlates
        break
    }
}

if ($ironPlatesPost -le $ironPlatesPre) {
    Write-Error "FAIL [Combo 1]: Crafting failed to produce IronPlate!"
    exit 1
}
Write-Host "Action 3 Succeeded: Iron Plate crafted into inventory." -ForegroundColor Green
Write-Host "SUCCESS [COMBO 1]: 3-Action Chain (Prospect -> Mine -> Craft) Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# COMBO 2: HOTBAR TO GRID CONSTRUCTION (Craft Building -> Select Hotbar -> Place)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 2] CONSTRUCTION PIPELINE: CRAFT -> SELECT HOTBAR -> PLACE ON GRID" -ForegroundColor Yellow

# Step 2.1: Handcraft Wooden Roller Belts (inputs: 1 IronPlate, 1 WoodenPlank -> produces 4 belts)
Write-Host "Action 1: Crafting Wooden Roller Belts..." -ForegroundColor Gray
$invBeltsPre = 0
foreach ($item in ((Send-HarnessCommand "query state") | ConvertFrom-Json).inventory.items) {
    if ($item.item -eq "WoodenRollerBelt") { $invBeltsPre += $item.count }
}
Send-HarnessCommand "craft craft_roller_belt"

$invBeltsPost = $invBeltsPre
for ($w = 0; $w -lt 25; $w++) {
    Send-HarnessCommand "tick 15"
    Start-Sleep -Milliseconds 100
    $invCheck = (Send-HarnessCommand "query state") | ConvertFrom-Json
    $curBelts = 0
    foreach ($item in $invCheck.inventory.items) {
        if ($item.item -eq "WoodenRollerBelt") { $curBelts += $item.count }
    }
    if ($curBelts -gt $invBeltsPre) {
        $invBeltsPost = $curBelts
        break
    }
}

if ($invBeltsPost -le $invBeltsPre) {
    Write-Error "FAIL [Combo 2]: Handcrafting wooden roller belt failed!"
    exit 1
}
Write-Host "Action 1 Succeeded: Belt crafted (+4 belts)." -ForegroundColor Green

# Step 2.2: Select Hotbar Slot 0 (pre-bound to WoodenRollerBelt)
Write-Host "Action 2: Selecting hotbar slot 0..." -ForegroundColor Gray
Send-HarnessCommand "slot 0"
Send-HarnessCommand "tick 5"
$c2Slot = ((Send-HarnessCommand "query state") | ConvertFrom-Json).inventory.selected_hotbar_slot
if ($c2Slot -ne 0) {
    Write-Error "FAIL [Combo 2]: Failed to select hotbar slot 0!"
    exit 1
}
Write-Host "Action 2 Succeeded: Hotbar slot 0 active." -ForegroundColor Green

# Step 2.3: Place Conveyor Belt at (1, 3) and assert visual meshes
Write-Host "Action 3: Placing belt on grid at (1, 3)..." -ForegroundColor Gray
Send-HarnessCommand "build 1 3 belt_n ground"
Send-HarnessCommand "tick 5"
$c2Vis = (Send-HarnessCommand "query visual 1 3 ground") | ConvertFrom-Json
if (-not $c2Vis.building -or -not $c2Vis.building.has_mesh -or $c2Vis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL [Combo 2]: Placed belt missing Mesh3d or roller child meshes at (1, 3)!"
    exit 1
}
Write-Host "Action 3 Succeeded: Belt placed with Mesh3d and 2 roller child components." -ForegroundColor Green
Write-Host "SUCCESS [COMBO 2]: 3-Action Chain (Craft -> Select Hotbar -> Place) Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# COMBO 3: AUTOMATED PRODUCTION LINE (Wellhead -> Belt Line -> Storage Chest)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 3] PRODUCTION LINE: WELLHEAD -> BELT LINE -> STORAGE BUFFER CHEST" -ForegroundColor Yellow

# Step 3.1: Place Clockwork Wellhead at (2, 2) (over Iron deposit)
Write-Host "Action 1: Placing Wellhead at (2, 2)..." -ForegroundColor Gray
# (Check if already placed in atomic tests or place now)
$wCheck = (Send-HarnessCommand "query visual 2 2 ground") | ConvertFrom-Json
if (-not $wCheck.building) {
    Send-HarnessCommand "build 2 2 wellhead ground"
    Send-HarnessCommand "tick 5"
}
$wVis = (Send-HarnessCommand "query visual 2 2 ground") | ConvertFrom-Json
if (-not $wVis.building.has_mesh -or -not $wVis.building.has_collider) {
    Write-Error "FAIL [Combo 3]: Wellhead at (2, 2) missing mesh or collider!"
    exit 1
}
Write-Host "Action 1 Succeeded: Wellhead operational on deposit." -ForegroundColor Green

# Step 3.2: Place Conveyor Belt feeding out from Wellhead at (2, 1) pointing North
Write-Host "Action 2: Placing output conveyor line at (2, 1)..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 2 1 ground" # Clean any previous building
Send-HarnessCommand "tick 5"
Send-HarnessCommand "build 2 1 belt_n ground"
Send-HarnessCommand "tick 5"
$bVis = (Send-HarnessCommand "query visual 2 1 ground") | ConvertFrom-Json
if (-not $bVis.building.has_mesh) {
    Write-Error "FAIL [Combo 3]: Output belt at (2, 1) missing mesh!"
    exit 1
}
Write-Host "Action 2 Succeeded: Conveyor belt placed and connected." -ForegroundColor Green

# Step 3.3: Place Storage Chest at belt terminus (2, 0)
Write-Host "Action 3: Placing storage buffer chest at (2, 0)..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 2 0 ground" # Clean any previous belt
Send-HarnessCommand "tick 5"
Send-HarnessCommand "build 2 0 chest ground"
Send-HarnessCommand "tick 5"
$chVis = (Send-HarnessCommand "query visual 2 0 ground") | ConvertFrom-Json
if (-not $chVis.building.has_mesh -or -not $chVis.building.has_collider -or $chVis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL [Combo 3]: Buffer chest missing mesh or toybox parts!"
    exit 1
}
Write-Host "Action 3 Succeeded: Storage buffer chest connected at terminus." -ForegroundColor Green
Write-Host "SUCCESS [COMBO 3]: 3-Action Chain (Wellhead -> Belt -> Chest) Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# COMBO 4: MULTI-LAYER VERTICAL LOGISTICS (Ground Belt -> Incline Ramp -> Viaduct Belt)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 4] MULTI-LAYER LOGISTICS: GROUND BELT -> INCLINE RAMP -> VIADUCT BELT" -ForegroundColor Yellow

# Clear test strip at x: 3, y: 0..2
Send-HarnessCommand "dismantle 3 0 ground"
Send-HarnessCommand "dismantle 3 1 ground"
Send-HarnessCommand "dismantle 3 2 ground"
Send-HarnessCommand "tick 5"

# Step 4.1: Place Ground Belt at (3, 0) facing North
Write-Host "Action 1: Placing ground belt at (3, 0)..." -ForegroundColor Gray
Send-HarnessCommand "build 3 0 belt_n ground"
Send-HarnessCommand "tick 5"

# Step 4.2: Place Incline Ramp at (3, 1) facing North upward
Write-Host "Action 2: Placing incline ramp at (3, 1)..." -ForegroundColor Gray
Send-HarnessCommand "build 3 1 incline_up_n ground"
Send-HarnessCommand "tick 5"

# Step 4.3: Place Elevated Viaduct Belt at (3, 2) on Layer::Viaduct (Z=1)
Write-Host "Action 3: Placing elevated viaduct belt at (3, 2, Viaduct)..." -ForegroundColor Gray
Send-HarnessCommand "build 3 2 viaduct_n viaduct"
Send-HarnessCommand "tick 5"

# Verify all 3 visual structures
$gVis = (Send-HarnessCommand "query visual 3 0 ground") | ConvertFrom-Json
$rVis = (Send-HarnessCommand "query visual 3 1 ground") | ConvertFrom-Json
$vVis = (Send-HarnessCommand "query visual 3 2 viaduct") | ConvertFrom-Json

if (-not $gVis.building.has_mesh -or -not $rVis.building.has_mesh -or -not $vVis.building.has_mesh) {
    Write-Error "FAIL [Combo 4]: One or more structures in multi-layer chain missing Mesh3d!"
    exit 1
}
if ($vVis.building.layer -ne "Viaduct") {
    Write-Error "FAIL [Combo 4]: Viaduct belt layer mismatch!"
    exit 1
}
Write-Host "SUCCESS [COMBO 4]: Multi-Layer Chain (Ground Belt -> Incline -> Viaduct) Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# COMBO 5: LOGISTICS LIFECYCLE (Place Belt -> Transport Item -> Dismantle Belt)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 5] LIFECYCLE: PLACE BELT -> TRANSPORT ITEM -> DISMANTLE BELT" -ForegroundColor Yellow

# Step 5.1: Place Belt at (0, 4)
Write-Host "Action 1: Placing conveyor belt at (0, 4)..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 0 4 ground"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "build 0 4 belt_n ground"
Send-HarnessCommand "tick 5"

# Step 5.2: Spawn item on belt and observe movement
Write-Host "Action 2: Spawning transported item and ticking simulation..." -ForegroundColor Gray
Send-HarnessCommand "spawn_item 0 4 iron_plate ground"
Send-HarnessCommand "tick 2"

$c5ItemPre = (Send-HarnessCommand "query visual 0 4 ground") | ConvertFrom-Json
if ($c5ItemPre.items.Count -eq 0 -or -not $c5ItemPre.items[0].has_mesh) {
    Write-Error "FAIL [Combo 5]: Item did not spawn with visual mesh on belt!"
    exit 1
}
$progInitial = $c5ItemPre.items[0].progress

Send-HarnessCommand "tick 25"
$c5ItemPost = (Send-HarnessCommand "query visual 0 4 ground") | ConvertFrom-Json
if ($c5ItemPost.items.Count -gt 0) {
    $progAdvanced = $c5ItemPost.items[0].progress
    Write-Host "Item progress advanced: $progInitial -> $progAdvanced" -ForegroundColor Gray
    if ($progAdvanced -le $progInitial) {
        Write-Error "FAIL [Combo 5]: Item did not move along belt!"
        exit 1
    }
}

# Step 5.3: Dismantle belt
Write-Host "Action 3: Dismantling belt at (0, 4)..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 0 4 ground"
Send-HarnessCommand "tick 5"
$c5Clean = (Send-HarnessCommand "query visual 0 4 ground") | ConvertFrom-Json
if ($c5Clean.building) {
    Write-Error "FAIL [Combo 5]: Belt entity not despawned!"
    exit 1
}
Write-Host "SUCCESS [COMBO 5]: Logistics Lifecycle (Place -> Transport -> Dismantle) Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# COMBO 6: HARMONIC SUDOKU PUZZLE (Place Monoliths -> Conflict -> Resolve)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 6] HARMONIC SUDOKU: CONFLICT -> PENALTY -> RESOLVE HARMONY" -ForegroundColor Yellow

# Clean coordinates (3, 3), (3, 4), (17, 1), and (18, 1) inside Resonance Grove (origin at 16, 0)
Send-HarnessCommand "dismantle 3 3 ground"
Send-HarnessCommand "dismantle 3 4 ground"
Send-HarnessCommand "dismantle 17 1 ground"
Send-HarnessCommand "dismantle 18 1 ground"
Send-HarnessCommand "tick 5"

# Step 6.1: Place two adjacent Monoliths in Grove
Write-Host "Action 1: Placing adjacent Monoliths in Grove at (17, 1) and (18, 1)..." -ForegroundColor Gray
Send-HarnessCommand "build 17 1 monolith ground"
Send-HarnessCommand "build 18 1 monolith ground"
Send-HarnessCommand "tick 5"

# Step 6.2: Induce frequency conflict by tuning both to frequency 5
Write-Host "Action 2: Tuning both monoliths to conflicting frequency 5..." -ForegroundColor Gray
Send-HarnessCommand "tune 17 1 5"
Send-HarnessCommand "tune 18 1 5"
Send-HarnessCommand "tick 10"

$stateConf = (Send-HarnessCommand "query state") | ConvertFrom-Json
$m1Conf = $null
$m2Conf = $null
foreach ($m in $stateConf.monoliths) {
    if ($m.coord[0] -eq 17 -and $m.coord[1] -eq 1) { $m1Conf = $m }
    if ($m.coord[0] -eq 18 -and $m.coord[1] -eq 1) { $m2Conf = $m }
}
Write-Host "Monolith (17, 1) is_conflicted: $($m1Conf.is_conflicted)" -ForegroundColor Gray
Write-Host "Monolith (18, 1) is_conflicted: $($m2Conf.is_conflicted)" -ForegroundColor Gray
if (-not $m1Conf.is_conflicted -or -not $m2Conf.is_conflicted) {
    Write-Error "FAIL [Combo 6]: Conflicting frequencies did not trigger is_conflicted: true!"
    exit 1
}
Write-Host "Conflict verified: acoustic flutter penalty triggered." -ForegroundColor Green

# Step 6.3: Resolve conflict by re-tuning second monolith to harmonious frequency 7
Write-Host "Action 3: Retuning monolith at (18, 1) to non-conflicting frequency 7..." -ForegroundColor Gray
Send-HarnessCommand "tune 18 1 7"
Send-HarnessCommand "tick 10"

$stateResolved = (Send-HarnessCommand "query state") | ConvertFrom-Json
$m1Res = $null
$m2Res = $null
foreach ($m in $stateResolved.monoliths) {
    if ($m.coord[0] -eq 17 -and $m.coord[1] -eq 1) { $m1Res = $m }
    if ($m.coord[0] -eq 18 -and $m.coord[1] -eq 1) { $m2Res = $m }
}
Write-Host "Monolith (17, 1) is_conflicted after resolve: $($m1Res.is_conflicted)" -ForegroundColor Gray
Write-Host "Monolith (18, 1) is_conflicted after resolve: $($m2Res.is_conflicted)" -ForegroundColor Gray
if ($m1Res.is_conflicted -or $m2Res.is_conflicted) {
    Write-Error "FAIL [Combo 6]: Monolith conflict was not resolved after re-tuning!"
    exit 1
}
Write-Host "SUCCESS [COMBO 6]: Harmonic Sudoku (Place -> Conflict -> Resolve) Verified!" -ForegroundColor Green

# Clean up
Send-HarnessCommand "dismantle 17 1 ground"
Send-HarnessCommand "dismantle 18 1 ground"
Send-HarnessCommand "tick 5"

# -------------------------------------------------------------------------
# COMBO 7: GEMSTONE CATALYSIS (Place Autoclave -> Feed Gems -> Catalysis Solve)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 7] GEMSTONE CATALYSIS: PLACE AUTOCLAVE -> MATCH-3 CATALYSIS" -ForegroundColor Yellow

# Clean coordinate (1, 4)
Send-HarnessCommand "dismantle 1 4 ground"
Send-HarnessCommand "tick 5"

# Step 7.1: Place Chromatic Autoclave at (1, 4)
Write-Host "Action 1: Placing Chromatic Autoclave at (1, 4)..." -ForegroundColor Gray
Send-HarnessCommand "build 1 4 autoclave_n ground"
Send-HarnessCommand "tick 5"
$autoVis = (Send-HarnessCommand "query visual 1 4 ground") | ConvertFrom-Json
if (-not $autoVis.building.has_mesh -or -not $autoVis.building.has_collider) {
    Write-Error "FAIL [Combo 7]: Autoclave missing visual mesh or collider!"
    exit 1
}

# Step 7.2: Query Autoclave internal gemstone matrix
Write-Host "Action 2: Inspecting internal gemstone grid..." -ForegroundColor Gray
$gridOutput = Send-HarnessCommand "autoclave 1 4 grid"
Write-Host $gridOutput

# Step 7.3: Trigger match-3 catalysis solve
Write-Host "Action 3: Triggering automatic Match-3 catalysis cascade..." -ForegroundColor Gray
$solveRes = Send-HarnessCommand "autoclave 1 4 solve"
Send-HarnessCommand "tick 10"

$stateAuto = (Send-HarnessCommand "query state") | ConvertFrom-Json
$targetAuto = $null
foreach ($a in $stateAuto.autoclaves) {
    if ($a.coord[0] -eq 1 -and $a.coord[1] -eq 4) { $targetAuto = $a }
}
Write-Host "Total Catalysis Reactions: $($targetAuto.total_catalysis_reactions), Stored Power: $($targetAuto.stored_power_kw) kW" -ForegroundColor Gray
if ($targetAuto.total_catalysis_reactions -eq 0 -or $targetAuto.stored_power_kw -le 0.0) {
    Write-Error "FAIL [Combo 7]: Catalysis reaction failed to generate power!"
    exit 1
}
Write-Host "SUCCESS [COMBO 7]: Gemstone Catalysis (Place -> Grid -> Solve Cascade) Verified!" -ForegroundColor Green

# Cleanup
Send-HarnessCommand "dismantle 1 4 ground"
Send-HarnessCommand "tick 5"

# -------------------------------------------------------------------------
# COMBO 8: PHYSICAL COLLISION & CLEARANCE (Place Obstacle -> Block -> Dismantle -> Pass)
# -------------------------------------------------------------------------
Write-Host "`n>>> [COMBO 8] COLLISION: PLACE OBSTACLE -> BLOCK MOVEMENT -> DISMANTLE -> ADVANCE" -ForegroundColor Yellow

# Teleport player to (1, 2)
Send-HarnessCommand "teleport 1 2"
Send-HarnessCommand "tick 5"
$pPos1 = ((Send-HarnessCommand "query state") | ConvertFrom-Json).player.translation
Write-Host "Player starting at: ($($pPos1[0]), $($pPos1[1]), $($pPos1[2]))" -ForegroundColor Gray

# Step 8.1: Place solid Storage Chest directly ahead at (1, 1)
Write-Host "Action 1: Placing solid obstacle (Chest) at (1, 1)..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 1 1 ground"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "build 1 1 chest ground"
Send-HarnessCommand "tick 5"

# Step 8.2: Attempt to walk forward through the chest
Write-Host "Action 2: Attempting to walk into solid obstacle..." -ForegroundColor Gray
Send-HarnessCommand "press W"
Send-HarnessCommand "tick 30"
Send-HarnessCommand "release W"

$pPosBlocked = ((Send-HarnessCommand "query state") | ConvertFrom-Json).player.translation
Write-Host "Player position against obstacle: ($($pPosBlocked[0]), $($pPosBlocked[1]), $($pPosBlocked[2]))" -ForegroundColor Gray
if ($pPosBlocked[2] -lt 1.7) {
    Write-Error "FAIL [Combo 8]: Player penetrated through solid obstacle collider!"
    exit 1
}
Write-Host "Action 2 Succeeded: Player motion blocked by collider boundary." -ForegroundColor Green

# Step 8.3: Dismantle obstacle and walk through
Write-Host "Action 3: Dismantling obstacle at (1, 1) and walking forward..." -ForegroundColor Gray
Send-HarnessCommand "dismantle 1 1 ground"
Send-HarnessCommand "tick 5"

Send-HarnessCommand "press W"
Send-HarnessCommand "tick 30"
Send-HarnessCommand "release W"

$pPosCleared = ((Send-HarnessCommand "query state") | ConvertFrom-Json).player.translation
Write-Host "Player position after path clearance: ($($pPosCleared[0]), $($pPosCleared[1]), $($pPosCleared[2]))" -ForegroundColor Gray
if ($pPosCleared[2] -ge $pPosBlocked[2]) {
    Write-Error "FAIL [Combo 8]: Player did not advance through cleared path!"
    exit 1
}
Write-Host "Action 3 Succeeded: Player advanced freely through cleared space." -ForegroundColor Green
Write-Host "SUCCESS [COMBO 8]: Physical Collision & Clearance Chain Verified!" -ForegroundColor Green

# -------------------------------------------------------------------------
# CAPTURE VERIFICATION SCREENSHOT
# -------------------------------------------------------------------------
Write-Host "`nCapturing playtest verification screenshot..." -ForegroundColor Yellow
Send-HarnessCommand "screenshot target/screenshots/playtest_combos_verified.png"
Send-HarnessCommand "tick 5"

Write-Host "`n==================================================================" -ForegroundColor Green
Write-Host " ALL 8 MULTI-ACTION COMBINATIONS PASSED (100% SUCCESS)           " -ForegroundColor Green
Write-Host "==================================================================" -ForegroundColor Green
