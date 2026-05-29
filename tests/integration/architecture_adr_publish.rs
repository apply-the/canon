use std::fs;

use assert_cmd::Command;
use canon_engine::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, RunContext, RunState, SystemContext};
use canon_engine::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use canon_engine::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};
use tempfile::TempDir;
use time::{Date, Month};

fn cli_command() -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--quiet",
        "--manifest-path",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
        "-p",
        "canon-cli",
        "--bin",
        "canon",
        "--",
    ]);
    command
}

fn persist_completed_architecture_run(workspace: &TempDir) -> String {
    let run_id = "R-20260529-architectureadr01".to_string();
    let created_at = Date::from_calendar_date(2026, Month::May, 29)
        .expect("valid fixture date")
        .with_hms(0, 0, 0)
        .expect("valid fixture timestamp")
        .assume_utc();
    let manifest = RunManifest {
        run_id: run_id.clone(),
        uuid: None,
        short_id: None,
        slug: None,
        title: None,
        mode: Mode::Architecture,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "staff-architect".to_string(),
        lineage: None,
        created_at,
    };
    let artifact_contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![
            ArtifactRequirement {
                file_name: "architecture-overview.md".to_string(),
                format: ArtifactFormat::Markdown,
                required_sections: vec!["Summary".to_string()],
                gates: Vec::new(),
                required: true,
            },
            ArtifactRequirement {
                file_name: "architecture-decisions.md".to_string(),
                format: ArtifactFormat::Markdown,
                required_sections: vec!["Decision".to_string()],
                gates: Vec::new(),
                required: true,
            },
            ArtifactRequirement {
                file_name: "tradeoff-matrix.md".to_string(),
                format: ArtifactFormat::Markdown,
                required_sections: vec!["Options Considered".to_string()],
                gates: Vec::new(),
                required: true,
            },
        ],
        required_verification_layers: Vec::new(),
    };
    let artifacts = vec![
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "architecture-overview.md".to_string(),
                    relative_path: format!("artifacts/{run_id}/architecture/architecture-overview.md"),
                    format: ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Architecture Overview\n\n## Summary\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "architecture-decisions.md".to_string(),
                    relative_path: format!("artifacts/{run_id}/architecture/architecture-decisions.md"),
                    format: ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Architecture Decisions\n\n## Decision\n\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Constraints\n\n- Preserve run identity and approval behavior.\n\n## Decision Drivers\n\n- Reviewers need the chosen direction and rationale without consulting chat history.\n\n## Recommendation\n\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Consequences\n\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n".to_string(),
            },
            PersistedArtifact {
                record: ArtifactRecord {
                    file_name: "tradeoff-matrix.md".to_string(),
                    relative_path: format!("artifacts/{run_id}/architecture/tradeoff-matrix.md"),
                    format: ArtifactFormat::Markdown,
                    provenance: None,
                },
                contents: "# Tradeoff Matrix\n\n## Options Considered\n\n- Keep the current generic decision summary.\n\n## Evaluation Criteria\n\n- Ownership clarity\n- Seam visibility\n\n## Pros\n\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n\n- The authored brief must carry more explicit decision content.\n\n## Why Not The Others\n\n- A new artifact family would widen scope beyond this slice.\n".to_string(),
            },
        ];
    let bundle = PersistedRunBundle {
        run: manifest.clone(),
        context: RunContext {
            repo_root: workspace.path().display().to_string(),
            owner: Some(manifest.owner.clone()),
            inputs: vec!["architecture.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: manifest.system_context,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: created_at,
        },
        state: RunStateManifest { state: RunState::Completed, updated_at: created_at },
        artifact_contract,
        artifacts,
        links: LinkManifest {
            artifacts: Vec::new(),
            decisions: Vec::new(),
            traces: Vec::new(),
            invocations: Vec::new(),
            evidence: None,
        },
        gates: Vec::new(),
        approvals: Vec::new(),
        verification_records: Vec::new(),
        evidence: None,
        invocations: Vec::new(),
    };

    WorkspaceStore::new(workspace.path())
        .persist_run_bundle(&bundle)
        .expect("persist completed architecture run");

    run_id
}

#[test]
fn architecture_publish_emits_a_standard_adr_by_default() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = persist_completed_architecture_run(&workspace);

    let publish_output = cli_command()
        .current_dir(workspace.path())
        .args(["publish", &run_id])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let publish_text = String::from_utf8(publish_output).expect("utf8 publish output");
    let adr_dir = workspace.path().join("docs").join("adr");
    let adr_entry = fs::read_dir(&adr_dir)
        .expect("adr registry dir")
        .next()
        .expect("adr entry")
        .expect("adr dir entry");
    let adr_name = adr_entry.file_name().to_string_lossy().to_string();
    let adr_path = adr_entry.path();
    let adr_text = fs::read_to_string(&adr_path).expect("generated adr");

    assert!(adr_name.starts_with("ADR-0001-"));
    assert!(publish_text.contains(&format!("docs/adr/{adr_name}")));
    assert!(adr_text.starts_with(
        "# ADR 0001: Use a dedicated context map to make architecture boundaries reviewable."
    ));
    assert!(adr_text.contains("**Date:** "));
    assert!(adr_text.contains("**Status:** Accepted"));
    assert!(adr_text.contains("## Context"));
    assert!(adr_text.contains(
        "Decision focus: map boundaries and tradeoffs for governed analysis-mode expansion."
    ));
    assert!(adr_text.contains("## Decision"));
    assert!(
        adr_text
            .contains("Use a dedicated context map to make architecture boundaries reviewable.")
    );
    assert!(adr_text.contains("## Consequences"));
    assert!(adr_text.contains(
        "The emitted packet records the chosen option and rejected alternatives explicitly."
    ));
    assert!(adr_text.contains("## Alternatives Considered"));
    assert!(adr_text.contains("Keep the current generic decision summary."));
}
