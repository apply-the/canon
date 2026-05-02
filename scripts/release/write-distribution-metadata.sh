#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: write-distribution-metadata.sh --version <VERSION> --dist-dir <DIR> --output <PATH> [--tag <TAG>] [--repo <OWNER/REPO>]
EOF
}

version=""
dist_dir=""
output=""
tag=""
repo="apply-the/canon"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      version="${2:-}"
      shift 2
      ;;
    --dist-dir)
      dist_dir="${2:-}"
      shift 2
      ;;
    --output)
      output="${2:-}"
      shift 2
      ;;
    --tag)
      tag="${2:-}"
      shift 2
      ;;
    --repo)
      repo="${2:-}"
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

if [[ -z "$version" || -z "$dist_dir" || -z "$output" ]]; then
  usage >&2
  exit 64
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to write distribution metadata" >&2
  exit 65
fi

if [[ -z "$tag" ]]; then
  tag="v${version}"
fi

expected_archives=(
  "canon-${version}-macos-arm64.tar.gz"
  "canon-${version}-macos-x86_64.tar.gz"
  "canon-${version}-linux-arm64.tar.gz"
  "canon-${version}-linux-x86_64.tar.gz"
  "canon-${version}-windows-x86_64.zip"
)

checksum_manifest="canon-${version}-SHA256SUMS.txt"
release_notes="release-notes.md"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "Missing required file: $path" >&2
    exit 66
  fi
}

lookup_sha() {
  local artifact="$1"
  awk -v file="$artifact" '$2 == file { print $1 }' "$dist_dir/$checksum_manifest"
}

require_file "$dist_dir/$checksum_manifest"
require_file "$dist_dir/$release_notes"

assets_json='[]'
download_base_url="https://github.com/${repo}/releases/download/${tag}"
release_url="https://github.com/${repo}/releases/tag/${tag}"

for artifact in "${expected_archives[@]}"; do
  require_file "$dist_dir/$artifact"

  sha256="$(lookup_sha "$artifact")"
  if [[ -z "$sha256" ]]; then
    echo "Checksum manifest missing entry for ${artifact}" >&2
    exit 67
  fi

  case "$artifact" in
    *macos-arm64.tar.gz)
      asset_id="macos-arm64"
      os="macos"
      arch="arm64"
      archive_format="tar.gz"
      binary_name="canon"
      channels='["homebrew"]'
      ;;
    *macos-x86_64.tar.gz)
      asset_id="macos-x86_64"
      os="macos"
      arch="x86_64"
      archive_format="tar.gz"
      binary_name="canon"
      channels='["homebrew"]'
      ;;
    *linux-arm64.tar.gz)
      asset_id="linux-arm64"
      os="linux"
      arch="arm64"
      archive_format="tar.gz"
      binary_name="canon"
      channels='["homebrew"]'
      ;;
    *linux-x86_64.tar.gz)
      asset_id="linux-x86_64"
      os="linux"
      arch="x86_64"
      archive_format="tar.gz"
      binary_name="canon"
      channels='["homebrew"]'
      ;;
    *windows-x86_64.zip)
      asset_id="windows-x86_64"
      os="windows"
      arch="x86_64"
      archive_format="zip"
      binary_name="canon.exe"
      channels='["winget","scoop"]'
      ;;
    *)
      echo "Unsupported artifact for metadata emission: ${artifact}" >&2
      exit 68
      ;;
  esac

  asset_json="$({
    jq -cn \
      --arg asset_id "$asset_id" \
      --arg filename "$artifact" \
      --arg os "$os" \
      --arg arch "$arch" \
      --arg archive_format "$archive_format" \
      --arg binary_name "$binary_name" \
      --arg sha256 "$sha256" \
      --arg download_url "${download_base_url}/${artifact}" \
      --argjson channels "$channels" \
      '{
        asset_id: $asset_id,
        filename: $filename,
        os: $os,
        arch: $arch,
        archive_format: $archive_format,
        binary_name: $binary_name,
        sha256: $sha256,
        download_url: $download_url,
        channels: $channels
      }'
  })"

  assets_json="$(jq -cn --argjson assets "$assets_json" --argjson asset "$asset_json" '$assets + [$asset]')"
done

mkdir -p "$(dirname "$output")"

channel_contracts="$({
  jq -cn \
    --argjson assets "$assets_json" \
    '[
      {
        channel: "homebrew",
        asset_ids: [$assets[] | select(.channels | index("homebrew")) | .asset_id],
        generated_artifacts: ["canon.rb"]
      },
      {
        channel: "winget",
        asset_ids: [$assets[] | select(.channels | index("winget")) | .asset_id],
        generated_artifacts: [
          "ApplyThe.Canon.yaml",
          "ApplyThe.Canon.locale.en-US.yaml",
          "ApplyThe.Canon.installer.yaml"
        ]
      },
      {
        channel: "scoop",
        asset_ids: [$assets[] | select(.channels | index("scoop")) | .asset_id],
        generated_artifacts: ["canon.json"]
      }
    ]'
})"

jq -n \
  --arg version "$version" \
  --arg tag "$tag" \
  --arg release_url "$release_url" \
  --arg release_notes "$release_notes" \
  --arg checksum_manifest "$checksum_manifest" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson assets "$assets_json" \
  --argjson channel_contracts "$channel_contracts" \
  '{
    version: $version,
    tag: $tag,
    release_url: $release_url,
    release_notes: $release_notes,
    checksum_manifest: $checksum_manifest,
    generated_at: $generated_at,
    source_of_truth: {
      kind: "github-releases",
      artifact_inventory: "assets",
      checksum_source: $checksum_manifest,
      release_notes_source: $release_notes
    },
    assets: $assets,
    channels: $channel_contracts
  }' > "$output"

printf '%s\n' "$output"