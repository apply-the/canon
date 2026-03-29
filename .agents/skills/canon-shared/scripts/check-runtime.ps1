param(
  [string]$Command = "",
  [string]$RepoRoot = (Get-Location).Path,
  [switch]$RequireInit,
  [string]$RunId = "",
  [string]$Owner = "",
  [string]$Risk = "",
  [string]$Zone = "",
  [Alias("Input")]
  [string[]]$InputPath = @(),
  [Alias("Ref")]
  [string[]]$RefName = @()
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$CompatFile = Join-Path (Join-Path $ScriptDir "..") "references/runtime-compatibility.toml"
$CompatText = Get-Content -Raw $CompatFile
$InstallCommand = [regex]::Match($CompatText, 'install_command = "(.*)"').Groups[1].Value
$ExpectedVersion = [regex]::Match($CompatText, 'expected_workspace_version = "(.*)"').Groups[1].Value

function Emit-Failure {
  param([string]$Status, [int]$Code, [string]$Message, [string]$Action)
  Write-Output "STATUS=$Status"
  Write-Output "CODE=$Code"
  Write-Output "MESSAGE=$Message"
  Write-Output "ACTION=$Action"
  exit $Code
}

$CanonCommand = Get-Command canon -ErrorAction SilentlyContinue
if (-not $CanonCommand) {
  Emit-Failure "cli-missing" 10 "Canon CLI is not installed or is not on PATH." "Install it with: $InstallCommand"
}

$DetectedVersion = "unavailable"
$VersionKind = "command-contract"
try {
  $VersionOutput = & canon --version 2>$null
  if ($LASTEXITCODE -eq 0 -and $VersionOutput) {
    $VersionKind = "semver"
    $DetectedVersion = ($VersionOutput -split ' ')[1]
    if ($ExpectedVersion -and $DetectedVersion -ne $ExpectedVersion) {
      Emit-Failure "version-incompatible" 11 "Detected Canon version $DetectedVersion, expected $ExpectedVersion." "Reinstall Canon with: $InstallCommand"
    }
  } else {
    throw "no-version"
  }
} catch {
  $ProbeOutput = & canon inspect modes --output json 2>$null
  $ProbeText = ($ProbeOutput | Out-String)
  if ($LASTEXITCODE -ne 0 -or $ProbeText -notmatch "requirements" -or $ProbeText -notmatch "brownfield-change" -or $ProbeText -notmatch "pr-review") {
    Emit-Failure "version-incompatible" 11 "Canon is present, but it does not satisfy the expected CLI command contract for this repo." "Reinstall Canon with: $InstallCommand"
  }
}

try {
  & git -C $RepoRoot rev-parse --show-toplevel *> $null
  if ($LASTEXITCODE -ne 0) {
    throw "not-git"
  }
} catch {
  Emit-Failure "wrong-repo-context" 12 "The current working directory is not inside a Git repository." "Switch into the intended repository root before invoking this skill."
}

if ($RequireInit -and -not (Test-Path (Join-Path $RepoRoot ".canon"))) {
  Emit-Failure "repo-not-initialized" 13 "This workflow requires an initialized .canon/ directory." "Run `$canon-init or canon init in $RepoRoot first."
}

if ($RunId -and $RunId -ne "<RUN_ID>" -and -not (Test-Path (Join-Path $RepoRoot ".canon/runs/$RunId"))) {
  Emit-Failure "missing-input" 14 "Run id $RunId was not found under .canon/runs/." "Check the run id and retry the command."
}

if ($Owner -and -not $Risk) {
  Emit-Failure "missing-input" 14 "Risk class is required when owner is provided for a run-starting skill." "Retry with --risk <RISK>."
}

if ($Owner -and -not $Zone) {
  Emit-Failure "missing-input" 14 "Usage zone is required when owner is provided for a run-starting skill." "Retry with --zone <ZONE>."
}

foreach ($Item in $InputPath) {
  if (-not $Item) {
    Emit-Failure "missing-input" 14 "A required input value was empty." "Provide the missing input and retry."
  }
  if ($Item -notlike "refs/*" -and $Item -ne "HEAD" -and -not (Test-Path (Join-Path $RepoRoot $Item)) -and -not (Test-Path $Item)) {
    Emit-Failure "missing-input" 14 "Input $Item was not found from $RepoRoot." "Provide an existing file path or retry with a valid ref."
  }
}

foreach ($RefItem in $RefName) {
  if (-not $RefItem) {
    Emit-Failure "missing-input" 14 "A required Git ref was empty." "Provide the missing ref and retry."
  }
}

Write-Output "STATUS=ready"
Write-Output "CODE=0"
Write-Output "COMMAND=$Command"
Write-Output "REPO_ROOT=$RepoRoot"
Write-Output "VERSION_KIND=$VersionKind"
Write-Output "DETECTED_VERSION=$DetectedVersion"
