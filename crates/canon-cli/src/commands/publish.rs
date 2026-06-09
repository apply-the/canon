use canon_engine::EngineService;

use crate::app::PublishCommand;
use crate::error::CliResult;

pub fn execute(service: &EngineService, command: PublishCommand) -> CliResult<i32> {
    let summary = if let Some(ref profile_name) = command.profile {
        let profile: canon_engine::domain::publish_profile::PublishProfile =
            profile_name.parse().map_err(|e: String| canon_engine::EngineError::Validation(e))?;
        service.publish_with_profile(&command.run_id, profile, command.to)?
    } else {
        service.publish(&command.run_id, command.to, command.adr)?
    };

    println!("Published run {}", summary.run_id);
    println!("Mode: {}", summary.mode);
    println!("Destination: {}", summary.published_to);
    if !summary.published_files.is_empty() {
        println!("Files:");
        for path in summary.published_files {
            println!("- {path}");
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use canon_engine::domain::artifact::{
        ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
    };
    use tempfile::tempdir;
    use time::{Date, Month};

    use super::execute;
    use crate::app::PublishCommand;
    use canon_engine::domain::mode::Mode;
    use canon_engine::domain::policy::{RiskClass, UsageZone};
    use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
    use canon_engine::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
    use canon_engine::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};
    use canon_engine::{EngineService, RunRequest};

    fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
        format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
    }

    fn requirements_request(risk: RiskClass, zone: UsageZone) -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk,
            zone,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    #[allow(dead_code)]
    fn complete_requirements_brief() -> &'static str {
        "# Requirements Brief\n\n## Problem\n\nPublish completed requirements artifacts.\n\n## Outcome\n\nArtifacts are persisted and publishable.\n\n## Constraints\n\n- Keep output local-first.\n\n## Non-Negotiables\n\n- Artifacts must persist under .canon/.\n\n## Options\n\n1. Publish to default path.\n\n## Recommended Path\n\nPublish to the default mode directory.\n\n## Tradeoffs\n\n- Simpler path at cost of flexibility.\n\n## Consequences\n\n- Reviewers can inspect the packet.\n\n## Out of Scope\n\n- No hosted publishing.\n\n## Deferred Work\n\n- Remote destinations deferred.\n\n## Decision Checklist\n\n- [x] Scope is explicit.\n\n## Open Questions\n\n- None at this time.\n"
    }

    #[allow(dead_code)]
    fn architecture_request() -> RunRequest {
        RunRequest {
            mode: Mode::Architecture,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["architecture.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn manifest_for(mode: Mode, run_id: &str) -> RunManifest {
        let created_at = Date::from_calendar_date(2026, Month::April, 22)
            .expect("valid fixture date")
            .with_hms(0, 0, 0)
            .expect("valid fixture timestamp")
            .assume_utc();

        RunManifest {
            run_id: run_id.to_string(),
            uuid: None,
            short_id: None,
            slug: None,
            title: None,
            mode,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: if matches!(mode, Mode::Architecture) {
                Some(SystemContext::Existing)
            } else {
                None
            },
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            lineage: None,
            created_at,
        }
    }

    fn sample_context(
        repo_root: &Path,
        manifest: &RunManifest,
    ) -> canon_engine::domain::run::RunContext {
        canon_engine::domain::run::RunContext {
            repo_root: repo_root.display().to_string(),
            workspace_identity: canon_engine::domain::run::WorkspaceIdentity::same_root(
                repo_root.display().to_string(),
            ),
            owner: Some(manifest.owner.clone()),
            inputs: vec!["canon-input/publish.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: manifest.system_context,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: manifest.created_at,
        }
    }

    fn artifact_contract_for_files(files: &[&str]) -> ArtifactContract {
        ArtifactContract {
            version: 1,
            artifact_requirements: files
                .iter()
                .map(|file_name| ArtifactRequirement {
                    file_name: (*file_name).to_string(),
                    format: ArtifactFormat::Markdown,
                    required_sections: vec!["Summary".to_string()],
                    gates: Vec::new(),
                    required: true,
                })
                .collect(),
            required_verification_layers: Vec::new(),
        }
    }

    fn persisted_markdown_artifact(
        run_id: &str,
        mode: Mode,
        file_name: &str,
        contents: &str,
    ) -> PersistedArtifact {
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: file_name.to_string(),
                relative_path: format!("artifacts/{run_id}/{}/{file_name}", mode.as_str()),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: contents.to_string(),
        }
    }

    fn persist_publish_fixture(
        repo_root: &Path,
        manifest: &RunManifest,
        artifact_contract: ArtifactContract,
        artifacts: Vec<PersistedArtifact>,
    ) {
        let store = WorkspaceStore::new(repo_root);
        let bundle = PersistedRunBundle {
            run: manifest.clone(),
            context: sample_context(repo_root, manifest),
            state: RunStateManifest {
                state: canon_engine::domain::run::RunState::Completed,
                updated_at: manifest.created_at,
            },
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

        store.persist_run_bundle(&bundle).expect("persist publish fixture");
    }

    fn persist_completed_requirements_run(workspace: &Path) -> String {
        let run_id = "R-20260422-publishreq01".to_string();
        let manifest = manifest_for(Mode::Requirements, &run_id);
        persist_publish_fixture(
            workspace,
            &manifest,
            artifact_contract_for_files(&[
                "01-problem-statement.md",
                "02-constraints.md",
                "07-prd.md",
            ]),
            vec![
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "01-problem-statement.md",
                    "# Problem Statement\n\n## Summary\n\nPublish completed requirements artifacts.\n",
                ),
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "02-constraints.md",
                    "# Constraints\n\n## Summary\n\n- Keep output local-first.\n",
                ),
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "07-prd.md",
                    "# PRD\n\n## Summary\n\nArtifacts are persisted and publishable.\n",
                ),
            ],
        );
        run_id
    }

    fn persist_completed_architecture_run(workspace: &Path) -> String {
        let run_id = "R-20260422-publisharch01".to_string();
        let manifest = manifest_for(Mode::Architecture, &run_id);
        persist_publish_fixture(
            workspace,
            &manifest,
            artifact_contract_for_files(&[
                "architecture-overview.md",
                "architecture-decisions.md",
                "tradeoff-matrix.md",
            ]),
            vec![
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "architecture-overview.md",
                    "# Architecture Overview\n\n## Summary\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\n",
                ),
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "architecture-decisions.md",
                    "# Architecture Decisions\n\n## Decision\n\nUse a dedicated context map to make architecture boundaries reviewable.\n",
                ),
                persisted_markdown_artifact(
                    &run_id,
                    manifest.mode,
                    "tradeoff-matrix.md",
                    "# Tradeoff Matrix\n\n## Options Considered\n\n- Keep domain boundaries implicit.\n\n## Pros\n\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n\n- The authored brief must carry more explicit decision content.\n",
                ),
            ],
        );
        run_id
    }

    #[allow(dead_code)]
    fn architecture_brief() -> &'static str {
        "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Consequences\n- Architecture reviewers can inspect a durable ADR without reopening the run history.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Deployment\n- `canon-cli` runs on developer laptops and CI runners.\n- `canon-engine` shares the same Rust process boundary as the CLI.\n- `.canon/` remains the local runtime store on the active workspace filesystem.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
    }

    #[test]
    fn execute_publishes_completed_run_to_default_mode_directory() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let run_id = persist_completed_requirements_run(workspace.path());

        let code = execute(
            &service,
            PublishCommand { run_id: run_id.clone(), to: None, adr: false, profile: None },
        )
        .expect("publish should succeed");
        let published_dir =
            workspace.path().join("specs").join(default_publish_leaf(&run_id, "requirements"));

        assert_eq!(code, 0);
        assert!(published_dir.join("01-problem-statement.md").exists());
        assert!(published_dir.join("07-prd.md").exists());
        assert!(published_dir.join("02-constraints.md").exists());
        assert!(published_dir.join("packet-metadata.json").exists());
    }

    #[test]
    fn execute_rejects_publish_when_run_is_still_awaiting_approval() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("idea.md"),
            "# Idea\n\nSystemic requirements framing needs approval.\n",
        )
        .expect("write idea file");

        let service = EngineService::new(workspace.path());
        let run = service
            .run(requirements_request(RiskClass::SystemicImpact, UsageZone::Yellow))
            .expect("requirements run should start");

        let error = execute(
            &service,
            PublishCommand { run_id: run.run_id, to: None, adr: false, profile: None },
        )
        .expect_err("publish should fail for approval-gated run");

        assert!(error.to_string().contains("cannot publish run"));
    }

    #[test]
    fn execute_publishes_to_explicit_override_path() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let run_id = persist_completed_requirements_run(workspace.path());

        let override_path = PathBuf::from("tech-docs/public/prd");
        let code = execute(
            &service,
            PublishCommand {
                run_id: run_id.clone(),
                to: Some(override_path.clone()),
                adr: false,
                profile: None,
            },
        )
        .expect("publish should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join(&override_path).join("01-problem-statement.md").exists());
        assert!(workspace.path().join(&override_path).join("07-prd.md").exists());
        assert!(!workspace.path().join("specs").join(run_id).exists());
        assert!(workspace.path().join(&override_path).join("packet-metadata.json").exists());
    }

    #[test]
    fn execute_publishes_architecture_adr_by_default() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let run_id = persist_completed_architecture_run(workspace.path());

        let code =
            execute(&service, PublishCommand { run_id, to: None, adr: false, profile: None })
                .expect("publish should succeed");

        let adr_dir = workspace.path().join("tech-docs").join("adr");
        let adr_name = fs::read_dir(&adr_dir)
            .expect("adr dir")
            .next()
            .expect("adr entry")
            .expect("adr dir entry")
            .file_name()
            .to_string_lossy()
            .to_string();

        assert_eq!(code, 0);
        assert!(adr_name.starts_with("ADR-0001-"));
    }

    #[test]
    fn execute_publishes_project_memory_profile_to_canonical_surface() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let run_id = persist_completed_requirements_run(workspace.path());

        let code = execute(
            &service,
            PublishCommand {
                run_id,
                to: None,
                adr: false,
                profile: Some("project-memory".to_string()),
            },
        )
        .expect("profile publish should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join("tech-docs/project/product-context.md").exists());
        assert!(
            workspace
                .path()
                .join("tech-docs/project/product-context.packet-metadata.json")
                .exists()
        );
    }
}
