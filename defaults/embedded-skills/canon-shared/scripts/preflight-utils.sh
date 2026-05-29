#!/usr/bin/env bash
# preflight-utils.sh — Shared utility functions for canon-preflight.sh
# Part of Canon Skill Runtime Contracts (061)
set -euo pipefail

# trim(): Remove leading and trailing whitespace from a string.
# Usage: result=$(trim "  hello  ")
trim() {
  local var="$1"
  var="${var#"${var%%[![:space:]]*}"}"
  var="${var%"${var##*[![:space:]]}"}"
  printf '%s' "$var"
}

# is_placeholder(): Return 0 if the value looks like a placeholder or is empty.
# Usage: if is_placeholder "$val"; then ...
is_placeholder() {
  local val="$1"
  val=$(trim "$val")
  if [[ -z "$val" ]]; then
    return 0
  fi
  # Common placeholder patterns
  if [[ "$val" =~ ^\<.*\>$ ]] || [[ "$val" =~ ^\{.*\}$ ]] || [[ "$val" == "TODO" ]] || [[ "$val" == "TBD" ]]; then
    return 0
  fi
  return 1
}

# json_escape(): Escape a string for safe inclusion in JSON values.
# Handles backslash, double-quote, newline, tab, carriage return.
# Usage: escaped=$(json_escape "$raw")
json_escape() {
  local str="$1"
  str="${str//\\/\\\\}"
  str="${str//\"/\\\"}"
  str="${str//$'\n'/\\n}"
  str="${str//$'\t'/\\t}"
  str="${str//$'\r'/\\r}"
  printf '%s' "$str"
}

# json_string(): Wrap a value in JSON quotes with escaping, or emit "null".
# Usage: json_string "$val"   -> "escaped_val" or null
json_string() {
  local val="${1:-}"
  if [[ -z "$val" ]]; then
    printf 'null'
  else
    printf '"%s"' "$(json_escape "$val")"
  fi
}

# json_bool(): Emit JSON boolean from shell truthy.
# Usage: json_bool true -> true; json_bool false -> false
json_bool() {
  if [[ "${1:-false}" == "true" ]]; then
    printf 'true'
  else
    printf 'false'
  fi
}

# json_int_or_null(): Emit integer or null.
# Usage: json_int_or_null "3" -> 3; json_int_or_null "" -> null
json_int_or_null() {
  local val="${1:-}"
  if [[ -z "$val" ]] || ! [[ "$val" =~ ^[0-9]+$ ]]; then
    printf 'null'
  else
    printf '%s' "$val"
  fi
}

# iso_timestamp(): Emit current UTC timestamp in ISO 8601 format.
# Usage: ts=$(iso_timestamp)
iso_timestamp() {
  date -u '+%Y-%m-%dT%H:%M:%SZ'
}
