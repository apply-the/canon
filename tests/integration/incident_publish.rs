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
    git(workspace, &["commit", "-m", "seed incident publish repo"]);
}

fn complete_brief() -> &'static str {
    "# Incident Brief\n\n## Incident Scope\n\npayments-api and checkout flow only.\n\n## Trigger And Current State\n\nelevated 5xx responses after the last deploy.\n\n## Operational Constraints\n\n- no autonomous remediation\n- no schema changes\n\n## Known Facts\n\n- errors started after the deploy\n- rollback remains available\n\n## Working Hypotheses\n\n- retry amplification is exhausting the service\n\n## Evidence Gaps\n\n- database saturation is not yet confirmed\n\n## Impacted Surfaces\n\n- payments-api\n- checkout flow\n\n## Propagation Paths\n\n- checkout request path\n\n## Confidence And Unknowns\n\n- medium confidence until saturation evidence is collected\n\n## Immediate Actions\n\n- disable async retries\n\n## Ordered Sequence\n\n1. capture blast radius\n2. disable retries\n3. reassess error rate\n\n## Stop Conditions\n\n- error rate stabilizes below the alert threshold\n\n## Decision Points\n\n- decide whether rollback is still required\n\n## Approved Actions\n\n- disable retries within the bounded surface\n\n## Deferred Actions\n\n- schema-level changes remain out of scope\n\n## Verification Checks\n\n- confirm 5xx rate drops\n\n## Release Readiness\n\n- keep recommendation-only posture until the owner accepts the packet\n\n## Follow-Up Work\n\n- add a saturation dashboard and post-incident review item\n"
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

#[test]
fn approval_gated_incident_packet_is_publishable_before_risk_approval() {
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
    assert_eq!(json["state"], "AwaitingApproval");

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    let published = workspace
        .path()
        .join("docs")
        .join("incidents")
        .join(default_publish_leaf(run_id, "incident"))
        .join("follow-up-verification.md");
    let published_text = fs::read_to_string(published).expect("published follow-up verification");
    assert!(
        workspace
            .path()
            .join("docs")
            .join("incidents")
            .join(default_publish_leaf(run_id, "incident"))
            .join("packet-metadata.json")
            .exists()
    );
    assert!(
        published_text.contains("recommendation-only posture until the owner accepts the packet")
    );
}
