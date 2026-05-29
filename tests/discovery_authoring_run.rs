use std::fs;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn discovery_request(input: &str) -> RunRequest {
    RunRequest {
        mode: Mode::Discovery,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "researcher".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_discovery_brief() -> &'static str {
    r#"# Discovery Brief

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
"#
}

#[test]
fn discovery_run_starts_draft_refinement_and_persists_working_brief() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("discovery.md"), complete_discovery_brief())
        .expect("discovery brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(discovery_request("discovery.md")).expect("discovery run");

    assert_eq!(summary.state, "Draft");

    let refinement =
        summary.refinement_state.expect("discovery run should expose refinement state");
    let working_brief = fs::read_to_string(workspace.path().join(&refinement.working_brief_path))
        .expect("working brief");

    assert_eq!(refinement.workflow_family, "planning");
    assert_eq!(refinement.current_mode, "discovery");
    assert!(refinement.explicit_continuation_required);
    assert!(working_brief.contains("# Discovery Brief"));
    assert!(
        working_brief
            .contains("## Repo Surface\n\n- crates/canon-engine/src/orchestrator/service/")
    );
    assert!(working_brief.contains("## Clarification Provenance"));
    assert!(working_brief.contains("## Readiness Delta"));
}

#[test]
fn discovery_run_blocks_follow_up_when_required_heading_is_absent() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\n## Problem Domain\n\nClarify governed runtime depth for analysis modes.\n\n## Immediate Tensions\n\n- Discovery authoring should stay explicit and reviewable.\n\n## Downstream Handoff\n\nTranslate this packet into requirements mode with concrete scope cuts.\n",
    )
    .expect("discovery brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(discovery_request("discovery.md")).expect("discovery run");

    assert_eq!(summary.state, "Draft");

    let refinement =
        summary.refinement_state.expect("discovery run should expose refinement state");
    let working_brief = fs::read_to_string(workspace.path().join(&refinement.working_brief_path))
        .expect("working brief");
    assert!(working_brief.contains("# Discovery Brief"));
    assert!(working_brief.contains("## Clarification Provenance"));

    let resumed = service.resume(&summary.run_id).expect("resume discovery run");
    assert_eq!(resumed.state, "Blocked");
    assert_eq!(resumed.blocking_classification.as_deref(), Some("artifact-blocked"));
}
