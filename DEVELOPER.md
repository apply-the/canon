# Canon — Developer Install Guide

This document covers installing Canon locally from source for development and
testing. For contributing rules, build commands, and repo layout, see
[`CONTRIBUTING.md`](CONTRIBUTING.md). For end-user install from release
archives, see [`README.md`](README.md).

## Prerequisites

- **Rust 1.94.1** — install via `rustup`
- **Git**
- `cargo-nextest` (optional, recommended for faster test runs)
- `cargo-deny` (required for license/advisory checks)

```bash
rustup toolchain install 1.94.1 --profile minimal --component rustfmt --component clippy
cargo install cargo-nextest cargo-deny
```

## 1. Clone and Build

```bash
git clone https://github.com/apply-the/canon.git
cd canon
cargo build
```

## 2. Install Locally

Choose your preferred method.

### Option 1: Persistent Installation (Recommended)

Installs the `canon` binary into your Cargo bin directory (`~/.cargo/bin`),
which is already on PATH after a standard `rustup` setup.

```bash
# From the repo root — always matches the current source
cargo +1.94.1 install --path crates/canon-cli --bin canon
```

Then verify:

```bash
canon --version
command -v canon
```

To upgrade after pulling new changes, repeat the `cargo install` command.
The `--force` flag is not needed; Cargo replaces the existing binary automatically.

### Option 2: One-time / In-Tree Usage

Build a release binary and run it directly from the output directory without
touching your PATH.

```bash
cargo build --release -p canon-cli --bin canon
./target/release/canon --version
```

Useful when you need to test a specific branch without overwriting the installed binary.

### Option 3: Debug Build

For faster iteration during development. Not suitable for benchmarking or
release testing.

```bash
cargo build
./target/debug/canon --version
```

## 3. Verify

After any install method, confirm Canon resolves correctly:

```bash
canon --version
command -v canon    # should show expected path
```

If the resolved path points to an unexpected location, check PATH ordering and
rerun the verification commands.

## 4. Install Git Hooks

```bash
./scripts/install-hooks.sh
```

## 5. Smoke Test

Run the minimum validation suite to confirm your local build is sound:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo nextest run --locked
cargo deny check licenses advisories bans sources
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full validation checklist
required before opening a pull request.
