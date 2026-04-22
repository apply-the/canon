# Validation Report: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Date**: 2026-04-22  
**Status**: Planned — populated with evidence as implementation lands.

## Validation layers

This feature inherits Canon's layered-verification posture and is
bounded-impact, so the validation plan is structural + logical +
independent review, with adversarial review reserved for the resolver
ambiguity behavior.

### 1. Structural validation

| Check                                                        | Tooling                                                      | Owner       | Evidence path                                |
|--------------------------------------------------------------|--------------------------------------------------------------|-------------|----------------------------------------------|
| Formatting                                                   | `cargo fmt --check`                                          | implementer | CI log                                       |
| Lint                                                         | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | implementer | CI log                                       |
| Manifest schema contract                                     | `tests/contract/run_identity_contract.rs`                    | implementer | `target/nextest/.../run_identity_contract`   |
| Path parser (`split_once("--")`) contract                    | same                                                          | implementer | same                                         |
| Slug sanitizer regex                                         | unit tests in `crates/canon-engine/src/persistence/slug.rs`  | implementer | nextest output                               |
| Skill validators still pass                                  | `scripts/validate-canon-skills.sh`, `scripts/validate-canon-skills.ps1` | implementer | CI log                                       |

### 2. Logical validation

| Behavior                                                            | Tooling                                                              | Owner       | Evidence path                                  |
|---------------------------------------------------------------------|----------------------------------------------------------------------|-------------|------------------------------------------------|
| New run lands at `.canon/runs/YYYY/MM/R-…/`                         | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| Persist + reload preserves `uuid`, `run_id`, `short_id`, `created_at` | `tests/integration/run_lookup.rs`                                  | implementer | nextest output                                 |
| Lookup by full `run_id` resolves                                    | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| Lookup by full `uuid` resolves                                      | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| Lookup by unique `short_id` resolves                                | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| Ambiguous `short_id` reports matches and exits non-zero             | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| `@last` resolves to latest `created_at`; empty repo errors clearly  | `tests/integration/run_lookup.rs`                                    | implementer | nextest output                                 |
| Authored files in `canon-input/` are not mutated                    | `tests/integration/inputs_snapshot_immutability.rs`                  | implementer | nextest output                                 |
| Snapshot under `.canon/runs/<…>/inputs/` is byte-stable             | `tests/integration/inputs_snapshot_immutability.rs`                  | implementer | nextest output                                 |
| Legacy UUID-keyed run directory is readable / listable / resumable  | `tests/integration/run_lookup.rs` + `tests/fixtures/legacy_uuid_run/`| implementer | nextest output                                 |
| `status`, `inspect evidence`, `inspect artifacts`, `approve`, `resume` work for both layouts | existing tests + new ones                       | implementer | nextest output                                 |
| Listing command emits required columns and sort order               | `tests/contract/run_identity_contract.rs` (CLI snapshot)             | implementer | snapshot file under `tests/snapshots/`         |

### 3. Independent validation

| Activity                                                            | Performed by                                  | Evidence path                                                    |
|---------------------------------------------------------------------|------------------------------------------------|------------------------------------------------------------------|
| Walkthrough of [quickstart.md](quickstart.md), all steps green      | independent human reviewer (not implementer)  | reviewer transcript appended to this report                      |
| Adversarial review of resolver ambiguity behavior                   | independent reviewer                           | review notes appended to this report                             |
| Confirmation that no code path writes under `canon-input/`          | independent reviewer (grep + spot-check)      | review notes appended to this report                             |
| Confirmation that no legacy run directory is moved or rewritten     | independent reviewer                           | review notes appended to this report                             |

### 4. Coverage expectation

- Workspace coverage MUST NOT regress.
- Touched-patch coverage on `persistence/lookup.rs`, `persistence/slug.rs`,
  `persistence/layout.rs`, and the manifest read shim MUST be ≥ 85 %.

## Approval gate

bounded-impact merge requires:

- [ ] All structural checks green in CI.
- [ ] All logical tests green in CI.
- [ ] Independent reviewer sign-off on the quickstart walkthrough and
  on the four review activities above.
- [ ] Decision log up to date in [decision-log.md](decision-log.md).

## Evidence index (filled during implementation)

- CI run: _to be filled_
- Coverage report: _to be filled_
- Reviewer transcript: _to be filled_
- Reviewer notes: _to be filled_
