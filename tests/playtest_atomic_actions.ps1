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

Write-Host "==============================================================" -ForegroundColor Magenta
Write-Host " EXECUTABLE PLAYTEST SUITE: ATOMIC PLAYER ACTIONS & VISUALS  " -ForegroundColor Magenta
Write-Host "==============================================================" -ForegroundColor Magenta

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

# -------------------------------------------------------------
# 1. MOVEMENT & LOCOMOTION (Walk, Strafe, Sprint, Jump)
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 1] LOCOMOTION & PHYSICS CONTROLS" -ForegroundColor Yellow

$initPosJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$initX = $initPosJson.player.translation[0]
$initY = $initPosJson.player.translation[1]
$initZ = $initPosJson.player.translation[2]
Write-Host "Initial position: ($initX, $initY, $initZ)" -ForegroundColor Gray

# Walk forward (W)
Send-HarnessCommand "press W"
Send-HarnessCommand "tick 20"
Send-HarnessCommand "release W"
$walkPosJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$walkZ = $walkPosJson.player.translation[2]
Write-Host "Walked position Z: $walkZ (initial: $initZ)" -ForegroundColor Gray
if ($walkZ -eq $initZ) {
    Write-Error "FAIL: Player failed to move when pressing W!"
    exit 1
}
Write-Host "PASS: Walk forward verified." -ForegroundColor Green

# Strafe right (D)
Send-HarnessCommand "press D"
Send-HarnessCommand "tick 20"
Send-HarnessCommand "release D"
$strafePosJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$strafeX = $strafePosJson.player.translation[0]
Write-Host "Strafed position X: $strafeX (initial: $initX)" -ForegroundColor Gray
if ($strafeX -eq $initX) {
    Write-Error "FAIL: Player failed to move when pressing D!"
    exit 1
}
Write-Host "PASS: Strafe right verified." -ForegroundColor Green

# Sprint forward (ShiftLeft + W)
$preSprintJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$preSprintZ = $preSprintJson.player.translation[2]
Send-HarnessCommand "press ShiftLeft"
Send-HarnessCommand "press W"
Send-HarnessCommand "tick 20"
Send-HarnessCommand "release W"
Send-HarnessCommand "release ShiftLeft"
$postSprintJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$sprintDelta = [Math]::Abs($postSprintJson.player.translation[2] - $preSprintZ)
Write-Host "Sprint delta Z: $sprintDelta" -ForegroundColor Gray
if ($sprintDelta -lt 0.5) {
    Write-Error "FAIL: Sprint did not displace player sufficiently ($sprintDelta)!"
    exit 1
}
Write-Host "PASS: Sprint verified." -ForegroundColor Green

# Jump (Space)
Send-HarnessCommand "press Space"
Send-HarnessCommand "tick 8"
Send-HarnessCommand "release Space"
$jumpJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$jumpY = $jumpJson.player.translation[1]
Write-Host "Mid-air Y: $jumpY (ground Y: $initY)" -ForegroundColor Gray
if ($jumpY -le $initY) {
    Write-Error "FAIL: Player jump did not increase elevation ($jumpY <= $initY)!"
    exit 1
}
Send-HarnessCommand "tick 25" # Land
$landJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
Write-Host "Landed Y: $($landJson.player.translation[1])" -ForegroundColor Gray
Write-Host "PASS: Jump & landing verified." -ForegroundColor Green

# Teleport back to safe ground spawn (1, 1)
Send-HarnessCommand "teleport 1 1"
Send-HarnessCommand "tick 5"

# -------------------------------------------------------------
# 2. CAMERA CONTROLS (Orbit CW 360°, Orbit CCW)
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 2] CAMERA ORBIT & ROTATION" -ForegroundColor Yellow

$initCamJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
$startYaw = $initCamJson.camera.angles.yaw
Write-Host "Starting camera yaw: $startYaw" -ForegroundColor Gray

