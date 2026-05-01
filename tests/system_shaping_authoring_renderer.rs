use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_system_shaping_artifact,
};

const FULL_BRIEF: &str = r#"# System Shaping Brief

Intent: shape the authored packet contract for remaining bounded analysis modes.
Constraint: preserve approval, evidence, and publish behavior while strengthening authored fidelity.

## System Shape

Bound the packet around authored-body preservation, critique visibility, and durable packet review.

## Boundary Decisions

- Keep runtime governance unchanged.
- Keep authored-body extraction inside the markdown rendering surface.

## Domain Responsibilities

- Preserve authored packet sections verbatim.
- Surface explicit gaps when a required authored section is absent.

## Candidate Bounded Contexts

- Runtime Governance
- Artifact Authoring

## Core And Supporting Domain Hypotheses

- Runtime Governance remains core.
- Artifact Authoring remains supporting.

## Ubiquitous Language

- Packet: the emitted artifact bundle for one mode.

## Domain Invariants

- Approval semantics remain unchanged.

## Boundary Risks And Open Questions

- Shared helpers may still leak responsibilities across contexts.

## Structural Options

- Reuse the existing authored-section helper instead of inventing a second renderer path.

## Selected Boundaries

- Keep authoring concerns in artifact rendering and runtime orchestration separate.

## Rationale

- The packet stays reviewable when the contract is explicit.

## Why Not The Others

- A second renderer path would duplicate authored-body extraction rules and make future packet evolution harder to review.

## Capabilities

- Preserve canonical headings.
- Emit explicit missing-body markers.

## Dependencies

- Renderer helper reuse.
- Existing artifact contract metadata.

## Gaps

- Implementation and refactor still need authored-source handoff.

## Delivery Phases

1. Fix authored-source handoff.
2. Preserve authored sections verbatim.

## Sequencing Rationale

- Fix the renderer contract before broad docs sync.

## Risk per Phase

- Runtime regressions if existing run posture changes.

## Hotspots

- Mixed summary input hides authored headings.

## Mitigation Status

- Use targeted renderer and run tests.

## Unresolved Risks

- Remaining rollout for later unspecialized modes.
"#;

const MISSING_BOUNDARY_DECISIONS_BRIEF: &str = r#"# System Shaping Brief

Intent: shape the authored packet contract for remaining bounded analysis modes.
Constraint: preserve approval, evidence, and publish behavior while strengthening authored fidelity.

## System Shape

Bound the packet around authored-body preservation, critique visibility, and durable packet review.

## Domain Responsibilities

- Preserve authored packet sections verbatim.
"#;

const NEAR_MISS_BRIEF: &str = r#"# System Shaping Brief

Intent: shape the authored packet contract for remaining bounded analysis modes.
Constraint: preserve approval, evidence, and publish behavior while strengthening authored fidelity.

## System Shape

Bound the packet around authored-body preservation, critique visibility, and durable packet review.

## Boundary Decision

This near-miss heading should not satisfy the canonical contract.

## Domain Responsibilities

- Preserve authored packet sections verbatim.
"#;

#[test]
fn system_shaping_renderer_preserves_authored_sections_verbatim() {
    let system_shape =
        render_system_shaping_artifact("system-shape.md", FULL_BRIEF, "generated", "critique");
    let architecture_outline = render_system_shaping_artifact(
        "architecture-outline.md",
        FULL_BRIEF,
        "generated",
        "critique",
    );

    assert!(
        system_shape
            .contains("## System Shape\n\nBound the packet around authored-body preservation")
    );
    assert!(system_shape.contains("## Boundary Decisions\n\n- Keep runtime governance unchanged."));
    assert!(
        system_shape.contains(
            "## Domain Responsibilities\n\n- Preserve authored packet sections verbatim."
        )
    );
    assert!(!system_shape.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(
        architecture_outline
            .contains("## Structural Options\n\n- Reuse the existing authored-section helper")
    );
    assert!(architecture_outline.contains("## Selected Boundaries\n\n- Keep authoring concerns in artifact rendering and runtime orchestration separate."));
    assert!(
        architecture_outline.contains(
            "## Rationale\n\n- The packet stays reviewable when the contract is explicit."
        )
    );
    assert!(
        architecture_outline.contains(
            "## Why Not The Others\n\n- A second renderer path would duplicate authored-body extraction rules and make future packet evolution harder to review."
        )
    );
}

#[test]
fn system_shaping_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered = render_system_shaping_artifact(
        "system-shape.md",
        MISSING_BOUNDARY_DECISIONS_BRIEF,
        "generated",
        "critique",
    );

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Boundary Decisions`"));
}

#[test]
fn system_shaping_renderer_treats_near_miss_heading_as_missing() {
    let rendered =
        render_system_shaping_artifact("system-shape.md", NEAR_MISS_BRIEF, "generated", "critique");

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Boundary Decisions`"));
    assert!(!rendered.contains(
        "## Boundary Decision\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
