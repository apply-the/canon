use std::fs;
use std::path::Path;

use tempfile::tempdir;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use super::{PublishMetadata, default_publish_directory, resolve_destination};
use crate::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement, RuntimePacketMetadata,
};
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::publish_profile::{
    ExpertiseInputMetadata, PROJECT_MEMORY_PACKET_METADATA_FILE_NAME, PromotionState,
    PublishProfile, SEMANTIC_ARTIFACT_CONTRACT_LINE_V1, SemanticArtifactDescriptor,
    SemanticEligibilityState, SemanticProvenanceBoundary, UpdateStrategy,
};
use crate::domain::run::{
    ClassificationProvenance, RunContext, RunState, SystemContext, WorkspaceIdentity,
};
use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use crate::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};

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

fn manifest_for(mode: Mode, run_id: &str) -> RunManifest {
    let mut manifest = sample_manifest(run_id);
    manifest.mode = mode;
    manifest.system_context = Some(SystemContext::Existing);
    manifest.title = None;
    manifest.slug = None;
    manifest
}

fn markdown_artifact(
    run_id: &str,
    mode: Mode,
    file_name: &str,
    contents: &str,
) -> PersistedArtifact {
    PersistedArtifact {
        record: ArtifactRecord {
            file_name: file_name.to_string(),
            relative_path: format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str()),
            format: ArtifactFormat::Markdown,
            provenance: None,
        },
        contents: contents.to_string(),
    }
}

fn json_artifact(run_id: &str, mode: Mode, file_name: &str, contents: &str) -> PersistedArtifact {
    PersistedArtifact {
        record: ArtifactRecord {
            file_name: file_name.to_string(),
            relative_path: format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str()),
            format: ArtifactFormat::Json,
            provenance: None,
        },
        contents: contents.to_string(),
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

fn artifact_contract_for_files(files: &[(&str, bool)]) -> ArtifactContract {
    ArtifactContract {
        version: 1,
        artifact_requirements: files
            .iter()
            .map(|(file_name, required)| ArtifactRequirement {
                file_name: (*file_name).to_string(),
                format: ArtifactFormat::Markdown,
                required_sections: vec!["Summary".to_string()],
                gates: Vec::new(),
                required: *required,
            })
            .collect(),
        required_verification_layers: Vec::new(),
    }
}

fn sample_context(repo_root: &Path, manifest: &RunManifest) -> RunContext {
    RunContext {
        repo_root: repo_root.display().to_string(),
        workspace_identity: WorkspaceIdentity::same_root(repo_root.display().to_string()),
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

fn persist_publish_fixture(
    repo_root: &Path,
    manifest: &RunManifest,
    state: RunState,
    artifact_contract: ArtifactContract,
    artifacts: Vec<PersistedArtifact>,
) {
    let store = WorkspaceStore::new(repo_root);
    let bundle = PersistedRunBundle {
        run: manifest.clone(),
        context: sample_context(repo_root, manifest),
        state: RunStateManifest { state, updated_at: manifest.created_at },
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

    store.persist_run_bundle(&bundle).expect("persist publish bundle");
}

#[test]
fn render_project_memory_surface_returns_trimmed_single_artifact() {
    let artifact = markdown_artifact(
        "R-test",
        Mode::Requirements,
        "product-context.md",
        "\n# Product Context\n\nBounded summary.\n\n",
    );

    let rendered = super::render_project_memory_surface(&[artifact]);

    assert_eq!(rendered, "# Product Context\n\nBounded summary.");
    assert!(!rendered.contains("## Product Context"));
}

#[test]
fn infer_runtime_packet_metadata_ignores_sidecars_and_maps_legacy_aliases() {
    let artifacts = vec![
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "01-problem-statement.md",
            "# Problem Statement\n\nBounded summary.",
        ),
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "02-scope-cuts.md",
            "# Scope Cuts\n\nBounded summary.",
        ),
        json_artifact("R-test", Mode::Requirements, "view-manifest.json", "{\"views\":[]}"),
    ];

    let metadata = super::infer_runtime_packet_metadata(&artifacts);

    assert_eq!(metadata.primary_artifact, "01-problem-statement.md");
    assert_eq!(
        metadata.artifact_order,
        vec!["01-problem-statement.md".to_string(), "02-scope-cuts.md".to_string(),]
    );
    assert_eq!(
        metadata.legacy_aliases.as_ref().and_then(|aliases| aliases.get("problem-statement.md")),
        Some(&"01-problem-statement.md".to_string())
    );
}

#[test]
fn runtime_packet_metadata_falls_back_to_inferred_order_when_sidecar_is_unparseable() {
    let artifacts = vec![
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "01-problem-statement.md",
            "# Problem Statement\n\nBounded summary.",
        ),
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "02-scope-cuts.md",
            "# Scope Cuts\n\nBounded summary.",
        ),
        json_artifact(
            "R-test",
            Mode::Requirements,
            PROJECT_MEMORY_PACKET_METADATA_FILE_NAME,
            "{not-json}",
        ),
    ];

    let metadata = super::runtime_packet_metadata(&artifacts);

    assert_eq!(metadata.primary_artifact, "01-problem-statement.md");
    assert_eq!(metadata.artifact_order.len(), 2);
    assert!(metadata.legacy_aliases.is_some());
}

