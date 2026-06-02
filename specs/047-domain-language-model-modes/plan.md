# Implementation Plan: Domain Language And Domain Model Modes

**Branch**: `047-domain-language-model-modes` | **Date**: 2026-05-12 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/047-domain-language-model-modes/spec.md`

## Summary

Add two new first-class Canon modes: `domain-language` (governed ubiquitous-language packet, 10 artifacts) and `domain-model` (governed ontology/concept model packet, 13 artifacts including machine-readable JSON). Both modes follow established Canon patterns for artifact contracts, renderers, gatekeepers, summarizers, publish targets, inspect clarity, governance adapter capabilities, skills, templates, and examples.

## Governance Context

**Execution Mode**: change
**Risk Classification**: systemic-impact: two new Mode enum variants propagate across the entire Canon workspace.
**Scope In**: Mode enum, artifact contracts, renderers, gatekeepers, summarizers, publish targets, inspect clarity, governance adapter, CLI, skills, templates, examples, docs, version bump, CHANGELOG, ROADMAP.
**Scope Out**: Semantic-web ontology; `--from-run` seeding; Boundline integration; localization.

**Invariants**:

- All existing 352+ tests must continue to pass.
- Ordinal-prefixed filename convention must be followed.
- Missing authored sections produce `## Missing Authored Body` markers.

**Decision Log**: `specs/047-domain-language-model-modes/`
**Validation Ownership**: Tests generate output; `cargo nextest run` validates; `cargo clippy` checks quality.
**Approval Gates**: None required for bounded-impact file-backed modes; systemic-impact risk is bounded to additive code paths.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024
**Framework**: Canon workspace (canon-cli, canon-engine, canon-adapters)
**Architecture**: Mode enum in domain layer; artifact contracts in `artifacts/contract.rs`; markdown renderers in `artifacts/markdown.rs`; gatekeepers in `orchestrator/gatekeeper.rs`; summarizers in `orchestrator/service/summarizers.rs`; mode shaping in `orchestrator/service/mode_shaping.rs`; publish in `orchestrator/publish.rs`.

## Implementation Phases

### Phase 1: Core Domain Infrastructure

1. Bump workspace version to `0.47.0` across all crates, plugin manifests, and runtime-compatibility references.
2. Add `DomainLanguage` and `DomainModel` variants to the `Mode` enum.
3. Add mode display names, CLI parsing, input binding rules, and system-context policy.
4. Add artifact contracts for both modes in `contract_for_mode()` with ordinal-prefixed filenames.

### Phase 2: Renderers And Gatekeepers

5. Add markdown renderer functions for all domain-language artifacts (10 functions).
6. Add markdown renderer functions for all domain-model artifacts (13 functions).
7. Add gatekeeper evaluation functions for both modes.
8. Add summarizer functions for both modes.

### Phase 3: Orchestration And Publish

9. Add mode shaping integration for both modes.
10. Add publish target configuration (`tech-docs/domain/language/` and `tech-docs/domain/model/`).
11. Add inspect clarity support for both modes.
12. Add governance adapter capability output for both modes.

### Phase 4: Skills, Templates, And Docs

13. Add governed method entries for both modes.
14. Add repo-local and embedded skills for both modes.
15. Add input templates under `defaults/templates/`.
16. Add worked examples under `tech-docs/examples/`.
17. Update Mode Guide, README, and flow diagrams.

### Phase 5: Quality Gates And Release

18. Add contract tests for both modes.
19. Add integration/run tests for both modes.
20. Add publish target tests.
21. Run `cargo fmt`, `cargo clippy`, `cargo nextest run`.
22. Verify coverage maintains baseline.
23. Update CHANGELOG and ROADMAP.
24. Generate conventional commit message.

## Risk Mitigations

- **Additive-only change**: Both modes add new enum variants and code paths without modifying existing mode behavior.
- **Pattern reuse**: Follow established patterns from `security-assessment`, `system-assessment`, and `supply-chain-analysis` modes which were added in the same way.
- **Test isolation**: New tests are independent; existing tests remain unchanged.

## Dependencies

- Ordinal-prefix convention from 046 must be present on `main` (confirmed: merged).
- No external dependencies required.