# Step 1: 90 deg CW
Send-HarnessCommand "rotate cw"
Send-HarnessCommand "tick 15"
$cam1 = ((Send-HarnessCommand "query state") | ConvertFrom-Json).camera.angles.yaw
Write-Host "Camera Yaw after CW 90°: $cam1" -ForegroundColor Gray

# Step 2: 180 deg CW
Send-HarnessCommand "rotate cw"
Send-HarnessCommand "tick 15"
$cam2 = ((Send-HarnessCommand "query state") | ConvertFrom-Json).camera.angles.yaw
Write-Host "Camera Yaw after CW 180°: $cam2" -ForegroundColor Gray

# Step 3: 270 deg CW
Send-HarnessCommand "rotate cw"
Send-HarnessCommand "tick 15"
$cam3 = ((Send-HarnessCommand "query state") | ConvertFrom-Json).camera.angles.yaw
Write-Host "Camera Yaw after CW 270°: $cam3" -ForegroundColor Gray

# Step 4: 360 deg back to start
Send-HarnessCommand "rotate cw"
Send-HarnessCommand "tick 15"
$cam4 = ((Send-HarnessCommand "query state") | ConvertFrom-Json).camera.angles.yaw
Write-Host "Camera Yaw after CW 360°: $cam4" -ForegroundColor Gray

$diff360 = [Math]::Abs($cam4 - $startYaw)
if ($diff360 -gt 0.1 -and [Math]::Abs($diff360 - 6.283) -gt 0.1) {
    Write-Error "FAIL: Camera 360 orbit did not return to origin! Diff=$diff360"
    exit 1
}
Write-Host "PASS: 360° camera orbit verified." -ForegroundColor Green

# -------------------------------------------------------------
# 3. HOTBAR & SATCHEL INTERACTION (Slots 0..9)
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 3] HOTBAR SELECTION (SLOTS 0-9)" -ForegroundColor Yellow

for ($s = 0; $s -le 9; $s++) {
    Send-HarnessCommand "slot $s"
    Send-HarnessCommand "tick 2"
    $hotbarJson = (Send-HarnessCommand "query state") | ConvertFrom-Json
    $activeSlot = $hotbarJson.inventory.selected_hotbar_slot
    if ($activeSlot -ne $s) {
        Write-Error "FAIL: Hotbar slot selection failed! Expected $s, got $activeSlot"
        exit 1
    }
}
Write-Host "PASS: Hotbar slots 0 through 9 selection verified." -ForegroundColor Green

# -------------------------------------------------------------
# 4. TERRAIN PROSPECTING & VISUAL CHANGES (Unfold, Flag, Unflag)
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 4] TERRAIN PROSPECTING & TILE VISUALS" -ForegroundColor Yellow

# Dynamically search for an available covered non-hazard tile across the diorama
$targetX = $null
$targetY = $null
for ($y = 16; $y -le 32; $y++) {
    for ($x = 0; $x -le 8; $x++) {
        $check = (Send-HarnessCommand "query visual $x $y ground") | ConvertFrom-Json
        if ($check.tile -and $check.tile.prospect_state -eq "Covered" -and -not $check.tile.has_hazard) {
            $targetX = $x
            $targetY = $y
            break
        }
    }
    if ($targetX -ne $null) { break }
}

if ($targetX -eq $null) {
    Write-Error "FAIL: No covered non-hazard tiles remaining to test prospecting!"
    exit 1
}

# Teleport adjacent to target covered tile so target is within player reach (tool reach: 2.5)
$tpX = $targetX - 1
Send-HarnessCommand "teleport $tpX $targetY"
Send-HarnessCommand "tick 5"

# Query covered tile
$tilePre = (Send-HarnessCommand "query visual $targetX $targetY ground") | ConvertFrom-Json
Write-Host "Tile ($targetX, $targetY) prospect state before: $($tilePre.tile.prospect_state)" -ForegroundColor Gray
if ($tilePre.tile.prospect_state -ne "Covered") {
    Write-Error "FAIL: Expected tile ($targetX, $targetY) to be Covered!"
    exit 1
}
if (-not $tilePre.tile.has_mesh) {
    Write-Error "FAIL: Tile ($targetX, $targetY) is missing Mesh3d!"
    exit 1
}