#[test]
fn resolve_expertise_input_metadata_prefers_normalized_packet_metadata() {
    let packet_metadata = RuntimePacketMetadata {
        expertise_input: Some(ExpertiseInputMetadata {
            expertise_kind: Mode::DomainLanguage.governed_expertise_kind().expect("expertise kind"),
            domain_families: vec![
                " react ".to_string(),
                "systems".to_string(),
                "react".to_string(),
                " ".to_string(),
            ],
        }),
        ..RuntimePacketMetadata::default()
    };

    let metadata = super::resolve_expertise_input_metadata(
        Path::new("/does/not/matter"),
        Mode::DomainLanguage,
        &packet_metadata,
    )
    .expect("normalized expertise metadata");

    assert_eq!(metadata.domain_families, vec!["react".to_string(), "systems".to_string()]);
}

#[test]
fn resolve_expertise_input_metadata_infers_package_and_repo_language_families() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(
        workspace.path().join("package.json"),
        r#"{
            "dependencies": {
                "react": "18.0.0",
                "vue": "3.0.0",
                "@angular/core": "18.0.0",
                "express": "5.0.0"
            }
        }"#,
    )
    .expect("write package json");
    fs::write(workspace.path().join("Cargo.toml"), "[package]\nname = \"fixture\"\n")
        .expect("write cargo toml");
    fs::write(workspace.path().join("pyproject.toml"), "[project]\nname = \"fixture\"\n")
        .expect("write pyproject");
    fs::write(
        workspace.path().join("build.gradle.kts"),
        "plugins { kotlin(\"jvm\") version \"1.9.0\" }\n",
    )
    .expect("write gradle");
    fs::write(workspace.path().join("Gemfile"), "source \"https://rubygems.org\"\n")
        .expect("write gemfile");
    fs::write(workspace.path().join("composer.json"), "{}\n").expect("write composer");
    fs::create_dir_all(workspace.path().join("clients/dotnet")).expect("create dotnet dir");
    fs::write(
        workspace.path().join("clients/dotnet/Fixture.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>\n",
    )
    .expect("write csproj");

    let metadata = super::resolve_expertise_input_metadata(
        workspace.path(),
        Mode::DomainLanguage,
        &RuntimePacketMetadata::default(),
    )
    .expect("inferred expertise metadata");

    assert_eq!(
        metadata.domain_families,
        vec![
            "angular".to_string(),
            "dotnet_service".to_string(),
            "jvm_service".to_string(),
            "node_service".to_string(),
            "php".to_string(),
            "python_service".to_string(),
            "react".to_string(),
            "ruby".to_string(),
            "systems".to_string(),
            "vue".to_string(),
            "web_ui".to_string(),
        ]
    );
}

#[test]
fn resolve_expertise_input_metadata_falls_back_to_node_service_from_repo_extensions() {
    let workspace = tempdir().expect("temp workspace");
    fs::write(workspace.path().join("package.json"), "{\"name\":\"fixture\"}\n")
        .expect("write package json");
    fs::create_dir_all(workspace.path().join("src/client")).expect("create source dir");
    fs::create_dir_all(workspace.path().join("node_modules/ignored"))
        .expect("create node_modules dir");
    fs::create_dir_all(workspace.path().join(".git/hooks")).expect("create dot git dir");
    fs::write(workspace.path().join("src/client/app.tsx"), "export const app = 1;\n")
        .expect("write tsx file");
    fs::write(workspace.path().join("node_modules/ignored/index.js"), "module.exports = {};\n")
        .expect("write ignored js file");
    fs::write(workspace.path().join(".git/hooks/pre-commit"), "echo ignored\n")
        .expect("write ignored git file");

    let metadata = super::resolve_expertise_input_metadata(
        workspace.path(),
        Mode::DomainLanguage,
        &RuntimePacketMetadata::default(),
    )
    .expect("fallback expertise metadata");

    assert_eq!(metadata.domain_families, vec!["node_service".to_string()]);
}

#[test]
fn resolve_expertise_input_metadata_returns_none_for_missing_repo_root() {
    let workspace = tempdir().expect("temp workspace");
    let missing_root = workspace.path().join("missing-root");

    let metadata = super::resolve_expertise_input_metadata(
        &missing_root,
        Mode::DomainLanguage,
        &RuntimePacketMetadata::default(),
    );

    assert!(metadata.is_none());
}

#[test]
fn publish_metadata_round_trips_semantic_descriptor() {
    let metadata = PublishMetadata {
        run_id: "R-test".to_string(),
        mode: "requirements".to_string(),
        risk: "low-impact".to_string(),
        zone: "green".to_string(),
        publish_timestamp: "2026-04-22T08:00:00Z".to_string(),
        descriptor: "publish-scope".to_string(),
        destination: "specs/2026-04-22-publish-scope".to_string(),
        source_artifacts: vec![
            ".canon/artifacts/R-test/requirements/01-problem-statement.md".to_string(),
        ],
        primary_artifact: "01-problem-statement.md".to_string(),
        artifact_order: vec!["01-problem-statement.md".to_string()],
        publish_order: None,
        legacy_aliases: None,
        artifact_indexing: None,
        semantic_descriptor: Some(SemanticArtifactDescriptor {
            semantic_contract_line: SEMANTIC_ARTIFACT_CONTRACT_LINE_V1.to_string(),
            semantic_eligibility: SemanticEligibilityState::Eligible,
            semantic_provenance_boundary: Some(SemanticProvenanceBoundary::ManagedBlock),
            semantic_provenance_ref: Some(
                "tech-docs/project/overview.md#managed-block-1".to_string(),
            ),
            semantic_labels: vec!["project-memory".to_string()],
            semantic_exclusion_reason: None,
        }),
        profile: Some(PublishProfile::ProjectMemory),
        promotion_state: Some(PromotionState::Auto),
        update_strategy: Some(UpdateStrategy::ManagedBlocks),
        publication_target_class: None,
        expertise_input: None,
        lineage: None,
    };

    let round_trip: PublishMetadata =
        serde_json::from_value(serde_json::to_value(&metadata).expect("serialize metadata"))
            .expect("deserialize metadata");

    assert_eq!(round_trip.semantic_descriptor, metadata.semantic_descriptor);
}

