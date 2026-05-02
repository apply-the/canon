#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: verify-release-surface.sh --version <VERSION> --dist-dir <DIR> --release-notes <PATH> [--write-checksums] [--distribution-metadata <PATH>] [--homebrew-formula <PATH>] [--winget-manifest-dir <PATH>] [--scoop-manifest <PATH>]
EOF
}

version=""
dist_dir=""
release_notes=""
write_checksums="false"
distribution_metadata=""
homebrew_formula=""
winget_manifest_dir=""
scoop_manifest=""

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
    --release-notes)
      release_notes="${2:-}"
      shift 2
      ;;
    --write-checksums)
      write_checksums="true"
      shift
      ;;
    --distribution-metadata)
      distribution_metadata="${2:-}"
      shift 2
      ;;
    --homebrew-formula)
      homebrew_formula="${2:-}"
      shift 2
      ;;
    --winget-manifest-dir)
      winget_manifest_dir="${2:-}"
      shift 2
      ;;
    --scoop-manifest)
      scoop_manifest="${2:-}"
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

if [[ -z "$version" || -z "$dist_dir" || -z "$release_notes" ]]; then
  usage >&2
  exit 64
fi

expected_archives=(
  "canon-${version}-macos-arm64.tar.gz"
  "canon-${version}-macos-x86_64.tar.gz"
  "canon-${version}-linux-arm64.tar.gz"
  "canon-${version}-linux-x86_64.tar.gz"
  "canon-${version}-windows-x86_64.zip"
)

checksum_manifest="canon-${version}-SHA256SUMS.txt"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "Missing required file: $path" >&2
    exit 65
  fi
}

verify_tarball() {
  local path="$1"
  local listing
  listing="$(tar -tzf "$path")"
  if [[ "$listing" != "canon" ]]; then
    echo "Unexpected tarball contents in $path: $listing" >&2
    exit 66
  fi
}

verify_zip() {
  local path="$1"
  local listing
  listing="$(unzip -Z1 "$path")"
  if [[ "$listing" != "canon.exe" ]]; then
    echo "Unexpected zip contents in $path: $listing" >&2
    exit 67
  fi
}

lookup_sha() {
  local artifact="$1"
  awk -v file="$artifact" '$2 == file { print $1 }' "${dist_dir}/${checksum_manifest}"
}

distribution_expectations() {
  local artifact="$1"
  case "$artifact" in
    *macos-arm64.tar.gz)
      printf '%s\n' "macos-arm64|macos|arm64|tar.gz|canon|[\"homebrew\"]"
      ;;
    *macos-x86_64.tar.gz)
      printf '%s\n' "macos-x86_64|macos|x86_64|tar.gz|canon|[\"homebrew\"]"
      ;;
    *linux-arm64.tar.gz)
      printf '%s\n' "linux-arm64|linux|arm64|tar.gz|canon|[\"homebrew\"]"
      ;;
    *linux-x86_64.tar.gz)
      printf '%s\n' "linux-x86_64|linux|x86_64|tar.gz|canon|[\"homebrew\"]"
      ;;
    *windows-x86_64.zip)
      printf '%s\n' "windows-x86_64|windows|x86_64|zip|canon.exe|[\"winget\",\"scoop\"]"
      ;;
    *)
      echo "Unsupported artifact for distribution validation: ${artifact}" >&2
      exit 72
      ;;
  esac
}

channel_contract_expectations() {
  local channel="$1"
  case "$channel" in
    homebrew)
      printf '%s\n' '["macos-arm64","macos-x86_64","linux-arm64","linux-x86_64"]|["canon.rb"]'
      ;;
    winget)
      printf '%s\n' '["windows-x86_64"]|["ApplyThe.Canon.yaml","ApplyThe.Canon.locale.en-US.yaml","ApplyThe.Canon.installer.yaml"]'
      ;;
    scoop)
      printf '%s\n' '["windows-x86_64"]|["canon.json"]'
      ;;
    *)
      echo "Unsupported channel contract for distribution validation: ${channel}" >&2
      exit 73
      ;;
  esac
}

