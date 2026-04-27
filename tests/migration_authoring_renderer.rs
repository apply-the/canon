use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_migration_artifact};

const FULL_BRIEF: &str = r#"# Migration Brief

## Current State

- auth-v1 serves login and token refresh traffic.

## Target State

- auth-v2 serves the same bounded traffic surface.

## Transition Boundaries

- login and token refresh only.

## Guaranteed Compatibility

- existing tokens continue to validate

## Temporary Incompatibilities

- admin reporting stays on v1 during the rollout

## Coexistence Rules

- dual-write session metadata during cutover

## Ordered Steps

1. enable shadow reads
2. start dual-write
3. cut traffic to auth-v2

## Parallelizable Work

- docs and dashboards can update in parallel

## Cutover Criteria

- error rate and token validation remain stable

## Rollback Triggers

- token validation failures or elevated login errors

## Fallback Paths

- route bounded traffic back to auth-v1

## Re-Entry Criteria

- compatibility regressions are resolved and revalidated

## Verification Checks

- login and token validation pass against auth-v2

## Residual Risks

- admin reporting remains temporarily inconsistent

## Release Readiness

- keep recommendation-only posture until the owner accepts the packet

## Migration Decisions

- retain dual-write during the bounded cutover

## Deferred Decisions

- move admin reporting after the bounded migration completes

## Approval Notes

- explicit migration-lead sign-off is required before broader rollout
"#;

const MISSING_ROLLBACK_TRIGGERS_BRIEF: &str = r#"# Migration Brief

## Current State

- auth-v1 serves login and token refresh traffic.

## Target State

- auth-v2 serves the same bounded traffic surface.

## Fallback Paths

- route bounded traffic back to auth-v1

## Re-Entry Criteria

- compatibility regressions are resolved and revalidated
"#;

const NEAR_MISS_BRIEF: &str = r#"# Migration Brief

## Fallback Paths

- route bounded traffic back to auth-v1

## Re-Entry Criteria

- compatibility regressions are resolved and revalidated

## Rollback Plan

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn migration_renderer_preserves_authored_sections_verbatim() {
    let source_target_map = render_migration_artifact("source-target-map.md", FULL_BRIEF);
    let fallback_plan = render_migration_artifact("fallback-plan.md", FULL_BRIEF);

    assert!(
        source_target_map
            .contains("## Current State\n\n- auth-v1 serves login and token refresh traffic.")
    );
    assert!(
        source_target_map
            .contains("## Target State\n\n- auth-v2 serves the same bounded traffic surface.")
    );
    assert!(!source_target_map.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(
        fallback_plan.contains(
            "## Rollback Triggers\n\n- token validation failures or elevated login errors"
        )
    );
    assert!(fallback_plan.contains("## Fallback Paths\n\n- route bounded traffic back to auth-v1"));
}

#[test]
fn migration_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_migration_artifact("fallback-plan.md", MISSING_ROLLBACK_TRIGGERS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Rollback Triggers`"));
}

#[test]
fn migration_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_migration_artifact("fallback-plan.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Rollback Triggers`"));
    assert!(!rendered.contains(
        "## Rollback Plan\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
