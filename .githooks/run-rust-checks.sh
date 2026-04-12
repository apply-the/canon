#!/bin/sh
set -eu

hook_name=${1:-hook}

repo_root=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$repo_root"

step_index=1
step_total=3

run_step() {
  step_label=$1
  failure_hint=$2
  shift 2

  printf '%s\n' "[$hook_name] [$step_index/$step_total] Running: $step_label"
  if "$@"; then
    printf '%s\n' "[$hook_name] [$step_index/$step_total] OK: $step_label"
  else
    status=$?
    printf '%s\n' "[$hook_name] [$step_index/$step_total] FAILED: $step_label" >&2
    printf '%s\n' "[$hook_name] Hint: $failure_hint" >&2
    exit "$status"
  fi

  step_index=$((step_index + 1))
}

printf '%s\n' "[$hook_name] Running Rust quality checks in $repo_root"

run_step \
  "cargo fmt --check" \
  "Run 'cargo fmt', restage any formatting changes, then retry." \
  cargo fmt --check

run_step \
  "cargo clippy --all-targets --all-features -- -D warnings" \
  "Run 'cargo clippy --all-targets --all-features -- -D warnings' and fix the reported warnings." \
  cargo clippy --all-targets --all-features -- -D warnings

run_step \
  "cargo test" \
  "Run 'cargo test' and fix the failing test or regression before retrying." \
  cargo test

printf '%s\n' "[$hook_name] All Rust quality checks passed."