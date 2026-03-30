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

function Trim-Value([string]$Value) {
  if ($null -eq $Value) { return "" }
  return $Value.Trim()
}

function Test-Placeholder([string]$Value) {
  return $Value -match '^<[^>]+>$'
}

function Test-MissingValue([string]$Value) {
  $Trimmed = Trim-Value $Value
  return [string]::IsNullOrWhiteSpace($Trimmed) -or (Test-Placeholder $Trimmed)
}

function Write-Result {
  param(
    [string]$Status,
    [int]$Code,
    [string]$Phase,
    [string]$Message,
    [string]$Action,
    [hashtable]$Extra = @{}
  )

  Write-Output "STATUS=$Status"
  Write-Output "CODE=$Code"
  Write-Output "PHASE=$Phase"
  Write-Output "COMMAND=$Command"
  Write-Output "REPO_ROOT=$RepoRoot"
  Write-Output "MESSAGE=$Message"
  Write-Output "ACTION=$Action"
  foreach ($Key in $Extra.Keys) {
    if ($null -ne $Extra[$Key] -and $Extra[$Key] -ne "") {
      Write-Output "$Key=$($Extra[$Key])"
    }
  }
}

function Emit-Failure {
  param(
    [string]$Status,
    [int]$Code,
    [string]$Message,
    [string]$Action,
    [hashtable]$Extra = @{}
  )

  Write-Result -Status $Status -Code $Code -Phase 'preflight' -Message $Message -Action $Action -Extra $Extra
  exit $Code
}

function Normalize-Risk([string]$Value) {
  switch ($Value) {
    'low-impact' { return 'low-impact' }
    'LowImpact' { return 'low-impact' }
    'bounded-impact' { return 'bounded-impact' }
    'BoundedImpact' { return 'bounded-impact' }
    'systemic-impact' { return 'systemic-impact' }
    'SystemicImpact' { return 'systemic-impact' }
    default { return $null }
  }
}

function Normalize-Zone([string]$Value) {
  switch ($Value) {
    'green' { return 'green' }
    'Green' { return 'green' }
    'yellow' { return 'yellow' }
    'Yellow' { return 'yellow' }
    'red' { return 'red' }
    'Red' { return 'red' }
    default { return $null }
  }
}

function Normalize-InputPath([string]$Value) {
  if ([string]::IsNullOrWhiteSpace($Value)) {
    return ""
  }
  if ([System.IO.Path]::IsPathRooted($Value) -and $Value.StartsWith($RepoRoot + [System.IO.Path]::DirectorySeparatorChar)) {
    return $Value.Substring($RepoRoot.Length + 1)
  }
  return $Value
}

function Test-LocalBranchExists([string]$RefName) {
  & git -C $RepoRoot show-ref --verify --quiet $RefName *> $null
  return ($LASTEXITCODE -eq 0)
}

function Get-LocalBranches {
  $Output = & git -C $RepoRoot for-each-ref refs/heads '--format=%(refname:short)' 2>$null
  return @($Output | Where-Object { $_ -and $_.Trim() -ne '' })
}

function Get-RemoteNames {
  $Output = & git -C $RepoRoot remote 2>$null
  return @($Output | Where-Object { $_ -and $_.Trim() -ne '' })
}

function Test-RemoteLikeRef([string]$Value) {
  if ($Value.StartsWith('refs/remotes/')) {
    return $true
  }
  if ($Value -match '/') {
    $FirstSegment = ($Value -split '/')[0]
    foreach ($RemoteName in Get-RemoteNames) {
      if ($FirstSegment -eq $RemoteName) {
        return $true
      }
    }
  }
  return $false
}

function Get-LocalRefSuggestion([string]$Value) {
  $Branches = Get-LocalBranches
  switch ($Value) {
    'main' {
      if ($Branches -contains 'master') { return 'master' }
    }
    'master' {
      if ($Branches -contains 'main') { return 'main' }
    }
  }
  return $null
}

