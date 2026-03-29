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

if ($Errors -ne 0) {
  exit 1
}

Write-Output "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
