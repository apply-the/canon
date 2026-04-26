use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_implementation_artifact,
};

const FULL_BRIEF: &str = r#"# Implementation Brief

## Task Mapping

1. Add bounded authored-body extraction for execution artifacts.
2. Keep recommendation-only posture unchanged.

## Bounded Changes

- execution artifact rendering only
- authored-section extraction only

## Mutation Bounds

- crates/canon-engine/src/artifacts/markdown.rs
- crates/canon-engine/src/orchestrator/service/mode_change.rs

## Allowed Paths

- crates/canon-engine/src/artifacts/markdown.rs
- crates/canon-engine/src/orchestrator/service/mode_change.rs

## Executed Changes

- No workspace mutation is implied by the guidance packet itself.

## Task Linkage

- Map authored packet sections to emitted implementation artifacts.

## Completion Evidence

- Focused renderer and run suites protect the authored contract.

## Remaining Risks

- Existing inline briefs may still exist during the transition.

## Safety-Net Evidence

- `cargo test --test implementation_authoring_renderer`

## Independent Checks

- Review emitted packet artifacts without relying on chat context.

## Rollback Triggers

- Packet artifacts stop preserving authored bodies verbatim.

## Rollback Steps

1. Revert the authored-body renderer change.
2. Re-run the focused implementation suites.
"#;

const MISSING_ROLLBACK_STEPS_BRIEF: &str = r#"# Implementation Brief

## Task Mapping

1. Add bounded authored-body extraction for execution artifacts.

## Bounded Changes

- execution artifact rendering only

## Rollback Triggers

- Packet artifacts stop preserving authored bodies verbatim.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Implementation Brief

## Rollback Triggers

- Packet artifacts stop preserving authored bodies verbatim.

## Rollback Plan

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn implementation_renderer_preserves_authored_sections_verbatim() {
    let task_mapping = render_implementation_artifact("task-mapping.md", FULL_BRIEF, "maintainer");
    let rollback_notes =
        render_implementation_artifact("rollback-notes.md", FULL_BRIEF, "maintainer");

    assert!(task_mapping.contains(
        "## Task Mapping\n\n1. Add bounded authored-body extraction for execution artifacts."
    ));
    assert!(task_mapping.contains("## Bounded Changes\n\n- execution artifact rendering only"));
    assert!(!task_mapping.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(rollback_notes.contains(
        "## Rollback Triggers\n\n- Packet artifacts stop preserving authored bodies verbatim."
    ));
    assert!(
        rollback_notes
            .contains("## Rollback Steps\n\n1. Revert the authored-body renderer change.")
    );
}

#[test]
fn implementation_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_implementation_artifact(
        "rollback-notes.md",
        MISSING_ROLLBACK_STEPS_BRIEF,
        "maintainer",
    );

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Rollback Steps`"));
}

#[test]
fn implementation_renderer_treats_near_miss_heading_as_missing() {
    let rendered =
        render_implementation_artifact("rollback-notes.md", NEAR_MISS_BRIEF, "maintainer");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Rollback Steps`"));
    assert!(!rendered.contains(
        "## Rollback Plan\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
