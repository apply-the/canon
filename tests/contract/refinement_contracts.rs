use std::path::{Path, PathBuf};

use canon_engine::{
    artifacts::render_refinement_working_brief,
    domain::{
        mode::Mode,
        run::{
            ClarificationAnswerKind, ClarificationRecord, ClarificationRefinementContext,
            ClarificationRefinementStatus, ClarificationResolutionState,
            ContinuationCandidateSummary, ReadinessDeltaItem, ReadinessDeltaSourceKind,
            RefinementWorkflowFamily, RunState,
        },
    },
};
use time::OffsetDateTime;

fn refinement_fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/clarify-run-refinement")
}

fn feature_contract_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("specs")
        .join("062-clarify-run-refinement")
        .join("contracts")
}

fn template_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("defaults").join("templates").join("canon-input")
}

const REQUIREMENTS_TEMPLATE_HEADINGS: &[&str] =
    &["## Problem", "## Constraints", "## Options", "## Recommended Path", "## Open Questions"];
const DISCOVERY_TEMPLATE_HEADINGS: &[&str] = &[
    "## Problem Domain",
    "## Unknowns",
    "## Constraints",
    "## Recommended Direction",
    "## Open Questions",
];
const SYSTEM_SHAPING_TEMPLATE_HEADINGS: &[&str] =
    &["Intent:", "Constraint:", "## System Shape", "## Structural Options", "## Unresolved Risks"];
const ARCHITECTURE_TEMPLATE_HEADINGS: &[&str] = &[
    "## Decision",
    "## Constraints",
    "## Options Considered",
    "## Recommendation",
    "## Unresolved Questions",
];
const CHANGE_TEMPLATE_HEADINGS: &[&str] = &[
    "## System Slice",
    "## Intended Change",
    "## Validation Strategy",
    "## Recommendation",
    "## Unresolved Questions",
];

fn sample_refinement_context(run_id: &str) -> ClarificationRefinementContext {
    ClarificationRefinementContext {
        workflow_family: RefinementWorkflowFamily::Planning,
        current_mode: Mode::Requirements,
        working_brief_path: format!(".canon/runs/{run_id}/artifacts/requirements/working-brief.md"),
        template_ref: "defaults/templates/canon-input/requirements.md".to_string(),
        status: ClarificationRefinementStatus::Active,
        explicit_continuation_required: true,
        authoritative_input_refs: vec!["canon-input/requirements/brief.md".to_string()],
        supporting_input_refs: vec!["canon-input/requirements/context-links.md".to_string()],
        suggested_candidate: Some(ContinuationCandidateSummary {
            run_id: "R-20260529-prev0001".to_string(),
            mode: Mode::Requirements,
            state: RunState::Draft,
            match_reason: "same authoritative input fingerprint".to_string(),
            advisory: true,
        }),
        records: vec![
            ClarificationRecord {
                id: "cq-001".to_string(),
                prompt: "Which actor owns the problem?".to_string(),
                answer: "platform operators".to_string(),
                answer_kind: ClarificationAnswerKind::Explicit,
                affected_sections: vec!["Actors".to_string(), "Problem Statement".to_string()],
                resolution_state: ClarificationResolutionState::Resolved,
                recorded_at: OffsetDateTime::UNIX_EPOCH,
            },
            ClarificationRecord {
                id: "cq-004".to_string(),
                prompt: "Who validates the release walkthrough?".to_string(),
                answer: "Validation owner defaulted to repository maintainer review".to_string(),
                answer_kind: ClarificationAnswerKind::Defaulted,
                affected_sections: vec!["Validation Strategy".to_string()],
                resolution_state: ClarificationResolutionState::Resolved,
                recorded_at: OffsetDateTime::UNIX_EPOCH,
            },
            ClarificationRecord {
                id: "cq-005".to_string(),
                prompt: "Which downstream team owns rollout sign-off?".to_string(),
                answer: "Still awaiting owner confirmation".to_string(),
                answer_kind: ClarificationAnswerKind::Deferred,
                affected_sections: vec!["Rollout Plan".to_string()],
                resolution_state: ClarificationResolutionState::Deferred,
                recorded_at: OffsetDateTime::UNIX_EPOCH,
            },
        ],
        readiness_delta: vec![ReadinessDeltaItem {
            id: "rd-001".to_string(),
            section: "Validation Strategy".to_string(),
            summary: "Independent validation owner is not yet named.".to_string(),
            blocking: true,
            source_kind: ReadinessDeltaSourceKind::MissingContext,
            default_available: false,
            resolved: false,
        }],
    }
}

#[test]
fn refinement_fixture_root_exists_for_contract_flows() {
    assert!(refinement_fixture_root().exists());
}

#[test]
fn working_brief_contract_requires_standard_refinement_appendix_sections() {
    let refinement = sample_refinement_context("R-20260529-ab12cd34");
    let rendered = render_refinement_working_brief(
        "# Requirements Brief\n\n## Problem\nClarify same-work continuation.\n",
        &refinement,
    );

    assert!(refinement.working_brief_path.ends_with("/artifacts/requirements/working-brief.md"));
    assert!(rendered.contains("## Clarification Provenance"));
    assert!(rendered.contains("### Applied Answers"));
    assert!(rendered.contains("### Applied Defaults"));
    assert!(rendered.contains("## Source Snapshots"));
    assert!(rendered.contains("## Unresolved Questions"));
    assert!(rendered.contains("## Readiness Delta"));
    assert!(rendered.contains("## Continuation State"));
}

#[test]
fn status_and_inspect_refinement_contract_requires_additive_fields_and_headings() {
    let contract_path = feature_contract_root().join("status-and-inspect-refinement-contract.md");
    let contract = std::fs::read_to_string(contract_path).expect("status and inspect contract");
    let normalized_contract = contract.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "\"refinement_state\"",
        "\"suggested_continuation\"",
        "\"working_brief_path\"",
        "\"clarification_records\"",
        "\"readiness_delta\"",
        "\"mutation_allowed\"",
        "## Refinement State",
        "## Working Brief",
        "## Clarification Records",
        "## Readiness Delta",
        "## Continuation Guidance",
        "## Lineage",
    ] {
        assert!(
            contract.contains(required),
            "status and inspect refinement contract is missing `{required}`"
        );
    }

    assert!(
        normalized_contract
            .contains("candidate detection is advisory and continuation requires explicit intent"),
        "status and inspect refinement contract is missing the approved continuation wording"
    );
}

#[test]
fn targeted_templates_preserve_required_refinement_seed_headings() {
    for (template_name, required_headings) in [
        ("requirements.md", REQUIREMENTS_TEMPLATE_HEADINGS),
        ("discovery.md", DISCOVERY_TEMPLATE_HEADINGS),
        ("system-shaping.md", SYSTEM_SHAPING_TEMPLATE_HEADINGS),
        ("architecture.md", ARCHITECTURE_TEMPLATE_HEADINGS),
        ("change.md", CHANGE_TEMPLATE_HEADINGS),
    ] {
        let contents = std::fs::read_to_string(template_root().join(template_name))
            .unwrap_or_else(|error| panic!("load targeted template `{template_name}`: {error}"));

        for heading in required_headings {
            assert!(
                contents.contains(heading),
                "targeted template `{template_name}` is missing required heading `{heading}`"
            );
        }
    }
}
