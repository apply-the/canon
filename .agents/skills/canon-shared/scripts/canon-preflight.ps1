# canon-preflight.ps1 — Structured JSON preflight output for Canon skills
# Part of Canon Skill Runtime Contracts (061)
#
# Usage: canon-preflight.ps1 -Mode <mode-name>
# Output: JSON to stdout conforming to contracts/preflight-json-schema.json

[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string]$Mode = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
. "$ScriptDir/preflight-utils.ps1"

function Get-PreflightReport {
    param([string]$ModeName)

    $timestamp = Get-IsoTimestamp

    if ([string]::IsNullOrWhiteSpace($ModeName)) {
        $errorMsg = "no --mode argument provided"
        $report = [ordered]@{
            schema_version = 1
            timestamp      = $timestamp
            mode           = ""
            canon          = [ordered]@{ available = $false; version = $null; initialized = $false; error = $errorMsg }
            workspace      = [ordered]@{ path = $null; git_branch = $null; git_user = $null; error = $errorMsg }
            input          = [ordered]@{ file_exists = $false; file_path = ""; file_empty = $null; folder_exists = $false; folder_path = ""; folder_empty = $null; resolved_path = $null; ambiguous = $false; error = $errorMsg }
            runs           = [ordered]@{ active = $null; pending_approvals = $null; error = $errorMsg }
        }
        $report | ConvertTo-Json -Depth 4
        exit 1
    }

    # --- Canon section ---
    $canonAvailable = $false
    $canonVersion = $null
    $canonInitialized = $false
    $canonError = $null

    $canonCmd = Get-Command "canon" -ErrorAction SilentlyContinue
    if ($canonCmd) {
        $canonAvailable = $true
        try {
            $versionOutput = & canon --version 2>$null
            if ($versionOutput -match '(\d+\.\d+\.\d+)') {
                $canonVersion = $Matches[1]
            }
            else {
                $canonError = "could not parse canon version"
            }
        }
        catch {
            $canonError = "could not parse canon version"
        }
    }
    else {
        $canonError = "canon binary not found on PATH"
    }

    if (Test-Path ".canon" -PathType Container) {
        $canonInitialized = $true
    }

    # --- Workspace section ---
    $workspacePath = (Get-Location).Path
    $gitBranch = $null
    $gitUser = $null
    $workspaceError = $null

    $gitCmd = Get-Command "git" -ErrorAction SilentlyContinue
    if ($gitCmd) {
        try {
            $null = & git rev-parse --git-dir 2>$null
            if ($LASTEXITCODE -eq 0) {
                $gitBranch = (& git symbolic-ref --short HEAD 2>$null)
                if ($LASTEXITCODE -ne 0) {
                    $gitBranch = (& git rev-parse --short HEAD 2>$null)
                }
                $gitUser = (& git config user.email 2>$null)
            }
            else {
                $workspaceError = "git not available or not a git repository"
            }
        }
        catch {
            $workspaceError = "git not available or not a git repository"
        }
    }
    else {
        $workspaceError = "git not available or not a git repository"
    }

    # --- Input section ---
    $inputFilePath = "canon-input/${ModeName}.md"
    $inputFolderPath = "canon-input/${ModeName}/"
    $inputFileExists = $false
    $inputFileEmpty = $null
    $inputFolderExists = $false
    $inputFolderEmpty = $null
    $inputResolvedPath = $null
    $inputAmbiguous = $false
    $inputError = $null

    if (Test-Path $inputFilePath -PathType Leaf) {
        $inputFileExists = $true
        $fileInfo = Get-Item $inputFilePath
        $inputFileEmpty = ($fileInfo.Length -eq 0)
    }

    if (Test-Path $inputFolderPath -PathType Container) {
        $inputFolderExists = $true
        $files = Get-ChildItem -Path $inputFolderPath -File -ErrorAction SilentlyContinue
        $inputFolderEmpty = ($files.Count -eq 0)
    }

    # File-first resolution per C-003
    if ($inputFileExists) {
        $inputResolvedPath = $inputFilePath
        if ($inputFolderExists) {
            $inputAmbiguous = $true
        }
    }
    elseif ($inputFolderExists) {
        $inputResolvedPath = $inputFolderPath
    }

    if (-not $inputFileExists -and -not $inputFolderExists) {
        $inputError = "unknown mode: ${ModeName}"
    }

    # --- Runs section ---
    $runsActive = $null
    $runsPending = $null
    $runsError = $null

    if ($canonAvailable -and (Test-Path ".canon/runs" -PathType Container)) {
        $runsActive = 0
        $runsPending = 0
        $runDirs = Get-ChildItem -Path ".canon/runs" -Directory -ErrorAction SilentlyContinue
        foreach ($runDir in $runDirs) {
            $manifest = Join-Path $runDir.FullName "manifest.toml"
            if (Test-Path $manifest) {
                $runsActive++
                $content = Get-Content $manifest -Raw -ErrorAction SilentlyContinue
                if ($content -match 'state.*=.*"AwaitingApproval"') {
                    $runsPending++
                }
            }
        }
    }
    elseif (-not $canonAvailable) {
        $runsError = "canon not available; cannot query runs"
    }
    elseif (-not (Test-Path ".canon/runs" -PathType Container)) {
        $runsActive = 0
        $runsPending = 0
    }

    # --- JSON assembly ---
    $report = [ordered]@{
        schema_version = 1
        timestamp      = $timestamp
        mode           = $ModeName
        canon          = [ordered]@{
            available   = $canonAvailable
            version     = $canonVersion
            initialized = $canonInitialized
            error       = $canonError
        }
        workspace      = [ordered]@{
            path       = $workspacePath
            git_branch = $gitBranch
            git_user   = $gitUser
            error      = $workspaceError
        }
        input          = [ordered]@{
            file_exists   = $inputFileExists
            file_path     = $inputFilePath
            file_empty    = $inputFileEmpty
            folder_exists = $inputFolderExists
            folder_path   = $inputFolderPath
            folder_empty  = $inputFolderEmpty
            resolved_path = $inputResolvedPath
            ambiguous     = $inputAmbiguous
            error         = $inputError
        }
        runs           = [ordered]@{
            active            = $runsActive
            pending_approvals = $runsPending
            error             = $runsError
        }
    }

    $report | ConvertTo-Json -Depth 4
}

Get-PreflightReport -ModeName $Mode
