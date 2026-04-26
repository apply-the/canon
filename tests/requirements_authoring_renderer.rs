use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_requirements_artifact_from_evidence,
};

const FULL_BRIEF: &str = r#"# Requirements Brief

## Problem

Bound the firmware-flashing workflow to a USB-only CLI surface.

## Outcome

Operators can flash firmware safely with explicit logs and a reversible path.

## Constraints

- USB transport only
- Preserve explicit audit logs

## Non-Negotiables

- Human ownership remains explicit
- Artifacts persist under `.canon/`

## Options

1. Deliver the CLI first.
2. Defer broader orchestration.

## Recommended Path

Deliver the bounded CLI slice first.

## Tradeoffs

- Governance adds upfront structure.

## Consequences

- Reviewers can inspect the packet without chat history.

## Out of Scope

- No GUI in this slice.

## Deferred Work

- Hosted rollout stays deferred.

## Decision Checklist

- [x] Scope is explicit
- [x] Ownership is explicit

## Open Questions

- How is bootloader mode entered?
"#;

const MISSING_OUTCOME_BRIEF: &str = r#"# Requirements Brief

## Problem

Bound the firmware-flashing workflow to a USB-only CLI surface.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Requirements Brief

## Problem

Bound the firmware-flashing workflow to a USB-only CLI surface.

## Outcomes

This near-miss heading should not count.
"#;

#[test]
fn requirements_renderer_preserves_authored_sections_verbatim() {
    let problem_statement = render_requirements_artifact_from_evidence(
        "problem-statement.md",
        "Bound the firmware-flashing workflow before planning.",
        FULL_BRIEF,
        "",
        "",
        "",
    );
    let scope_cuts = render_requirements_artifact_from_evidence(
        "scope-cuts.md",
        "Bound the firmware-flashing workflow before planning.",
        FULL_BRIEF,
        "",
        "",
        "",
    );

    assert!(
        problem_statement.contains(
            "## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface."
        )
    );
    assert!(problem_statement.contains("## Outcome\n\nOperators can flash firmware safely with explicit logs and a reversible path."));
    assert!(!problem_statement.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(scope_cuts.contains("## Scope Cuts\n\n- No GUI in this slice."));
    assert!(scope_cuts.contains("## Deferred Work\n\n- Hosted rollout stays deferred."));
}

#[test]
fn requirements_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_requirements_artifact_from_evidence(
        "problem-statement.md",
        "Bound the firmware-flashing workflow before planning.",
        MISSING_OUTCOME_BRIEF,
        "",
        "",
        "",
    );

    assert!(
        rendered.contains(
            "## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface."
        )
    );
    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Outcome`"));
}

#[test]
fn requirements_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_requirements_artifact_from_evidence(
        "problem-statement.md",
        "Bound the firmware-flashing workflow before planning.",
        NEAR_MISS_BRIEF,
        "",
        "",
        "",
    );

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Outcome`"));
    assert!(!rendered.contains("## Outcomes"));
}
