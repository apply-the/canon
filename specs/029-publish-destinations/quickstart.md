# Quickstart: Structured External Publish Destinations

## Goal

Validate that Canon publishes packets into readable external directories,
preserves canonical run traceability through published metadata, and ships the
slice as `0.29.0` without changing `.canon/` runtime storage or publish
override semantics.

## Recommended Validation Flow

1. Run focused publish-path tests for default destinations, override behavior,
   and approval-gated operational publishing.
2. Publish at least one packet with the default destination and verify the
   resulting directory uses a date-prefixed descriptor.
3. Inspect the published packet metadata and confirm run id, mode, risk, zone,
   publish timestamp, and source artifact lineage remain recoverable.
4. Run release-surface checks for `Cargo.toml`, `Cargo.lock`, shared runtime
   compatibility references, `README.md`, `ROADMAP.md`, `docs/guides/modes.md`,
   and `CHANGELOG.md`.
5. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets
   --all-features -- -D warnings`, `cargo nextest run --workspace
   --all-features`, and `cargo llvm-cov --workspace --all-features --lcov
   --output-path lcov.info` before closeout.

## Representative Walkthroughs

- **Default publish**: run `publish` without `--to` and verify the packet lands
  under the expected family root with a date-prefixed descriptor path.
- **Override publish**: run `publish --to <custom-path>` and verify Canon uses
  the override path unchanged.
- **Operational publish**: publish an approval-gated operational packet that is
  already publishable under current policy and verify the same structured
  contract applies.