function Resolve-Ref {
  param([string]$RawValue, [string]$SlotName)

  $Trimmed = Trim-Value $RawValue
  if (Test-MissingValue $Trimmed) {
    Emit-Failure 'missing-input' 14 "Required ref slot $SlotName is missing." "Retry with --ref <$($SlotName.ToUpper())>." @{ FAILED_SLOT = $SlotName; FAILED_KIND = 'RefInput' }
  }

  if ($Trimmed -eq 'HEAD') {
    return 'HEAD'
  }

  if ($Trimmed.StartsWith('refs/heads/')) {
    if (Test-LocalBranchExists $Trimmed) {
      return $Trimmed
    }
  }
  else {
    $LocalCandidate = "refs/heads/$Trimmed"
    if (Test-LocalBranchExists $LocalCandidate) {
      return $LocalCandidate
    }
  }

  if (Test-RemoteLikeRef $Trimmed) {
    Emit-Failure 'invalid-ref' 16 "Ref $Trimmed is remote-like and unsupported in this patch." 'Retry with a local branch, explicit refs/heads/<name>, or HEAD.' @{ FAILED_SLOT = $SlotName; FAILED_KIND = 'unsupported-remote-ref' }
  }

  $Suggestion = Get-LocalRefSuggestion $Trimmed
  $Action = 'Retry with an existing local branch, explicit refs/heads/<name>, or HEAD.'
  if ($Suggestion) {
    $Action = "Retry with $Suggestion or explicit refs/heads/$Suggestion."
  }

  Emit-Failure 'invalid-ref' 16 "Ref $Trimmed did not resolve in the current repository context." $Action @{ FAILED_SLOT = $SlotName; FAILED_KIND = 'RefInput' }
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

$RunStartCommands = @('requirements', 'brownfield-change', 'pr-review')
$RunIdCommands = @('status', 'inspect-invocations', 'inspect-evidence', 'inspect-artifacts', 'approve', 'resume')
$RunStartCommand = $RunStartCommands -contains $Command
$RunIdCommand = $RunIdCommands -contains $Command
$PrReviewCommand = ($Command -eq 'pr-review')

$NormalizedRunId = ''
$NormalizedInput1 = ''
$NormalizedRef1 = ''
$NormalizedRef2 = ''

if ($RunIdCommand) {
  $NormalizedRunId = Trim-Value $RunId
  if (Test-MissingValue $NormalizedRunId) {
    Emit-Failure 'missing-input' 14 "Run id is required for $Command." 'Retry with --run-id <RUN_ID>.' @{ FAILED_SLOT = 'run-id'; FAILED_KIND = 'RunIdInput' }
  }
  if (-not (Test-Path (Join-Path $RepoRoot ".canon/runs/$NormalizedRunId"))) {
    Emit-Failure 'invalid-input' 17 "Run id $NormalizedRunId was not found under .canon/runs/." 'Check the run id and retry with an existing run.' @{ FAILED_SLOT = 'run-id'; FAILED_KIND = 'RunIdInput' }
  }
}

if ($RunStartCommand) {
  $Owner = Trim-Value $Owner
  $Risk = Trim-Value $Risk
  $Zone = Trim-Value $Zone

  if (Test-MissingValue $Owner) {
    Emit-Failure 'missing-input' 14 "Owner is required for $Command." 'Retry with --owner <OWNER>.' @{ FAILED_SLOT = 'owner'; FAILED_KIND = 'OwnerField' }
  }

  if (Test-MissingValue $Risk) {
    Emit-Failure 'missing-input' 14 "Risk class is required for $Command." 'Retry with --risk <RISK>.' @{ FAILED_SLOT = 'risk'; FAILED_KIND = 'RiskField' }
  }

  $NormalizedRisk = Normalize-Risk $Risk
  if (-not $NormalizedRisk) {
    Emit-Failure 'invalid-input' 17 "Risk class $Risk is not supported by the Canon runtime contract." 'Retry with low-impact, bounded-impact, systemic-impact, or the runtime-recognized aliases LowImpact, BoundedImpact, SystemicImpact.' @{ FAILED_SLOT = 'risk'; FAILED_KIND = 'RiskField' }
  }

  if (Test-MissingValue $Zone) {
    Emit-Failure 'missing-input' 14 "Usage zone is required for $Command." 'Retry with --zone <ZONE>.' @{ FAILED_SLOT = 'zone'; FAILED_KIND = 'ZoneField' }
  }

  $NormalizedZone = Normalize-Zone $Zone
  if (-not $NormalizedZone) {
    Emit-Failure 'invalid-input' 17 "Usage zone $Zone is not supported by the Canon runtime contract." 'Retry with green, yellow, red, or the runtime-recognized aliases Green, Yellow, Red.' @{ FAILED_SLOT = 'zone'; FAILED_KIND = 'ZoneField' }
  }

  if ($PrReviewCommand) {
    if ($RefName.Count -eq 0) {
      Emit-Failure 'missing-input' 14 'Base ref is required for pr-review.' 'Retry with --ref <BASE_REF> --ref <HEAD_REF>.' @{ FAILED_SLOT = 'base-ref'; FAILED_KIND = 'RefInput' }
    }

    $NormalizedRef1 = Resolve-Ref -RawValue $RefName[0] -SlotName 'base-ref'

    if ($RefName.Count -lt 2) {
      Emit-Failure 'missing-input' 14 'Head ref is required for pr-review.' 'Retry with --ref <BASE_REF> --ref <HEAD_REF>.' @{ FAILED_SLOT = 'head-ref'; FAILED_KIND = 'RefInput'; NORMALIZED_REF_1 = $NormalizedRef1 }
    }

    $NormalizedRef2 = Resolve-Ref -RawValue $RefName[1] -SlotName 'head-ref'

    if ($NormalizedRef1 -eq $NormalizedRef2) {
      Emit-Failure 'malformed-ref-pair' 18 'Base and head refs normalize to the same Canon binding, so the diff range is empty.' 'Retry with distinct base and head refs.' @{ FAILED_SLOT = 'ref-pair'; FAILED_KIND = 'RefPairInput'; NORMALIZED_REF_1 = $NormalizedRef1; NORMALIZED_REF_2 = $NormalizedRef2 }
    }
  }
  else {
    if ($InputPath.Count -eq 0) {
      Emit-Failure 'missing-input' 14 "Input path is required for $Command." 'Retry with --input <INPUT_PATH>.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
    }

    $LocalInput = Trim-Value $InputPath[0]
    if (Test-MissingValue $LocalInput) {
      Emit-Failure 'missing-input' 14 "Input path is required for $Command." 'Retry with --input <INPUT_PATH>.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
    }

    if (-not (Test-Path (Join-Path $RepoRoot $LocalInput)) -and -not (Test-Path $LocalInput)) {
      Emit-Failure 'missing-file' 15 "Input $LocalInput was not found from $RepoRoot." 'Retry with an existing file path.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
    }

    $NormalizedInput1 = Normalize-InputPath $LocalInput
  }
}

$Extra = @{
  VERSION_KIND = $VersionKind
  DETECTED_VERSION = $DetectedVersion
}

if ($NormalizedRunId) { $Extra['NORMALIZED_RUN_ID'] = $NormalizedRunId }
if ($NormalizedInput1) { $Extra['NORMALIZED_INPUT_1'] = $NormalizedInput1 }
if ($NormalizedRef1) { $Extra['NORMALIZED_REF_1'] = $NormalizedRef1 }
if ($NormalizedRef2) { $Extra['NORMALIZED_REF_2'] = $NormalizedRef2 }

Write-Result -Status 'ready' -Code 0 -Phase 'preflight' -Message 'Typed preflight checks passed.' -Action 'Invoke Canon using the normalized contract for this command.' -Extra $Extra