#[test]
fn profile_metadata_path_uses_directory_and_file_conventions() {
    assert_eq!(
        super::profile_metadata_path(Path::new("tech-docs/project/custom-dest")),
        Path::new("tech-docs/project/custom-dest/packet-metadata.json")
    );
    assert_eq!(
        super::profile_metadata_path(Path::new("tech-docs/project/open-risks.proposal.md")),
        Path::new("tech-docs/project/open-risks.proposal.packet-metadata.json")
    );
}

#[test]
fn default_publish_directory_maps_supported_modes() {
    assert_eq!(default_publish_directory(Mode::Requirements), "specs");
    assert_eq!(default_publish_directory(Mode::Discovery), "tech-docs/discovery");
    assert_eq!(default_publish_directory(Mode::SystemShaping), "tech-docs/architecture/shaping");
    assert_eq!(default_publish_directory(Mode::Change), "tech-docs/changes");
    assert_eq!(default_publish_directory(Mode::Backlog), "tech-docs/planning");
    assert_eq!(default_publish_directory(Mode::Architecture), "tech-docs/architecture/decisions");
    assert_eq!(default_publish_directory(Mode::Implementation), "tech-docs/implementation");
    assert_eq!(default_publish_directory(Mode::Refactor), "tech-docs/refactors");
    assert_eq!(default_publish_directory(Mode::Verification), "tech-docs/verification");
    assert_eq!(default_publish_directory(Mode::Review), "tech-docs/reviews");
    assert_eq!(default_publish_directory(Mode::PrReview), "tech-docs/reviews/prs");
    assert_eq!(default_publish_directory(Mode::Incident), "tech-docs/incidents");
    assert_eq!(
        default_publish_directory(Mode::SystemAssessment),
        "tech-docs/architecture/assessments"
    );
    assert_eq!(
        default_publish_directory(Mode::SecurityAssessment),
        "tech-docs/security-assessments"
    );
    assert_eq!(default_publish_directory(Mode::Migration), "tech-docs/migrations");
    assert_eq!(default_publish_directory(Mode::SupplyChainAnalysis), "tech-docs/supply-chain");
}

#[test]
fn resolve_destination_uses_structured_default_or_override() {
    let repo_root = Path::new("/repo");
    let manifest = sample_manifest("R-20260422-abcd1234");

    assert_eq!(
        resolve_destination(repo_root, &manifest, None),
        Path::new("/repo/specs/2026-04-22-publish-scope")
    );
    assert_eq!(
        resolve_destination(repo_root, &manifest, Some(Path::new("tech-docs/public/prd"))),
        Path::new("/repo/tech-docs/public/prd")
    );
}

#[test]
fn resolve_destination_suffixes_collisions_for_other_runs() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-abcd1234");
    let collision_path = workspace.path().join("specs").join("2026-04-22-publish-scope");
    fs::create_dir_all(&collision_path).expect("collision dir");
    fs::write(
        collision_path.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME),
        serde_json::to_vec_pretty(&PublishMetadata {
            run_id: "R-20260422-deadbeef".to_string(),
            mode: "requirements".to_string(),
            risk: "low-impact".to_string(),
            zone: "green".to_string(),
            publish_timestamp: "2026-04-22T08:00:00Z".to_string(),
            descriptor: "publish-scope".to_string(),
            destination: "specs/2026-04-22-publish-scope".to_string(),
            source_artifacts: vec![
                ".canon/artifacts/R-20260422-deadbeef/requirements/01-problem-statement.md"
                    .to_string(),
            ],
            primary_artifact: "01-problem-statement.md".to_string(),
            artifact_order: vec!["01-problem-statement.md".to_string()],
            publish_order: None,
            legacy_aliases: None,
            artifact_indexing: None,
            semantic_descriptor: None,
            profile: None,
            promotion_state: None,
            update_strategy: None,
            publication_target_class: None,
            expertise_input: None,
            lineage: None,
        })
        .expect("metadata json"),
    )
    .expect("write collision metadata");

    assert_eq!(
        resolve_destination(workspace.path(), &manifest, None),
        workspace.path().join("specs").join("2026-04-22-publish-scope--abcd1234")
    );
}

#[test]
fn publish_run_allows_operational_packets_awaiting_approval_and_exports_adr() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = manifest_for(Mode::Migration, "R-20260422-migrate3");
    let artifacts = vec![
        persisted_markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "source-target-map.md",
            "# Source-Target Map\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n",
        ),
        persisted_markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "compatibility-matrix.md",
            "# Compatibility Matrix\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n",
        ),
        persisted_markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n",
        ),
    ];
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::AwaitingApproval,
        artifact_contract_for_files(&[
            ("source-target-map.md", true),
            ("compatibility-matrix.md", true),
            ("decision-record.md", true),
        ]),
        artifacts,
    );

    let summary =
        super::publish_run(workspace.path(), workspace.path(), &manifest.run_id, None, true)
            .expect("publish run");

    assert!(summary.published_to.contains("tech-docs/migrations"));
    assert!(summary.published_files.iter().any(|file| file.starts_with("tech-docs/adr/ADR-")));
}

