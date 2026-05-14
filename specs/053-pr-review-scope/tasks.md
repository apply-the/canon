# Tasks: pr-review Explicit Comment Scope

**Feature**: 053-pr-review-scope
**Branch**: `053-pr-review-scope`
**Plan**: [plan.md](plan.md) | **Spec**: [spec.md](spec.md)

## Status Legend

- `[ ]` Not started
- `[~]` In progress
- `[x]` Complete

---

## TASK-001 — Version Bump

**Depends on**: nothing
**Priority**: P0 (must be first)

Bump workspace version in `Cargo.toml` from `0.52.0` to `0.53.0`.

**Files**: `Cargo.toml`

**Acceptance**:
- `[workspace.package] version = "0.53.0"` in `Cargo.toml`.
- `cargo check` passes after the bump.

- [ ] Bump `version` in `[workspace.package]` from `0.52.0` to `0.53.0` in `Cargo.toml`.
- [ ] Verify `cargo check -p canon-engine -p canon-cli` passes.

---

## TASK-002 — ConventionalCommentScope Enum

**Depends on**: TASK-001
**Priority**: P1

Define the `ConventionalCommentScope` enum in `crates/canon-engine/src/review/findings.rs`.

**Files**: `crates/canon-engine/src/review/findings.rs`

**Requirements covered**: FR-001, FR-002, D-001

**Acceptance**:
- Enum `ConventionalCommentScope` with variants `Pr`, `File`, `Surface`.
- `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]`
- `#[serde(rename_all = "kebab-case")]` and `#[strum(serialize_all = "kebab-case")]`
- `as_str() -> &'static str` method.
- Serde round-trip unit test: `pr`, `file`, `surface` serialize correctly.

- [ ] Add `ConventionalCommentScope` enum with correct derives and serde/strum attributes.
- [ ] Add `as_str()` method.
- [ ] Add `all()` helper returning all variants.
- [ ] Add unit test `conventional_comment_scope_serde_round_trip` verifying kebab-case serialization.

---

## TASK-003 — scope Field on ReviewFinding + Derivation Logic

**Depends on**: TASK-002
**Priority**: P1

Add `scope: ConventionalCommentScope` to `ReviewFinding` and implement deterministic scope
derivation in `ReviewPacket::from_diff` and `ReviewPacket::from_evidence`.

**Files**: `crates/canon-engine/src/review/findings.rs`

**Requirements covered**: FR-002, FR-004, FR-005, FR-006, FR-007, D-002, D-003

**Acceptance**:
- `ReviewFinding` has `pub scope: ConventionalCommentScope`.
- All existing `ReviewFinding` construction sites in `findings.rs` populate `scope`.
- Private `fn derive_scope(changed_surfaces: &[String], category: FindingCategory) -> ConventionalCommentScope` helper:
  - returns `Pr` when `changed_surfaces` is empty.
  - returns `Surface` when all surfaces match the same group heuristic (all test, all contract, all boundary).
  - returns `File` otherwise.
- Unit tests covering: empty surfaces → `Pr`; all test surfaces → `Surface`; mixed surfaces → `File`; non-empty surfaces without group → `File`.
- All existing tests that construct `ReviewFinding` are updated to include `scope`.

- [ ] Add `scope: ConventionalCommentScope` field to `ReviewFinding` struct.
- [ ] Implement `derive_scope` private helper.
- [ ] Update all `ReviewFinding { ... }` construction sites in `findings.rs` to call `derive_scope`.
- [ ] Add unit tests: `scope_pr_when_no_surfaces`, `scope_surface_when_all_test_files`, `scope_surface_when_all_contract_files`, `scope_file_when_mixed_surfaces`, `scope_file_when_non_empty_but_no_group`.
- [ ] Update any existing tests in `findings.rs` that construct `ReviewFinding` directly to include the `scope` field.

---

## TASK-004 — Renderer: Scope Annotation in Conventional Comments

**Depends on**: TASK-003
**Priority**: P1

Update `render_conventional_comments` in `crates/canon-engine/src/artifacts/markdown.rs` to
include the scope annotation in each Conventional Comment entry. Update the evidence posture
text in the `conventional-comments.md` branch.

**Files**: `crates/canon-engine/src/artifacts/markdown.rs`

**Requirements covered**: FR-003, FR-005, FR-006, FR-007, SC-001, SC-002

**Acceptance**:
- Each rendered comment entry includes a `scope: <value>` line showing `pr`, `file`, or `surface`.
- When scope is `File` or `Surface`, the entry also lists the affected surfaces.
- The evidence posture paragraph in `conventional-comments.md` notes the scope model.
- No fabricated line-level anchors appear.
- Unit test `render_conventional_comments_includes_scope_annotation` verifies scope appears in output.
- Unit test `render_conventional_comments_pr_scope_when_no_surfaces` verifies `pr` scope for empty surfaces.
- Regression test: existing `render_pr_review_artifacts_handle_empty_and_populated_findings` still passes.

- [ ] Update `render_conventional_comments` to include `scope` in each entry.
- [ ] When scope is `File` or `Surface`, list `changed_surfaces` in the entry.
- [ ] Update `conventional-comments.md` evidence posture text to mention the scope model.
- [ ] Add unit test `render_conventional_comments_includes_scope_annotation`.
- [ ] Add unit test `render_conventional_comments_pr_scope_when_no_surfaces`.
- [ ] Fix any compilation failures in existing tests that construct `ReviewFinding` without `scope`.

---

## TASK-005 — Skill Documentation Update

**Depends on**: TASK-004
**Priority**: P2

Update `.agents/skills/canon-pr-review/SKILL.md` to document the three scope levels and the
deterministic derivation rule for Conventional Comments.

**Files**: `.agents/skills/canon-pr-review/SKILL.md`

**Requirements covered**: FR-010

**Acceptance**:
- Skill document mentions `pr`, `file`, and `surface` scope levels.
- Skill document explains that scope is derived deterministically from `changed_surfaces`.
- Skill document notes that line-level anchors are not emitted in the current slice.

- [ ] Add or update a section on Conventional Comment scope in the skill document.

---

## TASK-006 — Validation and Coverage

**Depends on**: TASK-004, TASK-005
**Priority**: P0 (must be last)

Run the full validation suite, measure coverage on modified files, and fix any remaining
clippy warnings.

**Files**: no new source files; runs `cargo` toolchain commands.

**Requirements covered**: FR-011, FR-012, SC-005, SC-006

**Acceptance**:
- `cargo fmt --check` exits 0.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits 0.
- `cargo nextest run` all pass.
- `cargo llvm-cov` reports 95% line coverage on `findings.rs` and `markdown.rs` (modified sections).

- [ ] Run `cargo fmt` and fix any formatting issues.
- [ ] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix all warnings.
- [ ] Run `cargo nextest run` and confirm all tests pass.
- [ ] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and verify coverage.
- [ ] Document any coverage gaps and add targeted tests if coverage falls below 95%.
