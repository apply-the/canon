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

if [[ "${errors}" -ne 0 ]]; then
  exit 1
fi

echo "PASS: Canon skill structure, support-state labels, overlap boundaries, and fake-run protections are valid."
