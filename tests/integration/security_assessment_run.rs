use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use tempfile::TempDir;

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

fn init_repo(workspace: &TempDir) {
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
fn run_security_assessment_emits_a_recommendation_only_packet_and_publishes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("security-assessment.md"), complete_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "security-assessment",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "security-lead",
            "--input",
            "security-assessment.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("security-assessment");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["artifact_count"], 7);
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Assessment Overview"));
    assert!(
        json["approval_targets"].as_array().is_some_and(|targets| targets
            .iter()
            .any(|target| target.as_str() == Some("gate:risk")))
    );
    assert!(artifact_root.join("assessment-overview.md").exists());
    assert!(artifact_root.join("threat-model.md").exists());
    assert!(artifact_root.join("risk-register.md").exists());
    assert!(artifact_root.join("mitigations.md").exists());
    assert!(artifact_root.join("assumptions-and-gaps.md").exists());
    assert!(artifact_root.join("compliance-anchors.md").exists());
    assert!(artifact_root.join("assessment-evidence.md").exists());

    let threat_model =
        fs::read_to_string(artifact_root.join("threat-model.md")).expect("threat model");
    assert!(threat_model.contains("## Boundary Threats"));

    let compliance =
        fs::read_to_string(artifact_root.join("compliance-anchors.md")).expect("compliance");
    assert!(compliance.contains("## Scope Limits"));
    assert!(!compliance.contains("## Missing Authored Body"));

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "security-lead",
            "--decision",
            "approve",
            "--rationale",
            "bounded security packet accepted for governed review",
        ])
        .assert()
        .success();

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Assessment Overview")
    );

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("security-assessments")
            .join(default_publish_leaf(run_id, "security-assessment"))
            .join("assessment-overview.md")
            .exists()
    );
}

#[test]
fn run_security_assessment_blocks_when_a_required_authored_section_is_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("security-assessment.md"), incomplete_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "security-assessment",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "security-lead",
            "--input",
            "security-assessment.md",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("security-assessment");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");

    let threat_model =
        fs::read_to_string(artifact_root.join("threat-model.md")).expect("threat model");
    assert!(threat_model.contains("## Missing Authored Body"));
    assert!(threat_model.contains("`## Boundary Threats`"));
}