#[test]
fn publish_run_reports_missing_required_persisted_artifacts() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-missing-required");
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::Completed,
        artifact_contract_for_files(&[("01-problem-statement.md", true)]),
        Vec::new(),
    );

    let error =
        super::publish_run(workspace.path(), workspace.path(), &manifest.run_id, None, false)
            .expect_err("missing required artifact should fail");

    assert!(error.to_string().contains("has no publishable artifacts"));
}

#[test]
fn publish_run_rejects_empty_publishable_artifacts() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-empty-publish");
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::Completed,
        artifact_contract_for_files(&[]),
        Vec::new(),
    );

    let error =
        super::publish_run(workspace.path(), workspace.path(), &manifest.run_id, None, false)
            .expect_err("empty publish packet should fail");

    assert!(error.to_string().contains("has no publishable artifacts"));
}

#[test]
fn publish_run_with_profile_reports_missing_required_persisted_artifacts() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-profile-missing-required");
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::Completed,
        artifact_contract_for_files(&[("01-problem-statement.md", true)]),
        Vec::new(),
    );

    let error = super::publish_run_with_profile(
        workspace.path(),
        workspace.path(),
        &manifest.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect_err("missing required artifact should fail");

    assert!(error.to_string().contains("has no publishable artifacts"));
}

#[test]
fn publish_run_with_profile_rejects_empty_publishable_artifacts() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-profile-empty");
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::Completed,
        artifact_contract_for_files(&[]),
        Vec::new(),
    );

    let error = super::publish_run_with_profile(
        workspace.path(),
        workspace.path(),
        &manifest.run_id,
        PublishProfile::ProjectMemory,
        None,
    )
    .expect_err("empty profile publish packet should fail");

    assert!(error.to_string().contains("has no publishable artifacts"));
}

#[test]
fn publish_run_with_profile_writes_metadata_for_directory_override() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-profile-success");
    persist_publish_fixture(
        workspace.path(),
        &manifest,
        RunState::Completed,
        artifact_contract_for_files(&[("01-problem-statement.md", true)]),
        vec![persisted_markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "01-problem-statement.md",
            "# Problem Statement\n\n## Summary\n\nBounded publish packet.\n",
        )],
    );

    let summary = super::publish_run_with_profile(
        workspace.path(),
        workspace.path(),
        &manifest.run_id,
        PublishProfile::ProjectMemory,
        Some(Path::new("tech-docs/project/custom-dest")),
    )
    .expect("profile publish");

    assert!(summary.published_to.contains("tech-docs/project/custom-dest"));
    assert!(summary.published_files.iter().any(|file| file.ends_with("packet-metadata.json")));
}

#[test]
fn adr_export_policy_distinguishes_default_opt_in_and_unsupported_modes() {
    assert!(super::adr_export_enabled(Mode::Architecture, false).expect("architecture policy"));
    assert!(!super::adr_export_enabled(Mode::Change, false).expect("change default policy"));
    assert!(super::adr_export_enabled(Mode::Migration, true).expect("migration opt-in policy"));
    assert!(
        !super::adr_export_enabled(Mode::Requirements, false).expect("requirements default policy")
    );
    assert!(super::adr_export_enabled(Mode::Incident, true).is_err());
}

#[test]
fn build_adr_export_rejects_unsupported_modes_when_called_directly() {
    let workspace = tempdir().expect("temp workspace");
    let manifest = sample_manifest("R-20260422-abcd1234");

    let error = super::build_adr_export(
        workspace.path(),
        &manifest,
        &[],
        Path::new("specs/2026-04-22-publish-scope"),
    )
    .expect_err("unsupported mode should be rejected");

    assert!(error.to_string().contains("ADR export is not supported"));
}

#[test]
fn build_change_adr_maps_change_packet_sections() {
    let manifest = manifest_for(Mode::Change, "R-20260422-change01");
    let artifacts = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "change-surface.md",
            "# Change Surface\n\n## Change Surface\n\n- session repository\n\n## Cross-Context Risks\n\n- cleanup scheduling can leak into notification flows\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "implementation-plan.md",
            "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Options Considered\n\n- Option 1 keeps the additive repository helper inside the auth boundary.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Boundary Tradeoffs\n\n- keep cleanup logic inside the auth boundary\n",
        ),
    ];

    let adr = super::build_change_adr(&manifest, &artifacts, "tech-docs/changes/2026-04-22-change")
        .expect("change adr");

    assert_eq!(
        adr.title,
        "Prefer additive change over normalization to preserve operator expectations."
    );
    assert!(adr.context.contains("### Implementation Plan"));
    assert!(adr.context.contains("### Change Surface"));
    assert!(adr.decision.contains("Prefer additive change over normalization"));
    assert!(adr.consequences.contains("### Boundary Tradeoffs"));
    assert!(
        adr.alternatives
            .as_deref()
            .is_some_and(|value| value.contains("Option 1 keeps the additive repository helper"))
    );
}

#[test]
fn build_change_adr_reports_missing_decision_section() {
    let manifest = manifest_for(Mode::Change, "R-20260422-change-missing");
    let artifacts = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "change-surface.md",
            "# Change Surface\n\n## Change Surface\n\n- session repository\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "implementation-plan.md",
            "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods.\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Consequences\n\n- preserved surface remains explicit\n",
        ),
    ];

    let error = super::build_change_adr(
        &manifest,
        &artifacts,
        "tech-docs/changes/2026-04-22-change-missing",
    )
    .expect_err("missing decision section should fail");

    assert!(error.to_string().contains("missing a decision section"));
}

