param(
  [string]$Command = "",
  [string]$RepoRoot = (Get-Location).Path,
  [switch]$RequireInit,
  [string]$RunId = "",
  [string]$Owner = "",
  [string]$Risk = "",
  [string]$Zone = "",
  [string]$SystemContext = "",
  [Alias("Input")]
  [string[]]$InputPath = @(),
  [Alias("InputText")]
  [string[]]$InlineInputText = @(),
  [Alias("Ref")]
  [string[]]$RefName = @()
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$CompatFile = Join-Path (Join-Path $ScriptDir "..") "references/runtime-compatibility.toml"
$CompatText = Get-Content -Raw $CompatFile
$InstallCommand = [regex]::Match($CompatText, 'install_command = "(.*)"').Groups[1].Value
$InstallGuidanceRef = [regex]::Match($CompatText, 'install_guidance_ref = "(.*)"').Groups[1].Value
$ReleaseSurface = [regex]::Match($CompatText, 'release_surface = "(.*)"').Groups[1].Value
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

function Get-InstallAction {
  param([switch]$Update)

  if (-not [string]::IsNullOrWhiteSpace($InstallGuidanceRef) -and -not [string]::IsNullOrWhiteSpace($ReleaseSurface)) {
    return "Follow the install guide at $InstallGuidanceRef and download the matching release from $ReleaseSurface."
  }

  if (-not [string]::IsNullOrWhiteSpace($InstallGuidanceRef)) {
    return "Follow the install guide at $InstallGuidanceRef."
  }

  if (-not [string]::IsNullOrWhiteSpace($InstallCommand)) {
    if ($Update) {
      return "Reinstall Canon with: $InstallCommand"
    }

    return "Install it with: $InstallCommand"
  }

  return 'Install Canon and ensure it is on PATH.'
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

function Normalize-SystemContext([string]$Value) {
  switch ($Value) {
    'new' { return 'new' }
    'New' { return 'new' }
    'existing' { return 'existing' }
    'Existing' { return 'existing' }
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

function Resolve-ExistingInputPath([string]$Value) {
  if ([string]::IsNullOrWhiteSpace($Value)) {
    return $null
  }

  $Candidate = $Value
  if (-not [System.IO.Path]::IsPathRooted($Candidate)) {
    $Candidate = Join-Path $RepoRoot $Candidate
  }

  if (-not (Test-Path -LiteralPath $Candidate)) {
    return $null
  }

  return (Resolve-Path -LiteralPath $Candidate).Path
}

function Get-AuthoredInputRetryAction([string]$CommandName) {
  $InputHint = Get-CanonicalModeInputHint $CommandName

  if ($CommandName -eq 'review') {
    if ($InputHint) {
      return "Retry with $InputHint, or pass exactly one non-empty --input-text note."
    }

    return 'Retry with exactly one non-empty authored input or exactly one non-empty --input-text note.'
  }

  if ($InputHint) {
    return "Retry with $InputHint, another non-empty authored path, or non-empty --input-text."
  }

  return 'Retry with a non-empty authored file path or non-empty --input-text.'
}

function Get-AuthoredInputContentStatus([string]$ResolvedPath) {
  if ([string]::IsNullOrWhiteSpace($ResolvedPath)) {
    return 'missing'
  }

  $Item = Get-Item -LiteralPath $ResolvedPath -ErrorAction Stop
  if ($Item.PSIsContainer) {
    $Files = @(Get-ChildItem -LiteralPath $ResolvedPath -Recurse -File -ErrorAction SilentlyContinue)
    if ($Files.Count -eq 0) {
      return 'empty-dir'
    }

    foreach ($File in $Files) {
      $Content = Get-Content -LiteralPath $File.FullName -Raw -ErrorAction SilentlyContinue
      if (-not [string]::IsNullOrWhiteSpace($Content)) {
        return 'usable'
      }
    }

    return 'whitespace-only'
  }

  $Content = Get-Content -LiteralPath $ResolvedPath -Raw -ErrorAction SilentlyContinue
  if ([string]::IsNullOrWhiteSpace($Content)) {
    return 'whitespace-only'
  }

  return 'usable'
}

function Get-CanonicalModeInputHint([string]$CommandName) {
  switch ($CommandName) {
    'requirements' { return 'canon-input/requirements.md or canon-input/requirements/' }
    'discovery' { return 'canon-input/discovery.md or canon-input/discovery/' }
    'review' { return 'canon-input/review.md or canon-input/review/' }
    'system-shaping' { return 'canon-input/system-shaping.md or canon-input/system-shaping/' }
    'architecture' { return 'canon-input/architecture.md or canon-input/architecture/' }
    'change' { return 'canon-input/change.md or canon-input/change/' }
    'implementation' { return 'canon-input/implementation.md or canon-input/implementation/' }
    'incident' { return 'canon-input/incident.md or canon-input/incident/' }
    'system-assessment' { return 'canon-input/system-assessment.md or canon-input/system-assessment/' }
    'migration' { return 'canon-input/migration.md or canon-input/migration/' }
    'supply-chain-analysis' { return 'canon-input/supply-chain-analysis.md or canon-input/supply-chain-analysis/' }
    'refactor' { return 'canon-input/refactor.md or canon-input/refactor/' }
    'verification' { return 'canon-input/verification.md or canon-input/verification/' }
    default { return $null }
  }
}

function Test-LocalBranchExists([string]$RefName) {
  & git -C $RepoRoot show-ref --verify --quiet $RefName *> $null
  return ($LASTEXITCODE -eq 0)
}

function Test-RemoteTrackingRefExists([string]$RefName) {
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

function Get-CanonicalRemoteRef([string]$Value) {
  if ($Value.StartsWith('refs/remotes/')) {
    return $Value
  }
  if (Test-RemoteLikeRef $Value) {
    return "refs/remotes/$Value"
  }
  return $null
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

  $RemoteCandidate = Get-CanonicalRemoteRef $Trimmed
  if ($RemoteCandidate -and (Test-RemoteTrackingRefExists $RemoteCandidate)) {
    return $RemoteCandidate
  }

  $Suggestion = Get-LocalRefSuggestion $Trimmed
  $Action = 'Retry with an existing local branch, a fetched remote-tracking ref, explicit refs/heads/<name>, explicit refs/remotes/<remote>/<name>, or HEAD.'
  if ($Suggestion) {
    $Action = "Retry with $Suggestion, explicit refs/heads/$Suggestion, a fetched remote-tracking ref such as origin/$Suggestion, or HEAD."
  }

  Emit-Failure 'invalid-ref' 16 "Ref $Trimmed did not resolve in the current repository context." $Action @{ FAILED_SLOT = $SlotName; FAILED_KIND = 'RefInput' }
}

$CanonCommand = Get-Command canon -ErrorAction SilentlyContinue
if (-not $CanonCommand) {
  Emit-Failure "cli-missing" 10 "Canon CLI is not installed or is not on PATH." (Get-InstallAction)
}

$DetectedVersion = "unavailable"
$VersionKind = "command-contract"
try {
  $VersionOutput = & canon --version 2>$null
  if ($LASTEXITCODE -eq 0 -and $VersionOutput) {
    $VersionKind = "semver"
    $DetectedVersion = ($VersionOutput -split ' ')[1]
    if ($ExpectedVersion -and $DetectedVersion -ne $ExpectedVersion) {
      Emit-Failure "version-incompatible" 11 "Detected Canon version $DetectedVersion, expected $ExpectedVersion." (Get-InstallAction -Update)
    }
  } else {
    throw "no-version"
  }
} catch {
  $ProbeOutput = & canon inspect modes --output json 2>$null
  $ProbeText = ($ProbeOutput | Out-String)
  if ($LASTEXITCODE -ne 0 -or $ProbeText -notmatch "requirements" -or $ProbeText -notmatch "change" -or $ProbeText -notmatch "review" -or $ProbeText -notmatch "verification" -or $ProbeText -notmatch "pr-review") {
    Emit-Failure "version-incompatible" 11 "Canon is present, but it does not satisfy the expected CLI command contract for this repo." (Get-InstallAction -Update)
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

$RunStartCommands = @('requirements', 'discovery', 'system-shaping', 'architecture', 'change', 'review', 'verification', 'pr-review')
$RunIdCommands = @('status', 'inspect-invocations', 'inspect-evidence', 'inspect-artifacts', 'approve', 'resume')
$RunStartCommand = $RunStartCommands -contains $Command
$RunIdCommand = $RunIdCommands -contains $Command
$PrReviewCommand = ($Command -eq 'pr-review')

$NormalizedRunId = ''
$NormalizedRisk = ''
$NormalizedZone = ''
$NormalizedSystemContext = ''
$NormalizedInput1 = ''
$NormalizedInlineInput1 = ''
$NormalizedRef1 = ''
$NormalizedRef2 = ''
$InferredRisk = ''
$InferredZone = ''
$InferenceConfidence = ''
$InferenceHeadline = ''
$InferenceRationale = ''
$RiskRationale = ''
$ZoneRationale = ''
$RiskWasSupplied = ''
$ZoneWasSupplied = ''
$InferenceSignals = @()
$RiskSignals = @()
$ZoneSignals = @()

function Invoke-ClassificationInference {
  param(
    [string]$ModeName,
    [string[]]$InputValues = @(),
    [string[]]$InlineInputValues = @()
  )

  $Arguments = @('inspect', 'risk-zone', '--mode', $ModeName, '--output', 'text')
  if ($NormalizedRisk) { $Arguments += @('--risk', $NormalizedRisk) }
  if ($NormalizedZone) { $Arguments += @('--zone', $NormalizedZone) }
  foreach ($InputValue in $InputValues) {
    $Arguments += @('--input', $InputValue)
  }
  foreach ($InlineInputValue in $InlineInputValues) {
    $Arguments += @('--input-text', $InlineInputValue)
  }

  $Output = & canon @Arguments 2>$null
  if ($LASTEXITCODE -ne 0) {
    Emit-Failure 'classification-unavailable' 20 'Canon could not infer risk and zone from the supplied intake.' 'Provide --risk and --zone explicitly, or fix the authored input surface before retrying.' @{ FAILED_KIND = 'ClassificationInference' }
  }

  $script:InferredRisk = ''
  $script:InferredZone = ''
  $script:InferenceConfidence = ''
  $script:InferenceHeadline = ''
  $script:InferenceRationale = ''
  $script:RiskRationale = ''
  $script:ZoneRationale = ''
  $script:RiskWasSupplied = ''
  $script:ZoneWasSupplied = ''
  $script:InferenceSignals = @()
  $script:RiskSignals = @()
  $script:ZoneSignals = @()

  foreach ($Line in @($Output | Where-Object { $_ -and $_ -match '=' })) {
    $Key, $Value = $Line -split '=', 2
    switch -Wildcard ($Key) {
      'INFERRED_RISK' { $script:InferredRisk = $Value }
      'INFERRED_ZONE' { $script:InferredZone = $Value }
      'INFERENCE_CONFIDENCE' { $script:InferenceConfidence = $Value }
      'INFERENCE_HEADLINE' { $script:InferenceHeadline = $Value }
      'INFERENCE_RATIONALE' { $script:InferenceRationale = $Value }
      'RISK_RATIONALE' { $script:RiskRationale = $Value }
      'ZONE_RATIONALE' { $script:ZoneRationale = $Value }
      'RISK_WAS_SUPPLIED' { $script:RiskWasSupplied = $Value }
      'ZONE_WAS_SUPPLIED' { $script:ZoneWasSupplied = $Value }
      'SIGNAL_*' { $script:InferenceSignals += $Value }
      'RISK_SIGNAL_*' { $script:RiskSignals += $Value }
      'ZONE_SIGNAL_*' { $script:ZoneSignals += $Value }
    }
  }

  if (-not $InferredRisk -or -not $InferredZone) {
    Emit-Failure 'classification-unavailable' 20 'Canon returned an incomplete risk/zone inference payload.' 'Provide --risk and --zone explicitly, or inspect the authored intake before retrying.' @{ FAILED_KIND = 'ClassificationInference' }
  }
}

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
  $SystemContext = Trim-Value $SystemContext

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
    $AuthoredInputCount = $InputPath.Count + $InlineInputText.Count
    if ($AuthoredInputCount -eq 0) {
      Emit-Failure 'missing-input' 14 "Authored input is required for $Command." 'Retry with --input <INPUT_PATH> or --input-text <INPUT_TEXT>.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
    }

    if ($Command -eq 'review' -and $AuthoredInputCount -ne 1) {
      Emit-Failure 'invalid-input' 17 'Review requires exactly one authored input at canon-input/review.md or canon-input/review/, or exactly one explicit --input-text note.' 'Retry with canon-input/review.md or canon-input/review/, or pass exactly one --input-text note.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
    }

    foreach ($InlineInput in $InlineInputText) {
      $TrimmedInlineInput = Trim-Value $InlineInput
      if (Test-MissingValue $TrimmedInlineInput) {
        Emit-Failure 'invalid-input' 17 "Inline authored input for $Command is empty or whitespace-only." 'Retry with non-empty --input-text content.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
      }

      if (-not $NormalizedInlineInput1) {
        $NormalizedInlineInput1 = $TrimmedInlineInput
      }
    }

    foreach ($Input in $InputPath) {
      $LocalInput = Trim-Value $Input
      if (Test-MissingValue $LocalInput) {
        Emit-Failure 'missing-input' 14 "Input path is required for $Command." 'Retry with --input <INPUT_PATH> or --input-text <INPUT_TEXT>.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
      }

      if (-not (Test-Path (Join-Path $RepoRoot $LocalInput)) -and -not (Test-Path $LocalInput)) {
        if ($Command -eq 'review') {
          Emit-Failure 'missing-file' 15 "Review input $LocalInput was not found from $RepoRoot." 'Retry with canon-input/review.md or canon-input/review/, or pass exactly one --input-text note.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
        }
        Emit-Failure 'missing-file' 15 "Input $LocalInput was not found from $RepoRoot." 'Retry with an existing file path or non-empty --input-text.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
      }

      $ResolvedInput = Resolve-ExistingInputPath $LocalInput
      $CanonRoot = Resolve-ExistingInputPath '.canon'
      if ($ResolvedInput -and $CanonRoot -and ($ResolvedInput -eq $CanonRoot -or $ResolvedInput.StartsWith($CanonRoot + [System.IO.Path]::DirectorySeparatorChar) -or $ResolvedInput.StartsWith($CanonRoot + [System.IO.Path]::AltDirectorySeparatorChar))) {
        $InputHint = Get-CanonicalModeInputHint $Command
        $InputAction = if ($InputHint) {
          "Retry with $InputHint, another authored file path outside .canon/, or non-empty --input-text."
        }
        else {
          'Retry with an authored file path outside .canon/ or non-empty --input-text.'
        }
        Emit-Failure 'invalid-input' 17 "Input $LocalInput points inside .canon/ and cannot be used as authored input for $Command." $InputAction @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
      }

      if ($Command -eq 'review') {
        $ResolvedReviewFile = Resolve-ExistingInputPath 'canon-input/review.md'
        $ResolvedReviewDir = Resolve-ExistingInputPath 'canon-input/review'
        if ($ResolvedInput -ne $ResolvedReviewFile -and $ResolvedInput -ne $ResolvedReviewDir) {
          Emit-Failure 'invalid-input' 17 "Review accepts only canon-input/review.md or canon-input/review/, not $LocalInput." 'Move or author the review packet at canon-input/review.md or canon-input/review/, or pass exactly one --input-text note, then retry.' @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
        }
      }

      $InputContentStatus = Get-AuthoredInputContentStatus $ResolvedInput
      switch ($InputContentStatus) {
        'empty-dir' {
          Emit-Failure 'invalid-input' 17 "Input $LocalInput expands to files with no usable authored content." (Get-AuthoredInputRetryAction $Command) @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
        }
        'whitespace-only' {
          Emit-Failure 'invalid-input' 17 "Input $LocalInput is empty or whitespace-only." (Get-AuthoredInputRetryAction $Command) @{ FAILED_SLOT = 'input-path'; FAILED_KIND = 'FilePathInput' }
        }
      }

      if (-not $NormalizedInput1) {
        $NormalizedInput1 = Normalize-InputPath $LocalInput
      }
    }
  }

  if (-not (Test-MissingValue $Risk)) {
    $NormalizedRisk = Normalize-Risk $Risk
    if (-not $NormalizedRisk) {
      Emit-Failure 'invalid-input' 17 "Risk class $Risk is not supported by the Canon runtime contract." 'Retry with low-impact, bounded-impact, systemic-impact, or the runtime-recognized aliases LowImpact, BoundedImpact, SystemicImpact.' @{ FAILED_SLOT = 'risk'; FAILED_KIND = 'RiskField' }
    }
  }

  if (-not (Test-MissingValue $Zone)) {
    $NormalizedZone = Normalize-Zone $Zone
    if (-not $NormalizedZone) {
      Emit-Failure 'invalid-input' 17 "Usage zone $Zone is not supported by the Canon runtime contract." 'Retry with green, yellow, red, or the runtime-recognized aliases Green, Yellow, Red.' @{ FAILED_SLOT = 'zone'; FAILED_KIND = 'ZoneField' }
    }
  }

  $SystemContextUsage = switch ($Command) {
    'system-shaping' { 'new|existing' }
    'architecture' { 'new|existing' }
    'change' { 'existing' }
    'system-assessment' { 'existing' }
    default { '' }
  }

  if ($SystemContextUsage -and (Test-MissingValue $SystemContext)) {
    Emit-Failure 'missing-input' 14 "System context is required for $Command." "Retry with --system-context $SystemContextUsage." @{ FAILED_SLOT = 'system-context'; FAILED_KIND = 'SystemContextField' }
  }

  if (-not (Test-MissingValue $SystemContext)) {
    $NormalizedSystemContext = Normalize-SystemContext $SystemContext
    if (-not $NormalizedSystemContext) {
      Emit-Failure 'invalid-input' 17 "System context $SystemContext is not supported by the Canon runtime contract." 'Retry with new, existing, or the runtime-recognized aliases New, Existing.' @{ FAILED_SLOT = 'system-context'; FAILED_KIND = 'SystemContextField' }
    }
  }

  if (($Command -eq 'change' -or $Command -eq 'system-assessment') -and $NormalizedSystemContext -and $NormalizedSystemContext -ne 'existing') {
    Emit-Failure 'invalid-input' 17 "Mode $Command currently supports only --system-context existing in this release." 'Retry with --system-context existing.' @{ FAILED_SLOT = 'system-context'; FAILED_KIND = 'SystemContextField' }
  }

  if (-not $NormalizedRisk -or -not $NormalizedZone) {
    if ($PrReviewCommand) {
      Invoke-ClassificationInference -ModeName $Command -InputValues @($NormalizedRef1, $NormalizedRef2)
    }
    elseif ($NormalizedInput1) {
      Invoke-ClassificationInference -ModeName $Command -InputValues @($NormalizedInput1)
    }
    else {
      Invoke-ClassificationInference -ModeName $Command -InlineInputValues @($NormalizedInlineInput1)
    }

    $Extra = @{
      VERSION_KIND = $VersionKind
      DETECTED_VERSION = $DetectedVersion
      NEEDS_CONFIRMATION = 'true'
      INFERRED_RISK = $InferredRisk
      INFERRED_ZONE = $InferredZone
      INFERENCE_CONFIDENCE = $InferenceConfidence
      INFERENCE_HEADLINE = $InferenceHeadline
      INFERENCE_RATIONALE = $InferenceRationale
      RISK_RATIONALE = $RiskRationale
      ZONE_RATIONALE = $ZoneRationale
      RISK_WAS_SUPPLIED = if ($RiskWasSupplied) { $RiskWasSupplied } else { 'false' }
      ZONE_WAS_SUPPLIED = if ($ZoneWasSupplied) { $ZoneWasSupplied } else { 'false' }
    }

    if ($NormalizedInput1) { $Extra['NORMALIZED_INPUT_1'] = $NormalizedInput1 }
    if ($NormalizedSystemContext) { $Extra['NORMALIZED_SYSTEM_CONTEXT'] = $NormalizedSystemContext }
    if ($NormalizedRef1) { $Extra['NORMALIZED_REF_1'] = $NormalizedRef1 }
    if ($NormalizedRef2) { $Extra['NORMALIZED_REF_2'] = $NormalizedRef2 }

    for ($Index = 0; $Index -lt $InferenceSignals.Count; $Index++) {
      $Extra["SIGNAL_$($Index + 1)"] = $InferenceSignals[$Index]
    }
    for ($Index = 0; $Index -lt $RiskSignals.Count; $Index++) {
      $Extra["RISK_SIGNAL_$($Index + 1)"] = $RiskSignals[$Index]
    }
    for ($Index = 0; $Index -lt $ZoneSignals.Count; $Index++) {
      $Extra["ZONE_SIGNAL_$($Index + 1)"] = $ZoneSignals[$Index]
    }

    Write-Result -Status 'needs-classification-confirmation' -Code 19 -Phase 'preflight' -Message $InferenceHeadline -Action 'Confirm or override the inferred classification, then invoke Canon with explicit --risk and --zone.' -Extra $Extra
    exit 19
  }
}

$Extra = @{
  VERSION_KIND = $VersionKind
  DETECTED_VERSION = $DetectedVersion
}

if ($NormalizedRunId) { $Extra['NORMALIZED_RUN_ID'] = $NormalizedRunId }
if ($NormalizedInput1) { $Extra['NORMALIZED_INPUT_1'] = $NormalizedInput1 }
if ($NormalizedSystemContext) { $Extra['NORMALIZED_SYSTEM_CONTEXT'] = $NormalizedSystemContext }
if ($NormalizedRef1) { $Extra['NORMALIZED_REF_1'] = $NormalizedRef1 }
if ($NormalizedRef2) { $Extra['NORMALIZED_REF_2'] = $NormalizedRef2 }
if ($NormalizedRisk) { $Extra['NORMALIZED_RISK'] = $NormalizedRisk }
if ($NormalizedZone) { $Extra['NORMALIZED_ZONE'] = $NormalizedZone }

Write-Result -Status 'ready' -Code 0 -Phase 'preflight' -Message 'Typed preflight checks passed.' -Action 'Invoke Canon using the normalized contract for this command.' -Extra $Extra
