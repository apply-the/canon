use canon_engine::artifacts::markdown::{
    C4_MISSING_AUTHORED_BODY_MARKER, MISSING_AUTHORED_BODY_MARKER, render_architecture_artifact,
};

const FULL_BRIEF: &str = r#"# Architecture Brief

## System Context

The bounded `analytics-cli` consumes raw event files and produces aggregated
reports for downstream finance teams. External actors:

- finance-analyst (reads reports)
- ops-pipeline (writes raw event files)

## Containers

- `analytics-cli` (single-binary Rust CLI)
- `report-store` (object storage bucket)
- `metrics-sink` (managed time-series store)

## Components

- `event-loader` reads raw events from disk.
- `aggregator` collapses events into report rows.
- `report-writer` persists rows to `report-store`.
- `metrics-emitter` pushes counters to `metrics-sink`.
"#;

const PARTIAL_BRIEF: &str = r#"# Architecture Brief

## System Context

The bounded service is `payments-api` only.

## Containers

- `payments-api` (HTTP service)
- `payments-db` (managed Postgres)
"#;

const NEAR_MISS_BRIEF: &str = r#"# Architecture Brief

## C4 - System Context

This brief uses a near-miss heading rather than the canonical `## System Context`.

## Container View

This brief uses a near-miss heading rather than the canonical `## Containers`.

## Component View

This brief uses a near-miss heading rather than the canonical `## Components`.
"#;

const DECISION_BRIEF: &str = r#"# Architecture Brief

## Decision

Adopt a dedicated architecture decision shape so the packet records the winning decision explicitly.

## Constraints

- Preserve the current C4 artifacts.
- Keep missing authored context explicit.

## Evaluation Criteria

- Reviewability
- Boundary clarity

## Decision Drivers

- Reviewers need to see the chosen option and its rationale without reading chat history.
- Architecture packets must remain critique-first.

## Options Considered

- Keep the existing generic architecture summary.
- Preserve authored decision and option-analysis sections directly in the decision artifacts.

## Pros

- Reviewers can inspect the actual decision record.
- The packet becomes reusable outside the original conversation.

## Cons

- Authors must provide more explicit decision content.

## Recommendation

Preserve the authored decision and option-analysis sections in the existing architecture decision artifacts.

## Consequences

- Reviewers can see the chosen direction without relying on chat history.
- Authors must keep the decision packet explicit and current.

## Why Not The Others

- The generic summary shape hides rejected alternatives.
- A brand new artifact family would widen scope unnecessarily.
"#;

const MISSING_DECISION_SECTION_BRIEF: &str = r#"# Architecture Brief

## Decision

Adopt a dedicated architecture decision shape.

## Constraints

- Preserve the current C4 artifacts.

## Evaluation Criteria

- Reviewability

## Decision Drivers

- Reviewers need explicit recommendation rationale.

## Options Considered

- Keep the current generic summary.
- Preserve authored decision sections.

## Pros

- Clear recommendation trace.

## Cons

- Requires more authored content.

## Recommendation

Preserve authored decision sections in the emitted artifacts.
"#;

#[test]
fn renderer_preserves_authored_c4_sections_verbatim() {
    let system_context = render_architecture_artifact("system-context.md", FULL_BRIEF, "", "");
    assert!(system_context.contains("# System Context"));
    assert!(system_context.contains("finance-analyst (reads reports)"));
    assert!(!system_context.contains(C4_MISSING_AUTHORED_BODY_MARKER));

    let container_view = render_architecture_artifact("container-view.md", FULL_BRIEF, "", "");
    assert!(container_view.contains("# Container View"));
    assert!(container_view.contains("`analytics-cli` (single-binary Rust CLI)"));
    assert!(!container_view.contains(C4_MISSING_AUTHORED_BODY_MARKER));

    let component_view = render_architecture_artifact("component-view.md", FULL_BRIEF, "", "");
    assert!(component_view.contains("# Component View"));
    assert!(component_view.contains("`metrics-emitter` pushes counters to `metrics-sink`."));
    assert!(!component_view.contains(C4_MISSING_AUTHORED_BODY_MARKER));
}

#[test]
fn renderer_emits_missing_body_marker_for_each_omitted_c4_section() {
    let component_view = render_architecture_artifact("component-view.md", PARTIAL_BRIEF, "", "");
    assert!(component_view.contains(C4_MISSING_AUTHORED_BODY_MARKER));
    assert!(component_view.contains("`## Components`"));

    let system_context = render_architecture_artifact("system-context.md", PARTIAL_BRIEF, "", "");
    assert!(!system_context.contains(C4_MISSING_AUTHORED_BODY_MARKER));
}

#[test]
fn renderer_emits_missing_body_marker_when_all_c4_sections_are_absent() {
    let empty_brief = "# Architecture Brief\n\nNothing authored.\n";
    for file in ["system-context.md", "container-view.md", "component-view.md"] {
        let rendered = render_architecture_artifact(file, empty_brief, "", "");
        assert!(
            rendered.contains(C4_MISSING_AUTHORED_BODY_MARKER),
            "{file} should emit missing-body marker"
        );
    }
}

#[test]
fn renderer_emits_missing_body_marker_for_near_miss_headings() {
    for file in ["system-context.md", "container-view.md", "component-view.md"] {
        let rendered = render_architecture_artifact(file, NEAR_MISS_BRIEF, "", "");
        assert!(
            rendered.contains(C4_MISSING_AUTHORED_BODY_MARKER),
            "{file} should not consume near-miss heading variants"
        );
    }
}

#[test]
fn renderer_preserves_authored_architecture_decision_sections_verbatim() {
    let rendered =
        render_architecture_artifact("architecture-decisions.md", DECISION_BRIEF, "", "");
    assert!(rendered.starts_with("# Architecture Decisions"));
    assert!(rendered.contains("## Decision"));
    assert!(rendered.contains("## Constraints"));
    assert!(rendered.contains("## Decision Drivers"));
    assert!(rendered.contains("## Recommendation"));
    assert!(rendered.contains("winning decision explicitly"));
    assert!(!rendered.contains("# Architecture Brief"));
    assert!(!rendered.contains("## Options Considered"));
    assert!(!rendered.contains(MISSING_AUTHORED_BODY_MARKER));
}

#[test]
fn renderer_emits_missing_body_marker_for_omitted_decision_sections() {
    let rendered =
        render_architecture_artifact("tradeoff-matrix.md", MISSING_DECISION_SECTION_BRIEF, "", "");
    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Why Not The Others`"));
}