#[test]
fn build_migration_adr_maps_migration_packet_sections() {
    let manifest = manifest_for(Mode::Migration, "R-20260422-migrate1");
    let artifacts = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "source-target-map.md",
            "# Source-Target Map\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "compatibility-matrix.md",
            "# Compatibility Matrix\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n\n## Ecosystem Health\n\n- auth-v2 dependencies are healthy enough for bounded cutover\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n",
        ),
    ];

    let adr = super::build_migration_adr(
        &manifest,
        &artifacts,
        "tech-docs/migrations/2026-04-22-migration",
    )
    .expect("migration adr");

    assert_eq!(adr.title, "retain dual-write during the bounded cutover");
    assert!(adr.context.contains("### Current State"));
    assert!(adr.context.contains("### Target State"));
    assert!(adr.decision.contains("retain dual-write during the bounded cutover"));
    assert!(adr.consequences.contains("### Tradeoff Analysis"));
    assert!(adr.alternatives.as_deref().is_some_and(|value| {
        value.contains("Option 1 keeps dual-write through the cutover window.")
    }));
}

#[test]
fn build_migration_adr_reports_missing_decision_section() {
    let manifest = manifest_for(Mode::Migration, "R-20260422-migrate-missing");
    let artifacts = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "source-target-map.md",
            "# Source-Target Map\n\n## Current State\n\n- auth-v1\n\n## Target State\n\n- auth-v2\n\n## Transition Boundaries\n\n- login only\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "compatibility-matrix.md",
            "# Compatibility Matrix\n\n## Options Matrix\n\n- dual-write\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Tradeoff Analysis\n\n- bounded dual-write\n",
        ),
    ];

    let error = super::build_migration_adr(
        &manifest,
        &artifacts,
        "tech-docs/migrations/2026-04-22-migration-missing",
    )
    .expect_err("missing migration decision section should fail");

    assert!(error.to_string().contains("missing a decision section"));
}

#[test]
fn build_architecture_adr_reports_missing_required_sections() {
    let manifest = manifest_for(Mode::Architecture, "R-20260422-arch0001");
    let tradeoffs = markdown_artifact(
        &manifest.run_id,
        manifest.mode,
        "tradeoff-matrix.md",
        "# Tradeoff Matrix\n\n## Options Considered\n\n- keep the current generic decision summary\n\n## Pros\n\n- reviewers can reuse the packet outside the originating conversation\n",
    );

    let missing_summary = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "architecture-overview.md",
            "# Architecture Overview\n\n## Included Views\n\n- context map\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "architecture-decisions.md",
            "# Architecture Decisions\n\n## Decision\n\nUse a dedicated context map to make architecture boundaries reviewable.\n",
        ),
        tradeoffs.clone(),
    ];
    let missing_summary_error = super::build_architecture_adr(
        &manifest,
        &missing_summary,
        "tech-docs/architecture/decisions/2026-04-22-architecture",
    )
    .expect_err("missing summary should fail");
    assert!(missing_summary_error.to_string().contains("`## Summary`"));

    let missing_decision = vec![
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "architecture-overview.md",
            "# Architecture Overview\n\n## Summary\n\nDecision focus: map boundaries and tradeoffs.\n",
        ),
        markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "architecture-decisions.md",
            "# Architecture Decisions\n\n## Decision Drivers\n\n- reviewers need the chosen direction\n",
        ),
        tradeoffs,
    ];
    let missing_decision_error = super::build_architecture_adr(
        &manifest,
        &missing_decision,
        "tech-docs/architecture/decisions/2026-04-22-architecture",
    )
    .expect_err("missing decision should fail");
    assert!(missing_decision_error.to_string().contains("`## Decision`"));
}

#[test]
fn helper_functions_cover_section_selection_title_fallback_and_registry_numbering() {
    assert_eq!(
        super::preferred_section(
            "## Recommendation\n\n- use the fallback\n",
            &["Decision", "Recommendation"]
        ),
        Some("- use the fallback".to_string())
    );
    assert!(super::labeled_sections(&[("Only", None)]).is_err());

    let with_title = sample_manifest("R-20260422-title001");
    assert_eq!(super::fallback_adr_title(&with_title), "Publish Scope");

    let mut slug_only = sample_manifest("R-20260422-title002");
    slug_only.title = None;
    slug_only.slug = Some("durable-decision".to_string());
    assert_eq!(super::fallback_adr_title(&slug_only), "Durable Decision");
    assert_eq!(super::titleize_slug(""), "");
    assert_eq!(super::normalize_adr_title_line("- keep dual-write"), "keep dual-write");

    let mut no_short_id = sample_manifest("run-12345678");
    no_short_id.short_id = None;
    assert_eq!(super::short_id_fragment(&no_short_id), "12345678");

    let registry = tempdir().expect("registry tempdir");
    assert_eq!(super::next_adr_number(registry.path()).expect("empty registry"), 1);
    fs::write(registry.path().join("ADR-0002-existing.md"), "# ADR 0002\n").expect("write adr 2");
    fs::write(registry.path().join("notes.txt"), "ignore\n").expect("write other file");
    fs::write(registry.path().join("ADR-0005"), "missing separator\n")
        .expect("write malformed adr without suffix");
    fs::write(registry.path().join("ADR-00x4-bad.md"), "bad digits\n")
        .expect("write malformed adr digits");
    assert_eq!(super::next_adr_number(registry.path()).expect("numbered registry"), 3);
}

