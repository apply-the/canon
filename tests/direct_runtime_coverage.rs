use std::fs;
use std::process::Command as ProcessCommand;

use canon_adapters::shell::ShellAdapter;
use canon_adapters::{
    AdapterError, CapabilityKind, InvocationOrientation, SideEffectClass, TrustBoundaryKind,
};
use canon_engine::EngineService;
use canon_engine::artifacts::contract::{
    contract_for_mode, validate_artifact, validate_release_bundle,
};
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::ClassificationProvenance;
use canon_engine::orchestrator::service::{AiTool, InspectEntry, InspectTarget, RunRequest};
use tempfile::TempDir;

fn request(
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    owner: &str,
    inputs: Vec<&str>,
) -> RunRequest {
    RunRequest {
        mode,
        risk,
        zone,
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: inputs.into_iter().map(ToString::to_string).collect(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_brownfield_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("tests/session.md"),
        "# Session Checks\n\n- revocation formatting remains stable\n",
    )
    .expect("test file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed brownfield repo"]);
}

fn init_review_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{label}\")\n}\n",
    )
    .expect("base source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\nExisting tests cover the formatting helper.\n",
    )
    .expect("base tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "base review helper"]);
    git(workspace, &["checkout", "-b", "feature/pr-review"]);
}

fn add_completed_review_diff(workspace: &TempDir) {
    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.trim())\n}\n",
    )
    .expect("updated source");
    fs::write(
        workspace.path().join("tests/reviewer.md"),
        "# Review Checks\n\n- formatting trims trailing whitespace before labeling\n",
    )
    .expect("updated tests");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "trim review labels"]);
}

fn artifact_names(entries: &[InspectEntry]) -> Vec<String> {
    entries
        .iter()
        .map(|entry| match entry {
            InspectEntry::Name(name) => name.clone(),
            other => panic!("expected name entry, got {other:?}"),
        })
        .collect()
}

#[test]
fn engine_service_initializes_runtime_and_materializes_skills() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let init = service.init(None).expect("init summary");
    assert!(init.methods_materialized > 0);
    assert!(init.policies_materialized > 0);
    assert!(workspace.path().join(".canon").exists());

    let modes = service.inspect(InspectTarget::Modes).expect("inspect modes");
    let mode_names = artifact_names(&modes.entries);
    assert!(mode_names.contains(&"discovery".to_string()));
    assert!(mode_names.contains(&"architecture".to_string()));

    let methods = service.inspect(InspectTarget::Methods).expect("inspect methods");
    assert!(!methods.entries.is_empty());

    let policies = service.inspect(InspectTarget::Policies).expect("inspect policies");
    assert!(!policies.entries.is_empty());

    let installed = service.skills_install(AiTool::Codex).expect("skills install");
    assert!(installed.skills_materialized > 0);

    let listed = service.skills_list();
    assert!(listed.iter().any(|entry| entry.name == "canon-discovery"));
    assert!(listed.iter().any(|entry| entry.name == "canon-inspect-clarity"));

    let updated = service.skills_update(AiTool::Codex).expect("skills update");
    assert!(updated.skills_materialized > 0 || updated.skills_skipped > 0);
}

#[test]
fn requirements_direct_run_approves_invocation_and_resumes_to_completion() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        "# Idea\n\nSystemic requirements framing still needs governed approval.\n",
    )
    .expect("idea file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Requirements,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
            "product-lead",
            vec!["idea.md"],
        ))
        .expect("requirements run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.invocations_total >= 3);
    assert_eq!(summary.blocking_classification.as_deref(), Some("approval-gated"));

    let invocations = service
        .inspect(InspectTarget::Invocations { run_id: summary.run_id.clone() })
        .expect("inspect invocations");
    let pending_request_id = invocations
        .entries
        .iter()
        .find_map(|entry| match entry {
            InspectEntry::Invocation(summary) if summary.policy_decision == "NeedsApproval" => {
                Some(summary.request_id.clone())
            }
            _ => None,
        })
        .expect("pending invocation approval");

    let approval = service
        .approve(
            &summary.run_id,
            &format!("invocation:{pending_request_id}"),
            "principal-engineer",
            ApprovalDecision::Approve,
            "Systemic framing may proceed with explicit human ownership.",
        )
        .expect("approval summary");
    assert_eq!(approval.state, "AwaitingApproval");

    let resumed = service.resume(&summary.run_id).expect("resume requirements run");
    assert_eq!(resumed.state, "Completed");
    assert_eq!(resumed.invocations_denied, 1);

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: summary.run_id.clone() })
        .expect("inspect artifacts");
    let artifact_paths = artifact_names(&artifacts.entries);
    assert!(artifact_paths.iter().any(|path| path.ends_with("problem-statement.md")));
    assert!(artifact_paths.iter().any(|path| path.ends_with("decision-checklist.md")));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert_eq!(status.pending_invocation_approvals, 0);
}

