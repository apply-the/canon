use canon_engine::artifacts::markdown::{MISSING_AUTHORED_BODY_MARKER, render_discovery_artifact};

const FULL_BRIEF: &str = r#"# Discovery Brief

## Problem Domain

Clarify governed runtime depth for analysis modes.

## Repo Surface

- crates/canon-engine/src/orchestrator/service/
- tests/integration/discovery_run.rs

## Immediate Tensions

- Discovery authoring should stay explicit and reviewable.

## Downstream Handoff

Translate this packet into requirements mode with concrete scope cuts.

## Unknowns

- Which downstream mode should consume repo-grounded discovery first?

## Assumptions

- The existing Canon persistence model remains stable.

## Validation Targets

- Confirm authored headings survive into emitted artifacts.

## Confidence Levels

- Medium until end-to-end runs confirm the new contract.

## In-Scope Context

- Governed analysis modes only.

## Out-of-Scope Context

- No architecture or review-mode changes in this packet.

## Translation Trigger

Translate this packet into requirements mode with concrete scope cuts.

## Options

1. Tighten discovery authoring contracts first.

## Constraints

- Stay within the existing Canon persistence model.

## Recommended Direction

Tighten discovery authoring contracts first.

## Next-Phase Shape

Translate this packet into requirements mode with concrete scope cuts.

## Pressure Points

- Repo-local skills and runtime outputs can drift without a shared authored contract.

## Blocking Decisions

- Decide whether the first slice stays bounded to discovery or spans multiple modes.

## Open Questions

- Which downstream mode should consume repo-grounded discovery first?

## Recommended Owner

- researcher
"#;

const MISSING_SURFACE_BRIEF: &str = r#"# Discovery Brief

## Problem Domain

Clarify governed runtime depth for analysis modes.

## Immediate Tensions

- Discovery authoring should stay explicit and reviewable.

## Downstream Handoff

Translate this packet into requirements mode with concrete scope cuts.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Discovery Brief

## Problem Domain

Clarify governed runtime depth for analysis modes.

## Repository Surface

This near-miss heading should not count.

## Immediate Tensions

- Discovery authoring should stay explicit and reviewable.

## Downstream Handoff

Translate this packet into requirements mode with concrete scope cuts.
"#;

#[test]
fn discovery_renderer_preserves_authored_sections_verbatim() {
    let problem_map = render_discovery_artifact("problem-map.md", FULL_BRIEF);
    let boundary = render_discovery_artifact("context-boundary.md", FULL_BRIEF);

    assert!(
        problem_map
            .contains("## Problem Domain\n\nClarify governed runtime depth for analysis modes.")
    );
    assert!(
        problem_map.contains("## Repo Surface\n\n- crates/canon-engine/src/orchestrator/service/")
    );
    assert!(!problem_map.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(boundary.contains("## Translation Trigger\n\nTranslate this packet into requirements mode with concrete scope cuts."));
}

#[test]
fn discovery_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_discovery_artifact("problem-map.md", MISSING_SURFACE_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Repo Surface`"));
}

#[test]
fn discovery_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_discovery_artifact("problem-map.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Repo Surface`"));
    assert!(!rendered.contains("## Repository Surface"));
}
