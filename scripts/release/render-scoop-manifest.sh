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
supports_scoop="$(jq -r 'any(.assets[]; .asset_id == "windows-x86_64" and (.channels | index("scoop")))' "$metadata")"

for pair in \
  "$version:version" \
  "$installer_url:windows-x86_64 download_url" \
  "$installer_sha256:windows-x86_64 sha256"
do
  require_field "${pair%%:*}" "${pair#*:}"
done

if [[ "$supports_scoop" != "true" ]]; then
  echo "Distribution metadata does not mark windows-x86_64 as a Scoop channel" >&2
  exit 69
fi

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