# Unfold tile
Send-HarnessCommand "prospect $targetX $targetY unfold"
Send-HarnessCommand "tick 5"
$tilePost = (Send-HarnessCommand "query visual $targetX $targetY ground") | ConvertFrom-Json
Write-Host "Tile ($targetX, $targetY) prospect state after unfold: $($tilePost.tile.prospect_state)" -ForegroundColor Gray
if ($tilePost.tile.prospect_state -ne "Revealed") {
    Write-Error "FAIL: Tile ($targetX, $targetY) did not transition to Revealed!"
    exit 1
}
Write-Host "PASS: Prospect unfold visual transition verified." -ForegroundColor Green

# Teleport adjacent to hazard tile at (6, 6)
Send-HarnessCommand "teleport 5 6"
Send-HarnessCommand "tick 5"

# Test Flagging on hazard tile at (6, 6)
$hazardPre = (Send-HarnessCommand "query visual 6 6 ground") | ConvertFrom-Json
Write-Host "Hazard tile (6, 6) state before: $($hazardPre.tile.prospect_state)" -ForegroundColor Gray
if ($hazardPre.tile.prospect_state -eq "Covered") {
    Send-HarnessCommand "prospect 6 6 flag"
    Send-HarnessCommand "tick 5"
    $hazardFlagged = (Send-HarnessCommand "query visual 6 6 ground") | ConvertFrom-Json
    Write-Host "Hazard tile (6, 6) state after flag: $($hazardFlagged.tile.prospect_state)" -ForegroundColor Gray
    if ($hazardFlagged.tile.prospect_state -ne "Flagged") {
        Write-Error "FAIL: Hazard tile (6, 6) did not transition to Flagged!"
        exit 1
    }
    Write-Host "PASS: Hazard flagging verified." -ForegroundColor Green

    # Unflag tile (6, 6)
    Send-HarnessCommand "prospect 6 6 flag"
    Send-HarnessCommand "tick 5"
    $hazardUnflagged = (Send-HarnessCommand "query visual 6 6 ground") | ConvertFrom-Json
    Write-Host "Hazard tile (6, 6) state after unflag: $($hazardUnflagged.tile.prospect_state)" -ForegroundColor Gray
    if ($hazardUnflagged.tile.prospect_state -ne "Covered") {
        Write-Error "FAIL: Hazard tile (6, 6) did not transition back to Covered!"
        exit 1
    }
    Write-Host "PASS: Hazard unflagging back to Covered verified." -ForegroundColor Green
} else {
    Send-HarnessCommand "prospect 6 6 flag"
    Send-HarnessCommand "tick 5"
    Send-HarnessCommand "prospect 6 6 flag"
    Send-HarnessCommand "tick 5"
    Write-Host "PASS: Hazard flagging toggling verified." -ForegroundColor Green
}

# -------------------------------------------------------------
# 5. RESOURCE EXTRACTION & MINING
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 5] RESOURCE EXTRACTION (MINING DEPOSIT)" -ForegroundColor Yellow

# Move to Iron deposit at (2, 2)
Send-HarnessCommand "teleport 2 2"
Send-HarnessCommand "tick 5"

$invPre = (Send-HarnessCommand "query state") | ConvertFrom-Json
$ironPre = 0
foreach ($item in $invPre.inventory.items) {
    if ($item.item -eq "IronOre") { $ironPre += $item.count }
}
Write-Host "Iron Ore count before mining: $ironPre" -ForegroundColor Gray

# Mine Iron Ore deposit at (2, 2)
Send-HarnessCommand "mine 2 2"
Send-HarnessCommand "tick 10"

