use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_system_assessment_artifact,
};

const FULL_BRIEF: &str = r#"# System Assessment Brief

## Assessment Objective

- establish an as-is architecture packet before planning the next bounded change

## Stakeholders

- principal architect
- platform maintainer

## Primary Concerns

- current component ownership
- deployment boundaries
- integration choke points

## Assessment Posture

- read-only and evidence-first

## Stakeholder Concerns

- boundary clarity for downstream architecture work
- confidence in deployment assumptions

## Assessed Views

- functional
- component
- deployment
- technology
- integration

## Partial Or Skipped Coverage

- production queue topology remains partially assessed

## Confidence By Surface

- component view: high
- deployment view: medium

## Assessed Assets

- api service
- worker service
- postgres datastore

## Critical Dependencies

- stripe webhook delivery
- redis job queue

## Boundary Notes

- worker crosses the queue boundary to reach the datastore

## Ownership Signals

- platform team owns the deployment manifests

## Responsibilities

- api validates inbound requests and schedules jobs

## Primary Flows

- inbound checkout events enter the api and fan out to workers

## Observed Boundaries

- queue and datastore boundaries are explicit in repository config

## Components

- api handlers
- billing worker
- persistence gateway

## Interfaces

- http handlers call the persistence gateway through the billing worker

## Execution Environments

- containers on a shared kubernetes cluster

## Network And Runtime Boundaries

- ingress to api
- api to redis
- worker to postgres

## Deployment Signals

- helm values declare api and worker replicas separately

## Coverage Gaps

- background cron topology is not represented in the repository

## Technology Stack

- rust services
- postgres
- redis

## Platform Dependencies

- kubernetes
- github actions

## Version Or Lifecycle Signals

- redis image pinning is present but postgres lifecycle policy is not explicit

## Evidence Gaps

- no live runtime manifests confirm the cluster namespace layout

## Integrations

- stripe webhooks
- email provider callback

## Data Exchanges

- checkout events enter via http and persist through the billing worker

## Trust And Failure Boundaries

- inbound webhook trust ends at the api boundary

## Inference Notes

- the repository suggests one worker pool, but runtime shard count is inferred

## Observed Risks

- queue retry policy could amplify billing reprocessing

## Risk Triggers

- retry configuration is partly declared and partly inferred

## Impact Notes

- duplicate billing work could reach downstream reconciliation

## Likely Follow-On Modes

- architecture
- change

## Observed Findings

- the repository contains separate api and worker deployment inputs

## Inferred Findings

- one queue-backed worker pool appears to own async billing execution

## Assessment Gaps

- no repository evidence confirms production queue partitioning

## Evidence Sources

- src/api
- deploy/helm
- infra/queue
"#;

const MISSING_TRUST_BOUNDARY_BRIEF: &str = r#"# System Assessment Brief

## Assessment Objective

- capture current integration behavior only

## Integrations

- stripe webhooks

## Data Exchanges

- inbound webhook events arrive over http
"#;

const NEAR_MISS_BRIEF: &str = r#"# System Assessment Brief

## Integrations

- stripe webhooks

## Data Exchanges

- inbound webhook events arrive over http

## Trust Boundaries

This near-miss heading should not satisfy the canonical `Trust And Failure Boundaries` section.
"#;

#[test]
fn system_assessment_renderer_preserves_authored_sections_verbatim() {
    let overview = render_system_assessment_artifact("assessment-overview.md", FULL_BRIEF);
    let coverage = render_system_assessment_artifact("coverage-map.md", FULL_BRIEF);
    let component = render_system_assessment_artifact("component-view.md", FULL_BRIEF);
    let evidence = render_system_assessment_artifact("assessment-evidence.md", FULL_BRIEF);

    assert!(overview.contains(
        "## Assessment Objective\n\n- establish an as-is architecture packet before planning the next bounded change"
    ));
    assert!(overview.contains("## Stakeholders\n\n- principal architect"));
    assert!(!overview.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(coverage.contains(
        "## Partial Or Skipped Coverage\n\n- production queue topology remains partially assessed"
    ));
    assert!(component.contains(
        "## Interfaces\n\n- http handlers call the persistence gateway through the billing worker"
    ));
    assert!(evidence.contains(
        "## Assessment Gaps\n\n- no repository evidence confirms production queue partitioning"
    ));
}

#[test]
fn system_assessment_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered =
        render_system_assessment_artifact("integration-view.md", MISSING_TRUST_BOUNDARY_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Trust And Failure Boundaries`"));
}

#[test]
fn system_assessment_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_system_assessment_artifact("integration-view.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Trust And Failure Boundaries`"));
    assert!(!rendered.contains(
        "## Trust Boundaries\n\nThis near-miss heading should not satisfy the canonical `Trust And Failure Boundaries` section."
    ));
}
