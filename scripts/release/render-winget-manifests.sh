#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: render-winget-manifests.sh --metadata <PATH> --output-dir <DIR>
EOF
}

metadata=""
output_dir=""
template_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/packaging/winget"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --metadata)
      metadata="${2:-}"
      shift 2
      ;;
    --output-dir)
      output_dir="${2:-}"
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

if [[ -z "$metadata" || -z "$output_dir" ]]; then
  usage >&2
  exit 64
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to render winget manifests" >&2
  exit 65
fi

if [[ ! -f "$metadata" ]]; then
  echo "Metadata file not found: $metadata" >&2
  exit 66
fi

version_template="$template_root/version.yaml.tpl"
locale_template="$template_root/defaultLocale.yaml.tpl"
installer_template="$template_root/installer.yaml.tpl"

for template in "$version_template" "$locale_template" "$installer_template"; do
  if [[ ! -f "$template" ]]; then
    echo "Template file not found: $template" >&2
    exit 67
  fi
done

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

package_identifier="ApplyThe.Canon"
package_locale="en-US"
publisher="ApplyThe"
package_name="Canon"
license="MIT"
short_description="Governed local-first method engine for AI-assisted software engineering"
homepage="https://github.com/apply-the/canon"
publisher_support_url="https://github.com/apply-the/canon/issues"
install_guide_url="https://github.com/apply-the/canon#install"

version="$(jq -r '.version' "$metadata")"
release_url="$(jq -r '.release_url' "$metadata")"
installer_url="$(asset_field "windows-x86_64" "download_url")"
installer_sha256="$(asset_field "windows-x86_64" "sha256")"

require_channel_contract "winget"
require_generated_artifact "winget" "ApplyThe.Canon.yaml"
require_generated_artifact "winget" "ApplyThe.Canon.locale.en-US.yaml"
require_generated_artifact "winget" "ApplyThe.Canon.installer.yaml"
require_channel_asset "winget" "windows-x86_64"

for pair in \
  "$version:version" \
  "$release_url:release_url" \
  "$installer_url:windows-x86_64 download_url" \
  "$installer_sha256:windows-x86_64 sha256"
do
  require_field "${pair%%:*}" "${pair#*:}"
done

mkdir -p "$output_dir"

version_output="$output_dir/ApplyThe.Canon.yaml"
locale_output="$output_dir/ApplyThe.Canon.locale.en-US.yaml"
installer_output="$output_dir/ApplyThe.Canon.installer.yaml"

sed \
  -e "s|__PACKAGE_IDENTIFIER__|$(escape_replacement "$package_identifier")|g" \
  -e "s|__PACKAGE_VERSION__|$(escape_replacement "$version")|g" \
  -e "s|__DEFAULT_LOCALE__|$(escape_replacement "$package_locale")|g" \
  "$version_template" > "$version_output"

sed \
  -e "s|__PACKAGE_IDENTIFIER__|$(escape_replacement "$package_identifier")|g" \
  -e "s|__PACKAGE_VERSION__|$(escape_replacement "$version")|g" \
  -e "s|__PACKAGE_LOCALE__|$(escape_replacement "$package_locale")|g" \
  -e "s|__PUBLISHER__|$(escape_replacement "$publisher")|g" \
  -e "s|__PUBLISHER_URL__|$(escape_replacement "$homepage")|g" \
  -e "s|__PUBLISHER_SUPPORT_URL__|$(escape_replacement "$publisher_support_url")|g" \
  -e "s|__PACKAGE_NAME__|$(escape_replacement "$package_name")|g" \
  -e "s|__PACKAGE_URL__|$(escape_replacement "$homepage")|g" \
  -e "s|__LICENSE__|$(escape_replacement "$license")|g" \
  -e "s|__SHORT_DESCRIPTION__|$(escape_replacement "$short_description")|g" \
  -e "s|__RELEASE_NOTES_URL__|$(escape_replacement "$release_url")|g" \
  -e "s|__INSTALL_GUIDE_URL__|$(escape_replacement "$install_guide_url")|g" \
  "$locale_template" > "$locale_output"

sed \
  -e "s|__PACKAGE_IDENTIFIER__|$(escape_replacement "$package_identifier")|g" \
  -e "s|__PACKAGE_VERSION__|$(escape_replacement "$version")|g" \
  -e "s|__INSTALLER_URL__|$(escape_replacement "$installer_url")|g" \
  -e "s|__INSTALLER_SHA256__|$(escape_replacement "$installer_sha256")|g" \
  "$installer_template" > "$installer_output"

printf '%s\n' "$output_dir"