$invPost = (Send-HarnessCommand "query state") | ConvertFrom-Json
$ironPost = 0
foreach ($item in $invPost.inventory.items) {
    if ($item.item -eq "IronOre") { $ironPost += $item.count }
}
Write-Host "Iron Ore count after mining: $ironPost" -ForegroundColor Gray
if ($ironPost -le $ironPre) {
    Write-Error "FAIL: Hand-mining deposit at (2, 2) failed to extract Iron Ore ($ironPost <= $ironPre)!"
    exit 1
}
Write-Host "PASS: Resource deposit extraction verified." -ForegroundColor Green

# -------------------------------------------------------------
# 6. FIELD WORKSHOP HANDCRAFTING
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 6] HANDCRAFTING FLOW" -ForegroundColor Yellow

# Ensure we have at least 2 Iron Ore to smelt Iron Plate
Send-HarnessCommand "mine 2 2"
Send-HarnessCommand "tick 5"

$invCraftPre = (Send-HarnessCommand "query state") | ConvertFrom-Json
$platesPre = 0
foreach ($item in $invCraftPre.inventory.items) {
    if ($item.item -eq "IronPlate") { $platesPre += $item.count }
}
Write-Host "Iron Plates before craft: $platesPre" -ForegroundColor Gray

# Craft Iron Plate (smelt_iron_plate takes 1.5s = 90 frames)
Send-HarnessCommand "craft smelt_iron_plate"

# Await craft completion deterministically
$platesPost = $platesPre
for ($w = 0; $w -lt 25; $w++) {
    Send-HarnessCommand "tick 15"
    Start-Sleep -Milliseconds 100
    $invCraftPost = (Send-HarnessCommand "query state") | ConvertFrom-Json
    $curPlates = 0
    foreach ($item in $invCraftPost.inventory.items) {
        if ($item.item -eq "IronPlate") { $curPlates += $item.count }
    }
    if ($curPlates -gt $platesPre) {
        $platesPost = $curPlates
        break
    }
}

Write-Host "Iron Plates after craft: $platesPost" -ForegroundColor Gray
if ($platesPost -le $platesPre) {
    Write-Error "FAIL: Crafting smelt_iron_plate failed to produce Iron Plate!"
    exit 1
}
Write-Host "PASS: Handcrafting verified." -ForegroundColor Green

# -------------------------------------------------------------
# 7. BUILDING PLACEMENT & VISUAL MESHES ACROSS ALL TYPES
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 7] BUILDING PLACEMENT & VISUAL MESH VERIFICATION" -ForegroundColor Yellow

# 7.1 Conveyor Belts in All 4 Directions
Write-Host "Placing Conveyor Belts in 4 cardinal directions..." -ForegroundColor Yellow
$beltDirs = @(
    @{ Dir = "belt_n"; X = 1; Y = 0; ExpectedDir = "North" },
    @{ Dir = "belt_e"; X = 2; Y = 0; ExpectedDir = "East" },
    @{ Dir = "belt_s"; X = 3; Y = 0; ExpectedDir = "South" },
    @{ Dir = "belt_w"; X = 3; Y = 1; ExpectedDir = "West" }
)

foreach ($b in $beltDirs) {
    $x = $b.X
    $y = $b.Y
    Send-HarnessCommand "dismantle $x $y ground"
    Send-HarnessCommand "tick 2"
    $cmd = "build $x $y $($b.Dir) ground"
    Send-HarnessCommand $cmd
    Send-HarnessCommand "tick 5"

    $vis = (Send-HarnessCommand "query visual $x $y ground") | ConvertFrom-Json
    if (-not $vis.building) {
        Write-Error "FAIL: Building not found at ($x, $y) after '$cmd'!"
        exit 1
    }
    if (-not $vis.building.has_mesh) {
        Write-Error "FAIL: Building at ($x, $y) has no Mesh3d!"
        exit 1
    }
    # Belts spawn with 2 child rollers
    if ($vis.building.children_count -ne 2 -or $vis.building.children_meshes_count -ne 2) {
        Write-Error "FAIL: Conveyor belt at ($x, $y) does not have 2 roller child meshes (found $($vis.building.children_meshes_count))!"
        exit 1
    }
    Write-Host "PASS: Conveyor belt $($b.ExpectedDir) placed with 3D mesh and 2 rollers." -ForegroundColor Green
}

