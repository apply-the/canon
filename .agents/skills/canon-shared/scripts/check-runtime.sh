#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REF_DIR="$(cd "${SCRIPT_DIR}/../references" && pwd)"
COMPAT_FILE="${REF_DIR}/runtime-compatibility.toml"

command_name=""
repo_root="$(pwd)"
require_init="false"
run_id=""
owner=""
risk=""
zone=""
inputs=()
refs=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --command)
      command_name="${2:-}"
      shift 2
      ;;
    --repo-root)
      repo_root="${2:-}"
      shift 2
      ;;
    --require-init)
      require_init="true"
      shift
      ;;
    --run-id)
      run_id="${2:-}"
      shift 2
      ;;
    --owner)
      owner="${2:-}"
      shift 2
      ;;
    --risk)
      risk="${2:-}"
      shift 2
      ;;
    --zone)
      zone="${2:-}"
      shift 2
      ;;
    --input)
      inputs+=("${2:-}")
      shift 2
      ;;
    --ref)
      refs+=("${2:-}")
      shift 2
      ;;
    *)
      echo "STATUS=invalid-usage"
      echo "CODE=14"
      echo "MESSAGE=Unknown argument: $1"
      exit 14
      ;;
  esac
done

install_command="$(sed -n 's/^install_command = "\(.*\)"/\1/p' "${COMPAT_FILE}")"
expected_version="$(sed -n 's/^expected_workspace_version = "\(.*\)"/\1/p' "${COMPAT_FILE}")"

emit_failure() {
  echo "STATUS=$1"
  echo "CODE=$2"
  echo "MESSAGE=$3"
  echo "ACTION=$4"
  exit "$2"
}

if ! command -v canon >/dev/null 2>&1; then
  emit_failure "cli-missing" 10 \
    "Canon CLI is not installed or is not on PATH." \
    "Install it with: ${install_command}"
fi

detected_version="unavailable"
version_kind="command-contract"
if canon --version >/dev/null 2>&1; then
  version_kind="semver"
  detected_version="$(canon --version 2>/dev/null | awk '{print $2}')"
  if [[ -n "${expected_version}" && "${detected_version}" != "${expected_version}" ]]; then
    emit_failure "version-incompatible" 11 \
      "Detected Canon version ${detected_version}, expected ${expected_version}." \
      "Reinstall Canon with: ${install_command}"
  fi
else
  probe_output="$(canon inspect modes --output json 2>/dev/null || true)"
  if [[ -z "${probe_output}" ]] || [[ "${probe_output}" != *"requirements"* ]] || [[ "${probe_output}" != *"brownfield-change"* ]] || [[ "${probe_output}" != *"pr-review"* ]]; then
    emit_failure "version-incompatible" 11 \
      "Canon is present, but it does not satisfy the expected CLI command contract for this repo." \
      "Reinstall Canon with: ${install_command}"
  fi
fi

if ! git -C "${repo_root}" rev-parse --show-toplevel >/dev/null 2>&1; then
  emit_failure "wrong-repo-context" 12 \
    "The current working directory is not inside a Git repository." \
    "Switch into the intended repository root before invoking this skill."
fi

if [[ "${require_init}" == "true" ]] && [[ ! -d "${repo_root}/.canon" ]]; then
  emit_failure "repo-not-initialized" 13 \
    "This workflow requires an initialized .canon/ directory." \
    "Run \$canon-init or canon init in ${repo_root} first."
fi

if [[ -n "${run_id}" && "${run_id}" != "<RUN_ID>" ]] && [[ ! -d "${repo_root}/.canon/runs/${run_id}" ]]; then
  emit_failure "missing-input" 14 \
    "Run id ${run_id} was not found under .canon/runs/." \
    "Check the run id and retry the command."
fi

if [[ -n "${owner}" && -z "${risk}" ]]; then
  emit_failure "missing-input" 14 \
    "Risk class is required when owner is provided for a run-starting skill." \
    "Retry with --risk <RISK>."
fi

if [[ -n "${owner}" && -z "${zone}" ]]; then
  emit_failure "missing-input" 14 \
    "Usage zone is required when owner is provided for a run-starting skill." \
    "Retry with --zone <ZONE>."
fi

if (( ${#inputs[@]} > 0 )); then
  for input in "${inputs[@]}"; do
    if [[ -z "${input}" ]]; then
      emit_failure "missing-input" 14 \
        "A required input value was empty." \
        "Provide the missing input and retry."
    fi
    if [[ "${input}" != refs/* ]] && [[ "${input}" != "HEAD" ]] && [[ ! -e "${repo_root}/${input}" ]] && [[ ! -e "${input}" ]]; then
      emit_failure "missing-input" 14 \
        "Input ${input} was not found from ${repo_root}." \
        "Provide an existing file path or retry with a valid ref."
    fi
  done
fi

if (( ${#refs[@]} > 0 )); then
  for ref_value in "${refs[@]}"; do
    if [[ -z "${ref_value}" ]]; then
      emit_failure "missing-input" 14 \
        "A required Git ref was empty." \
        "Provide the missing ref and retry."
    fi
  done
fi

echo "STATUS=ready"
echo "CODE=0"
echo "COMMAND=${command_name}"
echo "REPO_ROOT=${repo_root}"
echo "VERSION_KIND=${version_kind}"
echo "DETECTED_VERSION=${detected_version}"
