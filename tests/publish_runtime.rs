use std::fs;
use std::path::Path;

use canon_engine::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::publish_profile::PublishProfile;
use canon_engine::domain::run::{ClassificationProvenance, RunContext, RunState, SystemContext};
use canon_engine::orchestrator::publish::{publish_run, publish_run_with_profile};
use canon_engine::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use canon_engine::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};
use canon_engine::{EngineService, RunRequest};
use tempfile::tempdir;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

fn complete_requirements_brief() -> &'static str {
    "# Requirements Brief\n\n## Problem\n\nPublish engine unit test coverage.\n\n## Outcome\n\nPublish functions are exercised under full artifact contracts.\n\n## Constraints\n\n- Keep output local-first.\n\n## Non-Negotiables\n\n- Artifacts must persist under .canon/.\n\n## Options\n\n1. Publish to default path.\n\n## Recommended Path\n\nPublish to the default mode directory.\n\n## Tradeoffs\n\n- Simpler path at cost of flexibility.\n\n## Consequences\n\n- Reviewers can inspect the packet.\n\n## Out of Scope\n\n- No hosted publishing.\n\n## Deferred Work\n\n- Remote destinations deferred.\n\n## Decision Checklist\n\n- [x] Scope is explicit.\n\n## Open Questions\n\n- None at this time.\n"
}