#[test]
fn build_adr_export_assigns_numbered_paths_for_supported_opt_in_modes() {
    let workspace = tempdir().expect("temp workspace");
    let registry_root = workspace.path().join("tech-docs").join("adr");
    fs::create_dir_all(&registry_root).expect("registry root");
    fs::write(registry_root.join("ADR-0002-existing.md"), "# ADR 0002\n")
        .expect("write existing adr");

    let change_manifest = manifest_for(Mode::Change, "R-20260422-change02");
    let change_artifacts = vec![
        markdown_artifact(
            &change_manifest.run_id,
            change_manifest.mode,
            "change-surface.md",
            "# Change Surface\n\n## Change Surface\n\n- session repository\n",
        ),
        markdown_artifact(
            &change_manifest.run_id,
            change_manifest.mode,
            "implementation-plan.md",
            "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n",
        ),
        markdown_artifact(
            &change_manifest.run_id,
            change_manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n",
        ),
    ];
    let change_adr = super::build_adr_export(
        workspace.path(),
        &change_manifest,
        &change_artifacts,
        Path::new("tech-docs/changes/2026-04-22-change"),
    )
    .expect("change adr export");
    assert!(change_adr.display_path.starts_with("tech-docs/adr/ADR-0003-"));

    fs::write(&change_adr.destination, &change_adr.contents).expect("persist change adr");

    let migration_manifest = manifest_for(Mode::Migration, "R-20260422-migrate2");
    let migration_artifacts = vec![
        markdown_artifact(
            &migration_manifest.run_id,
            migration_manifest.mode,
            "source-target-map.md",
            "# Source-Target Map\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n",
        ),
        markdown_artifact(
            &migration_manifest.run_id,
            migration_manifest.mode,
            "compatibility-matrix.md",
            "# Compatibility Matrix\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n",
        ),
        markdown_artifact(
            &migration_manifest.run_id,
            migration_manifest.mode,
            "decision-record.md",
            "# Decision Record\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n",
        ),
    ];
    let migration_adr = super::build_adr_export(
        workspace.path(),
        &migration_manifest,
        &migration_artifacts,
        Path::new("tech-docs/migrations/2026-04-22-migration"),
    )
    .expect("migration adr export");
    assert!(migration_adr.display_path.starts_with("tech-docs/adr/ADR-0004-"));
}

#[test]
fn evaluate_promotion_policy_maps_completed_analysis_to_auto() {
    let manifest = sample_manifest("R-test");
    for mode in [
        Mode::SystemShaping,
        Mode::Discovery,
        Mode::Requirements,
        Mode::DomainLanguage,
        Mode::DomainModel,
        Mode::Backlog,
    ] {
        let mut m = manifest.clone();
        m.mode = mode;
        assert_eq!(
            super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
            PromotionState::Auto,
            "expected Auto for completed {mode:?}"
        );
    }
}

#[test]
fn evaluate_promotion_policy_maps_completed_gated_to_auto_if_approved() {
    let manifest = sample_manifest("R-test");
    for mode in
        [Mode::Architecture, Mode::Change, Mode::Implementation, Mode::Refactor, Mode::Migration]
    {
        let mut m = manifest.clone();
        m.mode = mode;
        assert_eq!(
            super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
            PromotionState::AutoIfApproved,
            "expected AutoIfApproved for completed {mode:?}"
        );
    }
}

#[test]
fn evaluate_promotion_policy_maps_non_completed_gated_to_pending() {
    let manifest = sample_manifest("R-test");
    for mode in [Mode::Architecture, Mode::Change] {
        let mut m = manifest.clone();
        m.mode = mode;
        assert_eq!(
            super::evaluate_promotion_policy(mode, &RunState::AwaitingApproval, &m),
            PromotionState::PendingIndex,
        );
    }
}

#[test]
fn evaluate_promotion_policy_maps_evidence_modes_to_evidence_only() {
    let manifest = sample_manifest("R-test");
    for mode in [
        Mode::Verification,
        Mode::Review,
        Mode::PrReview,
        Mode::SecurityAssessment,
        Mode::SupplyChainAnalysis,
        Mode::SystemAssessment,
    ] {
        let mut m = manifest.clone();
        m.mode = mode;
        assert_eq!(
            super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
            PromotionState::EvidenceOnly,
        );
    }
}

#[test]
fn evaluate_promotion_policy_maps_incident_states() {
    let manifest = sample_manifest("R-test");
    let mut m = manifest.clone();
    m.mode = Mode::Incident;
    assert_eq!(
        super::evaluate_promotion_policy(Mode::Incident, &RunState::Completed, &m),
        PromotionState::PendingIndex,
    );
    assert_eq!(
        super::evaluate_promotion_policy(Mode::Incident, &RunState::AwaitingApproval, &m),
        PromotionState::Manual,
    );
}

#[test]
fn default_update_strategy_for_modes() {
    assert_eq!(
        super::default_update_strategy_for(Mode::Architecture),
        UpdateStrategy::ManagedBlocks
    );
    assert_eq!(super::default_update_strategy_for(Mode::Migration), UpdateStrategy::ProposalFiles);
    assert_eq!(super::default_update_strategy_for(Mode::Incident), UpdateStrategy::ProposalFiles);
    assert_eq!(super::default_update_strategy_for(Mode::Review), UpdateStrategy::AppendOnlyIndex);
}

