#!/usr/bin/env bash
set -euo pipefail

profile=""
run_id=""
target=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) profile="${2:-}"; shift 2 ;;
    --run-id) run_id="${2:-}"; shift 2 ;;
    --target) target="${2:-}"; shift 2 ;;
    *) echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done

case "${profile}" in
  run-started)
    echo "Next:"
    echo "- Use \$canon-status for run ${run_id}."
    echo "- Use \$canon-inspect-invocations for request-level decisions."
    echo "- Use \$canon-inspect-evidence for evidence lineage."
    ;;
  status-completed)
    echo "Next:"
    echo "- Use \$canon-inspect-invocations for request-level decisions on run ${run_id}."
    echo "- Use \$canon-inspect-evidence for evidence lineage."
    echo "- Use \$canon-inspect-artifacts if you need emitted file paths."
    ;;
  status-gated)
    echo "Next:"
    echo "- Use \$canon-approve for target ${target} on run ${run_id}."
    echo "- Use \$canon-status after approval, or \$canon-resume if Canon still requires continuation."
    ;;
  inspect)
    echo "Next:"
    echo "- Use \$canon-status for the latest state of run ${run_id}."
    echo "- Use \$canon-inspect-artifacts if you need emitted file paths."
    ;;
  approval-recorded)
    echo "Next:"
    echo "- Use \$canon-status to confirm the post-approval run state."
    echo "- Use \$canon-resume for run ${run_id} only if Canon still requires continuation."
    ;;
  gated)
    echo "Next:"
    echo "- Use \$canon-approve for target ${target} on run ${run_id}."
    echo "- Use \$canon-status after approval, or \$canon-resume if Canon still requires continuation."
    ;;
  resumed)
    echo "Next:"
    echo "- Use \$canon-status for the resumed run ${run_id}."
    echo "- Use \$canon-inspect-evidence to review the updated evidence bundle."
    ;;
  inspect-artifacts)
    echo "Next:"
    echo "- Use \$canon-inspect-evidence for linked runtime evidence."
    echo "- Use \$canon-status for the latest state of run ${run_id}."
    ;;
  *)
    echo "Next:"
    echo "- Use \$canon-status to inspect the current Canon run."
    ;;
esac
