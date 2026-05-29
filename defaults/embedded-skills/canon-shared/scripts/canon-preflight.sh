#!/usr/bin/env bash
# canon-preflight.sh — Structured JSON preflight output for Canon skills
# Part of Canon Skill Runtime Contracts (061)
#
# Usage: canon-preflight.sh --mode <mode-name>
# Output: JSON to stdout conforming to contracts/preflight-json-schema.json
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=preflight-utils.sh
source "${SCRIPT_DIR}/preflight-utils.sh"

# --- Argument parsing ---
MODE=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="${2:-}"
      shift 2
      ;;
    --mode=*)
      MODE="${1#--mode=}"
      shift
      ;;
    *)
      shift
      ;;
  esac
done

if [[ -z "$MODE" ]]; then
  # Emit minimal error JSON when mode is missing
  cat <<EOF
{
  "schema_version": 1,
  "timestamp": "$(iso_timestamp)",
  "mode": "",
  "canon": { "available": false, "version": null, "initialized": false, "error": "no --mode argument provided" },
  "workspace": { "path": null, "git_branch": null, "git_user": null, "error": "no --mode argument provided" },
  "input": { "file_exists": false, "file_path": "", "file_empty": null, "folder_exists": false, "folder_path": "", "folder_empty": null, "resolved_path": null, "ambiguous": false, "error": "no --mode argument provided" },
  "runs": { "active": null, "pending_approvals": null, "error": "no --mode argument provided" }
}
EOF
  exit 1
fi

# --- Canon section (T016) ---
canon_available="false"
canon_version=""
canon_initialized="false"
canon_error=""

if command -v canon >/dev/null 2>&1; then
  canon_available="true"
  canon_version_raw="$(canon --version 2>/dev/null || true)"
  # Extract version number (e.g., "canon 0.61.0" -> "0.61.0")
  canon_version="$(echo "$canon_version_raw" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || true)"
  if [[ -z "$canon_version" ]]; then
    canon_version=""
    canon_error="could not parse canon version"
  fi
else
  canon_error="canon binary not found on PATH"
fi

if [[ -d ".canon" ]]; then
  canon_initialized="true"
fi

# --- Workspace section (T017) ---
workspace_path="$(pwd)"
workspace_git_branch=""
workspace_git_user=""
workspace_error=""

if command -v git >/dev/null 2>&1 && git rev-parse --git-dir >/dev/null 2>&1; then
  workspace_git_branch="$(git symbolic-ref --short HEAD 2>/dev/null || git rev-parse --short HEAD 2>/dev/null || true)"
  workspace_git_user="$(git config user.email 2>/dev/null || true)"
else
  workspace_error="git not available or not a git repository"
fi

# --- Input section (T018) ---
input_file_path="canon-input/${MODE}.md"
input_folder_path="canon-input/${MODE}/"
input_file_exists="false"
input_file_empty="null"
input_folder_exists="false"
input_folder_empty="null"
input_resolved_path=""
input_ambiguous="false"
input_error=""

if [[ -f "$input_file_path" ]]; then
  input_file_exists="true"
  if [[ -s "$input_file_path" ]]; then
    input_file_empty="false"
  else
    input_file_empty="true"
  fi
fi

if [[ -d "$input_folder_path" ]]; then
  input_folder_exists="true"
  # Check if folder has files
  if [[ -z "$(find "$input_folder_path" -maxdepth 1 -type f 2>/dev/null | head -1)" ]]; then
    input_folder_empty="true"
  else
    input_folder_empty="false"
  fi
fi

# File-first resolution per C-003
if [[ "$input_file_exists" == "true" ]]; then
  input_resolved_path="$input_file_path"
  if [[ "$input_folder_exists" == "true" ]]; then
    input_ambiguous="true"
  fi
elif [[ "$input_folder_exists" == "true" ]]; then
  input_resolved_path="$input_folder_path"
fi

# Error for unknown mode (neither file nor folder)
if [[ "$input_file_exists" == "false" ]] && [[ "$input_folder_exists" == "false" ]]; then
  input_error="unknown mode: ${MODE}"
fi

# --- Runs section (T019) ---
runs_active=""
runs_pending=""
runs_error=""

if [[ "$canon_available" == "true" ]] && [[ -d ".canon/runs" ]]; then
  # Count active runs (directories with manifest.toml that don't have state=Completed)
  runs_active=0
  runs_pending=0
  for run_dir in .canon/runs/*/; do
    if [[ -d "$run_dir" ]]; then
      manifest="${run_dir}manifest.toml"
      if [[ -f "$manifest" ]]; then
        runs_active=$((runs_active + 1))
        # Check for pending approvals
        if grep -q 'state.*=.*"AwaitingApproval"' "$manifest" 2>/dev/null; then
          runs_pending=$((runs_pending + 1))
        fi
      fi
    fi
  done
elif [[ "$canon_available" == "false" ]]; then
  runs_error="canon not available; cannot query runs"
elif [[ ! -d ".canon/runs" ]]; then
  runs_active=0
  runs_pending=0
fi

# --- JSON assembly (T020) ---
timestamp="$(iso_timestamp)"

cat <<EOF
{
  "schema_version": 1,
  "timestamp": "${timestamp}",
  "mode": $(json_string "$MODE"),
  "canon": {
    "available": $(json_bool "$canon_available"),
    "version": $(json_string "$canon_version"),
    "initialized": $(json_bool "$canon_initialized"),
    "error": $(json_string "$canon_error")
  },
  "workspace": {
    "path": $(json_string "$workspace_path"),
    "git_branch": $(json_string "$workspace_git_branch"),
    "git_user": $(json_string "$workspace_git_user"),
    "error": $(json_string "$workspace_error")
  },
  "input": {
    "file_exists": $(json_bool "$input_file_exists"),
    "file_path": $(json_string "$input_file_path"),
    "file_empty": ${input_file_empty},
    "folder_exists": $(json_bool "$input_folder_exists"),
    "folder_path": $(json_string "$input_folder_path"),
    "folder_empty": ${input_folder_empty},
    "resolved_path": $(json_string "$input_resolved_path"),
    "ambiguous": $(json_bool "$input_ambiguous"),
    "error": $(json_string "$input_error")
  },
  "runs": {
    "active": $(json_int_or_null "$runs_active"),
    "pending_approvals": $(json_int_or_null "$runs_pending"),
    "error": $(json_string "$runs_error")
  }
}
EOF
