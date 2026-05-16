# Research: Adaptive Governance Semantics

## Decision: Keep `authority-governance-v1` as the required baseline and add `adaptive-governance-v1` only as an optional companion

- **Decision**: Canon preserves `authority-governance-v1` as the required S3 posture contract for downstream consumers and introduces `adaptive-governance-v1` only as an optional S4 semantic companion when machine-readable adaptive semantics are emitted.
- **Rationale**: This preserves compatibility for existing consumers and avoids forcing downstream runtimes to depend on a new required contract before the first S4 slice is useful.
- **Alternatives considered**:
  - Make `adaptive-governance-v1` mandatory immediately: rejected because it would block S4 adoption on a second hard dependency.
  - Expand `authority-governance-v1` in place: rejected because it would blur the established meaning of the required S3 posture contract.

## Decision: Keep adaptive-governance semantics advisory and semantic only

- **Decision**: Canon S4 semantics define meaning for governance states and rollout profiles, but they do not assign runtime confidence, trust, councils, reviewers, routes, overrides, or stop transitions.
- **Rationale**: The roadmap draws a strict line between Canon-owned semantic posture and Boundline-owned operational behavior. Keeping S4 companion semantics advisory preserves portability and ownership clarity.
- **Alternatives considered**:
  - Publish recommended councils or stop transitions in Canon metadata: rejected because that would make Canon a runtime orchestrator.
  - Publish runtime confidence scores in Canon metadata: rejected because confidence is evidence-derived runtime behavior, not stable semantic vocabulary.

## Decision: Publish the companion contract through existing packet metadata and governance-adapter surfaces

- **Decision**: If Canon emits `adaptive-governance-v1`, it should appear beside the existing governed packet metadata and machine-facing governance-adapter projection rather than through a new publication channel.
- **Rationale**: Existing packet metadata and integration docs are already the stable downstream surfaces. Reusing them minimizes adoption cost and keeps the contract boundary discoverable.
- **Alternatives considered**:
  - Create a separate adaptive-governance registry or manifest: rejected because it adds another synchronization surface without delivering independent value.
  - Publish the adaptive vocabulary only in prose docs: rejected because downstream consumers need a machine-readable contract path when the companion semantics are emitted.

## Decision: Keep the first companion envelope minimal

- **Decision**: The first machine-readable `adaptive-governance-v1` envelope should require only `contract_line`, `governance_state`, and `rollout_profile`, with optional semantic explanation fields remaining additive.
- **Rationale**: A minimal envelope communicates the core S4 semantic posture while avoiding accidental expansion into runtime decision directives.
- **Alternatives considered**:
  - Include detailed escalation or degradation instructions: rejected because those are runtime decisions owned downstream.
  - Mirror every S4 concept in the first envelope: rejected because most S4 concepts are operational rather than semantic.

## Decision: Keep rollout profiles distinct from authority zones and council profiles

- **Decision**: Canon documentation and machine-readable semantics must treat rollout profiles as governance-maturity labels, separate from S3 authority zones and downstream council profiles.
- **Rationale**: S4 is about progressive adoption and operational posture, not about replacing the S3 risk posture or downstream council sizing vocabulary.
- **Alternatives considered**:
  - Reuse council profile names for rollout profiles: rejected because it would conflate two different concepts.
  - Reuse authority zones for maturity labels: rejected because a zone describes posture, not governance adoption depth.

## Implementation Direction

- Add typed adaptive-governance vocabulary close to existing policy and artifact metadata surfaces.
- Project the optional companion semantics consistently through packet metadata and `docs/integration/governance-adapter.md` when emitted.
- Document the semantic boundary explicitly in Canon-owned governance docs so downstream maintainers do not need source-code inference.
- Preserve current governed publication and lineage behavior by attaching the companion semantics to existing governed metadata surfaces instead of creating a second publication workflow.

## Likely Touchpoints

- `crates/canon-engine/src/domain/policy.rs`
- `crates/canon-engine/src/domain/mode.rs`
- `crates/canon-engine/src/domain/artifact.rs`
- `crates/canon-engine/src/domain/publish_profile.rs`
- `crates/canon-engine/src/orchestrator/publish.rs`
- `crates/canon-engine/src/orchestrator/service/`
- `docs/integration/governance-adapter.md`
- `docs/governance-semantics-and-authority-zones.md`
- `tests/governance_adapter_surface.rs`
- `tests/mode_profiles.rs`
- `tests/policy_and_traces.rs`
- `README.md`
- `CHANGELOG.md`