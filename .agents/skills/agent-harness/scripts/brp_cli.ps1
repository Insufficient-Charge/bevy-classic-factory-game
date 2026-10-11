<#
.SYNOPSIS
    Command-line interface for Bevy Remote Protocol (BRP) JSON-RPC 2.0 API.

.DESCRIPTION
    Allows developers and LLM agents to query, inspect, and mutate Bevy ECS state
    in real time over HTTP (default: http://127.0.0.1:15702).

.EXAMPLES
    # Discover all registered BRP methods and schemas
    powershell -File brp_cli.ps1 discover

    # Query all entities with Transform and GridCoord
    powershell -File brp_cli.ps1 query -Components "bevy_transform::components::transform::Transform,bevy_classic_factory_game_2::game::grid::GridCoord"

    # List all components present on an entity
    powershell -File brp_cli.ps1 list -Entity 4294967301

    # Get specific components on an entity
    powershell -File brp_cli.ps1 get -Entity 4294967301 -Components "bevy_transform::components::transform::Transform"

    # Mutate a component value on an entity
    powershell -File brp_cli.ps1 mutate -Entity 4294967301 -Component "bevy_classic_factory_game_2::camera::diorama::DioramaCameraConfig" -ValueJson '{"rotation_step": 2}'

    # Read a global resource
    powershell -File brp_cli.ps1 get-resource -Resource "bevy_classic_factory_game_2::game::terrain::IslandGrid"

    # List all registered resources
    powershell -File brp_cli.ps1 list-resources

    # Send arbitrary JSON-RPC request
    powershell -File brp_cli.ps1 raw -RawJson '{"jsonrpc": "2.0", "method": "bevy/list_resources", "id": 1}'
#>

param(
    [Parameter(Position = 0, Mandatory = $true)]
    [ValidateSet("discover", "query", "get", "list", "spawn", "destroy", "mutate", "get-resource", "list-resources", "mutate-resource", "raw", "help")]
    [string]$Action,

    [string]$Entity,
    [string]$Component,
    [string]$Components,
    [string]$FilterWith,
    [string]$FilterWithout,
    [string]$Resource,
    [string]$ValueJson,
    [string]$RawJson,
    [string]$Url = "http://127.0.0.1:15702",
    [switch]$Compact
)

$script:RequestId = 1

function Invoke-BrpMethod {
    param(
        [string]$Method,
        [object]$Params = $null
    )

    $bodyObj = [ordered]@{
        "jsonrpc" = "2.0"
        "method"  = $Method
        "id"      = $script:RequestId++
    }
    if ($null -ne $Params) {
        $bodyObj["params"] = $Params
    }

    $jsonBody = $bodyObj | ConvertTo-Json -Depth 10 -Compress
    Invoke-BrpRaw -Payload $jsonBody
}

function Invoke-BrpRaw {
    param([string]$Payload)

    try {
        $response = Invoke-RestMethod -Uri $Url -Method Post -Body $Payload -ContentType "application/json" -TimeoutSec 10
        if (-not $Compact) {
            $response | ConvertTo-Json -Depth 10
        } else {
            $response | ConvertTo-Json -Depth 10 -Compress
        }
    } catch {
        Write-Error "BRP Request Failed ($Url): $_. Ensure the game is running with '--features agent_test'."
        exit 1
    }
}

switch ($Action.ToLower()) {
    "help" {
        Get-Help $MyInvocation.MyCommand.Path -Full
    }

    "discover" {
        Invoke-BrpMethod -Method "rpc.discover"
    }

    "list-resources" {
        Invoke-BrpMethod -Method "world.list_resources"
    }

    "get-resource" {
        if (-not $Resource) {
            Write-Error "Missing required parameter: -Resource <TypeName>"
            exit 1
        }
        Invoke-BrpMethod -Method "world.get_resources" -Params @{
            "resource" = $Resource
        }
    }

    "mutate-resource" {
        if (-not $Resource -or -not $ValueJson) {
            Write-Error "Usage: mutate-resource -Resource <TypeName> -ValueJson '<JsonString>'"
            exit 1
        }
        $valObj = $ValueJson | ConvertFrom-Json
        Invoke-BrpMethod -Method "world.mutate_resources" -Params @{
            "resource" = $Resource
            "value"    = $valObj
        }
    }

    "query" {
        $compList = New-Object System.Collections.ArrayList
        if ($Components) {
            $Components.Split(',') | ForEach-Object { [void]$compList.Add($_.Trim()) }
        }
        $withList = New-Object System.Collections.ArrayList
        if ($FilterWith) {
            $FilterWith.Split(',') | ForEach-Object { [void]$withList.Add($_.Trim()) }
        }
        $withoutList = New-Object System.Collections.ArrayList
        if ($FilterWithout) {
            $FilterWithout.Split(',') | ForEach-Object { [void]$withoutList.Add($_.Trim()) }
        }

        $params = [ordered]@{
            "data" = [ordered]@{
                "components" = $compList
            }
        }
        if ($withList.Count -gt 0 -or $withoutList.Count -gt 0) {
            $filter = [ordered]@{}
            if ($withList.Count -gt 0) { $filter["with"] = $withList }
            if ($withoutList.Count -gt 0) { $filter["without"] = $withoutList }
            $params["filter"] = $filter
        }

        Invoke-BrpMethod -Method "world.query" -Params $params
    }

    "list" {
        if (-not $Entity) {
            Write-Error "Missing required parameter: -Entity <EntityId>"
            exit 1
        }
        Invoke-BrpMethod -Method "world.list_components" -Params @{
            "entity" = [int64]$Entity
        }
    }

    "get" {
        if (-not $Entity) {
            Write-Error "Missing required parameter: -Entity <EntityId>"
            exit 1
        }
        $compList = if ($Components) { $Components.Split(',') | ForEach-Object { $_.Trim() } } else { @() }
        Invoke-BrpMethod -Method "world.get_components" -Params @{
            "entity"     = [int64]$Entity
            "components" = $compList
        }
    }

    "mutate" {
        if (-not $Entity -or -not $Component -or -not $ValueJson) {
            Write-Error "Usage: mutate -Entity <EntityId> -Component <Name> -ValueJson '<JsonString>'"
            exit 1
        }
        $valObj = $ValueJson | ConvertFrom-Json
        Invoke-BrpMethod -Method "bevy/mutate" -Params @{
            "entity"    = [int64]$Entity
            "component" = $Component
            "value"     = $valObj
        }
    }

    "spawn" {
        if (-not $ValueJson) {
            Write-Error "Usage: spawn -ValueJson '{\"components\": {\"Name\": {...}}}'"
            exit 1
        }
        $valObj = $ValueJson | ConvertFrom-Json
        Invoke-BrpMethod -Method "bevy/spawn" -Params $valObj
    }

    "destroy" {
        if (-not $Entity) {
            Write-Error "Usage: destroy -Entity <EntityId>"
            exit 1
        }
        Invoke-BrpMethod -Method "bevy/destroy" -Params @{
            "entity" = [int64]$Entity
        }
    }

    "raw" {
        if (-not $RawJson) {
            Write-Error "Usage: raw -RawJson '<JsonRpc2Payload>'"
            exit 1
        }
        Invoke-BrpRaw -Payload $RawJson
    }
}
