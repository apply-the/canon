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
  "canon-inspect-clarity",
  "canon-approve",
  "canon-resume",
  "canon-requirements",
  "canon-discovery",
  "canon-system-shaping",
  "canon-architecture",
  "canon-change",
  "canon-review",
  "canon-verification",
  "canon-pr-review"
)

$ModeledOnly = @(
  "canon-implementation",
  "canon-refactor",
  "canon-incident",
  "canon-migration"
)

$IntentionallyLimited = @()
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
if ((Get-Content -Raw (Join-Path $SkillsDir "canon-refactor/SKILL.md")) -notmatch "canon-change") { Fail "canon-refactor: must distinguish itself from canon-change" }
if ((Get-Content -Raw (Join-Path $SkillsDir "canon-discovery/SKILL.md")) -notmatch "canon-requirements") { Fail "canon-discovery: must distinguish itself from canon-requirements" }
$CanonInitText = Get-Content -Raw (Join-Path $SkillsDir "canon-init/SKILL.md")
if ($CanonInitText -notmatch [regex]::Escape('Do not automatically start another Canon skill or `canon run` in the same turn.')) { Fail "canon-init: must explicitly forbid chaining into follow-up runs" }
if ($CanonInitText -match "Run ID:|State:") { Fail "canon-init: must not describe run-id or run-state output" }

$RequirementsPath = Join-Path $SkillsDir "canon-requirements/SKILL.md"
$ChangePath = Join-Path $SkillsDir "canon-change/SKILL.md"
$SystemShapingPath = Join-Path $SkillsDir "canon-system-shaping/SKILL.md"
$ArchitecturePath = Join-Path $SkillsDir "canon-architecture/SKILL.md"
$ReviewPath = Join-Path $SkillsDir "canon-review/SKILL.md"
$VerificationPath = Join-Path $SkillsDir "canon-verification/SKILL.md"
$PrReviewPath = Join-Path $SkillsDir "canon-pr-review/SKILL.md"
$ClarityPath = Join-Path $SkillsDir "canon-inspect-clarity/SKILL.md"

Require-Text $RequirementsPath '--input <INPUT_PATH>' 'canon-requirements: preflight must keep file-path input binding'
Require-Text $RequirementsPath '--input-text <INPUT_TEXT>' 'canon-requirements: must document inline authored input binding'
Require-Text $ChangePath '--input <INPUT_PATH>' 'canon-change: preflight must keep file-path input binding'
Require-Text $ChangePath '--input-text <INPUT_TEXT>' 'canon-change: must document inline authored input binding'
Require-Text $ChangePath '--system-context existing' 'canon-change: must bind existing system context explicitly'
Require-Text $SystemShapingPath '--system-context <SYSTEM_CONTEXT>' 'canon-system-shaping: must require explicit system context in the command contract'
Require-Text $ArchitecturePath '--system-context <SYSTEM_CONTEXT>' 'canon-architecture: must require explicit system context in the command contract'
Require-Text $ReviewPath '--input <INPUT_PATH>' 'canon-review: preflight must keep file-path input binding'
Require-Text $ReviewPath '--input-text <INPUT_TEXT>' 'canon-review: must document inline authored input binding'
Require-Text $VerificationPath '--input <INPUT_PATH>' 'canon-verification: preflight must keep file-path input binding'
Require-Text $VerificationPath '--input-text <INPUT_TEXT>' 'canon-verification: must document inline authored input binding'
Require-Text $PrReviewPath '--ref <BASE_REF> --ref <HEAD_REF>' 'canon-pr-review: preflight must use --ref for base/head binding'
Require-Text $ClarityPath 'canon inspect clarity --mode <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]' 'canon-inspect-clarity: must promise the exact Canon CLI form'
Require-Text $ClarityPath '.canon/` is not required for this inspection surface' 'canon-inspect-clarity: must stay honest that this inspect surface is pre-run and does not require runtime state'
Require-Text $ClarityPath 'Preserve the already valid mode or input selection' 'canon-inspect-clarity: must preserve valid mode or input slots across retry'
Require-Text $ClarityPath 'Do not fabricate a started run, pending approval, or emitted artifact set' 'canon-inspect-clarity: must forbid fake run state'
Require-Text $ClarityPath 'prefer the directory when both exist' 'canon-inspect-clarity: must prefer canonical directories over a single child file when both canonical surfaces exist'
Require-Text $ClarityPath 'whole directory recursively' 'canon-inspect-clarity: must promise recursive folder inspection'
Require-Text $ClarityPath 'multiple explicit files or folders' 'canon-inspect-clarity: must describe aggregated multi-path inspection'

if ((Get-Content -Raw $ClarityPath) -match 'Run ID:|--run <RUN_ID>|AwaitingApproval') {
  Fail 'canon-inspect-clarity: must not describe run-scoped output or approval-gated state'
}

Forbid-Text $PrReviewPath 'check-runtime.sh --command pr-review --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <BASE_REF> --input <HEAD_REF>' 'canon-pr-review: preflight must not send base/head refs through --input'