verify_distribution_metadata() {
  local path="$1"

  require_file "$path"

  if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required to verify distribution metadata" >&2
    exit 73
  fi

  local tag="v${version}"
  local release_url="https://github.com/apply-the/canon/releases/tag/${tag}"
  local download_base="https://github.com/apply-the/canon/releases/download/${tag}"

  jq -e \
    --arg version "$version" \
    --arg tag "$tag" \
    --arg release_url "$release_url" \
    --arg release_notes "release-notes.md" \
    --arg checksum_manifest "$checksum_manifest" \
    '.version == $version
      and .tag == $tag
      and .release_url == $release_url
      and .release_notes == $release_notes
      and .checksum_manifest == $checksum_manifest
      and (.assets | type == "array")
      and (.channels | type == "array")' \
    "$path" >/dev/null || {
      echo "Distribution metadata top-level fields are invalid: $path" >&2
      exit 74
    }

  jq -e \
    --arg checksum_manifest "$checksum_manifest" \
    --arg release_notes "release-notes.md" \
    '.source_of_truth.kind == "github-releases"
      and .source_of_truth.artifact_inventory == "assets"
      and .source_of_truth.checksum_source == $checksum_manifest
      and .source_of_truth.release_notes_source == $release_notes' \
    "$path" >/dev/null || {
      echo "Distribution metadata source_of_truth is invalid: $path" >&2
      exit 75
    }

  for artifact in "${expected_archives[@]}"; do
    local sha
    sha="$(lookup_sha "$artifact")"
    if [[ -z "$sha" ]]; then
      echo "Missing checksum for ${artifact} while verifying distribution metadata" >&2
      exit 76
    fi

    IFS='|' read -r asset_id os arch archive_format binary_name channels <<< "$(distribution_expectations "$artifact")"

    jq -e \
      --arg artifact "$artifact" \
      --arg asset_id "$asset_id" \
      --arg os "$os" \
      --arg arch "$arch" \
      --arg archive_format "$archive_format" \
      --arg binary_name "$binary_name" \
      --arg sha "$sha" \
      --arg download_url "${download_base}/${artifact}" \
      --argjson channels "$channels" \
      'any(.assets[]?; .filename == $artifact and .asset_id == $asset_id and .os == $os and .arch == $arch and .archive_format == $archive_format and .binary_name == $binary_name and .sha256 == $sha and .download_url == $download_url and .channels == $channels)' \
      "$path" >/dev/null || {
        echo "Distribution metadata entry mismatch for ${artifact}" >&2
        exit 77
      }
  done

  for channel in homebrew winget scoop; do
    IFS='|' read -r asset_ids generated_artifacts <<< "$(channel_contract_expectations "$channel")"

    jq -e \
      --arg channel "$channel" \
      --argjson asset_ids "$asset_ids" \
      --argjson generated_artifacts "$generated_artifacts" \
      'any(.channels[]?; .channel == $channel and .asset_ids == $asset_ids and .generated_artifacts == $generated_artifacts)' \
      "$path" >/dev/null || {
        echo "Distribution metadata channel contract mismatch for ${channel}" >&2
        exit 78
      }
  done
}

verify_homebrew_formula() {
  local path="$1"
  local tag="v${version}"
  local download_base="https://github.com/apply-the/canon/releases/download/${tag}"

  require_file "$path"

  if ! grep -Fq 'class Canon < Formula' "$path"; then
    echo "Homebrew formula missing Canon class declaration: $path" >&2
    exit 77
  fi

  if ! grep -Fq 'bin.install "canon"' "$path"; then
    echo "Homebrew formula missing canonical install step: $path" >&2
    exit 78
  fi

  if ! grep -Fq 'system bin/"canon", "init", "--output", "json"' "$path"; then
    echo "Homebrew formula missing smoke test command: $path" >&2
    exit 79
  fi

  for artifact in "${expected_archives[@]}"; do
    if [[ "$artifact" == *windows-x86_64.zip ]]; then
      if grep -Fq "$artifact" "$path"; then
        echo "Homebrew formula must not reference Windows artifacts: $artifact" >&2
        exit 80
      fi
      continue
    fi

    local sha
    sha="$(lookup_sha "$artifact")"
    if [[ -z "$sha" ]]; then
      echo "Missing checksum for ${artifact} while verifying formula" >&2
      exit 81
    fi

    if ! grep -Fq "${download_base}/${artifact}" "$path"; then
      echo "Homebrew formula missing download URL for ${artifact}" >&2
      exit 82
    fi

    if ! grep -Fq "$sha" "$path"; then
      echo "Homebrew formula missing checksum for ${artifact}" >&2
      exit 83
    fi
  done
}

