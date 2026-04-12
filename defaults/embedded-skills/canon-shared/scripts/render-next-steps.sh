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

print_recommendation() {
  local recommended="$1"
  shift

  echo "Recommended Next Step:"
  echo "- ${recommended}"
  if [[ $# -gt 0 ]]; then
    echo
    echo "Possible Actions:"
    while [[ $# -gt 0 ]]; do
      echo "- $1"
      shift
    done
  fi
}

case "${profile}" in
  run-started)
    print_recommendation \
      "Use \$canon-status for run ${run_id}." \
      "Use \$canon-inspect-invocations for request-level decisions." \
      "Use \$canon-inspect-evidence for evidence lineage."
    ;;
  status-completed)
    print_recommendation \
      "Use \$canon-inspect-evidence for evidence lineage on run ${run_id}." \
      "Use \$canon-inspect-invocations for request-level decisions on run ${run_id}." \
      "Use \$canon-inspect-artifacts if you need emitted file paths."
    ;;
  status-gated)
    print_recommendation \
      "Use \$canon-inspect-evidence for run ${run_id} before recording approval." \
      "Use \$canon-approve for target ${target} on run ${run_id} after review." \
      "Use \$canon-status after approval, or \$canon-resume if Canon still requires continuation."
    ;;
  inspect)
    print_recommendation \
      "Use \$canon-status for the latest state of run ${run_id}." \
      "Use \$canon-inspect-artifacts if you need emitted file paths."
    ;;
  approval-recorded)
    print_recommendation \
      "Use \$canon-resume for run ${run_id} only if Canon still requires continuation." \
      "Use \$canon-status to confirm the post-approval run state."
    ;;
  gated)
    print_recommendation \
      "Use \$canon-inspect-evidence for run ${run_id} before recording approval." \
      "Use \$canon-approve for target ${target} on run ${run_id} after review." \
      "Use \$canon-status after approval, or \$canon-resume if Canon still requires continuation."
    ;;
  resumed)
    print_recommendation \
      "Use \$canon-inspect-evidence to review the updated evidence bundle for run ${run_id}." \
      "Use \$canon-status for the resumed run ${run_id}."
    ;;
  inspect-artifacts)
    print_recommendation \
      "Use \$canon-inspect-evidence for linked runtime evidence on run ${run_id}." \
      "Use \$canon-status for the latest state of run ${run_id}."
    ;;
  *)
    print_recommendation "Use \$canon-status to inspect the current Canon run."
    ;;
esac
