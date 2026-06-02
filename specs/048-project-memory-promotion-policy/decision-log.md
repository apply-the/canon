# Decision Log: Project Memory Promotion Policy

## D-001: Canon owns the shared contract

**Context**: Promotion policy, lineage metadata, and update strategies are
producer-side concerns.
**Decision**: Canon owns the contract; consumers rely on its semantics but must
not redefine them.
**Alternatives**: Consumer-defined promotion semantics; rejected because
divergent definitions break cross-repo trust.
**Consequences**: Consumers depend on Canon contract versioning; breaking changes
require a major version bump.

## D-002: Feature-local contract first, stable docs path later

**Context**: Design iteration benefits from co-location with the feature spec.
**Decision**: The initial contract lives in `specs/048-*/contracts/`; promotion
to `tech-docs/integration/` is a requirement of this feature slice.
**Alternatives**: Publish directly to `tech-docs/integration/`; rejected because
premature stabilization risks costly rework.
**Consequences**: The tasks must include a promotion step that copies the
accepted contract to the stable documentation path.

## D-003: Promotion state is a domain enum, not free text

**Context**: Consumers must switch on promotion state deterministically.
**Decision**: Define `PromotionState` as a closed Rust enum with serde support
(`auto`, `auto-if-approved`, `pending-index`, `index-only`, `evidence-only`,
`manual`).
**Alternatives**: Free-form string; rejected because it invites typos and
consumer-side guessing.
**Consequences**: Changing the vocabulary requires a contract version update
and consumer realignment; there is no guaranteed pre-1.0 compatibility grace
period.

## D-004: UpdateStrategy is a domain enum

**Context**: Non-destructive update semantics must be unambiguous.
**Decision**: Define `UpdateStrategy` as a closed enum (`managed-blocks`,
`proposal-files`, `append-only-index`).
**Alternatives**: Strategy as a trait object; rejected as over-engineering for
three well-defined behaviors.
**Consequences**: Strategy logic can be pattern-matched exhaustively.

## D-005: Default publish profile in TOML policy

**Context**: Operators should be able to override promotion policy per mode
without recompiling.
**Decision**: Ship `defaults/policies/publish-profiles.toml` with per-mode
defaults.
**Alternatives**: Hardcoded mapping; rejected because operator customization is
a stated Canon principle.
**Consequences**: The engine must load the policy file at publish time and
merge with built-in defaults.

## D-006: Backward-compatible publish path

**Context**: Existing `canon publish` invocations must not break.
**Decision**: When no `--profile` is supplied, existing behavior applies
unchanged.
**Alternatives**: Make profile mandatory; rejected because it breaks existing
scripts.
**Consequences**: Profile-aware logic is gated behind `Option<PublishProfile>`.
