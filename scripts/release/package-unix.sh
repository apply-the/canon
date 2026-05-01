#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: package-unix.sh --version <VERSION> --target <RUST_TARGET> --output-dir <DIR> [--binary-path <PATH>]
EOF
}

version=""
target=""
output_dir=""
binary_path=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      version="${2:-}"
      shift 2
      ;;
    --target)
      target="${2:-}"
      shift 2
      ;;
    --output-dir)
      output_dir="${2:-}"
      shift 2
      ;;
    --binary-path)
      binary_path="${2:-}"
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

if [[ -z "$version" || -z "$target" || -z "$output_dir" ]]; then
  usage >&2
  exit 64
fi

case "$target" in
  aarch64-apple-darwin)
    os="macos"
    arch="arm64"
    binary_name="canon"
    ;;
  x86_64-apple-darwin)
    os="macos"
    arch="x86_64"
    binary_name="canon"
    ;;
  aarch64-unknown-linux-gnu)
    os="linux"
    arch="arm64"
    binary_name="canon"
    ;;
  x86_64-unknown-linux-gnu)
    os="linux"
    arch="x86_64"
    binary_name="canon"
    ;;
  *)
    echo "Unsupported unix packaging target: $target" >&2
    exit 65
    ;;
esac

if [[ -z "$binary_path" ]]; then
  binary_path="target/${target}/release/${binary_name}"
fi

if [[ ! -f "$binary_path" ]]; then
  echo "Expected binary not found at ${binary_path}" >&2
  exit 66
fi

mkdir -p "$output_dir"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

cp "$binary_path" "${work_dir}/${binary_name}"
chmod 0755 "${work_dir}/${binary_name}"

artifact_name="canon-${version}-${os}-${arch}.tar.gz"
tar -C "$work_dir" -czf "${output_dir}/${artifact_name}" "$binary_name"

printf '%s\n' "${output_dir}/${artifact_name}"