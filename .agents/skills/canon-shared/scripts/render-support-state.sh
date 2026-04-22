#!/usr/bin/env bash
set -euo pipefail

skill=""
state=""
known=""
missing=""
nearest=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skill) skill="${2:-}"; shift 2 ;;
    --state) state="${2:-}"; shift 2 ;;
    --known) known="${2:-}"; shift 2 ;;
    --missing) missing="${2:-}"; shift 2 ;;
    --nearest) nearest="${2:-}"; shift 2 ;;
    *) echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done

echo "Support State: ${state}"
case "${state}" in
  modeled-only)
    echo "This Canon workflow is modeled, but not runnable end to end yet."
    ;;
  intentionally-limited)
    echo "This Canon workflow is intentionally limited in the current release."
    ;;
  experimental)
    echo "This Canon workflow is experimental."
    ;;
  *)
    echo "This Canon workflow is available now."
    ;;
esac
[[ -n "${known}" ]] && echo "Known Today: ${known}"
[[ -n "${missing}" ]] && echo "Missing: ${missing}"
[[ -n "${nearest}" ]] && echo "Nearest Runnable Skill: ${nearest}"
[[ -n "${skill}" ]] && echo "Skill: ${skill}"
