use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_incident_artifact};

const FULL_BRIEF: &str = r#"# Incident Brief

## Incident Scope

- payments-api and checkout flow only.

## Trigger And Current State

- elevated 5xx responses after the last deploy.

## Operational Constraints

- no autonomous remediation
- no schema changes

## Known Facts

- errors started after the deploy
- rollback remains available

## Working Hypotheses

- retry amplification is exhausting the service

## Evidence Gaps

- database saturation is not yet confirmed

## Impacted Surfaces

- payments-api
- checkout flow

## Propagation Paths

- checkout request path

## Confidence And Unknowns

- medium confidence until saturation evidence is collected

## Immediate Actions

- disable async retries

## Ordered Sequence

1. capture blast radius
2. disable retries
3. reassess error rate

## Stop Conditions

- error rate stabilizes below the alert threshold

## Decision Points

- decide whether rollback is still required

## Approved Actions

- disable retries within the bounded surface

## Deferred Actions

- schema-level changes remain out of scope

## Verification Checks

- confirm 5xx rate drops

## Release Readiness

- keep recommendation-only posture until the owner accepts the packet

## Follow-Up Work

- add a saturation dashboard and post-incident review item
"#;

const MISSING_STOP_CONDITIONS_BRIEF: &str = r#"# Incident Brief

## Incident Scope

- payments-api and checkout flow only.

## Trigger And Current State

- elevated 5xx responses after the last deploy.

## Operational Constraints

- no autonomous remediation

## Immediate Actions

- disable async retries

## Ordered Sequence

1. capture blast radius
2. disable retries
"#;

const NEAR_MISS_BRIEF: &str = r#"# Incident Brief

## Immediate Actions

- disable async retries

## Ordered Sequence

1. capture blast radius
2. disable retries

## Stop Rules

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn incident_renderer_preserves_authored_sections_verbatim() {
    let incident_frame = render_incident_artifact("incident-frame.md", FULL_BRIEF);
    let containment_plan = render_incident_artifact("containment-plan.md", FULL_BRIEF);

    assert!(incident_frame.contains("## Incident Scope\n\n- payments-api and checkout flow only."));
    assert!(incident_frame.contains(
        "## Trigger And Current State\n\n- elevated 5xx responses after the last deploy."
    ));
    assert!(!incident_frame.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(containment_plan.contains("## Immediate Actions\n\n- disable async retries"));
    assert!(
        containment_plan
            .contains("## Stop Conditions\n\n- error rate stabilizes below the alert threshold")
    );
}

#[test]
fn incident_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_incident_artifact("containment-plan.md", MISSING_STOP_CONDITIONS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Stop Conditions`"));
}

#[test]
fn incident_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_incident_artifact("containment-plan.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Stop Conditions`"));
    assert!(!rendered.contains(
        "## Stop Rules\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
