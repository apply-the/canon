# Tasks: Ordered Artifact Filenames

**Feature**: `046-ordered-artifact-filenames`
**Spec**: `specs/046-ordered-artifact-filenames/spec.md`
**Plan**: `specs/046-ordered-artifact-filenames/plan.md`

## Phase 0: Governance & Artifacts

- [x] T001 [P] [US1] Bump workspace version to `0.46.0` in `Cargo.toml`
- [x] T002 [P] [US1] Update version reference in `README.md`

## Phase 1: Core Filename Prefixing

**Goal**: Add numeric prefix to every artifact filename Canon emits.

**Independent Test**: After implementation, `cargo test` passes and artifact filenames start with `NN-`.

- [ ] T003 [P] [US1] Add `prefixed_filename(index: usize, slug: &str) -> String` helper in `crates/canon-engine/src/artifacts/contract.rs`
- [ ] T004 [US1] Update `contract_for_mode()` in `crates/canon-engine/src/artifacts/contract.rs` to emit prefixed filenames for all 16 modes
- [ ] T005 [US1] Update `render_*_artifact()` match arms in `crates/canon-engine/src/artifacts/markdown.rs` to match on prefixed filenames
- [ ] T006 [US1] Update architecture-specific rendering in `crates/canon-engine/src/orchestrator/service/mode_shaping.rs` to use prefixed filenames for Mermaid sidecars, `view-manifest.json`, and `packet-metadata.json`
- [ ] T007 [US1] Update `architecture_view_heading()` in `crates/canon-engine/src/orchestrator/service/mode_shaping.rs` to match prefixed filenames
- [ ] T008 [US1] Update `build_architecture_view_manifest()` and `build_architecture_packet_metadata()` to reference prefixed filenames in JSON output

## Phase 2: Manifest, Publish, and Summary Updates

**Goal**: Publish logic, manifests, and summaries use prefixed filenames.

**Independent Test**: `canon publish` copies prefixed files; manifest JSON references match filesystem.

- [ ] T009 [US2] Verify publish logic in `crates/canon-engine/src/orchestrator/publish.rs` uses `artifact.record.file_name` (already does; no change needed if contract emits prefixed names)
- [ ] T010 [US3] Update primary artifact references in `crates/canon-engine/src/orchestrator/service/summarizers.rs` to match prefixed filenames

## Phase 3: Tests

**Goal**: All tests pass with prefixed filenames.

**Independent Test**: `cargo nextest run` is green.

- [ ] T011 [P] [US1] Update integration tests referencing artifact filenames in `tests/`
- [ ] T012 [US1] Update contract tests referencing artifact filenames
- [ ] T013 [US1] Update snapshot files in `tests/snapshots/` if any reference artifact filenames
- [ ] T014 [US1] Add a focused test that verifies contiguous numbering when optional artifacts are omitted

## Phase 4: Documentation & Release

**Goal**: Docs, changelog, and roadmap reflect the change.

- [ ] T015 [US1] Update `docs/guides/modes.md` artifact listings to use prefixed filenames
- [ ] T016 [US1] Update `CHANGELOG.md` with the 0.46.0 entry
- [ ] T017 [US1] Update `ROADMAP.md` with the ordered artifact filenames entry
- [ ] T018 [US1] Run `cargo fmt`, `cargo clippy`, `cargo nextest run`, verify coverage >= 95%

## Dependencies

- T004 depends on T003 (helper must exist before contract uses it)
- T005 depends on T004 (renderer must match new filenames from contract)
- T006, T007, T008 depend on T004
- T010 depends on T004
- T011-T014 depend on T004, T005, T006
- T015-T018 depend on all prior phases

## Parallel Opportunities

- T003 and T001/T002 are independent
- T006, T007, T008 can be done in parallel once T004 is complete
- T011-T014 can be done in parallel once Phase 1 and Phase 2 are complete
- T015-T017 can be done in parallel
