use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_refactor_artifact};

const FULL_BRIEF: &str = r#"# Refactor Brief

## Preserved Behavior

- Session revocation formatting remains externally unchanged.

## Approved Exceptions

- none.

## Refactor Scope

- auth session boundary and repository composition only.

## Allowed Paths

- crates/canon-engine/src/artifacts/markdown.rs
- crates/canon-engine/src/orchestrator/service/mode_change.rs

## Structural Rationale

- Isolate authored-body rendering from evidence summaries.

## Untouched Surface

- approval semantics
- publish destinations

## Safety-Net Evidence

- focused renderer and run suites protect preserved behavior.

## Regression Findings

- no regression findings are accepted in this bounded packet.

## Contract Drift

- no public contract drift is allowed.

## Reviewer Notes

- reviewer confirms the packet still reads as preservation-only guidance.

## Feature Audit

- no new feature behavior is introduced in this refactor packet.

## Decision

- preserve behavior and stop if the authored contract expands.
"#;

const MISSING_DECISION_BRIEF: &str = r#"# Refactor Brief

## Feature Audit

- no new feature behavior is introduced in this refactor packet.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Refactor Brief

## Feature Audit

- no new feature behavior is introduced in this refactor packet.

## Decisions

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn refactor_renderer_preserves_authored_sections_verbatim() {
    let preserved_behavior =
        render_refactor_artifact("preserved-behavior.md", FULL_BRIEF, "maintainer");
    let no_feature_addition =
        render_refactor_artifact("no-feature-addition.md", FULL_BRIEF, "maintainer");

    assert!(preserved_behavior.contains(
        "## Preserved Behavior\n\n- Session revocation formatting remains externally unchanged."
    ));
    assert!(preserved_behavior.contains("## Approved Exceptions\n\n- none."));
    assert!(!preserved_behavior.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(no_feature_addition.contains(
        "## Feature Audit\n\n- no new feature behavior is introduced in this refactor packet."
    ));
    assert!(
        no_feature_addition.contains(
            "## Decision\n\n- preserve behavior and stop if the authored contract expands."
        )
    );
}

#[test]
fn refactor_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered =
        render_refactor_artifact("no-feature-addition.md", MISSING_DECISION_BRIEF, "maintainer");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Decision`"));
}

#[test]
fn refactor_renderer_treats_near_miss_heading_as_missing() {
    let rendered =
        render_refactor_artifact("no-feature-addition.md", NEAR_MISS_BRIEF, "maintainer");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Decision`"));
    assert!(!rendered.contains(
        "## Decisions\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
