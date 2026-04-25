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

    fs::create_dir_all(workspace.path().join("src/payments")).expect("src dir");
    fs::write(
        workspace.path().join("src/payments/service.rs"),
        "pub fn charge(label: &str) -> String {\n    format!(\"charge:{label}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed incident repo"]);
}

fn complete_brief() -> &'static str {
    "# Incident Brief\n\nIncident Scope: payments-api and checkout flow only.\nTrigger And Current State: elevated 5xx responses after the last deploy.\nOperational Constraints: no autonomous remediation and no schema changes.\nKnown Facts:\n- errors started after the deploy\n- rollback remains available\nWorking Hypotheses:\n- retry amplification is exhausting the service\nEvidence Gaps:\n- database saturation is not yet confirmed\nImpacted Surfaces:\n- payments-api\n- checkout flow\nPropagation Paths:\n- checkout request path\nConfidence And Unknowns:\n- medium confidence until saturation evidence is collected\nImmediate Actions:\n- disable async retries\nOrdered Sequence:\n- capture blast radius\n- disable retries\n- reassess error rate\nStop Conditions:\n- error rate stabilizes below the alert threshold\nDecision Points:\n- decide whether rollback is still required\nApproved Actions:\n- disable retries within the bounded surface\nDeferred Actions:\n- schema-level changes remain out of scope\nVerification Checks:\n- confirm 5xx rate drops\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nFollow-Up Work:\n- add a saturation dashboard and post-incident review item\n"
}

#[test]
fn run_incident_emits_a_governed_containment_packet_and_publishes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("incident.md"), complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "incident",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "red",
            "--owner",
            "incident-commander",
            "--input",
            "incident.md",
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("incident");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["artifact_count"], 6);
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Incident Frame"));
    assert!(
        json["approval_targets"].as_array().is_some_and(|targets| targets
            .iter()
            .any(|target| target.as_str() == Some("gate:risk")))
    );
    assert!(artifact_root.join("incident-frame.md").exists());
    assert!(artifact_root.join("containment-plan.md").exists());
    assert!(artifact_root.join("follow-up-verification.md").exists());

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "incident-commander",
            "--decision",
            "approve",
            "--rationale",
            "bounded containment packet accepted for operator review",
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
        Some("Incident Frame")
    );
    assert!(status_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()));

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("incidents")
            .join(run_id)
            .join("incident-frame.md")
            .exists()
    );
}
