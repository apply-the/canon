# preflight-utils.ps1 — Shared utility functions for canon-preflight.ps1
# Part of Canon Skill Runtime Contracts (061)

Set-StrictMode -Version Latest

function Trim-Value {
    <#
    .SYNOPSIS
    Remove leading and trailing whitespace from a string.
    #>
    param([string]$Value)
    return $Value.Trim()
}

function Test-Placeholder {
    <#
    .SYNOPSIS
    Return $true if the value looks like a placeholder or is empty/whitespace.
    #>
    param([string]$Value)
    $val = $Value.Trim()
    if ([string]::IsNullOrEmpty($val)) { return $true }
    if ($val -match '^\<.*\>$') { return $true }
    if ($val -match '^\{.*\}$') { return $true }
    if ($val -eq 'TODO' -or $val -eq 'TBD') { return $true }
    return $false
}

function ConvertTo-JsonEscaped {
    <#
    .SYNOPSIS
    Escape a string for safe inclusion in JSON values.
    #>
    param([string]$Value)
    $result = $Value
    $result = $result.Replace('\', '\\')
    $result = $result.Replace('"', '\"')
    $result = $result.Replace("`n", '\n')
    $result = $result.Replace("`t", '\t')
    $result = $result.Replace("`r", '\r')
    return $result
}

function Get-IsoTimestamp {
    <#
    .SYNOPSIS
    Return current UTC timestamp in ISO 8601 format.
    #>
    return (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')
}
