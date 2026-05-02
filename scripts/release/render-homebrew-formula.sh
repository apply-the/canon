#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: render-homebrew-formula.sh --metadata <PATH> --output <PATH> [--template <PATH>]
EOF
}

metadata=""
output=""
template="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/packaging/homebrew/canon.rb.tpl"

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
  echo "jq is required to render the Homebrew formula" >&2
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
require_field "$version" "version"

require_channel_contract "homebrew"
require_generated_artifact "homebrew" "canon.rb"
require_channel_asset "homebrew" "macos-arm64"
require_channel_asset "homebrew" "macos-x86_64"
require_channel_asset "homebrew" "linux-arm64"
require_channel_asset "homebrew" "linux-x86_64"

desc="Governed local-first method engine for AI-assisted software engineering"
homepage="https://github.com/apply-the/canon"
license="MIT"

macos_arm64_url="$(asset_field "macos-arm64" "download_url")"
macos_arm64_sha256="$(asset_field "macos-arm64" "sha256")"
macos_x86_64_url="$(asset_field "macos-x86_64" "download_url")"
macos_x86_64_sha256="$(asset_field "macos-x86_64" "sha256")"
linux_arm64_url="$(asset_field "linux-arm64" "download_url")"
linux_arm64_sha256="$(asset_field "linux-arm64" "sha256")"
linux_x86_64_url="$(asset_field "linux-x86_64" "download_url")"
linux_x86_64_sha256="$(asset_field "linux-x86_64" "sha256")"

for pair in \
  "$macos_arm64_url:macos-arm64 download_url" \
  "$macos_arm64_sha256:macos-arm64 sha256" \
  "$macos_x86_64_url:macos-x86_64 download_url" \
  "$macos_x86_64_sha256:macos-x86_64 sha256" \
  "$linux_arm64_url:linux-arm64 download_url" \
  "$linux_arm64_sha256:linux-arm64 sha256" \
  "$linux_x86_64_url:linux-x86_64 download_url" \
  "$linux_x86_64_sha256:linux-x86_64 sha256"
do
  require_field "${pair%%:*}" "${pair#*:}"
done

mkdir -p "$(dirname "$output")"

sed \
  -e "s|__DESC__|$(escape_replacement "$desc")|g" \
  -e "s|__HOMEPAGE__|$(escape_replacement "$homepage")|g" \
  -e "s|__VERSION__|$(escape_replacement "$version")|g" \
  -e "s|__LICENSE__|$(escape_replacement "$license")|g" \
  -e "s|__MACOS_ARM64_URL__|$(escape_replacement "$macos_arm64_url")|g" \
  -e "s|__MACOS_ARM64_SHA256__|$(escape_replacement "$macos_arm64_sha256")|g" \
  -e "s|__MACOS_X86_64_URL__|$(escape_replacement "$macos_x86_64_url")|g" \
  -e "s|__MACOS_X86_64_SHA256__|$(escape_replacement "$macos_x86_64_sha256")|g" \
  -e "s|__LINUX_ARM64_URL__|$(escape_replacement "$linux_arm64_url")|g" \
  -e "s|__LINUX_ARM64_SHA256__|$(escape_replacement "$linux_arm64_sha256")|g" \
  -e "s|__LINUX_X86_64_URL__|$(escape_replacement "$linux_x86_64_url")|g" \
  -e "s|__LINUX_X86_64_SHA256__|$(escape_replacement "$linux_x86_64_sha256")|g" \
  "$template" > "$output"

printf '%s\n' "$output"