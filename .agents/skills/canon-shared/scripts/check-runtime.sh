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

trim() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "$value"
}

is_placeholder() {
  local placeholder_pattern='^<[^>]+>$'
  [[ "$1" =~ $placeholder_pattern ]]
}

is_missing_value() {
  local value
  value="$(trim "$1")"
  [[ -z "$value" ]] || is_placeholder "$value"
}

emit_result() {
  local status="$1"
  local code="$2"
  local phase="$3"
  local message="$4"
  local action="$5"
  shift 5

  echo "STATUS=${status}"
  echo "CODE=${code}"
  echo "PHASE=${phase}"
  echo "COMMAND=${command_name}"
  echo "REPO_ROOT=${repo_root}"
  echo "MESSAGE=${message}"
  echo "ACTION=${action}"
  while [[ $# -gt 0 ]]; do
    echo "$1"
    shift
  done
}

emit_failure() {
  local status="$1"
  local code="$2"
  local message="$3"
  local action="$4"
  shift 4
  emit_result "$status" "$code" "preflight" "$message" "$action" "$@"
  exit "$code"
}

normalize_risk() {
  case "$1" in
    low-impact|LowImpact) printf '%s' 'low-impact' ;;
    bounded-impact|BoundedImpact) printf '%s' 'bounded-impact' ;;
    systemic-impact|SystemicImpact) printf '%s' 'systemic-impact' ;;
    *) return 1 ;;
  esac
}

normalize_zone() {
  case "$1" in
    green|Green) printf '%s' 'green' ;;
    yellow|Yellow) printf '%s' 'yellow' ;;
    red|Red) printf '%s' 'red' ;;
    *) return 1 ;;
  esac
}

normalize_input_path() {
  local raw="$1"
  if [[ "$raw" == /* ]] && [[ "$raw" == "${repo_root}"/* ]]; then
    printf '%s' "${raw#"${repo_root}/"}"
  else
    printf '%s' "$raw"
  fi
}

local_branch_exists() {
  git -C "${repo_root}" show-ref --verify --quiet "$1"
}

list_local_branches() {
  git -C "${repo_root}" for-each-ref refs/heads --format='%(refname:short)' 2>/dev/null || true
}

list_remote_names() {
  git -C "${repo_root}" remote 2>/dev/null || true
}

is_remote_like_ref() {
  local raw="$1"
  if [[ "$raw" == refs/remotes/* ]]; then
    return 0
  fi

  local remote_name
  local first_segment="${raw%%/*}"
  if [[ "$raw" == */* ]]; then
    while IFS= read -r remote_name; do
      [[ -z "$remote_name" ]] && continue
      if [[ "$first_segment" == "$remote_name" ]]; then
        return 0
      fi
    done < <(list_remote_names)
  fi

  return 1
}

suggest_local_ref() {
  local raw="$1"
  local branches
  branches="$(list_local_branches)"
  case "$raw" in
    main)
      if grep -Fxq 'master' <<<"$branches"; then
        printf '%s' 'master'
      fi
      ;;
    master)
      if grep -Fxq 'main' <<<"$branches"; then
        printf '%s' 'main'
      fi
      ;;
  esac
}