verify_winget_manifest_dir() {
  local dir="$1"
  local tag="v${version}"
  local artifact="canon-${version}-windows-x86_64.zip"
  local version_manifest="$dir/ApplyThe.Canon.yaml"
  local locale_manifest="$dir/ApplyThe.Canon.locale.en-US.yaml"
  local installer_manifest="$dir/ApplyThe.Canon.installer.yaml"
  local installer_url="https://github.com/apply-the/canon/releases/download/${tag}/${artifact}"
  local installer_sha

  installer_sha="$(lookup_sha "$artifact")"
  if [[ -z "$installer_sha" ]]; then
    echo "Missing checksum for ${artifact} while verifying winget manifests" >&2
    exit 84
  fi

  require_file "$version_manifest"
  require_file "$locale_manifest"
  require_file "$installer_manifest"

  for pair in \
    'PackageIdentifier: ApplyThe.Canon' \
    "PackageVersion: ${version}" \
    'DefaultLocale: en-US' \
    'ManifestType: version' \
    'ManifestVersion: 1.12.0'
  do
    if ! grep -Fq "$pair" "$version_manifest"; then
      echo "Winget version manifest missing ${pair}: ${version_manifest}" >&2
      exit 85
    fi
  done

  for pair in \
    'PackageIdentifier: ApplyThe.Canon' \
    "PackageVersion: ${version}" \
    'PackageLocale: en-US' \
    'Publisher: ApplyThe' \
    'PackageName: Canon' \
    'ManifestType: defaultLocale' \
    'ManifestVersion: 1.12.0'
  do
    if ! grep -Fq "$pair" "$locale_manifest"; then
      echo "Winget default locale manifest missing ${pair}: ${locale_manifest}" >&2
      exit 86
    fi
  done

  for pair in \
    'PackageIdentifier: ApplyThe.Canon' \
    "PackageVersion: ${version}" \
    'InstallerType: zip' \
    'NestedInstallerType: portable' \
    'RelativeFilePath: canon.exe' \
    'PortableCommandAlias: canon' \
    "InstallerUrl: ${installer_url}" \
    "InstallerSha256: ${installer_sha}" \
    'ManifestType: installer' \
    'ManifestVersion: 1.12.0'
  do
    if ! grep -Fq "$pair" "$installer_manifest"; then
      echo "Winget installer manifest missing ${pair}: ${installer_manifest}" >&2
      exit 87
    fi
  done
}

verify_scoop_manifest() {
  local path="$1"
  local tag="v${version}"
  local artifact="canon-${version}-windows-x86_64.zip"
  local installer_url="https://github.com/apply-the/canon/releases/download/${tag}/${artifact}"
  local installer_sha

  if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required to verify the Scoop manifest" >&2
    exit 88
  fi

  installer_sha="$(lookup_sha "$artifact")"
  if [[ -z "$installer_sha" ]]; then
    echo "Missing checksum for ${artifact} while verifying the Scoop manifest" >&2
    exit 89
  fi

  require_file "$path"

  jq -e \
    --arg version "$version" \
    --arg description "Governed local-first method engine for AI-assisted software engineering" \
    --arg homepage "https://github.com/apply-the/canon" \
    --arg license "MIT" \
    --arg installer_url "$installer_url" \
    --arg installer_sha "$installer_sha" \
    '.version == $version
      and .description == $description
      and .homepage == $homepage
      and .license == $license
      and .architecture["64bit"].url == $installer_url
      and .architecture["64bit"].hash == $installer_sha
      and .bin == "canon.exe"' \
    "$path" >/dev/null || {
      echo "Scoop manifest fields are invalid: $path" >&2
      exit 90
    }
}

mkdir -p "$dist_dir"

for artifact in "${expected_archives[@]}"; do
  require_file "${dist_dir}/${artifact}"
done

if [[ "$write_checksums" == "true" ]]; then
  (
    cd "$dist_dir"
    shasum -a 256 "${expected_archives[@]}" | awk '{print $1 "  " $2}' > "$checksum_manifest"
  )
fi

require_file "${dist_dir}/${checksum_manifest}"
require_file "$release_notes"

for artifact in "${expected_archives[@]}"; do
  if ! grep -Fq "  ${artifact}" "${dist_dir}/${checksum_manifest}"; then
    echo "Checksum manifest missing ${artifact}" >&2
    exit 68
  fi
  if ! grep -Fq "${artifact}" "$release_notes"; then
    echo "Release notes missing ${artifact}" >&2
    exit 69
  fi
done

if ! grep -Fq "${version}" "$release_notes"; then
  echo "Release notes do not mention version ${version}" >&2
  exit 70
fi

for artifact in "${expected_archives[@]}"; do
  case "$artifact" in
    *.tar.gz)
      verify_tarball "${dist_dir}/${artifact}"
      ;;
    *.zip)
      verify_zip "${dist_dir}/${artifact}"
      ;;
  esac

  version_file="${dist_dir}/${artifact}.version.txt"
  require_file "$version_file"
  if ! grep -Eq "(^| )${version}$" "$version_file"; then
    echo "Version evidence mismatch in ${version_file}" >&2
    exit 71
  fi
done

if [[ -n "$distribution_metadata" ]]; then
  verify_distribution_metadata "$distribution_metadata"
fi

if [[ -n "$homebrew_formula" ]]; then
  verify_homebrew_formula "$homebrew_formula"
fi

if [[ -n "$winget_manifest_dir" ]]; then
  verify_winget_manifest_dir "$winget_manifest_dir"
fi

if [[ -n "$scoop_manifest" ]]; then
  verify_scoop_manifest "$scoop_manifest"
fi

printf 'Verified %s archives, checksum manifest, release notes, and version evidence in %s\n' "${#expected_archives[@]}" "$dist_dir"