# Validation Report: Governed Reasoning Posture v2

## Purpose

Capture executable validation evidence, independent review findings, and the
human approval result for the `governed_reasoning_posture_v2` feature.

## Command Validation

- `cargo test --test governed_reasoning_posture_contract`: PASS. Final v2
	contract-harness validation completed with `8 passed; 0 failed` after the
	last review-driven wording, fixture, and vocabulary updates.
- `cargo test --no-run --all-targets`: PASS. Workspace compile validation
	succeeded after the typed publication-helper expansion.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
	PASS. The touched Rust surfaces remain warning-free under the workspace lint
	gate.
- `cargo fmt --check`: PASS. Workspace formatting validation succeeded after
	the semantic-helper and contract-harness changes.
- `cargo test -p canon-engine --lib`: PASS. Focused validation of the expanded
	helper test suite completed with `712 passed; 0 failed`.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`:
	PASS. Workspace coverage rerun completed and refreshed `lcov.info`.
- Direct file-level verification against `lcov.info` confirms every touched
	Rust source file emitted by `cargo llvm-cov` remains above the 95% threshold:
	`publish_profile.rs` 100.0%, `authority.rs` 100.0%, `publication.rs` 97.7%,
	and `semantic.rs` 99.6%. The contract harness file under `tests/contract/`
	is not emitted as a source file in this LCOV report and is tracked as N/A.

| File | Changed | Instrumented Changed | Covered | Coverage | Status |
|------|---------|----------------------|---------|----------|--------|
| `crates/canon-engine/src/domain/publish_profile/semantic.rs` | 857 | 514 | 511 | 99.4% | PASS |
| `crates/canon-engine/src/domain/publish_profile/publication.rs` | 224 | 155 | 150 | 96.8% | PASS |
| `crates/canon-engine/src/domain/publish_profile/authority.rs` | 122 | 64 | 64 | 100.0% | PASS |
| `tests/contract/governed_reasoning_posture_contract.rs` | 423 | 0 | 0 | N/A | Not emitted in LCOV |

## Quickstart Verification

- Verified branch `065-reasoning-posture-v2` and Canon workspace version
	`0.64.0`.
- Reviewed the stable contract, the feature-local mirror, the migration brief,
	and the examples brief. They consistently publish
	`governed_reasoning_posture_v2`, explicit selector and required-block rules,
	and the active-versus-legacy coexistence policy.
- Confirmed the executable fixture corpus under
	`tests/fixtures/governed_reasoning_posture_v2/` contains the valid payload,
	malformed TOML fixtures, release-metadata drift fixtures, dual-line
	coexistence fixtures, and migration-rejection fixtures required by the
	quickstart.
- Confirmed release-alignment surfaces advertise the same Canon `0.64.0`
	pairing and contract line across `Cargo.toml`, runtime compatibility,
	assistant plugin metadata, README, CHANGELOG, CLI reference, roadmap, and
	the stable contract.
- Re-ran the quickstart validation path. Contract, compile, Clippy, coverage,
	and formatting checks all passed.

## Independent Review

- An independent adversarial review was run against the v2 contract docs,
	fixture corpus, migration rules, and release surfaces.
- The review flagged four material gaps: explicit `rejection_mode` wording was
	too soft in the docs, the incomplete confidence-handoff omission case was not
	visible enough in fixtures, the harness did not assert the `fail_closed`
	vocabulary directly, and coexistence validation needed clearer executable
	grounding.
- Remediation completed in the implementation slice: the stable and feature-
	local contract docs now require explicit `rejection_mode = fail_closed`
	wording, the incomplete confidence-handoff fixture omits `rejection_mode`,
	the harness asserts supported rejection modes directly, and the publication /
	authority unit suites now exercise active-versus-legacy and migration-boundary
	behavior.
- Residual technical findings from that review are closed.

## Approval Result

- Approved on 2026-06-02.
- The user explicitly approved the v1/v2 semantic delta, the active-versus-
	legacy rule, and the release-surface claims after the final validation and
	LCOV verification pass.