#[allow(dead_code)]
fn requirements_request() -> RunRequest {
    RunRequest {
        mode: Mode::Requirements,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
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

#[allow(dead_code)]
fn architecture_brief() -> &'static str {
    "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Consequences\n- Architecture reviewers can inspect a durable ADR without reopening the run history.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Deployment\n- `canon-cli` runs on developer laptops and CI runners.\n- `canon-engine` shares the same Rust process boundary as the CLI.\n- `.canon/` remains the local runtime store on the active workspace filesystem.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
}

fn review_request() -> RunRequest {
    RunRequest {
        mode: Mode::Review,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "Reviewer <r@example.com>".to_string(),
        inputs: vec!["canon-input/review.md".to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_review_brief() -> &'static str {
    "# Review Brief\n\n## Review Target\n- bounded service boundary package.\n\n## Evidence Basis\n- owned interfaces, current tests, and rollback notes.\n\n## Boundary Findings\n- no boundary expansion beyond the authored review target was detected.\n\n## Ownership Notes\n- reviewer remains accountable for downstream acceptance.\n\n## Missing Evidence\nStatus: evidence-bounded\n- No critical evidence gaps remain from the authored review packet.\n\n## Collection Priorities\n- preserve the current evidence bundle for later inspection.\n\n## Decision Impact\n- downstream implementation remains reversible within the bounded package.\n\n## Reversibility Concerns\n- stop before broader rollout if the packet changes materially.\n\n## Final Disposition\nStatus: ready-with-review-notes\nRationale: the review packet is bounded enough for downstream inspection.\n\n## Accepted Risks\n- residual review notes remain bounded to this package.\n"
}

fn incident_request() -> RunRequest {
    RunRequest {
        mode: Mode::Incident,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "IC <ic@example.com>".to_string(),
        inputs: vec!["canon-input/incident.md".to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_incident_brief() -> &'static str {
    "# Incident Brief\n\n## Incident Scope\npayments-api and checkout flow only.\n\n## Trigger And Current State\nelevated 5xx responses after the last deploy.\n\n## Operational Constraints\n- no autonomous remediation\n- no schema changes\n\n## Known Facts\n- errors started after the deploy\n- rollback remains available\n\n## Working Hypotheses\n- retry amplification is exhausting the service\n\n## Evidence Gaps\n- database saturation is not yet confirmed\n\n## Impacted Surfaces\n- payments-api\n- checkout flow\n\n## Propagation Paths\n- checkout request path\n\n## Confidence And Unknowns\n- medium confidence until saturation evidence is collected\n\n## Immediate Actions\n- disable async retries\n\n## Ordered Sequence\n1. capture blast radius\n2. disable retries\n3. reassess error rate\n\n## Stop Conditions\n- error rate stabilizes below the alert threshold\n\n## Decision Points\n- decide whether rollback is still required\n\n## Approved Actions\n- disable retries within the bounded surface\n\n## Deferred Actions\n- schema-level changes remain out of scope\n\n## Verification Checks\n- confirm 5xx rate drops\n\n## Release Readiness\n- keep recommendation-only posture until the owner accepts the packet\n\n## Follow-Up Work\n- add a saturation dashboard and post-incident review item\n"
}

fn sample_manifest(run_id: &str) -> RunManifest {
    RunManifest {
        run_id: run_id.to_string(),
        uuid: Some("12345678123456781234567812345678".to_string()),
        short_id: Some("abcd1234".to_string()),
        slug: Some("publish-scope".to_string()),
        title: Some("Publish Scope".to_string()),
        mode: Mode::Requirements,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "Owner <owner@example.com>".to_string(),
        lineage: None,
        created_at: OffsetDateTime::parse("2026-04-22T08:00:00Z", &Rfc3339)
            .expect("parse timestamp"),
    }
}

fn persist_run_manifest_and_state(repo_root: &Path, manifest: &RunManifest, state: RunState) {
    let run_dir = repo_root.join(".canon").join("runs").join(&manifest.run_id);
    fs::create_dir_all(&run_dir).expect("create run dir");
    fs::write(run_dir.join("run.toml"), toml::to_string(manifest).expect("serialize manifest"))
        .expect("write manifest");

    let state = RunStateManifest { state, updated_at: manifest.created_at };
    fs::write(run_dir.join("state.toml"), toml::to_string(&state).expect("serialize state"))
        .expect("write state");
}

fn persist_completed_requirements_run(workspace_root: &Path) -> String {
    let run_id = "R-20260529-publishrequirements01".to_string();
    let created_at =
        OffsetDateTime::parse("2026-05-29T00:00:00Z", &Rfc3339).expect("parse fixture timestamp");
    let manifest = RunManifest {
        run_id: run_id.clone(),
        uuid: Some("12345678123456781234567812345678".to_string()),
        short_id: Some("abcd1234".to_string()),
        slug: Some("publish-scope".to_string()),
        title: Some("Publish Scope".to_string()),
        mode: Mode::Requirements,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "Owner <owner@example.com>".to_string(),
        lineage: None,
        created_at,
    };
    let artifact_contract = ArtifactContract {
        version: 1,
        artifact_requirements: vec![ArtifactRequirement {
            file_name: "01-problem-statement.md".to_string(),
            format: ArtifactFormat::Markdown,
            required_sections: vec!["Problem".to_string(), "Outcome".to_string()],
            gates: Vec::new(),
            required: true,
        }],
        required_verification_layers: Vec::new(),
    };
    let artifacts = vec![PersistedArtifact {
        record: ArtifactRecord {
            file_name: "01-problem-statement.md".to_string(),
            relative_path: format!("artifacts/{run_id}/requirements/01-problem-statement.md"),
            format: ArtifactFormat::Markdown,
            provenance: None,
        },
        contents: "# Problem Statement\n\n## Problem\n\nPublish engine unit test coverage.\n\n## Outcome\n\nPublish functions are exercised under full artifact contracts.\n".to_string(),
    }];
    let bundle = PersistedRunBundle {
        run: manifest.clone(),
        context: RunContext {
            repo_root: workspace_root.display().to_string(),
            owner: Some(manifest.owner.clone()),
            inputs: vec!["idea.md".to_string()],
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

    WorkspaceStore::new(workspace_root)
        .persist_run_bundle(&bundle)
        .expect("persist completed requirements run");

    run_id
}

fn persist_completed_architecture_run(workspace_root: &Path) -> String {
    let run_id = "R-20260529-publisharchitecture01".to_string();
    let created_at =
        OffsetDateTime::parse("2026-05-29T00:00:00Z", &Rfc3339).expect("parse fixture timestamp");
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
            contents: "# Architecture Decisions\n\n## Decision\n\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Recommendation\n\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Consequences\n\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n".to_string(),
        },
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: "tradeoff-matrix.md".to_string(),
                relative_path: format!("artifacts/{run_id}/architecture/tradeoff-matrix.md"),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Tradeoff Matrix\n\n## Options Considered\n\n- Keep the current generic decision summary.\n\n## Why Not The Others\n\n- A new artifact family would widen scope beyond this slice.\n".to_string(),
        },
    ];
    let bundle = PersistedRunBundle {
        run: manifest.clone(),
        context: RunContext {
            repo_root: workspace_root.display().to_string(),
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

    WorkspaceStore::new(workspace_root)
        .persist_run_bundle(&bundle)
        .expect("persist completed architecture run");

    run_id
}

#[test]
fn publish_run_rejects_destination_that_is_an_existing_file() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let run_id = persist_completed_requirements_run(workspace.path());
    let destination_file = workspace.path().join("published.txt");
    fs::write(&destination_file, "not a directory").expect("write file destination");

    let error = publish_run(workspace.path(), &run_id, Some(&destination_file), false)
        .expect_err("publish should reject file destination");

    assert!(error.to_string().contains("must be a directory"));
}

#[test]
fn publish_run_supports_absolute_override_outside_repo_root() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let external = tempdir().expect("external destination");
    let absolute_destination = external.path().join("published-packet");

    let run_id = persist_completed_requirements_run(workspace.path());

    let summary = publish_run(workspace.path(), &run_id, Some(&absolute_destination), false)
        .expect("publish should support absolute override");

    assert_eq!(summary.published_to, absolute_destination.display().to_string());
    assert!(absolute_destination.join("01-problem-statement.md").exists());
    assert!(summary.published_files.iter().any(|path| {
        path == &absolute_destination.join("01-problem-statement.md").display().to_string()
    }));
    assert!(absolute_destination.join("packet-metadata.json").exists());
}

#[test]
fn publish_run_writes_metadata_sidecar_for_default_destinations() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let run_id = persist_completed_requirements_run(workspace.path());

    let summary =
        publish_run(workspace.path(), &run_id, None, false).expect("publish should succeed");
    let metadata_path = workspace.path().join(&summary.published_to).join("packet-metadata.json");
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read metadata sidecar"))
            .expect("metadata json");

    assert_eq!(metadata["run_id"], run_id);
    assert_eq!(metadata["mode"], "requirements");
    assert_eq!(metadata["risk"], "low-impact");
    assert_eq!(metadata["zone"], "green");
    assert_eq!(metadata["primary_artifact"], "01-problem-statement.md");
    assert_eq!(metadata["artifact_order"][0], "01-problem-statement.md");
    assert!(metadata["destination"].as_str().is_some_and(|value| value.starts_with("specs/")));
    assert!(
        metadata["source_artifacts"].as_array().expect("source artifacts array").iter().any(
            |value| value.as_str().is_some_and(|path| path.ends_with("01-problem-statement.md"))
        )
    );
    assert!(summary.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));
}

#[test]
fn publish_run_generates_and_reports_architecture_adr_in_process() {
    let workspace = tempdir().expect("temp workspace");
    let run_id = persist_completed_architecture_run(workspace.path());

    let summary =
        publish_run(workspace.path(), &run_id, None, false).expect("publish should succeed");

    assert!(summary.published_files.iter().any(|path| path.starts_with("docs/adr/ADR-0001-")));
    assert!(workspace.path().join("docs").join("adr").exists());
}

#[test]
fn publish_run_rejects_incomplete_non_operational_run() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("run-awaiting-publish");
    persist_run_manifest_and_state(workspace.path(), &manifest, RunState::AwaitingApproval);

    let error = publish_run(workspace.path(), &manifest.run_id, None, false)
        .expect_err("incomplete requirements publish should fail");

    assert!(error.to_string().contains("approval and resume must complete first"));
}

#[test]
fn publish_run_reports_missing_artifact_contract() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("run-missing-contract-default");
    persist_run_manifest_and_state(workspace.path(), &manifest, RunState::Completed);

    let error = publish_run(workspace.path(), &manifest.run_id, None, false)
        .expect_err("publish should fail without artifact contract");

    assert!(error.to_string().contains("no publishable artifact contract"));
}

