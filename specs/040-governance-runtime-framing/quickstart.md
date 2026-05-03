# Quickstart: Governance Runtime Framing

## Human Reader Walkthrough

1. Open `README.md` and confirm the opening framing describes Canon as a governed packet runtime, not a generic agent framework.
2. Read `docs/guides/getting-started.md` and verify the human-driven happy path is explicit: `init`, `inspect clarity`, `run`, `status`, `publish`.
3. Confirm the docs do not imply that Canon itself is the higher-level orchestrator.

## Orchestrator Maintainer Walkthrough

1. Open `docs/integration/governance-adapter.md`.
2. Verify that the guide documents `canon governance capabilities --json`, `canon governance start --json`, and `canon governance refresh --json`.
3. Verify that the guide explains `status`, `approval_state`, `packet_readiness`, `reason_code`, and canonical workspace-relative refs.
4. Verify that the guide explains when to use `canon governance` instead of the human CLI.

## Release Maintainer Walkthrough

1. Verify the workspace version is `0.40.0` in version-bearing Cargo surfaces.
2. Verify `README.md`, `CHANGELOG.md`, and `ROADMAP.md` describe the same delivered `040` feature state.
3. Run the required validation commands:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```

4. Record the validation results in `specs/040-governance-runtime-framing/validation-report.md`.