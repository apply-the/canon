#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: render-scoop-manifest.sh --metadata <PATH> --output <PATH> [--template <PATH>]
EOF
}

metadata=""
output=""
template="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/packaging/scoop/canon.json.tpl"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --metadata)
      metadata="${2:-}"
      shift 2
      ;;
    --output)
      output="${2:-}"
      shift 2
      ;;
    --template)
      template="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 64
      ;;
  esac
done

if [[ -z "$metadata" || -z "$output" ]]; then
  usage >&2
  exit 64
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to render the Scoop manifest" >&2
  exit 65
fi

if [[ ! -f "$metadata" ]]; then
  echo "Metadata file not found: $metadata" >&2
  exit 66
fi

if [[ ! -f "$template" ]]; then
  echo "Template file not found: $template" >&2
  exit 67
fi

asset_field() {
  local asset_id="$1"
  local field="$2"
  jq -r --arg asset_id "$asset_id" --arg field "$field" '
    .assets[] | select(.asset_id == $asset_id) | .[$field]
  ' "$metadata"
}

require_channel_contract() {
  local channel="$1"
  if ! jq -e --arg channel "$channel" 'any(.channels[]?; .channel == $channel)' "$metadata" >/dev/null; then
    echo "Missing required channel contract: ${channel}" >&2
    exit 69
  fi
}

require_channel_asset() {
  local channel="$1"
  local asset_id="$2"
  if ! jq -e --arg channel "$channel" --arg asset_id "$asset_id" \
    'any(.channels[]?; .channel == $channel and (.asset_ids | index($asset_id)))' \
    "$metadata" >/dev/null; then
    echo "Channel contract ${channel} missing asset id: ${asset_id}" >&2
    exit 70
  fi

  if ! jq -e --arg channel "$channel" --arg asset_id "$asset_id" \
    'any(.assets[]?; .asset_id == $asset_id and (.channels | index($channel)))' \
    "$metadata" >/dev/null; then
    echo "Asset ${asset_id} does not advertise channel ${channel}" >&2
    exit 71
  fi
}

require_generated_artifact() {
  local channel="$1"
  local artifact="$2"
  if ! jq -e --arg channel "$channel" --arg artifact "$artifact" \
    'any(.channels[]?; .channel == $channel and (.generated_artifacts | index($artifact)))' \
    "$metadata" >/dev/null; then
    echo "Channel contract ${channel} missing generated artifact: ${artifact}" >&2
    exit 72
  fi
}

require_field() {
  local value="$1"
  local label="$2"
  if [[ -z "$value" || "$value" == "null" ]]; then
    echo "Missing required metadata field: $label" >&2
    exit 68
  fi
}

escape_replacement() {
  printf '%s' "$1" | sed 's/[&]/\\&/g'
}

version="$(jq -r '.version' "$metadata")"
installer_url="$(asset_field "windows-x86_64" "download_url")"
installer_sha256="$(asset_field "windows-x86_64" "sha256")"

require_channel_contract "scoop"
require_generated_artifact "scoop" "canon.json"
require_channel_asset "scoop" "windows-x86_64"

for pair in \
  "$version:version" \
  "$installer_url:windows-x86_64 download_url" \
  "$installer_sha256:windows-x86_64 sha256"
do
  require_field "${pair%%:*}" "${pair#*:}"
done

description="Governed local-first method engine for AI-assisted software engineering"
homepage="https://github.com/apply-the/canon"
license="MIT"

mkdir -p "$(dirname "$output")"

sed \
  -e "s|__VERSION__|$(escape_replacement "$version")|g" \
  -e "s|__DESCRIPTION__|$(escape_replacement "$description")|g" \
  -e "s|__HOMEPAGE__|$(escape_replacement "$homepage")|g" \
  -e "s|__LICENSE__|$(escape_replacement "$license")|g" \
  -e "s|__INSTALLER_URL__|$(escape_replacement "$installer_url")|g" \
  -e "s|__INSTALLER_SHA256__|$(escape_replacement "$installer_sha256")|g" \
  "$template" > "$output"

printf '%s\n' "$output"