#[test]
fn write_managed_block_inserts_and_replaces() {
    let workspace = tempdir().expect("temp workspace");
    let target = workspace.path().join("doc.md");

    super::write_managed_block(&target, "test-block", "initial content").expect("first write");
    let first = fs::read_to_string(&target).expect("read first");
    assert!(first.contains(
            "<!-- project-memory:managed:start producer=\"canon\" source_ref=\"test-block\" contract_version=\"v1\" -->"
        ));
    assert!(first.contains("initial content"));
    assert!(first.contains("<!-- project-memory:managed:end -->"));

    super::write_managed_block(&target, "test-block", "updated content").expect("second write");
    let second = fs::read_to_string(&target).expect("read second");
    assert!(second.contains("updated content"));
    assert!(!second.contains("initial content"));
}

#[test]
fn write_managed_block_preserves_curated_content() {
    let workspace = tempdir().expect("temp workspace");
    let target = workspace.path().join("curated.md");
    let curated = "# My Document\n\nHuman-authored context.\n\n<!-- project-memory:managed:start producer=\"canon\" source_ref=\"test\" contract_version=\"v1\" -->\nold Canon data\n<!-- project-memory:managed:end -->\n\nMore human content.\n";
    fs::write(&target, curated).expect("write curated");

    super::write_managed_block(&target, "test", "new Canon data").expect("update block");
    let result = fs::read_to_string(&target).expect("read result");
    assert!(result.contains("Human-authored context."));
    assert!(result.contains("new Canon data"));
    assert!(result.contains("More human content."));
    assert!(!result.contains("old Canon data"));
}

#[test]
fn write_managed_block_replaces_legacy_canon_marker() {
    let workspace = tempdir().expect("temp workspace");
    let target = workspace.path().join("legacy.md");
    let legacy = "<!-- canon:managed-block:test:start -->\nold Canon data\n<!-- canon:managed-block:test:end -->\n";
    fs::write(&target, legacy).expect("write legacy");

    super::write_managed_block(&target, "test", "new Canon data").expect("migrate block");
    let result = fs::read_to_string(&target).expect("read migrated");
    assert!(result.contains("project-memory:managed:start"));
    assert!(!result.contains("canon:managed-block"));
    assert!(result.contains("new Canon data"));
}

#[test]
fn write_managed_block_replaces_legacy_marker_without_end_marker() {
    let workspace = tempdir().expect("temp workspace");
    let target = workspace.path().join("legacy-open.md");
    let legacy = "preface\n<!-- canon:managed-block:test:start -->\nold Canon data\n";
    fs::write(&target, legacy).expect("write legacy");

    super::write_managed_block(&target, "test", "new Canon data").expect("migrate block");
    let result = fs::read_to_string(&target).expect("read migrated");
    assert!(result.contains("preface"));
    assert!(result.contains("project-memory:managed:start"));
    assert!(result.contains("<!-- project-memory:managed:end -->"));
    assert!(result.contains("new Canon data"));
}

#[test]
fn write_proposal_file_emits_sidecar() {
    let workspace = tempdir().expect("temp workspace");
    let dest = workspace.path().join("proposals");
    let lineage = super::LineageMetadata {
        contract_version: "v1".into(),
        producer: "canon".into(),
        source_ref: "canon-run:R-test".into(),
        source_artifacts: vec!["artifact.md".into()],
        promotion_state: PromotionState::PendingIndex,
        promoted_at: "2026-05-13T00:00:00Z".into(),
        content_digest: "sha256:abc123".into(),
        mode: Some("architecture".into()),
        stage: None,
        owner: None,
        risk: None,
        zone: None,
        approval_state: Some("AwaitingApproval".into()),
        packet_readiness: Some("partial".into()),
        promotion_profile: Some(PublishProfile::ProjectMemory),
    };

    let path =
        super::write_proposal_file(&dest, "decision-record.md", "# Proposal\nContent", &lineage)
            .expect("write proposal");
    assert!(path.ends_with("decision-record.proposal.md"));
    let content = fs::read_to_string(&path).expect("read proposal");
    assert!(content.contains("Canon proposal from canon-run:R-test"));
    assert!(content.contains("# Proposal\nContent"));
}

#[test]
fn write_proposal_file_uses_unknown_placeholders_for_missing_optional_lineage_fields() {
    let workspace = tempdir().expect("temp workspace");
    let dest = workspace.path().join("proposals");
    let lineage = super::LineageMetadata {
        contract_version: "v1".into(),
        producer: "canon".into(),
        source_ref: "canon-run:R-test".into(),
        source_artifacts: vec!["artifact.md".into()],
        promotion_state: PromotionState::PendingIndex,
        promoted_at: "2026-05-13T00:00:00Z".into(),
        content_digest: "sha256:abc123".into(),
        mode: None,
        stage: None,
        owner: None,
        risk: None,
        zone: None,
        approval_state: None,
        packet_readiness: None,
        promotion_profile: None,
    };

    let path = super::write_proposal_file(&dest, "decision-record.md", "# Proposal", &lineage)
        .expect("write proposal");
    let content = fs::read_to_string(&path).expect("read proposal");
    assert!(content.contains("Canon proposal from canon-run:R-test (unknown-mode)"));
    assert!(content.contains("promotion_state: pending-index | profile: unknown-profile"));
}

#[test]
fn content_digest_for_artifacts_is_stable_for_same_inputs() {
    let artifacts = vec![
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "product-context.md",
            "# Product Context\n\nBounded summary.\n",
        ),
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "delivery-map.md",
            "# Delivery Map\n\nBounded sequence.\n",
        ),
    ];

    let first = super::content_digest_for_artifacts(&artifacts);
    let second = super::content_digest_for_artifacts(&artifacts);

    assert_eq!(first, second);
    assert!(first.starts_with("sha256:"));
}