resolve_ref() {
  local raw="$1"
  local slot_name="$2"
  local normalized
  normalized="$(trim "$raw")"
  RESOLVED_REF=""

  if is_missing_value "$normalized"; then
    emit_failure "missing-input" 14 \
      "Required ref slot ${slot_name} is missing." \
      "Retry with --ref <${slot_name^^}>." \
      "FAILED_SLOT=${slot_name}" \
      "FAILED_KIND=RefInput"
  fi

  if [[ "$normalized" == "HEAD" ]]; then
    RESOLVED_REF='HEAD'
    return 0
  fi

  if [[ "$normalized" == refs/heads/* ]]; then
    if local_branch_exists "$normalized"; then
      RESOLVED_REF="$normalized"
      return 0
    fi
  else
    local local_candidate="refs/heads/${normalized}"
    if local_branch_exists "$local_candidate"; then
      RESOLVED_REF="$local_candidate"
      return 0
    fi
  fi

  if is_remote_like_ref "$normalized"; then
    emit_failure "invalid-ref" 16 \
      "Ref ${normalized} is remote-like and unsupported in this patch." \
      "Retry with a local branch, explicit refs/heads/<name>, or HEAD." \
      "FAILED_SLOT=${slot_name}" \
      "FAILED_KIND=unsupported-remote-ref"
  fi

  local suggestion
  suggestion="$(suggest_local_ref "$normalized")"
  local action="Retry with an existing local branch, explicit refs/heads/<name>, or HEAD."
  if [[ -n "$suggestion" ]]; then
    action="Retry with ${suggestion} or explicit refs/heads/${suggestion}."
  fi

  emit_failure "invalid-ref" 16 \
    "Ref ${normalized} did not resolve in the current repository context." \
    "$action" \
    "FAILED_SLOT=${slot_name}" \
    "FAILED_KIND=RefInput"
}

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

run_start_command="false"
run_id_command="false"
pr_review_command="false"

case "${command_name}" in
  requirements|brownfield-change)
    run_start_command="true"
    ;;
  pr-review)
    run_start_command="true"
    pr_review_command="true"
    ;;
  status|inspect-invocations|inspect-evidence|inspect-artifacts|approve|resume)
    run_id_command="true"
    ;;
esac

normalized_run_id=""
normalized_input_1=""
normalized_ref_1=""
normalized_ref_2=""
RESOLVED_REF=""

if [[ "${run_id_command}" == "true" ]]; then
  normalized_run_id="$(trim "${run_id}")"
  if is_missing_value "$normalized_run_id"; then
    emit_failure "missing-input" 14 \
      "Run id is required for ${command_name}." \
      "Retry with --run-id <RUN_ID>." \
      "FAILED_SLOT=run-id" \
      "FAILED_KIND=RunIdInput"
  fi
  if [[ ! -d "${repo_root}/.canon/runs/${normalized_run_id}" ]]; then
    emit_failure "invalid-input" 17 \
      "Run id ${normalized_run_id} was not found under .canon/runs/." \
      "Check the run id and retry with an existing run." \
      "FAILED_SLOT=run-id" \
      "FAILED_KIND=RunIdInput"
  fi
fi

if [[ "${run_start_command}" == "true" ]]; then
  owner="$(trim "${owner}")"
  risk="$(trim "${risk}")"
  zone="$(trim "${zone}")"

  if is_missing_value "$owner"; then
    emit_failure "missing-input" 14 \
      "Owner is required for ${command_name}." \
      "Retry with --owner <OWNER>." \
      "FAILED_SLOT=owner" \
      "FAILED_KIND=OwnerField"
  fi

  if is_missing_value "$risk"; then
    emit_failure "missing-input" 14 \
      "Risk class is required for ${command_name}." \
      "Retry with --risk <RISK>." \
      "FAILED_SLOT=risk" \
      "FAILED_KIND=RiskField"
  fi

  if ! normalized_risk="$(normalize_risk "$risk")"; then
    emit_failure "invalid-input" 17 \
      "Risk class ${risk} is not supported by the Canon runtime contract." \
      "Retry with low-impact, bounded-impact, systemic-impact, or the runtime-recognized aliases LowImpact, BoundedImpact, SystemicImpact." \
      "FAILED_SLOT=risk" \
      "FAILED_KIND=RiskField"
  fi

  if is_missing_value "$zone"; then
    emit_failure "missing-input" 14 \
      "Usage zone is required for ${command_name}." \
      "Retry with --zone <ZONE>." \
      "FAILED_SLOT=zone" \
      "FAILED_KIND=ZoneField"
  fi

  if ! normalized_zone="$(normalize_zone "$zone")"; then
    emit_failure "invalid-input" 17 \
      "Usage zone ${zone} is not supported by the Canon runtime contract." \
      "Retry with green, yellow, red, or the runtime-recognized aliases Green, Yellow, Red." \
      "FAILED_SLOT=zone" \
      "FAILED_KIND=ZoneField"
  fi

  if [[ "${pr_review_command}" == "true" ]]; then
    if (( ${#refs[@]} == 0 )); then
      emit_failure "missing-input" 14 \
        "Base ref is required for pr-review." \
        "Retry with --ref <BASE_REF> --ref <HEAD_REF>." \
        "FAILED_SLOT=base-ref" \
        "FAILED_KIND=RefInput"
    fi

    resolve_ref "${refs[0]}" 'base-ref'
    normalized_ref_1="${RESOLVED_REF}"

    if (( ${#refs[@]} < 2 )); then
      emit_failure "missing-input" 14 \
        "Head ref is required for pr-review." \
        "Retry with --ref <BASE_REF> --ref <HEAD_REF>." \
        "FAILED_SLOT=head-ref" \
        "FAILED_KIND=RefInput" \
        "NORMALIZED_REF_1=${normalized_ref_1}"
    fi

    resolve_ref "${refs[1]}" 'head-ref'
    normalized_ref_2="${RESOLVED_REF}"

    if [[ "${normalized_ref_1}" == "${normalized_ref_2}" ]]; then
      emit_failure "malformed-ref-pair" 18 \
        "Base and head refs normalize to the same Canon binding, so the diff range is empty." \
        "Retry with distinct base and head refs." \
        "FAILED_SLOT=ref-pair" \
        "FAILED_KIND=RefPairInput" \
        "NORMALIZED_REF_1=${normalized_ref_1}" \
        "NORMALIZED_REF_2=${normalized_ref_2}"
    fi
  else
    if (( ${#inputs[@]} == 0 )); then
      emit_failure "missing-input" 14 \
        "Input path is required for ${command_name}." \
        "Retry with --input <INPUT_PATH>." \
        "FAILED_SLOT=input-path" \
        "FAILED_KIND=FilePathInput"
    fi

    local_input="$(trim "${inputs[0]}")"
    if is_missing_value "$local_input"; then
      emit_failure "missing-input" 14 \
        "Input path is required for ${command_name}." \
        "Retry with --input <INPUT_PATH>." \
        "FAILED_SLOT=input-path" \
        "FAILED_KIND=FilePathInput"
    fi

    if [[ ! -e "${repo_root}/${local_input}" ]] && [[ ! -e "${local_input}" ]]; then
      emit_failure "missing-file" 15 \
        "Input ${local_input} was not found from ${repo_root}." \
        "Retry with an existing file path." \
        "FAILED_SLOT=input-path" \
        "FAILED_KIND=FilePathInput"
    fi

    normalized_input_1="$(normalize_input_path "$local_input")"
  fi
fi

extras=(
  "VERSION_KIND=${version_kind}"
  "DETECTED_VERSION=${detected_version}"
)

if [[ -n "${normalized_run_id}" ]]; then
  extras+=("NORMALIZED_RUN_ID=${normalized_run_id}")
fi
if [[ -n "${normalized_input_1}" ]]; then
  extras+=("NORMALIZED_INPUT_1=${normalized_input_1}")
fi
if [[ -n "${normalized_ref_1}" ]]; then
  extras+=("NORMALIZED_REF_1=${normalized_ref_1}")
fi
if [[ -n "${normalized_ref_2}" ]]; then
  extras+=("NORMALIZED_REF_2=${normalized_ref_2}")
fi

emit_result "ready" 0 "preflight" \
  "Typed preflight checks passed." \
  "Invoke Canon using the normalized contract for this command." \
  "${extras[@]}"