#[test]
fn publish_run_with_profile_promotes_completed_requirements() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let run_id = persist_completed_requirements_run(workspace.path());

    let summary =
        publish_run_with_profile(workspace.path(), &run_id, PublishProfile::ProjectMemory, None)
            .expect("profile publish should succeed");

    assert_eq!(summary.published_to, "docs/project/product-context.md");
    assert!(summary.published_files.iter().any(|p| p == "docs/project/product-context.md"));
    assert!(
        summary.published_files.iter().any(|p| p.ends_with("product-context.packet-metadata.json"))
    );

    let metadata_path = workspace.path().join("docs/project/product-context.packet-metadata.json");
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read metadata"))
            .expect("parse metadata");
    assert_eq!(metadata["profile"], "project-memory");
    assert_eq!(metadata["promotion_state"], "auto");
    assert_eq!(metadata["update_strategy"], "managed-blocks");
    assert_eq!(metadata["publication_target_class"], "stable");
    assert_eq!(metadata["artifact_indexing"]["artifact_class"], "managed-surface");
    assert_eq!(metadata["artifact_indexing"]["metadata_carrier"], "managed-surface-envelope");
    assert_eq!(
        metadata["artifact_indexing"]["discovery_rule"],
        "Read the project-memory managed-block start marker for producer attribution and use the adjacent <surface>.packet-metadata.json sidecar for the full promoted lineage envelope."
    );
    assert_eq!(metadata["primary_artifact"], "01-problem-statement.md");
    assert_eq!(metadata["artifact_order"][0], "01-problem-statement.md");
    assert!(metadata["expertise_input"].is_null());
    assert_eq!(metadata["lineage"]["contract_version"], "v1");
    assert_eq!(metadata["lineage"]["mode"], "requirements");
    assert_eq!(metadata["lineage"]["packet_readiness"], "complete");
}

