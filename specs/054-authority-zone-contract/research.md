# Research: Authority Zone Contract

## Current Stable Surfaces

- `crates/canon-engine/src/domain/policy.rs` already defines `RiskClass` and
  `UsageZone`, which are the closest existing semantic anchors for the new
  authority-zone contract.
- `crates/canon-engine/src/domain/mode.rs` already owns Canon modes and static
  mode profiles, making it the natural home for intended persona and advisory
  stage-role-hint metadata.
- `crates/canon-engine/src/domain/approval.rs` already models approval records
  and current approval decisions that the new contract can project rather than
  reinvent.
- `crates/canon-engine/src/domain/artifact.rs` and
  `crates/canon-engine/src/domain/publish_profile.rs` already define stable
  artifact metadata, packet sidecars, and publication lineage surfaces that can
  carry the authority-governance envelope.
- `docs/integration/governance-adapter.md` already defines the machine-facing
  adapter projection that downstream orchestrators consume.

## Boundaries Confirmed During Planning

- Canon should publish governed semantics and compatibility rules, not runtime
  councils, routing decisions, or stop transitions for downstream systems.
- `authority-governance-v1` needs an explicit required versus optional field
  profile so consumers can fail closed without blocking on provenance-only data.
- The new contract should extend existing packet metadata and adapter surfaces
  instead of creating a second publication channel.
- Existing `UsageZone` semantics should remain stable for current runtime policy
  code while the new authority-zone vocabulary is introduced as a distinct
  cross-repo contract surface.

## Decisions

### Decision: Introduce `AuthorityZone` as a distinct cross-repo contract type instead of renaming `UsageZone`

- **Rationale**: `UsageZone` already carries Canon runtime policy meaning. A
  distinct `AuthorityZone` lets Canon add the S3 vocabulary, including
  `restricted`, without silently changing the existing policy model.
- **Alternatives considered**:
  - Rename `UsageZone` directly: rejected because it would widen the blast
    radius across existing runtime policy code and docs.
  - Alias `authority_zone` to `UsageZone` permanently: rejected because S3 adds
    `restricted` and consumer-facing semantics that go beyond the current zone
    model.

### Decision: Model `authority-governance-v1` as a typed serde envelope with explicit required and optional fields

- **Rationale**: The constitution and repository language rules require stable
  serialization shapes to use typed models rather than ad hoc maps or repeated
  raw keys. A typed envelope also makes fail-closed behavior explicit.
- **Alternatives considered**:
  - Document the contract only in prose: rejected because downstream consumers
    need a machine-readable stable surface.
  - Store the contract as free-form JSON maps: rejected because it would invite
    drift and violate the stable-shape rules.

### Decision: Keep `stage_role_hints` optional and advisory in both packet metadata and adapter projection

- **Rationale**: Optional advisory hints preserve the Canon/Boundline ownership
  boundary while still giving downstream runtimes semantically meaningful
  context.
- **Alternatives considered**:
  - Make role hints required: rejected because missing hints should not block an
    otherwise valid authority contract.
  - Encode executable reviewer or route directives: rejected because that would
    move runtime choice into Canon.

## Implementation Direction

- Add typed `AuthorityZone`, `ChangeClass`, and `AuthorityGovernanceV1Envelope`
  models close to existing policy and artifact metadata surfaces.
- Extend mode metadata with intended persona and optional anti-behavior or
  advisory role-hint semantics without turning modes into runtime instructions.
- Project the contract consistently through packet metadata and the stable
  governance-adapter docs.
- Preserve current packet publication and lineage behavior by attaching the new
  envelope to existing metadata surfaces instead of creating a second publish
  path.

## Likely Touchpoints

- `crates/canon-engine/src/domain/policy.rs`
- `crates/canon-engine/src/domain/mode.rs`
- `crates/canon-engine/src/domain/artifact.rs`
- `crates/canon-engine/src/domain/publish_profile.rs`
- `crates/canon-engine/src/domain/approval.rs`
- `docs/integration/governance-adapter.md`
- `docs/guides/`
- `tests/governance_adapter_surface.rs`
- `tests/mode_profiles.rs`
- `tests/policy_and_traces.rs`
- `CHANGELOG.md`
- `Cargo.toml`