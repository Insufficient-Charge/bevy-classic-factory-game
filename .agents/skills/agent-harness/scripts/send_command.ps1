param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$Command,
    [string]$HostName = "127.0.0.1",
    [int]$Port = 6550,
    [int]$TimeoutSeconds = 5,
    [switch]$Raw
)

$client = New-Object System.Net.Sockets.TcpClient
try {
    $asyncResult = $client.BeginConnect($HostName, $Port, $null, $null)
    $success = $asyncResult.AsyncWaitHandle.WaitOne([TimeSpan]::FromSeconds($TimeoutSeconds))
    if (-not $success -or -not $client.Connected) {
        Write-Error "Failed to connect to ${HostName}:${Port} within ${TimeoutSeconds}s. Ensure the game is running with '--features agent_test'."
        exit 1
    }
    $client.EndConnect($asyncResult)

    $stream = $client.GetStream()
    $utf8NoBom = New-Object System.Text.UTF8Encoding $false
    $writer = New-Object System.IO.StreamWriter($stream, $utf8NoBom)
    $writer.AutoFlush = $true
    $reader = New-Object System.IO.StreamReader($stream, $utf8NoBom)

    $writer.WriteLine($Command)
    $response = $reader.ReadLine()

    if (-not $Raw -and $response -match '^\{.*\}$') {
        try {
            $json = $response | ConvertFrom-Json
            $json | ConvertTo-Json -Depth 6
        } catch {
            $response
        }
    } else {
        $response
    }
} catch {
    Write-Error $_.Exception.Message
    exit 1
} finally {
    if ($client) {
        $client.Close()
    }
}