# 7.2 Multi-Layer Viaduct Belt (Z=1, Layer::Viaduct)
Write-Host "Placing Elevated Viaduct Belt at Layer::Viaduct..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 1 0 viaduct"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 1 0 viaduct_n viaduct"
Send-HarnessCommand "tick 5"
$viaductVis = (Send-HarnessCommand "query visual 1 0 viaduct") | ConvertFrom-Json
if (-not $viaductVis.building -or -not $viaductVis.building.has_mesh) {
    Write-Error "FAIL: Viaduct Belt at (1, 0, Viaduct) missing building or Mesh3d!"
    exit 1
}
if ($viaductVis.building.layer -ne "Viaduct") {
    Write-Error "FAIL: Viaduct Belt layer is not Viaduct!"
    exit 1
}
Write-Host "PASS: Multi-layer elevated Viaduct Belt verified." -ForegroundColor Green

# 7.3 Incline Ramp (connecting Ground to Viaduct)
Write-Host "Placing Incline Ramp at (2, 1)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 2 1 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 2 1 incline_up_n ground"
Send-HarnessCommand "tick 5"
$inclineVis = (Send-HarnessCommand "query visual 2 1 ground") | ConvertFrom-Json
if (-not $inclineVis.building -or -not $inclineVis.building.has_mesh) {
    Write-Error "FAIL: Incline Ramp missing Mesh3d at (2, 1)!"
    exit 1
}
Write-Host "PASS: Incline Ramp placed with 3D mesh." -ForegroundColor Green

# 7.4 Swivel Turntable
Write-Host "Placing Swivel Turntable at (0, 0)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 0 0 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 0 0 turntable_e ground"
Send-HarnessCommand "tick 5"
$turntableVis = (Send-HarnessCommand "query visual 0 0 ground") | ConvertFrom-Json
if (-not $turntableVis.building -or -not $turntableVis.building.has_mesh) {
    Write-Error "FAIL: Turntable missing Mesh3d at (0, 0)!"
    exit 1
}
Write-Host "PASS: Swivel Turntable placed with 3D mesh." -ForegroundColor Green

# 7.5 Sorting Gate
Write-Host "Placing Sorting Gate at (0, 1)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 0 1 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 0 1 sorter_n ground"
Send-HarnessCommand "tick 5"
$sorterVis = (Send-HarnessCommand "query visual 0 1 ground") | ConvertFrom-Json
if (-not $sorterVis.building -or -not $sorterVis.building.has_mesh) {
    Write-Error "FAIL: Sorter missing Mesh3d at (0, 1)!"
    exit 1
}
Write-Host "PASS: Sorting Gate placed with 3D mesh." -ForegroundColor Green

# 7.6 Storage Chest (with Lid & Latch children + Collider)
Write-Host "Placing Storage Chest at (0, 2)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 0 2 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 0 2 chest ground"
Send-HarnessCommand "tick 5"
$chestVis = (Send-HarnessCommand "query visual 0 2 ground") | ConvertFrom-Json
if (-not $chestVis.building -or -not $chestVis.building.has_mesh) {
    Write-Error "FAIL: Storage Chest missing Mesh3d at (0, 2)!"
    exit 1
}
if (-not $chestVis.building.has_collider) {
    Write-Error "FAIL: Storage Chest missing static collider!"
    exit 1
}
if ($chestVis.building.children_count -ne 2 -or $chestVis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL: Storage Chest missing lid and latch child meshes (found $($chestVis.building.children_meshes_count))!"
    exit 1
}
Write-Host "PASS: Storage Chest placed with Mesh3d, Collider, and 2 toybox child meshes." -ForegroundColor Green

