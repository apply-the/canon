use std::path::{Path, PathBuf};

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

const TARGETED_MODES: [Mode; 5] =
    [Mode::Requirements, Mode::Discovery, Mode::SystemShaping, Mode::Architecture, Mode::Change];
const REPRESENTATIVE_NON_TARGETED_MODES: [Mode; 6] = [
    Mode::Review,
    Mode::Verification,
    Mode::Implementation,
    Mode::Refactor,
    Mode::Incident,
    Mode::Migration,
];

fn refinement_fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/clarify-run-refinement")
}

fn refinement_request(mode: Mode, owner: &str) -> RunRequest {
    RunRequest {
        mode,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: Vec::new(),
        inline_inputs: vec![refinement_brief(mode).to_string()],
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn refinement_brief(mode: Mode) -> &'static str {
    match mode {
        Mode::Requirements => {
            "# Requirements Brief\n\n## Problem\nClarify how Canon should refine an existing governed work item.\n\n## Desired Outcome\n- preserve run identity\n- keep source inputs immutable\n"
        }
        Mode::Discovery => {
            "# Discovery Brief\n\n## Problem\nDetermine whether follow-up context belongs to fresh work or continuation.\n\n## Unknowns\n- continuation intent\n- ambiguity handling\n"
        }
        Mode::SystemShaping => {
            "# System Shaping Brief\n\n## Opportunity\nModel refinement as one durable draft identity.\n\n## Constraints\n- no new persistence family\n"
        }
        Mode::Architecture => {
            "# Architecture Brief\n\n## Decision\nPersist refinement state on the existing run context.\n\n## Constraints\n- keep inspect clarity separate\n"
        }
        Mode::Change => {
            "# Change Brief\n\n## System Slice\nRun identity and refinement state.\n\n## Intended Change\nAdd explicit continuation gating.\n"
        }
        Mode::Review => {
            "# Review Brief\n\n## Review Scope\nReview same-work refinement semantics.\n"
        }
        Mode::Verification => {
            "# Verification Brief\n\n## Claims Under Test\n- continuation remains explicit\n"
        }
        Mode::Implementation => {
            "# Implementation Brief\n\n## Task Mapping\n1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\n\n## Bounded Changes\n- Auth session repository helper wiring.\n- Revocation service internal composition.\n\n## Mutation Bounds\nsrc/auth/session.rs; src/auth/repository.rs\n\n## Allowed Paths\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Safety-Net Evidence\nContract coverage protects revocation formatting and audit ordering before mutation.\n\n## Independent Checks\ncargo test --test session_contract\n\n## Rollback Triggers\nRevocation output drifts or audit ordering becomes unstable.\n\n## Rollback Steps\nRevert the bounded auth-session patch and redeploy the previous build.\n"
        }
        Mode::Refactor => {
            "# Refactor Brief\n\n## Preserved Behavior\n- existing runs remain inspectable\n"
        }
        Mode::Incident => {
            "# Incident Brief\n\n## Incident Summary\nFresh work attached to the wrong governed run.\n"
        }
        Mode::Migration => {
            "# Migration Brief\n\n## Migration Goal\nCarry refinement context through successor lineage.\n"
        }
        other => panic!("unsupported refinement fixture mode: {other:?}"),
    }
}

fn expected_workflow_family(mode: Mode) -> &'static str {
    match mode {
        Mode::Requirements
        | Mode::Discovery
        | Mode::SystemShaping
        | Mode::Architecture
        | Mode::Change => "planning",
        Mode::Implementation | Mode::Refactor | Mode::Migration => "execution",
        Mode::Review | Mode::Verification | Mode::Incident => "assessment",
        other => panic!("unsupported refinement workflow family mode: {other:?}"),
    }
}

#[test]
fn refinement_fixture_root_exists_for_integration_flows() {
    assert!(refinement_fixture_root().exists());
}

#[test]
fn targeted_modes_share_the_same_draft_refinement_lifecycle() {
    for mode in TARGETED_MODES {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        let summary = service.run(refinement_request(mode, "planner")).unwrap_or_else(|error| {
            panic!("targeted refinement run should succeed for {mode:?}: {error}")
        });

        assert_eq!(summary.mode, mode.as_str());
        assert_eq!(summary.state, "Draft");

        let refinement = summary
            .refinement_state
            .unwrap_or_else(|| panic!("targeted mode {:?} should expose refinement state", mode));
        assert_eq!(refinement.workflow_family, "planning");
        assert_eq!(refinement.current_mode, mode.as_str());
        assert!(refinement.explicit_continuation_required);
        assert!(
            refinement
                .working_brief_path
                .ends_with(&format!("/artifacts/{}/working-brief.md", mode.as_str()))
        );
    }
}

#[test]
fn representative_non_targeted_modes_preserve_advisory_continuity_in_status() {
    for mode in REPRESENTATIVE_NON_TARGETED_MODES {
        let workspace = TempDir::new().expect("temp dir");
        let service = EngineService::new(workspace.path());

        let first = service.run(refinement_request(mode, "operator")).unwrap_or_else(|error| {
            panic!("first non-targeted run should succeed for {mode:?}: {error}")
        });
        let second = service.run(refinement_request(mode, "operator")).unwrap_or_else(|error| {
            panic!("second non-targeted run should succeed for {mode:?}: {error}")
        });

        assert_ne!(
            first.run_id, second.run_id,
            "fresh non-targeted work should not silently reuse the prior run identity for {mode:?}"
        );

        let refinement = second.refinement_state.as_ref().unwrap_or_else(|| {
            panic!("non-targeted mode {:?} should surface refinement state", mode)
        });
        assert_eq!(refinement.current_mode, mode.as_str());
        assert_eq!(refinement.workflow_family, expected_workflow_family(mode));

        let candidate = refinement.suggested_candidate.as_ref().unwrap_or_else(|| {
            panic!("non-targeted mode {:?} should surface an advisory continuation candidate", mode)
        });
        assert_eq!(candidate.run_id, first.run_id);
        assert_eq!(candidate.mode, mode.as_str());
        assert!(candidate.advisory);

        let status = service
            .status(&second.run_id)
            .unwrap_or_else(|error| panic!("status should succeed for {mode:?}: {error}"));
        let status_refinement = status.refinement_state.unwrap_or_else(|| {
            panic!("status should expose refinement state for non-targeted mode {:?}", mode)
        });
        assert_eq!(status_refinement.current_mode, mode.as_str());
        assert_eq!(status_refinement.workflow_family, expected_workflow_family(mode));
        assert_eq!(
            status_refinement
                .suggested_candidate
                .unwrap_or_else(|| {
                    panic!(
                        "status should preserve the advisory continuation candidate for {:?}",
                        mode
                    )
                })
                .run_id,
            first.run_id
        );
    }
}
