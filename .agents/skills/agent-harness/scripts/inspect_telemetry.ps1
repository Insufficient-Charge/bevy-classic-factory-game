param(
    [string]$Path = "target/agent_telemetry.jsonl",
    [int]$Tail = 30,
    [switch]$ErrorsOnly
)

if (-not (Test-Path $Path)) {
    Write-Warning "Telemetry log not found at: $Path"
    exit 0
}

$lines = Get-Content $Path -Tail $Tail
foreach ($line in $lines) {
    if ([string]::IsNullOrWhiteSpace($line)) { continue }
    try {
        $obj = $line | ConvertFrom-Json
        if ($ErrorsOnly -and $obj.level -ne "ERROR") { continue }
        
        $timestamp = $obj.timestamp
        $timeStr = if ($timestamp) {
            try {
                [DateTimeOffset]::FromUnixTimeSeconds([int64]$timestamp).ToLocalTime().ToString("HH:mm:ss.fff")
            } catch {
                "$timestamp"
            }
        } else {
            "--:--:--"
        }
        
        $lvl = $obj.level
        $target = $obj.target
        $msg = if ($obj.fields -and $obj.fields.message) {
            $obj.fields.message
        } elseif ($obj.fields) {
            ($obj.fields | ConvertTo-Json -Compress)
        } else {
            ""
        }
        
        $color = switch ($lvl) {
            "ERROR" { "Red" }
            "WARN"  { "Yellow" }
            "INFO"  { "Cyan" }
            "DEBUG" { "DarkGray" }
            default { "White" }
        }
        
        Write-Host "[$timeStr] [$lvl] ($target) $msg" -ForegroundColor $color
    } catch {
        Write-Host $line
    }
}