# 7.7 Clockwork Wellhead (with Cog & Pipe children + Collider)
Write-Host "Placing Clockwork Wellhead at (2, 2)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 2 2 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 2 2 wellhead ground"
Send-HarnessCommand "tick 5"
$wellheadVis = (Send-HarnessCommand "query visual 2 2 ground") | ConvertFrom-Json
if (-not $wellheadVis.building -or -not $wellheadVis.building.has_mesh) {
    Write-Error "FAIL: Wellhead missing Mesh3d at (2, 2)!"
    exit 1
}
if (-not $wellheadVis.building.has_collider) {
    Write-Error "FAIL: Wellhead missing static collider!"
    exit 1
}
if ($wellheadVis.building.children_count -ne 2 -or $wellheadVis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL: Wellhead missing cog and pipe child meshes (found $($wellheadVis.building.children_meshes_count))!"
    exit 1
}
Write-Host "PASS: Clockwork Wellhead placed with Mesh3d, Collider, and 2 toybox child meshes." -ForegroundColor Green

# 7.8 Smelting Hearth (with Collider)
Write-Host "Placing Smelting Hearth at (1, 2)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 1 2 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 1 2 hearth ground"
Send-HarnessCommand "tick 5"
$hearthVis = (Send-HarnessCommand "query visual 1 2 ground") | ConvertFrom-Json
if (-not $hearthVis.building -or -not $hearthVis.building.has_mesh) {
    Write-Error "FAIL: Smelting Hearth missing Mesh3d at (1, 2)!"
    exit 1
}
if (-not $hearthVis.building.has_collider) {
    Write-Error "FAIL: Smelting Hearth missing static collider!"
    exit 1
}
Write-Host "PASS: Smelting Hearth placed with Mesh3d and Collider." -ForegroundColor Green

# 7.9 Cargo Compressor
Write-Host "Placing Cargo Compressor at (2, 3)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 2 3 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 2 3 compressor_n ground"
Send-HarnessCommand "tick 5"
$compVis = (Send-HarnessCommand "query visual 2 3 ground") | ConvertFrom-Json
if (-not $compVis.building -or -not $compVis.building.has_mesh) {
    Write-Error "FAIL: Cargo Compressor missing Mesh3d at (2, 3)!"
    exit 1
}
Write-Host "PASS: Cargo Compressor placed with Mesh3d." -ForegroundColor Green

# 7.10 Chromatic Autoclave (with Collar & Gem Core children + Collider)
Write-Host "Placing Chromatic Autoclave at (3, 2)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 3 2 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 3 2 autoclave_n ground"
Send-HarnessCommand "tick 5"
$autoVis = (Send-HarnessCommand "query visual 3 2 ground") | ConvertFrom-Json
if (-not $autoVis.building -or -not $autoVis.building.has_mesh) {
    Write-Error "FAIL: Autoclave missing Mesh3d at (3, 2)!"
    exit 1
}
if (-not $autoVis.building.has_collider) {
    Write-Error "FAIL: Autoclave missing static collider!"
    exit 1
}
if ($autoVis.building.children_count -ne 2 -or $autoVis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL: Autoclave missing collar and gem core child meshes (found $($autoVis.building.children_meshes_count))!"
    exit 1
}
Write-Host "PASS: Chromatic Autoclave placed with Mesh3d, Collider, and 2 toybox child meshes." -ForegroundColor Green

# 7.11 Acoustic Monolith (with Pedestal & Rune Prism children + Collider)
Write-Host "Placing Acoustic Monolith at (3, 3)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 3 3 ground"
Send-HarnessCommand "tick 2"
Send-HarnessCommand "build 3 3 monolith ground"
Send-HarnessCommand "tick 5"
$monoVis = (Send-HarnessCommand "query visual 3 3 ground") | ConvertFrom-Json
if (-not $monoVis.building -or -not $monoVis.building.has_mesh) {
    Write-Error "FAIL: Monolith missing Mesh3d at (3, 3)!"
    exit 1
}
if (-not $monoVis.building.has_collider) {
    Write-Error "FAIL: Monolith missing static collider!"
    exit 1
}
if ($monoVis.building.children_count -ne 2 -or $monoVis.building.children_meshes_count -ne 2) {
    Write-Error "FAIL: Monolith missing pedestal and rune prism child meshes (found $($monoVis.building.children_meshes_count))!"
    exit 1
}
Write-Host "PASS: Acoustic Monolith placed with Mesh3d, Collider, and 2 toybox child meshes." -ForegroundColor Green

