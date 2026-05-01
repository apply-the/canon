use std::fs;
use std::process::Command as ProcessCommand;

use canon_engine::EngineService;
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

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

fn init_security_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/webhooks")).expect("src dir");
    fs::create_dir_all(workspace.path().join("infra/proxy")).expect("infra dir");
    fs::write(
        workspace.path().join("src/webhooks/verifier.rs"),
        "pub fn verify_signature(_payload: &str, _signature: &str) -> bool {\n    true\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("infra/proxy/webhook-gateway.yaml"),
        "routes:\n  - path: /webhooks\n    forward_headers:\n      - x-signature\n",
    )
    .expect("proxy config");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed security assessment repo"]);
}

fn security_assessment_request(input: &str, risk: RiskClass, zone: UsageZone) -> RunRequest {
    RunRequest {
        mode: Mode::SecurityAssessment,
        risk,
        zone,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "security-lead".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

fn complete_brief() -> &'static str {
    "# Security Assessment Brief\n\n## Assessment Scope\n\n- webhook ingress and signature verification only\n\n## In-Scope Assets\n\n- edge webhook gateway\n- signature verification service\n\n## Trust Boundaries\n\n- internet to edge gateway\n- edge gateway to verification service\n\n## Out Of Scope\n\n- downstream analytics processing\n\n## Threat Inventory\n\n- forged webhook payloads\n- replay attempts with stale signatures\n\n## Attacker Goals\n\n- inject unauthorized events\n\n## Boundary Threats\n\n- proxies may strip signature headers before verification\n\n## Risk Findings\n\n- replay protection is currently not enforced for all webhook actions\n\n## Likelihood And Impact\n\n- likelihood is moderate and impact is high for privileged event handlers\n\n## Proposed Owners\n\n- platform security owns the recommendation backlog\n\n## Recommended Controls\n\n- enforce replay-window validation and reject stale signatures\n\n## Tradeoffs\n\n- tighter replay windows increase sensitivity to clock skew\n\n## Sequencing Notes\n\n1. capture replay-window telemetry\n2. enable bounded rejection after verification passes\n\n## Assumptions\n\n- secret rotation stays in the managed secret store\n\n## Evidence Gaps\n\n- no packet capture confirms all proxy layers preserve signature headers\n\n## Unobservable Surfaces\n\n- third-party sender retry timing remains partially opaque\n\n## Applicable Standards\n\n- OWASP ASVS request integrity guidance applies here\n\n## Control Families\n\n- request validation and secret handling are the primary control families\n\n## Scope Limits\n\n- this packet informs controls and does not certify compliance\n\n## Source Inputs\n\n- src/webhooks/verifier.rs\n- infra/proxy/webhook-gateway.yaml\n\n## Independent Checks\n\n- focused run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- run a bounded replay simulation after the control lands\n"
}

fn incomplete_brief() -> &'static str {
    "# Security Assessment Brief\n\n## Assessment Scope\n\n- webhook ingress only\n\n## Threat Inventory\n\n- forged webhook payloads\n\n## Attacker Goals\n\n- inject unauthorized events\n"
}

#[test]
fn security_assessment_direct_run_exercises_service_summary_and_publish_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_security_repo(&workspace);
    fs::write(workspace.path().join("security-assessment.md"), complete_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(security_assessment_request(
            "security-assessment.md",
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
        ))
        .expect("security assessment run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 7);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("assessment-overview.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("assessment-evidence.md")));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Assessment Overview");
    assert_eq!(mode_result.headline, "Security-assessment packet ready for governed review.");
    assert!(
        mode_result
            .artifact_packet_summary
            .contains("2 in-scope asset set(s), 2 threat set(s), and 1 rated risk set(s)")
    );
    assert!(
        mode_result.primary_artifact_path.ends_with("security-assessment/assessment-overview.md")
    );

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "security-lead",
            ApprovalDecision::Approve,
            "bounded security packet accepted for governed review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.approval_targets.is_empty());
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Assessment Overview")
    );
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );

    let published = service.publish(&summary.run_id, None).expect("publish should succeed");
    let leaf = default_publish_leaf(&summary.run_id, "security-assessment");
    assert!(published.published_to.ends_with(&format!("docs/security-assessments/{leaf}")));
    assert!(published.published_files.iter().any(|path| path.ends_with("assessment-overview.md")));
    assert!(published.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));

    let published_overview = workspace
        .path()
        .join("docs")
        .join("security-assessments")
        .join(&leaf)
        .join("assessment-overview.md");
    assert!(published_overview.exists());
    let overview_contents = fs::read_to_string(published_overview).expect("published overview");
    assert!(overview_contents.contains("## Assessment Scope"));
}

#[test]
fn security_assessment_direct_run_exposes_blocked_gate_and_missing_body_markers() {
    let workspace = TempDir::new().expect("temp dir");
    init_security_repo(&workspace);
    fs::write(workspace.path().join("security-assessment.md"), incomplete_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(security_assessment_request(
            "security-assessment.md",
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ))
        .expect("security assessment run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(summary.blocked_gates.iter().any(|gate| gate.gate == "architecture"));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Assessment Overview");
    assert!(mode_result.headline.contains("explicit missing-context marker(s)"));
    assert!(mode_result.artifact_packet_summary.contains("missing-context marker(s)"));

    let threat_model = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("security-assessment")
            .join("threat-model.md"),
    )
    .expect("threat model");
    assert!(threat_model.contains("## Missing Authored Body"));
    assert!(threat_model.contains("`## Boundary Threats`"));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "architecture"));
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
}
