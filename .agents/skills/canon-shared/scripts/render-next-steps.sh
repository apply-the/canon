#!/usr/bin/env bash
set -euo pipefail

profile=""
run_id=""
target=""
primary_artifact_path=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) profile="${2:-}"; shift 2 ;;
    --run-id) run_id="${2:-}"; shift 2 ;;
    --target) target="${2:-}"; shift 2 ;;
    --primary-artifact-path) primary_artifact_path="${2:-}"; shift 2 ;;
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
      "None. Review the returned Canon summary first for run ${run_id}." \
      "Use \$canon-status for run ${run_id} only if you need to refresh the run state." \
      "Use \$canon-inspect-invocations for request-level decisions." \
      "Use \$canon-inspect-evidence only if you need evidence lineage."
    ;;
  status-completed)
    if [[ -n "${primary_artifact_path}" ]]; then
      print_recommendation \
        "None. The run result is already readable for run ${run_id}." \
        "Open the primary artifact at ${primary_artifact_path} directly when your host supports it." \
        "Use \$canon-inspect-artifacts for the full emitted packet on run ${run_id}." \
        "Use \$canon-inspect-evidence only if you need lineage or policy rationale for run ${run_id}."
    else
      print_recommendation \
        "None. The run result is already readable for run ${run_id}." \
        "Use \$canon-inspect-artifacts for the full emitted packet on run ${run_id}." \
        "Use \$canon-inspect-evidence only if you need lineage or policy rationale for run ${run_id}."
    fi
    ;;
  status-gated)
    print_recommendation \
      "Use \$canon-inspect-evidence for run ${run_id} before recording approval." \
      "Use \$canon-approve for target ${target} on run ${run_id} after review." \
      "Use \$canon-status after approval, or \$canon-resume if Canon still requires continuation."
    ;;
  inspect)
    print_recommendation \
      "None. Review the current inspection output directly for run ${run_id}." \
      "Use \$canon-status only if you need to re-check the run state after follow-up work."
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
      "None. Review the emitted packet directly for run ${run_id}." \
      "Use \$canon-inspect-evidence only if you still need runtime lineage or policy rationale for run ${run_id}." \
      "Use \$canon-status only after follow-up work changes what you expect from run ${run_id}."
    ;;
  *)
    print_recommendation "Use \$canon-status to inspect the current Canon run."
    ;;
esac