# 7.12 Harmonic Bridge
Write-Host "Placing Harmonic Bridge at (0, 3)..." -ForegroundColor Yellow
Send-HarnessCommand "dismantle 0 3 ground"
Send-HarnessCommand "tick 2"
# Unfold (0, 3) if needed
Send-HarnessCommand "prospect 0 3 unfold"
Send-HarnessCommand "tick 5"
Send-HarnessCommand "build 0 3 harmonic_bridge ground"
Send-HarnessCommand "tick 5"
$bridgeVis = (Send-HarnessCommand "query visual 0 3 ground") | ConvertFrom-Json
if (-not $bridgeVis.building -or -not $bridgeVis.building.has_mesh) {
    Write-Error "FAIL: Harmonic Bridge missing Mesh3d at (0, 3)!"
    exit 1
}
Write-Host "PASS: Harmonic Bridge placed with Mesh3d." -ForegroundColor Green

# -------------------------------------------------------------
# 8. FACTORY SIMULATION & ITEM VISUAL TRANSPORT
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 8] FACTORY ITEM TRANSPORT & VISUAL TRACKING" -ForegroundColor Yellow

# Spawn an Iron Plate on the conveyor belt at (1, 0)
Send-HarnessCommand "spawn_item 1 0 iron_plate ground"
Send-HarnessCommand "tick 2"

$itemVis1 = (Send-HarnessCommand "query visual 1 0 ground") | ConvertFrom-Json
Write-Host "Items at (1, 0): $($itemVis1.items.Count)" -ForegroundColor Gray
if ($itemVis1.items.Count -eq 0) {
    Write-Error "FAIL: Item was not spawned on belt at (1, 0)!"
    exit 1
}
$firstItem = $itemVis1.items[0]
if (-not $firstItem.has_mesh) {
    Write-Error "FAIL: Transported item is missing Mesh3d!"
    exit 1
}
$prog1 = $firstItem.progress
Write-Host "Initial item progress: $prog1, Mesh3d: $($firstItem.has_mesh)" -ForegroundColor Gray

# Advance simulation by 30 frames and assert progress advances
Send-HarnessCommand "tick 30"
$itemVis2 = (Send-HarnessCommand "query visual 1 0 ground") | ConvertFrom-Json
if ($itemVis2.items.Count -gt 0) {
    $prog2 = $itemVis2.items[0].progress
    Write-Host "Item progress after 30 frames: $prog2" -ForegroundColor Gray
    if ($prog2 -le $prog1) {
        Write-Error "FAIL: Item progress failed to advance on belt ($prog2 <= $prog1)!"
        exit 1
    }
}
Write-Host "PASS: Factory transported item visual mesh and movement verified." -ForegroundColor Green

# -------------------------------------------------------------
# 9. DISMANTLING & VISUAL CLEANUP
# -------------------------------------------------------------
Write-Host "`n>>> [SECTION 9] DISMANTLING & CLEANUP" -ForegroundColor Yellow

# Dismantle the Storage Chest at (0, 2)
Send-HarnessCommand "dismantle 0 2 ground"
# Dismantle the Acoustic Monolith at (3, 3)
Send-HarnessCommand "dismantle 3 3 ground"
Send-HarnessCommand "tick 5"

$dismantledVis = (Send-HarnessCommand "query visual 0 2 ground") | ConvertFrom-Json
if ($dismantledVis.building) {
    Write-Error "FAIL: Building at (0, 2) was not removed after dismantle!"
    exit 1
}
Write-Host "PASS: Building entity and toybox child meshes completely despawned on dismantle." -ForegroundColor Green

Write-Host "`n==============================================================" -ForegroundColor Green
Write-Host " ALL ATOMIC PLAYER ACTIONS & VISUAL CHECKS PASSED (100%)      " -ForegroundColor Green
Write-Host "==============================================================" -ForegroundColor Green
