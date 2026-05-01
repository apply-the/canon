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
system_context=""
normalized_risk=""
normalized_zone=""
normalized_system_context=""
inferred_risk=""
inferred_zone=""
inference_confidence=""
inference_headline=""
inference_rationale=""
risk_rationale=""
zone_rationale=""
risk_was_supplied=""
zone_was_supplied=""
inputs=()
inline_inputs=()
refs=()
inference_signals=()
risk_signals=()
zone_signals=()

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

normalize_system_context() {
  case "$1" in
    new|New) printf '%s' 'new' ;;
    existing|Existing) printf '%s' 'existing' ;;
    *) return 1 ;;
  esac
}

infer_classification() {
  local mode="$1"
  shift

  local -a cmd=(canon inspect risk-zone --mode "$mode" --output text)
  if [[ -n "${normalized_risk}" ]]; then
    cmd+=(--risk "$normalized_risk")
  fi
  if [[ -n "${normalized_zone}" ]]; then
    cmd+=(--zone "$normalized_zone")
  fi
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --input|--input-text)
        cmd+=("$1" "${2:-}")
        shift 2
        ;;
      *)
        cmd+=(--input "$1")
        shift
        ;;
    esac
  done

  local output
  if ! output="$("${cmd[@]}" 2>/dev/null)"; then
    emit_failure "classification-unavailable" 20 \
      "Canon could not infer risk and zone from the supplied intake." \
      "Provide --risk and --zone explicitly, or fix the authored input surface before retrying." \
      "FAILED_KIND=ClassificationInference"
  fi

  inferred_risk=""
  inferred_zone=""
  inference_confidence=""
  inference_headline=""
  inference_rationale=""
  risk_rationale=""
  zone_rationale=""
  risk_was_supplied=""
  zone_was_supplied=""
  inference_signals=()
  risk_signals=()
  zone_signals=()

  while IFS='=' read -r key value; do
    case "$key" in
      INFERRED_RISK) inferred_risk="$value" ;;
      INFERRED_ZONE) inferred_zone="$value" ;;
      INFERENCE_CONFIDENCE) inference_confidence="$value" ;;
      INFERENCE_HEADLINE) inference_headline="$value" ;;
      INFERENCE_RATIONALE) inference_rationale="$value" ;;
      RISK_RATIONALE) risk_rationale="$value" ;;
      ZONE_RATIONALE) zone_rationale="$value" ;;
      RISK_WAS_SUPPLIED) risk_was_supplied="$value" ;;
      ZONE_WAS_SUPPLIED) zone_was_supplied="$value" ;;
      SIGNAL_*) inference_signals+=("$value") ;;
      RISK_SIGNAL_*) risk_signals+=("$value") ;;
      ZONE_SIGNAL_*) zone_signals+=("$value") ;;
    esac
  done <<<"$output"

  if [[ -z "${inferred_risk}" || -z "${inferred_zone}" ]]; then
    emit_failure "classification-unavailable" 20 \
      "Canon returned an incomplete risk/zone inference payload." \
      "Provide --risk and --zone explicitly, or inspect the authored intake before retrying." \
      "FAILED_KIND=ClassificationInference"
  fi
}

