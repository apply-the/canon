#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SKILLS_DIR="${ROOT}/.agents/skills"

required_sections=(
  "## Support State"
  "## Purpose"
  "## When To Trigger"
  "## When It Must Not Trigger"
  "## Required Inputs"
  "## Preflight Profile"
  "## Canon Command Contract"
  "## Expected Output Shape"
  "## Failure Handling Guidance"
  "## Next-Step Guidance"
  "## Related Skills"
)

available_now=(
  "canon-init"
  "canon-status"
  "canon-inspect-invocations"
  "canon-inspect-evidence"
  "canon-inspect-artifacts"
  "canon-inspect-clarity"
  "canon-approve"
  "canon-resume"
  "canon-requirements"
  "canon-discovery"
  "canon-system-shaping"
  "canon-architecture"
  "canon-brownfield"
  "canon-review"
  "canon-verification"
  "canon-pr-review"
)

modeled_only=(
  "canon-implementation"
  "canon-refactor"
  "canon-incident"
  "canon-migration"
)

intentionally_limited=()

require_text() {
  local path="$1"
  local pattern="$2"
  local message="$3"
  grep -Fq -- "$pattern" "$path" || fail "$message"
}

forbid_text() {
  local path="$1"
  local pattern="$2"
  local message="$3"
  if grep -Fq -- "$pattern" "$path"; then
    fail "$message"
  fi
}

errors=0

fail() {
  echo "FAIL: $1" >&2
  errors=$((errors + 1))
}

check_skill() {
  local skill="$1"
  local expected_state="$2"
  local path="${SKILLS_DIR}/${skill}/SKILL.md"

  [[ -f "${path}" ]] || { fail "Missing skill file: ${path}"; return; }
  grep -q '^---$' "${path}" || fail "${skill}: missing frontmatter fence"
  grep -q "^name: ${skill}$" "${path}" || fail "${skill}: frontmatter name mismatch"
  grep -q '^description: Use when ' "${path}" || fail "${skill}: description must start with 'Use when '"

  for section in "${required_sections[@]}"; do
    grep -q "^${section}$" "${path}" || fail "${skill}: missing section ${section}"
  done

  grep -q "\`${expected_state}\`" "${path}" || fail "${skill}: expected support state ${expected_state}"
}

for skill in "${available_now[@]}"; do
  check_skill "${skill}" "available-now"
done

for skill in "${modeled_only[@]}"; do
  check_skill "${skill}" "modeled-only"
  if grep -Eq 'canon run --mode|Run ID:|--run <RUN_ID>|gate:|invocation:' "${SKILLS_DIR}/${skill}/SKILL.md"; then
    fail "${skill}: modeled-only skill appears to fabricate runnable Canon behavior"
  fi
done

