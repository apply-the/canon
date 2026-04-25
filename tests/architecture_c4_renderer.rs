use canon_engine::artifacts::markdown::{
    C4_MISSING_AUTHORED_BODY_MARKER, render_architecture_artifact,
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
fn renderer_does_not_alter_legacy_architecture_artifact_shape() {
    let rendered = render_architecture_artifact(
        "architecture-decisions.md",
        "context summary text",
        "generation summary text",
        "critique summary text",
    );
    assert!(rendered.starts_with("# Architecture Decisions"));
    assert!(rendered.contains("## Decisions"));
    assert!(rendered.contains("## Tradeoffs"));
    assert!(rendered.contains("context summary text"));
    assert!(rendered.contains("generation summary text"));
    assert!(rendered.contains("critique summary text"));
}