#[test]
fn discovery_direct_run_persists_completed_artifacts_and_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\nProblem: reconcile Canon mode coverage with real governed runtime depth.\nConstraints: preserve the existing runtime shape and evidence model.\nNext Phase: translate discovery into requirements mode with repo-specific scope cuts.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Discovery,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "researcher",
            vec!["discovery.md"],
        ))
        .expect("discovery run");

    assert_eq!(summary.state, "Completed");
    assert_eq!(summary.invocations_total, 4);
    assert_eq!(summary.approval_targets, Vec::<String>::new());

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: summary.run_id.clone() })
        .expect("inspect artifacts");
    let artifact_paths = artifact_names(&artifacts.entries);
    assert_eq!(artifact_paths.len(), 5);
    assert!(artifact_paths.iter().any(|path| path.ends_with("problem-map.md")));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert!(!evidence_entry.generation_paths.is_empty());
    assert!(!evidence_entry.validation_paths.is_empty());

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.validation_independence_satisfied);
}

#[test]
fn greenfield_direct_run_covers_completed_and_blocked_paths() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("system-shaping.md"),
        "# System Shaping Brief\n\nIntent: shape a new governed Canon workflow surface for incomplete analysis modes.\nConstraint: keep the runtime adapters, policy gates, and evidence model intact.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let completed = service
        .run(request(
            Mode::Greenfield,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "architect",
            vec!["system-shaping.md"],
        ))
        .expect("greenfield run");

    assert_eq!(completed.state, "Completed");
    assert_eq!(completed.invocations_total, 3);

    let completed_status = service.status(&completed.run_id).expect("completed status");
    assert_eq!(completed_status.state, "Completed");
    assert!(!completed_status.validation_independence_satisfied);

    let blocked_workspace = TempDir::new().expect("temp dir");
    fs::write(
        blocked_workspace.path().join("system-shaping.md"),
        "# System Shaping Brief\n\nNeed a future-looking shape for analysis mode support.\n",
    )
    .expect("underspecified brief");

    let blocked_service = EngineService::new(blocked_workspace.path());
    let blocked = blocked_service
        .run(request(
            Mode::Greenfield,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "architect",
            vec!["system-shaping.md"],
        ))
        .expect("blocked greenfield run");

    assert_eq!(blocked.state, "Blocked");
    assert_eq!(blocked.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(blocked.blocked_gates.iter().any(|gate| {
        gate.gate == "architecture"
            && gate.blockers.iter().any(|blocker| blocker.contains("lacks sufficient evidence"))
    }));
}

#[test]
fn architecture_direct_run_requires_gate_approval_and_completes_after_status_refresh() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("architecture.md"),
        "# Architecture Brief\n\nDecision focus: identify boundary ownership and tradeoffs for analysis-mode expansion.\nConstraint: preserve Canon runtime contracts, approvals, and evidence persistence.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::Architecture,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
            "principal-architect",
            vec!["architecture.md"],
        ))
        .expect("architecture run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert!(summary.approval_targets.contains(&"gate:risk".to_string()));
    assert_eq!(summary.invocations_pending_approval, 0);

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "principal-engineer",
            ApprovalDecision::Approve,
            "Systemic architecture analysis may proceed with explicit ownership.",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    assert_eq!(evidence.entries.len(), 1);
}

#[test]
fn brownfield_direct_run_records_validation_paths_and_runtime_details() {
    let workspace = TempDir::new().expect("temp dir");
    init_brownfield_repo(&workspace);
    fs::write(
        workspace.path().join("brownfield.md"),
        "# Brownfield Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(request(
            Mode::BrownfieldChange,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "maintainer",
            vec!["brownfield.md"],
        ))
        .expect("brownfield run");

    assert_eq!(summary.state, "Completed");
    assert!(summary.invocations_total >= 3);

    let invocations = service
        .inspect(InspectTarget::Invocations { run_id: summary.run_id.clone() })
        .expect("inspect invocations");
    assert!(invocations.entries.iter().any(|entry| match entry {
        InspectEntry::Invocation(summary) => summary.capability == "ValidateWithTool",
        _ => false,
    }));

    let evidence = service
        .inspect(InspectTarget::Evidence { run_id: summary.run_id.clone() })
        .expect("inspect evidence");
    let evidence_entry = match evidence.entries.first().expect("evidence entry") {
        InspectEntry::Evidence(entry) => entry,
        other => panic!("expected evidence entry, got {other:?}"),
    };
    assert!(!evidence_entry.validation_paths.is_empty());

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.validation_independence_satisfied);
}

