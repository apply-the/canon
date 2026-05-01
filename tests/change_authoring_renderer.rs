use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_change_artifact};

const FULL_BRIEF: &str = r#"# Change Brief

## System Slice

auth session boundary and persistence layer.

## Domain Slice

Session lifecycle and cleanup semantics within the auth domain.

## Excluded Areas

- payment settlement
- billing reports

## Intended Change

Add bounded repository methods while preserving the public auth contract.

## Legacy Invariants

- session revocation remains eventually consistent
- audit log ordering stays stable

## Domain Invariants

- a revoked session must never become active again through cleanup retries
- audit trails must preserve causal order across repository updates

## Forbidden Normalization

- Do not collapse audit-ordering quirks that operators still rely on.

## Change Surface

- session repository
- auth service
- token cleanup job

## Ownership

- primary owner: maintainer

## Cross-Context Risks

- cleanup scheduling can leak into notification flows if repository boundaries widen

## Implementation Plan

Add bounded repository methods and preserve the public auth contract.

## Sequencing

1. Add bounded repository methods.
2. Switch callers behind the preserved contract.

## Validation Strategy

- contract tests
- invariant checks

## Independent Checks

- rollback rehearsal by a separate operator

## Decision Record

Prefer additive change over normalization to preserve operator expectations.

## Decision Drivers

- Preserve operator expectations.
- Keep the auth contract stable during the bounded repository change.

## Options Considered

- Option 1 keeps the additive repository helper inside the auth boundary.
- Option 2 normalizes scheduling and cleanup behavior in the same slice.

## Decision Evidence

- Existing operator workflows still depend on the current auth cleanup ordering.
- Contract tests already guard the preserved API surface.

## Boundary Tradeoffs

- keep cleanup logic inside the auth boundary even if that duplicates some scheduling code

## Recommendation

- Start with the additive repository helper and defer normalization to a later slice.

## Why Not The Others

- Normalizing cleanup behavior now would widen the change surface beyond the bounded auth slice.

## Consequences

- preserved surface remains explicit and reviewable

## Unresolved Questions

- should the cleanup job roll out in the same slice?

Owner: maintainer
Risk Level: bounded-impact
Zone: yellow
"#;

const MISSING_OWNERSHIP_BRIEF: &str = r#"# Change Brief

## System Slice

auth session boundary and persistence layer.

## Excluded Areas

- payment settlement

## Change Surface

- session repository
"#;

const NEAR_MISS_BRIEF: &str = r#"# Change Brief

## System Slice

auth session boundary and persistence layer.

## Excluded Areas

- payment settlement

## Change Surfaces

This near-miss heading should not count.
"#;

#[test]
fn change_renderer_preserves_authored_sections_verbatim() {
    let system_slice = render_change_artifact("system-slice.md", FULL_BRIEF, "maintainer");
    let implementation_plan =
        render_change_artifact("implementation-plan.md", FULL_BRIEF, "maintainer");
    let decision_record = render_change_artifact("decision-record.md", FULL_BRIEF, "maintainer");

    assert!(
        system_slice.contains("## System Slice\n\nauth session boundary and persistence layer.")
    );
    assert!(system_slice.contains(
        "## Domain Slice\n\nSession lifecycle and cleanup semantics within the auth domain."
    ));
    assert!(system_slice.contains("## Excluded Areas\n\n- payment settlement"));
    assert!(!system_slice.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(implementation_plan.contains("## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract."));
    assert!(implementation_plan.contains("## Sequencing\n\n1. Add bounded repository methods."));
    assert!(decision_record.contains("## Decision Drivers\n\n- Preserve operator expectations."));
    assert!(decision_record.contains("## Options Considered\n\n- Option 1 keeps the additive repository helper inside the auth boundary."));
    assert!(decision_record.contains("## Decision Evidence\n\n- Existing operator workflows still depend on the current auth cleanup ordering."));
    assert!(decision_record.contains("## Recommendation\n\n- Start with the additive repository helper and defer normalization to a later slice."));
    assert!(decision_record.contains("## Why Not The Others\n\n- Normalizing cleanup behavior now would widen the change surface beyond the bounded auth slice."));
}

#[test]
fn change_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered =
        render_change_artifact("change-surface.md", MISSING_OWNERSHIP_BRIEF, "maintainer");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Ownership`"));
}

#[test]
fn change_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_change_artifact("change-surface.md", NEAR_MISS_BRIEF, "maintainer");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Change Surface`"));
    assert!(!rendered.contains("## Change Surfaces"));
}
