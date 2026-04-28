use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_security_assessment_artifact,
};

const FULL_BRIEF: &str = r#"# Security Assessment Brief

## Assessment Scope

- webhook ingress and token verification only

## In-Scope Assets

- edge webhook gateway
- signature verification service

## Trust Boundaries

- internet to edge gateway
- edge gateway to internal verification service

## Out Of Scope

- downstream analytics processing

## Threat Inventory

- forged webhook payloads
- replay attempts against stale signatures

## Attacker Goals

- inject unauthorized events

## Boundary Threats

- signature headers can be stripped at the edge if proxy rules drift

## Risk Findings

- missing replay-window enforcement raises event forgery risk

## Likelihood And Impact

- likelihood is moderate and impact is high for privileged webhook actions

## Proposed Owners

- platform security owns the replay-window fix recommendation

## Recommended Controls

- enforce a bounded replay window and reject stale signatures

## Tradeoffs

- tighter replay windows increase operational sensitivity to clock skew

## Sequencing Notes

1. enable replay-window metrics
2. enforce rejection after validation proves stable

## Assumptions

- signature secrets stay in the current managed secret store

## Evidence Gaps

- no packet capture currently proves proxy header preservation

## Unobservable Surfaces

- third-party sender retry behavior is only partially visible

## Applicable Standards

- OWASP ASVS request integrity guidance applies to the webhook edge

## Control Families

- request validation and secret handling controls are most relevant

## Scope Limits

- this packet informs controls and does not certify compliance

## Source Inputs

- src/webhooks/verifier.rs
- infra/proxy/webhook-gateway.yaml

## Independent Checks

- focused renderer and run tests verify packet preservation and honesty behavior

## Deferred Verification

- perform a bounded replay simulation after the recommended control lands
"#;

const MISSING_BOUNDARY_THREATS_BRIEF: &str = r#"# Security Assessment Brief

## Assessment Scope

- webhook ingress only

## Threat Inventory

- forged webhook payloads

## Attacker Goals

- inject unauthorized events
"#;

const NEAR_MISS_BRIEF: &str = r#"# Security Assessment Brief

## Threat Inventory

- forged webhook payloads

## Attacker Goals

- inject unauthorized events

## Threat Boundaries

This near-miss heading should not satisfy the canonical `Boundary Threats` section.
"#;

#[test]
fn security_assessment_renderer_preserves_authored_sections_verbatim() {
    let overview = render_security_assessment_artifact("assessment-overview.md", FULL_BRIEF);
    let threat_model = render_security_assessment_artifact("threat-model.md", FULL_BRIEF);
    let risk_register = render_security_assessment_artifact("risk-register.md", FULL_BRIEF);
    let mitigations = render_security_assessment_artifact("mitigations.md", FULL_BRIEF);
    let assumptions = render_security_assessment_artifact("assumptions-and-gaps.md", FULL_BRIEF);
    let compliance = render_security_assessment_artifact("compliance-anchors.md", FULL_BRIEF);
    let evidence = render_security_assessment_artifact("assessment-evidence.md", FULL_BRIEF);

    assert!(
        overview.contains("## Assessment Scope\n\n- webhook ingress and token verification only")
    );
    assert!(overview.contains("## In-Scope Assets\n\n- edge webhook gateway"));
    assert!(!overview.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(threat_model.contains(
        "## Boundary Threats\n\n- signature headers can be stripped at the edge if proxy rules drift"
    ));
    assert!(risk_register.contains(
        "## Likelihood And Impact\n\n- likelihood is moderate and impact is high for privileged webhook actions"
    ));
    assert!(mitigations.contains(
        "## Recommended Controls\n\n- enforce a bounded replay window and reject stale signatures"
    ));
    assert!(assumptions.contains(
        "## Evidence Gaps\n\n- no packet capture currently proves proxy header preservation"
    ));
    assert!(compliance.contains(
        "## Scope Limits\n\n- this packet informs controls and does not certify compliance"
    ));
    assert!(evidence.contains(
        "## Deferred Verification\n\n- perform a bounded replay simulation after the recommended control lands"
    ));
}

#[test]
fn security_assessment_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered =
        render_security_assessment_artifact("threat-model.md", MISSING_BOUNDARY_THREATS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Boundary Threats`"));
}

#[test]
fn security_assessment_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_security_assessment_artifact("threat-model.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Boundary Threats`"));
    assert!(!rendered.contains(
        "## Threat Boundaries\n\nThis near-miss heading should not satisfy the canonical `Boundary Threats` section."
    ));
}
