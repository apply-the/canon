use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_verification_artifact,
};

const FULL_BRIEF: &str = r#"# Verification Brief

## Claims Under Test

- rollback remains bounded and auditable
- operator evidence remains tied to the rollback boundary

## Invariant Checks

- auth token metadata remains explicit during rollback

## Contract Assumptions

- rollback metadata must remain explicit

## Verification Outcome

Status: supported

## Challenge Findings

- the strongest adversarial path still terminates within the bounded rollback surface

## Contradictions

- none recorded

## Verified Claims

- rollback remains bounded and auditable

## Rejected Claims

- none recorded

## Overall Verdict

Status: supported

Rationale: the current evidence covers the authored claim set.

## Open Findings

Status: no-open-findings

- No unresolved findings remain from the current verification packet.

## Required Follow-Up

- Keep the verification packet attached to downstream release review.
"#;

const MISSING_OVERALL_VERDICT_BRIEF: &str = r#"# Verification Brief

## Verified Claims

- rollback remains bounded and auditable

## Rejected Claims

- none recorded
"#;

const NEAR_MISS_BRIEF: &str = r#"# Verification Brief

## Verified Claims

- rollback remains bounded and auditable

## Rejected Claims

- none recorded

## Verdict

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn verification_renderer_preserves_authored_sections_verbatim() {
    let invariants =
        render_verification_artifact("invariants-checklist.md", FULL_BRIEF, "", "", "");
    let report = render_verification_artifact("verification-report.md", FULL_BRIEF, "", "", "");

    assert!(
        invariants.contains("## Claims Under Test\n\n- rollback remains bounded and auditable")
    );
    assert!(
        invariants.contains(
            "## Invariant Checks\n\n- auth token metadata remains explicit during rollback"
        )
    );
    assert!(!invariants.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(report.contains("## Overall Verdict\n\nStatus: supported"));
    assert!(report.contains("## Verified Claims\n\n- rollback remains bounded and auditable"));
}

#[test]
fn verification_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_verification_artifact(
        "verification-report.md",
        MISSING_OVERALL_VERDICT_BRIEF,
        "",
        "",
        "",
    );

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Overall Verdict`"));
}

#[test]
fn verification_renderer_treats_near_miss_heading_as_missing() {
    let rendered =
        render_verification_artifact("verification-report.md", NEAR_MISS_BRIEF, "", "", "");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Overall Verdict`"));
    assert!(!rendered.contains(
        "## Verdict\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
