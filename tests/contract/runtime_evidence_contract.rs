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

fn init_backlog_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed backlog repo"]);
}

fn complete_incident_brief() -> &'static str {
    "# Incident Brief\n\nIncident Scope: payments-api and checkout flow only.\nTrigger And Current State: elevated 5xx responses after the last deploy.\nOperational Constraints: no autonomous remediation and no schema changes.\nKnown Facts:\n- errors started after the deploy\nWorking Hypotheses:\n- retry amplification is exhausting the service\nEvidence Gaps:\n- database saturation is not yet confirmed\nImpacted Surfaces:\n- payments-api\n- checkout flow\nPropagation Paths:\n- checkout request path\nConfidence And Unknowns:\n- medium confidence until saturation evidence is collected\nImmediate Actions:\n- disable async retries\nOrdered Sequence:\n- capture blast radius\n- disable retries\n- reassess error rate\nStop Conditions:\n- error rate stabilizes below the alert threshold\nDecision Points:\n- decide whether rollback is still required\nApproved Actions:\n- disable retries within the bounded surface\nDeferred Actions:\n- schema-level changes remain out of scope\nVerification Checks:\n- confirm 5xx rate drops\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nFollow-Up Work:\n- add a saturation dashboard and post-incident review item\n"
}

#[test]
fn requirements_run_persists_invocation_manifests_and_run_evidence_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(&idea_path, "# Idea\n\nGovern external execution before artifacts.\n")
        .expect("idea file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "product-lead",
            "--input",
            "idea.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");
    let run_root =
        canon_engine::persistence::layout::ProjectLayout::new(workspace.path()).run_dir(run_id);

    assert!(run_root.join("evidence.toml").exists(), "run-level evidence should exist");

    let invocations_dir = run_root.join("invocations");
    assert!(invocations_dir.is_dir(), "invocations directory should exist");

    let first_request = fs::read_dir(&invocations_dir)
        .expect("invocation dir")
        .next()
        .expect("at least one invocation")
        .expect("dir entry")
        .path();

    assert!(first_request.join("request.toml").exists(), "request manifest should exist");
    assert!(first_request.join("decision.toml").exists(), "decision manifest should exist");
    assert!(first_request.join("attempt-01.toml").exists(), "attempt manifest should exist");
}

#[test]
fn closure_limited_backlog_evidence_surfaces_risk_only_packet_and_findings() {
    let workspace = TempDir::new().expect("temp dir");
    init_backlog_repo(&workspace);

    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("packet root");
    fs::write(
        packet_root.join("brief.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n",
    )
    .expect("brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "backlog",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "staff-engineer",
            "--input",
            "canon-input/backlog",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");
    let entry = &evidence_json["entries"][0];

    assert_eq!(entry["closure_status"].as_str(), Some("downgraded"));
    assert_eq!(entry["decomposition_scope"].as_str(), Some("risk-only-packet"));
    assert!(
        entry["closure_findings"].as_array().is_some_and(|findings| findings.len() == 1),
        "expected one warning-only closure finding, got: {}",
        entry
    );
    assert_eq!(entry["closure_findings"][0]["category"].as_str(), Some("missing-exclusion"));
    assert!(entry["artifact_provenance_links"].as_array().is_some_and(|paths| paths.len() == 2));
}

#[test]
fn incident_evidence_surface_keeps_recommendation_only_posture_and_artifact_links() {
    let workspace = TempDir::new().expect("temp dir");
    init_backlog_repo(&workspace);
    fs::write(workspace.path().join("incident.md"), complete_incident_brief()).expect("brief");

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

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");
    let entry = &evidence_json["entries"][0];

    assert_eq!(entry["execution_posture"].as_str(), Some("recommendation-only"));
    assert!(entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(entry["artifact_provenance_links"].as_array().is_some_and(|paths| paths.len() == 6));
}