#[test]
fn publish_run_with_profile_uses_append_only_index_for_review() {
    let workspace = tempdir().expect("temp workspace");
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("create canon-input");
    fs::write(input_dir.join("review.md"), complete_review_brief()).expect("write review input");

    let service = EngineService::new(workspace.path());
    let run = service.run(review_request()).expect("review run");

    let summary = publish_run_with_profile(
        workspace.path(),
        &run.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect("profile publish for review");

    assert_eq!(summary.published_to, "docs/project/audit-log.md");
    assert!(summary.published_files.iter().any(|p| p == "docs/project/audit-log.md"));
    assert!(summary.published_files.iter().any(|p| p.ends_with("audit-log.packet-metadata.json")));
    assert!(summary.published_files.iter().any(|p| p.starts_with("docs/evidence/review/")));

    let metadata_path = workspace.path().join("docs/project/audit-log.packet-metadata.json");
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read")).expect("parse");
    assert_eq!(metadata["promotion_state"], "evidence-only");
    assert_eq!(metadata["update_strategy"], "append-only-index");
    assert_eq!(metadata["publication_target_class"], "evidence");
    assert_eq!(metadata["artifact_indexing"]["artifact_class"], "evidence-bundle");
    assert_eq!(metadata["artifact_indexing"]["metadata_carrier"], "packet-metadata-sidecar");
    assert_eq!(metadata["primary_artifact"], "01-review-brief.md");
}

#[test]
fn publish_run_with_profile_uses_proposal_files_for_incident() {
    let workspace = tempdir().expect("temp workspace");
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("create canon-input");
    fs::write(input_dir.join("incident.md"), complete_incident_brief())
        .expect("write incident input");

    let service = EngineService::new(workspace.path());
    let run = service.run(incident_request()).expect("incident run");

    let summary = publish_run_with_profile(
        workspace.path(),
        &run.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect("profile publish for incident");

    assert_eq!(summary.published_to, "docs/project/open-risks.proposal.md");
    assert!(summary.published_files.iter().any(|p| p == "docs/project/open-risks.proposal.md"));
    assert!(
        summary
            .published_files
            .iter()
            .any(|p| p.ends_with("open-risks.proposal.packet-metadata.json"))
    );

    let metadata_path =
        workspace.path().join("docs/project/open-risks.proposal.packet-metadata.json");
    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read")).expect("parse");
    assert_eq!(metadata["promotion_state"], "pending-index");
    assert_eq!(metadata["update_strategy"], "proposal-files");
    assert_eq!(metadata["publication_target_class"], "proposal");
    assert_eq!(metadata["artifact_indexing"]["artifact_class"], "proposal-artifact");
    assert_eq!(metadata["artifact_indexing"]["metadata_carrier"], "packet-metadata-sidecar");
    assert_eq!(metadata["primary_artifact"], "01-incident-frame.md");
}

#[test]
fn publish_run_with_profile_reports_missing_artifact_contract() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("run-missing-contract-profile");
    persist_run_manifest_and_state(workspace.path(), &manifest, RunState::Completed);

    let error = publish_run_with_profile(
        workspace.path(),
        &manifest.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect_err("profile publish should fail without artifact contract");

    assert!(error.to_string().contains("no publishable artifact contract"));
}

#[test]
fn publish_run_with_profile_rejects_manual_promotion() {
    let workspace = tempdir().expect("temp workspace");

    let mut manifest = sample_manifest("manual-run");
    manifest.mode = Mode::Incident;
    manifest.risk = RiskClass::BoundedImpact;
    manifest.zone = UsageZone::Yellow;
    manifest.system_context = Some(SystemContext::Existing);
    persist_run_manifest_and_state(workspace.path(), &manifest, RunState::AwaitingApproval);

    let error = publish_run_with_profile(
        workspace.path(),
        "manual-run",
        PublishProfile::ProjectMemory,
        None,
    )
    .expect_err("manual promotion should be rejected");

    assert!(error.to_string().contains("requires manual promotion"));
}

#[test]
fn publish_run_with_profile_rejects_override_file_destination() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let run_id = persist_completed_requirements_run(workspace.path());
    let override_file = workspace.path().join("custom-file.txt");
    fs::write(&override_file, "not a directory").expect("write override file");

    let error = publish_run_with_profile(
        workspace.path(),
        &run_id,
        PublishProfile::ProjectMemory,
        Some(Path::new("custom-file.txt")),
    )
    .expect_err("file override should be rejected");

    assert!(error.to_string().contains("must be a directory"));
}

#[test]
fn publish_run_with_profile_respects_destination_override() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
        .expect("write input");

    let run_id = persist_completed_requirements_run(workspace.path());

    let summary = publish_run_with_profile(
        workspace.path(),
        &run_id,
        PublishProfile::ProjectMemory,
        Some(Path::new("custom/dest")),
    )
    .expect("profile publish with override");

    assert_eq!(summary.published_to, "custom/dest");
    assert!(summary.published_files.iter().any(|p| p == "custom/dest/packet-metadata.json"));

    let metadata_path = workspace.path().join("custom/dest/packet-metadata.json");
    assert!(metadata_path.exists());

    let written_files = fs::read_dir(workspace.path().join("custom/dest"))
        .expect("read custom dest")
        .map(|entry| entry.expect("dir entry").file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    assert!(written_files.iter().any(|name| name.ends_with(".md")));

    let metadata: serde_json::Value =
        serde_json::from_slice(&fs::read(&metadata_path).expect("read metadata"))
            .expect("parse metadata");
    assert_eq!(metadata["destination"], "custom/dest");
    assert_eq!(metadata["update_strategy"], "managed-blocks");
    assert_eq!(metadata["artifact_indexing"]["artifact_class"], "managed-surface");
}

#[test]
fn publish_run_with_profile_review_override_writes_index_and_evidence_bundle() {
    let workspace = tempdir().expect("temp workspace");
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("create canon-input");
    fs::write(input_dir.join("review.md"), complete_review_brief()).expect("write review input");

    let service = EngineService::new(workspace.path());
    let run = service.run(review_request()).expect("review run");

    let summary = publish_run_with_profile(
        workspace.path(),
        &run.run_id,
        PublishProfile::ProjectMemory,
        Some(Path::new("custom/review")),
    )
    .expect("profile publish for review override");

    assert_eq!(summary.published_to, "custom/review");
    assert!(summary.published_files.iter().any(|p| p == "custom/review/index.md"));
    assert!(summary.published_files.iter().any(|p| p == "custom/review/packet-metadata.json"));
    assert!(summary.published_files.iter().any(|p| {
        p.starts_with("custom/review/")
            && p != "custom/review/index.md"
            && p != "custom/review/packet-metadata.json"
    }));

    let index_path = workspace.path().join("custom/review/index.md");
    let index_contents = fs::read_to_string(&index_path).expect("read index");
    assert!(index_contents.contains(&run.run_id));

    let evidence_root = workspace.path().join("custom/review").join(&run.run_id);
    let evidence_entries = fs::read_dir(&evidence_root).expect("read evidence root").count();
    assert!(evidence_entries > 0);
}

#[test]
fn publish_run_with_profile_incident_override_writes_proposal_files() {
    let workspace = tempdir().expect("temp workspace");
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("create canon-input");
    fs::write(input_dir.join("incident.md"), complete_incident_brief())
        .expect("write incident input");

    let service = EngineService::new(workspace.path());
    let run = service.run(incident_request()).expect("incident run");

    let summary = publish_run_with_profile(
        workspace.path(),
        &run.run_id,
        PublishProfile::ProjectMemory,
        Some(Path::new("custom/incident")),
    )
    .expect("profile publish for incident override");

    assert_eq!(summary.published_to, "custom/incident");
    assert!(summary.published_files.iter().any(|p| p == "custom/incident/packet-metadata.json"));

    let proposal_files = fs::read_dir(workspace.path().join("custom/incident"))
        .expect("read incident dir")
        .map(|entry| entry.expect("dir entry").file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".proposal.md"))
        .collect::<Vec<_>>();
    assert!(!proposal_files.is_empty());
}