#[test]
fn pr_review_direct_run_handles_committed_and_worktree_diffs() {
    let workspace = TempDir::new().expect("temp dir");
    init_review_repo(&workspace);
    add_completed_review_diff(&workspace);

    let service = EngineService::new(workspace.path());
    let committed = service
        .run(request(
            Mode::PrReview,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
            "reviewer",
            vec!["refs/heads/main", "HEAD"],
        ))
        .expect("committed review run");

    assert_eq!(committed.state, "Completed");
    assert_eq!(committed.artifact_count, 7);
    assert!(committed.artifact_paths.iter().any(|path| path.ends_with("pr-analysis.md")));

    let committed_status = service.status(&committed.run_id).expect("committed status");
    assert_eq!(committed_status.state, "Completed");

    fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String {\n    format!(\"review:{}\", label.to_uppercase())\n}\n",
    )
    .expect("worktree change");

    let worktree = service
        .run(request(
            Mode::PrReview,
            RiskClass::LowImpact,
            UsageZone::Green,
            "reviewer",
            vec!["refs/heads/main", "WORKTREE"],
        ))
        .expect("worktree review run");

    assert_eq!(worktree.state, "Completed");
    assert!(worktree.artifact_paths.iter().any(|path| path.ends_with("review-summary.md")));
}

#[test]
fn shell_adapter_reports_worktree_diff_and_enforces_mutation_policy() {
    let workspace = TempDir::new().expect("temp dir");
    git(&workspace, &["init", "-b", "main"]);
    git(&workspace, &["config", "user.name", "Canon Test"]);
    git(&workspace, &["config", "user.email", "canon@example.com"]);
    fs::write(workspace.path().join("notes.txt"), "base\n").expect("notes file");
    git(&workspace, &["add", "."]);
    git(&workspace, &["commit", "-m", "base notes"]);
    fs::write(workspace.path().join("notes.txt"), "base\nupdated\n").expect("updated notes");

    let shell = ShellAdapter;
    let read_request = shell.read_only_request("inspect worktree status");
    assert_eq!(read_request.capability, CapabilityKind::RunCommand);
    assert_eq!(read_request.orientation, Some(InvocationOrientation::Context));
    assert_eq!(read_request.side_effect, SideEffectClass::ReadOnly);
    assert_eq!(read_request.trust_boundary, Some(TrustBoundaryKind::LocalProcess));

    let status = shell
        .run(&read_request, "git", &["status", "--porcelain"], Some(workspace.path()), false)
        .expect("git status");
    assert_eq!(status.status_code, 0);
    assert!(status.stdout.contains("notes.txt"));

    assert!(shell.has_uncommitted_changes(workspace.path()).expect("uncommitted changes"));

    let diff = shell.git_diff_worktree("refs/heads/main", workspace.path()).expect("worktree diff");
    assert_eq!(diff.head_ref, "WORKTREE");
    assert!(diff.changed_files.contains(&"notes.txt".to_string()));
    assert!(diff.patch.contains("updated"));

    let validation_request = shell.validation_request("validate repository visibility");
    assert_eq!(validation_request.capability, CapabilityKind::ValidateWithTool);

    let mutation_request = shell.mutating_request("touch a file");
    let error = shell
        .run(&mutation_request, "git", &["status"], Some(workspace.path()), false)
        .expect_err("mutating request should be blocked when mutation is disallowed");
    assert!(matches!(error, AdapterError::MutationBlocked));
}

#[test]
fn artifact_contract_helpers_cover_analysis_profiles_and_validation_failures() {
    for mode in [Mode::Discovery, Mode::Greenfield, Mode::Architecture, Mode::Implementation] {
        let contract = contract_for_mode(mode);
        assert!(!contract.artifact_requirements.is_empty());

        let complete_bundle = contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let contents = requirement
                    .required_sections
                    .iter()
                    .map(|section| format!("## {section}\n\nRecorded content for {section}."))
                    .collect::<Vec<_>>()
                    .join("\n\n");
                (requirement.file_name.clone(), contents)
            })
            .collect::<Vec<_>>();

        assert!(validate_release_bundle(&contract, &complete_bundle).is_empty());

        let first_requirement = contract.artifact_requirements.first().expect("first requirement");
        let incomplete_contents = if first_requirement.required_sections.len() == 1 {
            "No required headings are present.".to_string()
        } else {
            format!("## {}\n\nOnly one section is present.", first_requirement.required_sections[0])
        };
        let blockers = validate_artifact(first_requirement, &incomplete_contents);
        assert!(!blockers.is_empty());

        let missing_bundle = complete_bundle.into_iter().skip(1).collect::<Vec<_>>();
        let bundle_blockers = validate_release_bundle(&contract, &missing_bundle);
        assert!(
            bundle_blockers.iter().any(|blocker| blocker.contains(&first_requirement.file_name))
        );
    }
}