Require-Text $RequirementsPath 'preserve valid ownership fields' 'canon-requirements: must describe preserving valid ownership fields across retry'
Require-Text $RequirementsPath 'asks only for the missing slot' 'canon-requirements: must describe single-slot retry behavior'
Require-Text $RequirementsPath 'exact Canon CLI retry form' 'canon-requirements: must promise the exact CLI retry form'
Require-Text $RequirementsPath 'inside Canon execution rather than before Canon execution' 'canon-requirements: must distinguish preflight failures from Canon-execution failures'
Require-Text $RequirementsPath 'guided fixed choices' 'canon-requirements: must require guided choices for enum fields'
Require-Text $RequirementsPath 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-requirements: must list canonical risk choices'
Require-Text $RequirementsPath 'green`, `yellow`, or `red' 'canon-requirements: must list canonical zone choices'
Require-Text $RequirementsPath 'empty, whitespace-only, or structurally insufficient' 'canon-requirements: must describe fail-fast authored-input validation'

Require-Text $ChangePath 'preserve valid ownership fields' 'canon-change: must describe preserving valid ownership fields across retry'
Require-Text $ChangePath 'asks only for the missing brief path or missing ownership slot' 'canon-change: must describe targeted retry behavior'
Require-Text $ChangePath 'exact Canon CLI retry form' 'canon-change: must promise the exact CLI retry form'
Require-Text $ChangePath 'Canon-execution outcome' 'canon-change: must distinguish Canon-execution outcomes from preflight failures'
Require-Text $ChangePath 'preflight failure' 'canon-change: must distinguish Canon-execution outcomes from preflight failures'
Require-Text $ChangePath 'guided fixed choices' 'canon-change: must require guided choices for enum fields'
Require-Text $ChangePath 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-change: must list canonical risk choices'
Require-Text $ChangePath 'green`, `yellow`, or `red' 'canon-change: must list canonical zone choices'
Require-Text $ChangePath 'empty, whitespace-only, or structurally insufficient' 'canon-change: must describe fail-fast authored-input validation'
Require-Text $SystemShapingPath 'guided fixed choices with the exact allowed values `new` and `existing`' 'canon-system-shaping: must list canonical system-context choices'
Require-Text $ArchitecturePath 'guided fixed choices with the exact allowed values `new` and `existing`' 'canon-architecture: must list canonical system-context choices'

Require-Text $ReviewPath 'preserve valid ownership fields' 'canon-review: must describe preserving valid ownership fields across retry'
Require-Text $ReviewPath 'asks only for the missing slot' 'canon-review: must describe single-slot retry behavior'
Require-Text $ReviewPath 'exact Canon CLI retry form' 'canon-review: must promise the exact CLI retry form'
Require-Text $ReviewPath 'inside Canon execution rather than before Canon execution' 'canon-review: must distinguish preflight failures from Canon-execution failures'
Require-Text $ReviewPath 'guided fixed choices' 'canon-review: must require guided choices for enum fields'
Require-Text $ReviewPath 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-review: must list canonical risk choices'
Require-Text $ReviewPath 'green`, `yellow`, or `red' 'canon-review: must list canonical zone choices'
Require-Text $ReviewPath 'canon-input/review.md` or `canon-input/review/' 'canon-review: must document canonical review input locations'
Require-Text $ReviewPath 'do not accept arbitrary code folders such as `src/`' 'canon-review: must reject arbitrary code folders as review inputs'
Require-Text $ReviewPath 'empty, whitespace-only, or structurally insufficient' 'canon-review: must describe fail-fast authored-input validation'

Require-Text $VerificationPath 'preserve valid ownership fields' 'canon-verification: must describe preserving valid ownership fields across retry'
Require-Text $VerificationPath 'asks only for the missing slot' 'canon-verification: must describe single-slot retry behavior'
Require-Text $VerificationPath 'exact Canon CLI retry form' 'canon-verification: must promise the exact CLI retry form'
Require-Text $VerificationPath 'inside Canon execution rather than before Canon execution' 'canon-verification: must distinguish preflight failures from Canon-execution failures'
Require-Text $VerificationPath 'guided fixed choices' 'canon-verification: must require guided choices for enum fields'
Require-Text $VerificationPath 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-verification: must list canonical risk choices'
Require-Text $VerificationPath 'green`, `yellow`, or `red' 'canon-verification: must list canonical zone choices'
Require-Text $VerificationPath 'canon-input/verification.md` or `canon-input/verification/' 'canon-verification: must document canonical verification input locations'
Require-Text $VerificationPath 'empty, whitespace-only, or structurally insufficient' 'canon-verification: must describe fail-fast authored-input validation'

Require-Text $PrReviewPath 'preserves the valid side of the pair' 'canon-pr-review: must describe preserving the valid ref side across retry'
Require-Text $PrReviewPath 'exact Canon CLI form' 'canon-pr-review: must promise the exact CLI form'
Require-Text $PrReviewPath 'accepts local refs plus resolved remote-tracking refs' 'canon-pr-review: must state remote-tracking refs are accepted when they resolve'
Require-Text $PrReviewPath 'inside Canon execution rather than before Canon execution' 'canon-pr-review: must distinguish preflight failures from Canon-execution failures'
Require-Text $PrReviewPath 'guided fixed choices' 'canon-pr-review: must require guided choices for enum fields'
Require-Text $PrReviewPath 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-pr-review: must list canonical risk choices'
Require-Text $PrReviewPath 'green`, `yellow`, or `red' 'canon-pr-review: must list canonical zone choices'
Require-Text $PrReviewPath 'guided choice between `WORKTREE` and providing a different head ref' 'canon-pr-review: must require a guided WORKTREE choice when refs collapse and worktree is dirty'

Forbid-Text $PrReviewPath 'valid file path or ref' 'canon-pr-review: must not blur ref slots with file-path guidance'

if ($Errors -ne 0) {
  exit 1
}

Write-Output "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
