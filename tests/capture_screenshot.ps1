param(
    [string]$ScreenshotPath = "target/screenshots/diorama_phase2.png"
)

$ErrorActionPreference = "Stop"

function Send-HarnessCommand {
    param([string]$Cmd)
    Write-Host ">>> SENDING: $Cmd" -ForegroundColor Cyan
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command $Cmd
    Write-Host $res
    return $res
}

Write-Host "Waiting for port 6550..." -ForegroundColor Yellow
for ($i = 0; $i -lt 30; $i++) {
    $conn = Get-NetTCPConnection -LocalPort 6550 -ErrorAction SilentlyContinue
    if ($conn) { break }
    Start-Sleep -Seconds 1
}

# 1. Wait for Screen::Gameplay
Write-Host "Waiting for Gameplay screen..." -ForegroundColor Yellow
for ($i = 0; $i -lt 30; $i++) {
    $res = powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "query state"
    if ($res -match '"screen":\s*"Gameplay"') {
        Write-Host "Entered Gameplay screen!" -ForegroundColor Green
        break
    }
    powershell -File .agents/skills/agent-harness/scripts/send_command.ps1 -Command "tick 20"
    Start-Sleep -Milliseconds 300
}

# 2. Advance 60 ticks so the 3D diorama scene and lighting finish setting up
Send-HarnessCommand "tick 60"

# 3. Unfold several tiles to show the runic slate and Minesweeper numbers
Send-HarnessCommand "prospect 3 3 unfold"
Send-HarnessCommand "prospect 4 4 unfold"
Send-HarnessCommand "prospect 5 5 flag"

# 4. Place buildings: Incline Ramp, Viaduct Belt, Cargo Compressor, Turntable
Send-HarnessCommand "build 2 2 incline_up_n ground"
Send-HarnessCommand "build 2 3 viaduct_n viaduct"
Send-HarnessCommand "build 4 3 turntable_n ground"
Send-HarnessCommand "build 4 4 compressor_n ground"

# 5. Advance 60 ticks for placement animations and 3D mesh sync
Send-HarnessCommand "tick 60"

# 6. Capture screenshot
Send-HarnessCommand "screenshot $ScreenshotPath"

# 7. Advance ticks so the render loop processes and flushes the screenshot observer to disk
Send-HarnessCommand "tick 30"
Start-Sleep -Seconds 2

# 8. Quit daemon
Send-HarnessCommand "quit"

Write-Host "Screenshot capture script finished!" -ForegroundColor Green
