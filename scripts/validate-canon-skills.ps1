$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
$SkillsDir = Join-Path $Root ".agents/skills"

$RequiredSections = @(
  "## Support State",
  "## Purpose",
  "## When To Trigger",
  "## When It Must Not Trigger",
  "## Required Inputs",
  "## Preflight Profile",
  "## Canon Command Contract",
  "## Expected Output Shape",
  "## Failure Handling Guidance",
  "## Next-Step Guidance",
  "## Related Skills"
)

$AvailableNow = @(
  "canon-init",
  "canon-status",
  "canon-inspect-invocations",
  "canon-inspect-evidence",
  "canon-inspect-artifacts",
  "canon-approve",
  "canon-resume",
  "canon-requirements",
  "canon-brownfield",
  "canon-pr-review"
)

$ModeledOnly = @(
  "canon-discovery",
  "canon-greenfield",
  "canon-architecture",
  "canon-implementation",
  "canon-refactor",
  "canon-review",
  "canon-incident",
  "canon-migration"
)

$IntentionallyLimited = @("canon-verification")
$Errors = 0

function Fail([string]$Message) {
  Write-Error $Message
  $script:Errors += 1
}

function Require-Text([string]$Path, [string]$Pattern, [string]$Message) {
  $Text = Get-Content -Raw $Path
  if ($Text -notmatch [regex]::Escape($Pattern)) {
    Fail $Message
  }
}

function Forbid-Text([string]$Path, [string]$Pattern, [string]$Message) {
  $Text = Get-Content -Raw $Path
  if ($Text -match [regex]::Escape($Pattern)) {
    Fail $Message
  }
}

function Check-Skill([string]$Skill, [string]$ExpectedState) {
  $Path = Join-Path $SkillsDir "$Skill/SKILL.md"
  if (-not (Test-Path $Path)) {
    Fail "Missing skill file: $Path"
    return
  }

  $Text = Get-Content -Raw $Path
  if ($Text -notmatch "(?m)^---$") { Fail "${Skill}: missing frontmatter fence" }
  if ($Text -notmatch "name: $Skill") { Fail "${Skill}: frontmatter name mismatch" }
  if ($Text -notmatch "description: Use when ") { Fail "${Skill}: description must start with 'Use when '" }
  foreach ($Section in $RequiredSections) {
    if ($Text -notmatch [regex]::Escape($Section)) {
      Fail "${Skill}: missing section $Section"
    }
  }
  $Backtick = [string][char]96
  $ExpectedStateToken = $Backtick + $ExpectedState + $Backtick
  if ($Text -notmatch [regex]::Escape($ExpectedStateToken)) {
    Fail "${Skill}: expected support state $ExpectedState"
  }
}

foreach ($Skill in $AvailableNow) {
  Check-Skill $Skill "available-now"
}

foreach ($Skill in $ModeledOnly) {
  Check-Skill $Skill "modeled-only"
  $Text = Get-Content -Raw (Join-Path $SkillsDir "$Skill/SKILL.md")
  if ($Text -match "canon run --mode|Run ID:|--run <RUN_ID>|gate:|invocation:") {
    Fail "${Skill}: modeled-only skill appears to fabricate runnable Canon behavior"
  }
}

foreach ($Skill in $IntentionallyLimited) {
  Check-Skill $Skill "intentionally-limited"
  $Text = Get-Content -Raw (Join-Path $SkillsDir "$Skill/SKILL.md")
  if ($Text -match "canon verify --run|Run ID:") {
    Fail "${Skill}: intentionally-limited skill appears to fabricate runnable Canon behavior"
  }
}

if ((Get-Content -Raw (Join-Path $SkillsDir "canon-review/SKILL.md")) -notmatch "canon-pr-review") { Fail "canon-review: must distinguish itself from canon-pr-review" }
if ((Get-Content -Raw (Join-Path $SkillsDir "canon-refactor/SKILL.md")) -notmatch "canon-brownfield") { Fail "canon-refactor: must distinguish itself from canon-brownfield" }
if ((Get-Content -Raw (Join-Path $SkillsDir "canon-discovery/SKILL.md")) -notmatch "canon-requirements") { Fail "canon-discovery: must distinguish itself from canon-requirements" }
$CanonInitText = Get-Content -Raw (Join-Path $SkillsDir "canon-init/SKILL.md")
if ($CanonInitText -notmatch [regex]::Escape('Do not automatically start another Canon skill or `canon run` in the same turn.')) { Fail "canon-init: must explicitly forbid chaining into follow-up runs" }
if ($CanonInitText -match "Run ID:|State:") { Fail "canon-init: must not describe run-id or run-state output" }

$RequirementsPath = Join-Path $SkillsDir "canon-requirements/SKILL.md"
$BrownfieldPath = Join-Path $SkillsDir "canon-brownfield/SKILL.md"
$PrReviewPath = Join-Path $SkillsDir "canon-pr-review/SKILL.md"

Require-Text $RequirementsPath '--input <INPUT_PATH>' 'canon-requirements: preflight must keep file-path input binding'
Require-Text $BrownfieldPath '--input <INPUT_PATH>' 'canon-brownfield: preflight must keep file-path input binding'
Require-Text $PrReviewPath '--ref <BASE_REF> --ref <HEAD_REF>' 'canon-pr-review: preflight must use --ref for base/head binding'

Forbid-Text $PrReviewPath 'check-runtime.sh --command pr-review --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <BASE_REF> --input <HEAD_REF>' 'canon-pr-review: preflight must not send base/head refs through --input'

Require-Text $RequirementsPath 'preserve valid ownership fields' 'canon-requirements: must describe preserving valid ownership fields across retry'
Require-Text $RequirementsPath 'asks only for the missing slot' 'canon-requirements: must describe single-slot retry behavior'
Require-Text $RequirementsPath 'exact Canon CLI retry form' 'canon-requirements: must promise the exact CLI retry form'
Require-Text $RequirementsPath 'inside Canon execution rather than before Canon execution' 'canon-requirements: must distinguish preflight failures from Canon-execution failures'

Require-Text $BrownfieldPath 'preserve valid ownership fields' 'canon-brownfield: must describe preserving valid ownership fields across retry'
Require-Text $BrownfieldPath 'asks only for the missing brief path or missing ownership slot' 'canon-brownfield: must describe targeted retry behavior'
Require-Text $BrownfieldPath 'exact Canon CLI retry form' 'canon-brownfield: must promise the exact CLI retry form'
Require-Text $BrownfieldPath 'Canon-execution outcome' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'
Require-Text $BrownfieldPath 'preflight failure' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'

Require-Text $PrReviewPath 'preserves the valid side of the pair' 'canon-pr-review: must describe preserving the valid ref side across retry'
Require-Text $PrReviewPath 'exact Canon CLI form' 'canon-pr-review: must promise the exact CLI form'
Require-Text $PrReviewPath 'rejects remote refs explicitly' 'canon-pr-review: must state remote refs are rejected explicitly'
Require-Text $PrReviewPath 'inside Canon execution rather than before Canon execution' 'canon-pr-review: must distinguish preflight failures from Canon-execution failures'

Forbid-Text $PrReviewPath 'valid file path or ref' 'canon-pr-review: must not blur ref slots with file-path guidance'

if ($Errors -ne 0) {
  exit 1
}

Write-Output "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