#[test]
fn content_digest_for_artifacts_ignores_publish_metadata_sidecar() {
    let artifacts = vec![
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            "product-context.md",
            "# Product Context\n\nBounded summary.\n",
        ),
        markdown_artifact(
            "R-test",
            Mode::Requirements,
            PROJECT_MEMORY_PACKET_METADATA_FILE_NAME,
            "{\"ignored\":true}\n",
        ),
    ];
    let without_sidecar = vec![markdown_artifact(
        "R-test",
        Mode::Requirements,
        "product-context.md",
        "# Product Context\n\nBounded summary.\n",
    )];

    assert_eq!(
        super::content_digest_for_artifacts(&artifacts),
        super::content_digest_for_artifacts(&without_sidecar)
    );
}

#[test]
fn append_index_entry_creates_and_appends() {
    let workspace = tempdir().expect("temp workspace");
    let target = workspace.path().join("index.md");

    super::append_index_entry(&target, "- Entry 1\n").expect("first append");
    let first = fs::read_to_string(&target).expect("read first");
    assert!(first.contains("- Entry 1"));

    super::append_index_entry(&target, "- Entry 2\n").expect("second append");
    let second = fs::read_to_string(&target).expect("read second");
    assert!(second.contains("- Entry 1"));
    assert!(second.contains("- Entry 2"));
}

#[test]
fn resolve_profile_destination_routes_by_promotion_state() {
    let repo_root = Path::new("/repo");
    let stable_manifest = sample_manifest("R-20260422-abcd1234");
    let pending_manifest = manifest_for(Mode::Architecture, "R-20260422-bcde2345");
    let evidence_manifest = manifest_for(Mode::Review, "R-20260422-cdef3456");

    let stable =
        super::resolve_profile_destination(repo_root, &stable_manifest, &PromotionState::Auto);
    assert_eq!(stable, Path::new("/repo/tech-docs/project/product-context.md"));

    let pending = super::resolve_profile_destination(
        repo_root,
        &pending_manifest,
        &PromotionState::PendingIndex,
    );
    assert_eq!(pending, Path::new("/repo/tech-docs/project/pending-decisions.md"));

    let evidence = super::resolve_profile_destination(
        repo_root,
        &evidence_manifest,
        &PromotionState::EvidenceOnly,
    );
    assert_eq!(evidence, Path::new("/repo/tech-docs/project/audit-log.md"));
}

#[test]
fn canonical_project_memory_surface_map_covers_all_modes() {
    let expected = [
        (
            Mode::Discovery,
            "tech-docs/project/overview.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Brainstorming,
            "tech-docs/project/overview.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Requirements,
            "tech-docs/project/product-context.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::SystemShaping,
            "tech-docs/project/architecture-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Architecture,
            "tech-docs/project/architecture-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::SystemAssessment,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/open-risks.md",
        ),
        (
            Mode::Change,
            "tech-docs/project/decision-index.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Backlog,
            "tech-docs/project/delivery-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::PrReview,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/audit-log.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Implementation,
            "tech-docs/project/delivery-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Refactor,
            "tech-docs/project/delivery-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Verification,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/audit-log.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Review,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/audit-log.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Incident,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/open-risks.md",
            "tech-docs/project/open-risks.md",
        ),
        (
            Mode::SecurityAssessment,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/open-risks.md",
        ),
        (
            Mode::Migration,
            "tech-docs/project/decision-index.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::SupplyChainAnalysis,
            "tech-docs/project/operational-context.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/open-risks.md",
        ),
        (
            Mode::DomainLanguage,
            "tech-docs/project/domain-language.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::DomainModel,
            "tech-docs/project/domain-model.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::Debugging,
            "tech-docs/project/delivery-map.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
        (
            Mode::PolicyShaping,
            "tech-docs/project/decision-index.md",
            "tech-docs/project/pending-decisions.md",
            "tech-docs/project/audit-log.md",
        ),
    ];

    assert_eq!(expected.len(), Mode::all().len());

    for (mode, stable, pending, evidence) in expected {
        assert_eq!(
            super::stable_project_memory_surface(mode),
            stable,
            "stable surface mismatch for {}",
            mode.as_str()
        );
        assert_eq!(
            super::pending_project_memory_surface(mode),
            pending,
            "pending surface mismatch for {}",
            mode.as_str()
        );
        assert_eq!(
            super::evidence_project_memory_surface(mode),
            evidence,
            "evidence surface mismatch for {}",
            mode.as_str()
        );
    }
}

#[test]
fn publish_metadata_backward_compatible_deserialization() {
    let legacy_json = r#"{
            "run_id": "R-test",
            "mode": "requirements",
            "risk": "low-impact",
            "zone": "green",
            "publish_timestamp": "2026-05-13T00:00:00Z",
            "descriptor": "test",
            "destination": "specs/test",
            "source_artifacts": ["artifact.md"]
        }"#;
    let metadata: PublishMetadata =
        serde_json::from_str(legacy_json).expect("legacy metadata should parse");
    assert!(metadata.primary_artifact.is_empty());
    assert!(metadata.artifact_order.is_empty());
    assert!(metadata.profile.is_none());
    assert!(metadata.promotion_state.is_none());
    assert!(metadata.publication_target_class.is_none());
    assert!(metadata.expertise_input.is_none());
    assert!(metadata.lineage.is_none());
}
