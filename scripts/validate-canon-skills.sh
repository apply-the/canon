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
  "canon-approve"
  "canon-resume"
  "canon-requirements"
  "canon-brownfield"
  "canon-pr-review"
)

modeled_only=(
  "canon-discovery"
  "canon-greenfield"
  "canon-architecture"
  "canon-implementation"
  "canon-refactor"
  "canon-review"
  "canon-incident"
  "canon-migration"
)

intentionally_limited=("canon-verification")

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

for skill in "${intentionally_limited[@]}"; do
  check_skill "${skill}" "intentionally-limited"
  if grep -Eq 'canon verify --run|Run ID:' "${SKILLS_DIR}/${skill}/SKILL.md"; then
    fail "${skill}: intentionally-limited skill appears to fabricate runnable Canon behavior"
  fi
done

grep -q 'canon-pr-review' "${SKILLS_DIR}/canon-review/SKILL.md" || fail "canon-review: must distinguish itself from canon-pr-review"
grep -q 'canon-brownfield' "${SKILLS_DIR}/canon-refactor/SKILL.md" || fail "canon-refactor: must distinguish itself from canon-brownfield"
grep -q 'canon-requirements' "${SKILLS_DIR}/canon-discovery/SKILL.md" || fail "canon-discovery: must distinguish itself from canon-requirements"
grep -q 'Do not automatically start another Canon skill or `canon run` in the same turn.' "${SKILLS_DIR}/canon-init/SKILL.md" || fail "canon-init: must explicitly forbid chaining into follow-up runs"
if grep -Eq 'Run ID:|State:' "${SKILLS_DIR}/canon-init/SKILL.md"; then
  fail "canon-init: must not describe run-id or run-state output"
fi

requirements_path="${SKILLS_DIR}/canon-requirements/SKILL.md"
brownfield_path="${SKILLS_DIR}/canon-brownfield/SKILL.md"
pr_review_path="${SKILLS_DIR}/canon-pr-review/SKILL.md"

require_text "$requirements_path" '--input <INPUT_PATH>' 'canon-requirements: preflight must keep file-path input binding'
require_text "$brownfield_path" '--input <INPUT_PATH>' 'canon-brownfield: preflight must keep file-path input binding'
require_text "$pr_review_path" '--ref <BASE_REF> --ref <HEAD_REF>' 'canon-pr-review: preflight must use --ref for base/head binding'

forbid_text "$pr_review_path" 'check-runtime.sh --command pr-review --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <BASE_REF> --input <HEAD_REF>' 'canon-pr-review: preflight must not send base/head refs through --input'

require_text "$requirements_path" 'preserve valid ownership fields' 'canon-requirements: must describe preserving valid ownership fields across retry'
require_text "$requirements_path" 'asks only for the missing slot' 'canon-requirements: must describe single-slot retry behavior'
require_text "$requirements_path" 'exact Canon CLI retry form' 'canon-requirements: must promise the exact CLI retry form'
require_text "$requirements_path" 'inside Canon execution rather than before Canon execution' 'canon-requirements: must distinguish preflight failures from Canon-execution failures'

require_text "$brownfield_path" 'preserve valid ownership fields' 'canon-brownfield: must describe preserving valid ownership fields across retry'
require_text "$brownfield_path" 'asks only for the missing brief path or missing ownership slot' 'canon-brownfield: must describe targeted retry behavior'
require_text "$brownfield_path" 'exact Canon CLI retry form' 'canon-brownfield: must promise the exact CLI retry form'
require_text "$brownfield_path" 'Canon-execution outcome' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'
require_text "$brownfield_path" 'preflight failure' 'canon-brownfield: must distinguish Canon-execution outcomes from preflight failures'

require_text "$pr_review_path" 'preserves the valid side of the pair' 'canon-pr-review: must describe preserving the valid ref side across retry'
require_text "$pr_review_path" 'exact Canon CLI form' 'canon-pr-review: must promise the exact CLI form'
require_text "$pr_review_path" 'rejects remote refs explicitly' 'canon-pr-review: must state remote refs are rejected explicitly'
require_text "$pr_review_path" 'inside Canon execution rather than before Canon execution' 'canon-pr-review: must distinguish preflight failures from Canon-execution failures'

forbid_text "$pr_review_path" 'valid file path or ref' 'canon-pr-review: must not blur ref slots with file-path guidance'

if [[ "${errors}" -ne 0 ]]; then
  exit 1
fi

echo "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