if ((${#intentionally_limited[@]} > 0)); then
  for skill in "${intentionally_limited[@]}"; do
    check_skill "${skill}" "intentionally-limited"
    if grep -Eq 'canon verify --run|Run ID:' "${SKILLS_DIR}/${skill}/SKILL.md"; then
      fail "${skill}: intentionally-limited skill appears to fabricate runnable Canon behavior"
    fi
  done
fi

grep -q 'canon-pr-review' "${SKILLS_DIR}/canon-review/SKILL.md" || fail "canon-review: must distinguish itself from canon-pr-review"
grep -q 'canon-brownfield' "${SKILLS_DIR}/canon-refactor/SKILL.md" || fail "canon-refactor: must distinguish itself from canon-brownfield"
grep -q 'canon-requirements' "${SKILLS_DIR}/canon-discovery/SKILL.md" || fail "canon-discovery: must distinguish itself from canon-requirements"
grep -q 'Do not automatically start another Canon skill or `canon run` in the same turn.' "${SKILLS_DIR}/canon-init/SKILL.md" || fail "canon-init: must explicitly forbid chaining into follow-up runs"
if grep -Eq 'Run ID:|State:' "${SKILLS_DIR}/canon-init/SKILL.md"; then
  fail "canon-init: must not describe run-id or run-state output"
fi

requirements_path="${SKILLS_DIR}/canon-requirements/SKILL.md"
brownfield_path="${SKILLS_DIR}/canon-brownfield/SKILL.md"
review_path="${SKILLS_DIR}/canon-review/SKILL.md"
verification_path="${SKILLS_DIR}/canon-verification/SKILL.md"
pr_review_path="${SKILLS_DIR}/canon-pr-review/SKILL.md"
clarity_path="${SKILLS_DIR}/canon-inspect-clarity/SKILL.md"

require_text "$requirements_path" '--input <INPUT_PATH>' 'canon-requirements: preflight must keep file-path input binding'
require_text "$requirements_path" '--input-text <INPUT_TEXT>' 'canon-requirements: must document inline authored input binding'
require_text "$brownfield_path" '--input <INPUT_PATH>' 'canon-brownfield: preflight must keep file-path input binding'
require_text "$brownfield_path" '--input-text <INPUT_TEXT>' 'canon-brownfield: must document inline authored input binding'
require_text "$review_path" '--input <INPUT_PATH>' 'canon-review: preflight must keep file-path input binding'
require_text "$review_path" '--input-text <INPUT_TEXT>' 'canon-review: must document inline authored input binding'
require_text "$verification_path" '--input <INPUT_PATH>' 'canon-verification: preflight must keep file-path input binding'
require_text "$verification_path" '--input-text <INPUT_TEXT>' 'canon-verification: must document inline authored input binding'
require_text "$pr_review_path" '--ref <BASE_REF> --ref <HEAD_REF>' 'canon-pr-review: preflight must use --ref for base/head binding'
require_text "$clarity_path" 'canon inspect clarity --mode <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]' 'canon-inspect-clarity: must promise the exact Canon CLI form'
require_text "$clarity_path" '.canon/` is not required for this inspection surface' 'canon-inspect-clarity: must stay honest that this inspect surface is pre-run and does not require runtime state'
require_text "$clarity_path" 'Preserve the already valid mode or input selection' 'canon-inspect-clarity: must preserve valid mode or input slots across retry'
require_text "$clarity_path" 'Do not fabricate a started run, pending approval, or emitted artifact set' 'canon-inspect-clarity: must forbid fake run state'
require_text "$clarity_path" 'prefer the directory when both exist' 'canon-inspect-clarity: must prefer canonical directories over a single child file when both canonical surfaces exist'
require_text "$clarity_path" 'whole directory recursively' 'canon-inspect-clarity: must promise recursive folder inspection'
require_text "$clarity_path" 'multiple explicit files or folders' 'canon-inspect-clarity: must describe aggregated multi-path inspection'

if grep -Eq 'Run ID:|--run <RUN_ID>|AwaitingApproval' "$clarity_path"; then
  fail 'canon-inspect-clarity: must not describe run-scoped output or approval-gated state'
fi

forbid_text "$pr_review_path" 'check-runtime.sh --command pr-review --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <BASE_REF> --input <HEAD_REF>' 'canon-pr-review: preflight must not send base/head refs through --input'

require_text "$requirements_path" 'preserve valid ownership fields' 'canon-requirements: must describe preserving valid ownership fields across retry'
require_text "$requirements_path" 'asks only for the missing slot' 'canon-requirements: must describe single-slot retry behavior'
require_text "$requirements_path" 'exact Canon CLI retry form' 'canon-requirements: must promise the exact CLI retry form'
require_text "$requirements_path" 'inside Canon execution rather than before Canon execution' 'canon-requirements: must distinguish preflight failures from Canon-execution failures'
require_text "$requirements_path" 'guided fixed choices' 'canon-requirements: must require guided choices for enum fields'
require_text "$requirements_path" 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-requirements: must list canonical risk choices'
require_text "$requirements_path" 'green`, `yellow`, or `red' 'canon-requirements: must list canonical zone choices'
require_text "$requirements_path" 'empty, whitespace-only, or structurally insufficient' 'canon-requirements: must describe fail-fast authored-input validation'

require_text "$brownfield_path" 'preserve valid ownership fields' 'canon-brownfield: must describe preserving valid ownership fields across retry'
require_text "$brownfield_path" 'asks only for the missing brief path or missing ownership slot' 'canon-brownfield: must describe targeted retry behavior'
require_text "$brownfield_path" 'exact Canon CLI retry form' 'canon-brownfield: must promise the exact CLI retry form'
require_text "$brownfield_path" 'Canon-execution outcome' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'
require_text "$brownfield_path" 'preflight failure' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'
require_text "$brownfield_path" 'guided fixed choices' 'canon-brownfield: must require guided choices for enum fields'
require_text "$brownfield_path" 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-brownfield: must list canonical risk choices'
require_text "$brownfield_path" 'green`, `yellow`, or `red' 'canon-brownfield: must list canonical zone choices'
require_text "$brownfield_path" 'empty, whitespace-only, or structurally insufficient' 'canon-brownfield: must describe fail-fast authored-input validation'

require_text "$review_path" 'preserve valid ownership fields' 'canon-review: must describe preserving valid ownership fields across retry'
require_text "$review_path" 'asks only for the missing slot' 'canon-review: must describe single-slot retry behavior'
require_text "$review_path" 'exact Canon CLI retry form' 'canon-review: must promise the exact CLI retry form'
require_text "$review_path" 'inside Canon execution rather than before Canon execution' 'canon-review: must distinguish preflight failures from Canon-execution failures'
require_text "$review_path" 'guided fixed choices' 'canon-review: must require guided choices for enum fields'
require_text "$review_path" 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-review: must list canonical risk choices'
require_text "$review_path" 'green`, `yellow`, or `red' 'canon-review: must list canonical zone choices'
require_text "$review_path" 'canon-input/review.md` or `canon-input/review/' 'canon-review: must document canonical review input locations'
require_text "$review_path" 'do not accept arbitrary code folders such as `src/`' 'canon-review: must reject arbitrary code folders as review inputs'
require_text "$review_path" 'empty, whitespace-only, or structurally insufficient' 'canon-review: must describe fail-fast authored-input validation'

require_text "$verification_path" 'preserve valid ownership fields' 'canon-verification: must describe preserving valid ownership fields across retry'
require_text "$verification_path" 'asks only for the missing slot' 'canon-verification: must describe single-slot retry behavior'
require_text "$verification_path" 'exact Canon CLI retry form' 'canon-verification: must promise the exact CLI retry form'
require_text "$verification_path" 'inside Canon execution rather than before Canon execution' 'canon-verification: must distinguish preflight failures from Canon-execution failures'
require_text "$verification_path" 'guided fixed choices' 'canon-verification: must require guided choices for enum fields'
require_text "$verification_path" 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-verification: must list canonical risk choices'
require_text "$verification_path" 'green`, `yellow`, or `red' 'canon-verification: must list canonical zone choices'
require_text "$verification_path" 'canon-input/verification.md` or `canon-input/verification/' 'canon-verification: must document canonical verification input locations'
require_text "$verification_path" 'empty, whitespace-only, or structurally insufficient' 'canon-verification: must describe fail-fast authored-input validation'

require_text "$pr_review_path" 'preserves the valid side of the pair' 'canon-pr-review: must describe preserving the valid ref side across retry'
require_text "$pr_review_path" 'exact Canon CLI form' 'canon-pr-review: must promise the exact CLI form'
require_text "$pr_review_path" 'accepts local refs plus resolved remote-tracking refs' 'canon-pr-review: must state remote-tracking refs are accepted when they resolve'
require_text "$pr_review_path" 'inside Canon execution rather than before Canon execution' 'canon-pr-review: must distinguish preflight failures from Canon-execution failures'
require_text "$pr_review_path" 'guided fixed choices' 'canon-pr-review: must require guided choices for enum fields'
require_text "$pr_review_path" 'low-impact`, `bounded-impact`, or `systemic-impact' 'canon-pr-review: must list canonical risk choices'
require_text "$pr_review_path" 'green`, `yellow`, or `red' 'canon-pr-review: must list canonical zone choices'
require_text "$pr_review_path" 'guided choice between `WORKTREE` and providing a different head ref' 'canon-pr-review: must require a guided WORKTREE choice when refs collapse and worktree is dirty'

forbid_text "$pr_review_path" 'valid file path or ref' 'canon-pr-review: must not blur ref slots with file-path guidance'

if [[ "${errors}" -ne 0 ]]; then
  exit 1
fi

echo "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