normalize_input_path() {
  local raw="$1"
  if [[ "$raw" == /* ]] && [[ "$raw" == "${repo_root}"/* ]]; then
    printf '%s' "${raw#"${repo_root}/"}"
  else
    printf '%s' "$raw"
  fi
}

resolve_existing_input_path() {
  local raw="$1"
  local candidate="$raw"
  if [[ "$candidate" != /* ]]; then
    candidate="${repo_root}/${candidate}"
  fi

  [[ -e "$candidate" ]] || return 1

  local base_name
  base_name="$(basename "$candidate")"
  local parent_dir
  parent_dir="$(cd "$(dirname "$candidate")" && pwd -P)"
  printf '%s/%s' "$parent_dir" "$base_name"
}

authored_input_retry_action() {
  local input_hint=""
  input_hint="$(canonical_mode_input_hint 2>/dev/null || true)"

  if [[ "${command_name}" == "review" ]]; then
    if [[ -n "${input_hint}" ]]; then
      printf '%s' "Retry with ${input_hint}, or pass exactly one non-empty --input-text note."
    else
      printf '%s' 'Retry with exactly one non-empty authored input or exactly one non-empty --input-text note.'
    fi
    return
  fi

  if [[ -n "${input_hint}" ]]; then
    printf '%s' "Retry with ${input_hint}, another non-empty authored path, or non-empty --input-text."
    return
  fi

  printf '%s' 'Retry with a non-empty authored file path or non-empty --input-text.'
}

authored_input_content_state() {
  local resolved="$1"

  if [[ -d "$resolved" ]]; then
    local found_file="false"
    while IFS= read -r -d '' file; do
      found_file="true"
      if grep -q '[^[:space:]]' "$file" 2>/dev/null; then
        printf '%s' 'usable'
        return 0
      fi
    done < <(find "$resolved" -type f -print0 2>/dev/null)

    if [[ "$found_file" == "true" ]]; then
      printf '%s' 'whitespace-only'
    else
      printf '%s' 'empty-dir'
    fi
    return 0
  fi

  if grep -q '[^[:space:]]' "$resolved" 2>/dev/null; then
    printf '%s' 'usable'
  else
    printf '%s' 'whitespace-only'
  fi
}

canonical_mode_input_hint() {
  case "$command_name" in
    requirements|discovery|architecture)
      printf '%s' "canon-input/${command_name}.md or canon-input/${command_name}/"
      ;;
    review|verification)
      printf '%s' "canon-input/${command_name}.md or canon-input/${command_name}/"
      ;;
    system-shaping)
      printf '%s' 'canon-input/system-shaping.md or canon-input/system-shaping/'
      ;;
    change)
      printf '%s' 'canon-input/change.md or canon-input/change/'
      ;;
    implementation|refactor|incident|security-assessment|system-assessment|migration|supply-chain-analysis)
      printf '%s' "canon-input/${command_name}.md or canon-input/${command_name}/"
      ;;
    *)
      return 1
      ;;
  esac
}

local_branch_exists() {
  git -C "${repo_root}" show-ref --verify --quiet "$1"
}

remote_tracking_ref_exists() {
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

canonical_remote_ref() {
  local raw="$1"

  if [[ "$raw" == refs/remotes/* ]]; then
    printf '%s' "$raw"
    return 0
  fi

  if is_remote_like_ref "$raw"; then
    printf '%s' "refs/remotes/${raw}"
    return 0
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

  local remote_candidate=""
  if remote_candidate="$(canonical_remote_ref "$normalized")"; then
    if remote_tracking_ref_exists "$remote_candidate"; then
      RESOLVED_REF="$remote_candidate"
      return 0
    fi
  fi

  local suggestion
  suggestion="$(suggest_local_ref "$normalized")"
  local action="Retry with an existing local branch, a fetched remote-tracking ref, explicit refs/heads/<name>, explicit refs/remotes/<remote>/<name>, or HEAD."
  if [[ -n "$suggestion" ]]; then
    action="Retry with ${suggestion}, explicit refs/heads/${suggestion}, a fetched remote-tracking ref such as origin/${suggestion}, or HEAD."
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
    --system-context)
      system_context="${2:-}"
      shift 2
      ;;
    --input)
      inputs+=("${2:-}")
      shift 2
      ;;
    --input-text)
      inline_inputs+=("${2:-}")
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
install_guidance_ref="$(sed -n 's/^install_guidance_ref = "\(.*\)"/\1/p' "${COMPAT_FILE}")"
release_surface="$(sed -n 's/^release_surface = "\(.*\)"/\1/p' "${COMPAT_FILE}")"
expected_version="$(sed -n 's/^expected_workspace_version = "\(.*\)"/\1/p' "${COMPAT_FILE}")"

install_action() {
  if [[ -n "$install_guidance_ref" && -n "$release_surface" ]]; then
    printf '%s' "Follow the install guide at ${install_guidance_ref} and download the matching release from ${release_surface}."
    return
  fi

  if [[ -n "$install_guidance_ref" ]]; then
    printf '%s' "Follow the install guide at ${install_guidance_ref}."
    return
  fi

  if [[ -n "$install_command" ]]; then
    printf '%s' "Install it with: ${install_command}"
    return
  fi

  printf '%s' 'Install Canon and ensure it is on PATH.'
}

reinstall_action() {
  if [[ -n "$install_guidance_ref" && -n "$release_surface" ]]; then
    printf '%s' "Follow the install guide at ${install_guidance_ref} and download the matching release from ${release_surface}."
    return
  fi

  if [[ -n "$install_guidance_ref" ]]; then
    printf '%s' "Follow the install guide at ${install_guidance_ref}."
    return
  fi

  if [[ -n "$install_command" ]]; then
    printf '%s' "Reinstall Canon with: ${install_command}"
    return
  fi

  printf '%s' 'Install or update Canon, then rerun canon --version.'
}

if ! command -v canon >/dev/null 2>&1; then
  emit_failure "cli-missing" 10 \
    "Canon CLI is not installed or is not on PATH." \
    "$(install_action)"
fi

detected_version="unavailable"
version_kind="command-contract"
if canon --version >/dev/null 2>&1; then
  version_kind="semver"
  detected_version="$(canon --version 2>/dev/null | awk '{print $2}')"
  if [[ -n "${expected_version}" && "${detected_version}" != "${expected_version}" ]]; then
    emit_failure "version-incompatible" 11 \
      "Detected Canon version ${detected_version}, expected ${expected_version}." \
      "$(reinstall_action)"
  fi
else
  probe_output="$(canon inspect modes --output json 2>/dev/null || true)"
  if [[ -z "${probe_output}" ]] || [[ "${probe_output}" != *"requirements"* ]] || [[ "${probe_output}" != *"change"* ]] || [[ "${probe_output}" != *"review"* ]] || [[ "${probe_output}" != *"verification"* ]] || [[ "${probe_output}" != *"pr-review"* ]]; then
    emit_failure "version-incompatible" 11 \
      "Canon is present, but it does not satisfy the expected CLI command contract for this repo." \
      "$(reinstall_action)"
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
  requirements|discovery|system-shaping|architecture|change|review|verification)
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
normalized_inline_input_1=""
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
  system_context="$(trim "${system_context}")"

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
    authored_input_count=$(( ${#inputs[@]} + ${#inline_inputs[@]} ))
    if (( authored_input_count == 0 )); then
      emit_failure "missing-input" 14 \
        "Authored input is required for ${command_name}." \
        "Retry with --input <INPUT_PATH> or --input-text <INPUT_TEXT>." \
        "FAILED_SLOT=input-path" \
        "FAILED_KIND=FilePathInput"
    fi

    if [[ "${command_name}" == "review" ]] && (( authored_input_count != 1 )); then
      emit_failure "invalid-input" 17 \
        "Review requires exactly one authored input at canon-input/review.md or canon-input/review/, or exactly one explicit --input-text note." \
        "Retry with canon-input/review.md or canon-input/review/, or pass exactly one --input-text note." \
        "FAILED_SLOT=input-path" \
        "FAILED_KIND=FilePathInput"
    fi

    if (( ${#inline_inputs[@]} > 0 )); then
      for inline_input in "${inline_inputs[@]}"; do
        trimmed_inline_input="$(trim "${inline_input}")"
        if is_missing_value "$trimmed_inline_input"; then
          emit_failure "invalid-input" 17 \
            "Inline authored input for ${command_name} is empty or whitespace-only." \
            "Retry with non-empty --input-text content." \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
        fi

        if [[ -z "${normalized_inline_input_1}" ]]; then
          normalized_inline_input_1="$trimmed_inline_input"
        fi
      done
    fi

    if (( ${#inputs[@]} > 0 )); then
      for input in "${inputs[@]}"; do
        local_input="$(trim "${input}")"
        if is_missing_value "$local_input"; then
          emit_failure "missing-input" 14 \
            "Input path is required for ${command_name}." \
            "Retry with --input <INPUT_PATH> or --input-text <INPUT_TEXT>." \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
        fi

      if [[ ! -e "${repo_root}/${local_input}" ]] && [[ ! -e "${local_input}" ]]; then
        if [[ "${command_name}" == "review" ]]; then
          emit_failure "missing-file" 15 \
            "Review input ${local_input} was not found from ${repo_root}." \
            "Retry with canon-input/review.md or canon-input/review/, or pass exactly one --input-text note." \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
        fi
        emit_failure "missing-file" 15 \
          "Input ${local_input} was not found from ${repo_root}." \
          "Retry with an existing file path or non-empty --input-text." \
          "FAILED_SLOT=input-path" \
          "FAILED_KIND=FilePathInput"
      fi

      resolved_input="$(resolve_existing_input_path "$local_input")"
      canon_root=""
      if [[ -d "${repo_root}/.canon" ]]; then
        canon_root="$(cd "${repo_root}/.canon" && pwd -P)"
      fi
      if [[ -n "${canon_root}" ]] && \
        ([[ "${resolved_input}" == "${canon_root}" ]] || [[ "${resolved_input}" == "${canon_root}/"* ]]); then
        input_hint=""
        if input_hint="$(canonical_mode_input_hint 2>/dev/null)"; then
          input_action="Retry with ${input_hint}, another authored file path outside .canon/, or non-empty --input-text."
        else
          input_action="Retry with an authored file path outside .canon/ or non-empty --input-text."
        fi
        emit_failure "invalid-input" 17 \
          "Input ${local_input} points inside .canon/ and cannot be used as authored input for ${command_name}." \
          "${input_action}" \
          "FAILED_SLOT=input-path" \
          "FAILED_KIND=FilePathInput"
      fi

      if [[ "${command_name}" == "review" ]]; then
        resolved_review_file=""
        resolved_review_dir=""
        if [[ -e "${repo_root}/canon-input/review.md" ]]; then
          resolved_review_file="$(resolve_existing_input_path "canon-input/review.md")"
        fi
        if [[ -e "${repo_root}/canon-input/review" ]]; then
          resolved_review_dir="$(resolve_existing_input_path "canon-input/review")"
        fi
        if [[ "${resolved_input}" != "${resolved_review_file}" ]] && [[ "${resolved_input}" != "${resolved_review_dir}" ]]; then
          emit_failure "invalid-input" 17 \
            "Review accepts only canon-input/review.md or canon-input/review/, not ${local_input}." \
            "Move or author the review packet at canon-input/review.md or canon-input/review/, or pass exactly one --input-text note, then retry." \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
        fi
      fi

      input_content_state="$(authored_input_content_state "$resolved_input")"
      case "$input_content_state" in
        empty-dir)
          emit_failure "invalid-input" 17 \
            "Input ${local_input} expands to files with no usable authored content." \
            "$(authored_input_retry_action)" \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
          ;;
        whitespace-only)
          emit_failure "invalid-input" 17 \
            "Input ${local_input} is empty or whitespace-only." \
            "$(authored_input_retry_action)" \
            "FAILED_SLOT=input-path" \
            "FAILED_KIND=FilePathInput"
          ;;
      esac

        if [[ -z "${normalized_input_1}" ]]; then
          normalized_input_1="$(normalize_input_path "$local_input")"
        fi
      done
    fi
  fi

  if ! is_missing_value "$risk"; then
    if ! normalized_risk="$(normalize_risk "$risk")"; then
      emit_failure "invalid-input" 17 \
        "Risk class ${risk} is not supported by the Canon runtime contract." \
        "Retry with low-impact, bounded-impact, systemic-impact, or the runtime-recognized aliases LowImpact, BoundedImpact, SystemicImpact." \
        "FAILED_SLOT=risk" \
        "FAILED_KIND=RiskField"
    fi
  fi

  if ! is_missing_value "$zone"; then
    if ! normalized_zone="$(normalize_zone "$zone")"; then
      emit_failure "invalid-input" 17 \
        "Usage zone ${zone} is not supported by the Canon runtime contract." \
        "Retry with green, yellow, red, or the runtime-recognized aliases Green, Yellow, Red." \
        "FAILED_SLOT=zone" \
        "FAILED_KIND=ZoneField"
    fi
  fi

  system_context_usage=""
  case "${command_name}" in
    system-shaping|architecture)
      system_context_usage="new|existing"
      ;;
    change|system-assessment)
      system_context_usage="existing"
      ;;
  esac

  if [[ -n "${system_context_usage}" ]] && is_missing_value "$system_context"; then
    emit_failure "missing-input" 14 \
      "System context is required for ${command_name}." \
      "Retry with --system-context ${system_context_usage}." \
      "FAILED_SLOT=system-context" \
      "FAILED_KIND=SystemContextField"
  fi

  if ! is_missing_value "$system_context"; then
    if ! normalized_system_context="$(normalize_system_context "$system_context")"; then
      emit_failure "invalid-input" 17 \
        "System context ${system_context} is not supported by the Canon runtime contract." \
        "Retry with new, existing, or the runtime-recognized aliases New, Existing." \
        "FAILED_SLOT=system-context" \
        "FAILED_KIND=SystemContextField"
    fi
  fi

  if [[ ("${command_name}" == "change" || "${command_name}" == "system-assessment") && -n "${normalized_system_context}" && "${normalized_system_context}" != "existing" ]]; then
    emit_failure "invalid-input" 17 \
      "Mode ${command_name} currently supports only --system-context existing in this release." \
      "Retry with --system-context existing." \
      "FAILED_SLOT=system-context" \
      "FAILED_KIND=SystemContextField"
  fi

  if [[ -z "${normalized_risk}" || -z "${normalized_zone}" ]]; then
    if [[ "${pr_review_command}" == "true" ]]; then
      infer_classification "$command_name" "$normalized_ref_1" "$normalized_ref_2"
    elif [[ -n "${normalized_input_1}" ]]; then
      infer_classification "$command_name" --input "$normalized_input_1"
    else
      infer_classification "$command_name" --input-text "$normalized_inline_input_1"
    fi

    extras=(
      "VERSION_KIND=${version_kind}"
      "DETECTED_VERSION=${detected_version}"
      "NEEDS_CONFIRMATION=true"
      "INFERRED_RISK=${inferred_risk}"
      "INFERRED_ZONE=${inferred_zone}"
      "INFERENCE_CONFIDENCE=${inference_confidence}"
      "INFERENCE_HEADLINE=${inference_headline}"
      "INFERENCE_RATIONALE=${inference_rationale}"
      "RISK_RATIONALE=${risk_rationale}"
      "ZONE_RATIONALE=${zone_rationale}"
      "RISK_WAS_SUPPLIED=${risk_was_supplied:-false}"
      "ZONE_WAS_SUPPLIED=${zone_was_supplied:-false}"
    )

    if [[ -n "${normalized_input_1}" ]]; then
      extras+=("NORMALIZED_INPUT_1=${normalized_input_1}")
    fi
    if [[ -n "${normalized_system_context}" ]]; then
      extras+=("NORMALIZED_SYSTEM_CONTEXT=${normalized_system_context}")
    fi
    if [[ -n "${normalized_ref_1}" ]]; then
      extras+=("NORMALIZED_REF_1=${normalized_ref_1}")
    fi
    if [[ -n "${normalized_ref_2}" ]]; then
      extras+=("NORMALIZED_REF_2=${normalized_ref_2}")
    fi

    for index in "${!inference_signals[@]}"; do
      extras+=("SIGNAL_$((index + 1))=${inference_signals[$index]}")
    done
    for index in "${!risk_signals[@]}"; do
      extras+=("RISK_SIGNAL_$((index + 1))=${risk_signals[$index]}")
    done
    for index in "${!zone_signals[@]}"; do
      extras+=("ZONE_SIGNAL_$((index + 1))=${zone_signals[$index]}")
    done

    emit_result "needs-classification-confirmation" 19 "preflight" \
      "${inference_headline}" \
      "Confirm or override the inferred classification, then invoke Canon with explicit --risk and --zone." \
      "${extras[@]}"
    exit 19
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
if [[ -n "${normalized_system_context}" ]]; then
  extras+=("NORMALIZED_SYSTEM_CONTEXT=${normalized_system_context}")
fi
if [[ -n "${normalized_ref_1}" ]]; then
  extras+=("NORMALIZED_REF_1=${normalized_ref_1}")
fi
if [[ -n "${normalized_ref_2}" ]]; then
  extras+=("NORMALIZED_REF_2=${normalized_ref_2}")
fi
if [[ -n "${normalized_risk}" ]]; then
  extras+=("NORMALIZED_RISK=${normalized_risk}")
fi
if [[ -n "${normalized_zone}" ]]; then
  extras+=("NORMALIZED_ZONE=${normalized_zone}")
fi

emit_result "ready" 0 "preflight" \
  "Typed preflight checks passed." \
  "Invoke Canon using the normalized contract for this command." \
  "${extras[@]}"
