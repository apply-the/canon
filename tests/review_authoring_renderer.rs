use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_review_artifact};

const FULL_BRIEF: &str = r#"# Review Brief

## Review Target

- bounded service boundary package.

## Evidence Basis

- owned interfaces, current tests, and rollback notes.

## Boundary Findings

- no boundary expansion beyond the authored review target was detected.

## Ownership Notes

- reviewer remains accountable for downstream acceptance.

## Missing Evidence

Status: evidence-bounded

- No critical evidence gaps remain from the authored review packet.

## Collection Priorities

- preserve the current evidence bundle for later inspection.

## Decision Impact

- downstream implementation remains reversible within the bounded package.

## Reversibility Concerns

- stop before broader rollout if the review packet changes materially.

## Final Disposition

Status: ready-with-review-notes

Rationale: the review packet is bounded enough for downstream inspection.

## Accepted Risks

- residual review notes remain bounded to this package.
"#;

const MISSING_FINAL_DISPOSITION_BRIEF: &str = r#"# Review Brief

## Review Target

- bounded service boundary package.

## Accepted Risks

- residual review notes remain bounded to this package.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Review Brief

## Accepted Risks

- residual review notes remain bounded to this package.

## Final Decision

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn review_renderer_preserves_authored_sections_verbatim() {
    let review_brief = render_review_artifact("review-brief.md", FULL_BRIEF, "", "", "");
    let disposition = render_review_artifact("review-disposition.md", FULL_BRIEF, "", "", "");

    assert!(review_brief.contains("## Review Target\n\n- bounded service boundary package."));
    assert!(
        review_brief.contains(
            "## Evidence Basis\n\n- owned interfaces, current tests, and rollback notes."
        )
    );
    assert!(!review_brief.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(disposition.contains("## Final Disposition\n\nStatus: ready-with-review-notes"));
    assert!(
        disposition.contains(
            "## Accepted Risks\n\n- residual review notes remain bounded to this package."
        )
    );
}

#[test]
fn review_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_review_artifact(
        "review-disposition.md",
        MISSING_FINAL_DISPOSITION_BRIEF,
        "",
        "",
        "",
    );

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Final Disposition`"));
}

#[test]
fn review_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_review_artifact("review-disposition.md", NEAR_MISS_BRIEF, "", "", "");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Final Disposition`"));
    assert!(!rendered.contains(
        "## Final Decision\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
