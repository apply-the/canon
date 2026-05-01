use std::fs;

use assert_cmd::Command;
use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

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

#[test]
fn init_materializes_the_runtime_contract_layout() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).arg("init").assert().success();

    let canon = workspace.path().join(".canon");
    assert!(canon.exists(), ".canon should exist after init");
    assert!(canon.join("methods").exists(), "methods directory should exist");
    assert!(canon.join("policies").exists(), "policies directory should exist");
    assert!(canon.join("runs").exists(), "runs directory should exist");
    assert!(canon.join("artifacts").exists(), "artifacts directory should exist");
    assert!(
        canon.join("methods").join("backlog.toml").exists(),
        "backlog method file should exist"
    );
    assert!(
        canon.join("methods").join("requirements.toml").exists(),
        "requirements method file should exist"
    );
    assert!(canon.join("policies").join("risk.toml").exists(), "risk policy file should exist");

    let run_entries = fs::read_dir(canon.join("runs")).expect("runs dir is readable").count();
    assert_eq!(run_entries, 0, "init should not create runs");
}

#[test]
fn engine_publish_allows_approval_gated_operational_packets() {
    let workspace = TempDir::new().expect("temp dir");
    let git = |args: &[&str]| {
        let output = std::process::Command::new("git")
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
    };

    git(&["init", "-b", "main"]);
    git(&["config", "user.name", "Canon Test"]);
    git(&["config", "user.email", "canon@example.com"]);
    fs::create_dir_all(workspace.path().join("src/payments")).expect("src dir");
    fs::write(
        workspace.path().join("src/payments/service.rs"),
        "pub fn charge(label: &str) -> String {\n    format!(\"charge:{label}\")\n}\n",
    )
    .expect("source file");
    git(&["add", "."]);
    git(&["commit", "-m", "seed incident repo"]);
    fs::write(
        workspace.path().join("incident.md"),
        "# Incident Brief\n\nIncident Scope: payments-api and checkout flow only.\nTrigger And Current State: elevated 5xx responses after the last deploy.\nOperational Constraints: no autonomous remediation and no schema changes.\nKnown Facts:\n- errors started after the deploy\nWorking Hypotheses:\n- retry amplification is exhausting the service\nEvidence Gaps:\n- database saturation is not yet confirmed\nImpacted Surfaces:\n- payments-api\nPropagation Paths:\n- checkout request path\nConfidence And Unknowns:\n- medium confidence until saturation evidence is collected\nImmediate Actions:\n- disable async retries\nOrdered Sequence:\n- disable retries\nStop Conditions:\n- error rate stabilizes below the alert threshold\nDecision Points:\n- decide whether rollback is still required\nApproved Actions:\n- disable retries within the bounded surface\nDeferred Actions:\n- schema-level changes remain out of scope\nVerification Checks:\n- confirm 5xx rate drops\nRelease Readiness:\n- keep recommendation-only posture until the owner accepts the packet\nFollow-Up Work:\n- add a saturation dashboard and post-incident review item\n",
    )
    .expect("incident brief");

    let service = EngineService::new(workspace.path());
    let run = service
        .run(RunRequest {
            mode: Mode::Incident,
            risk: RiskClass::SystemicImpact,
            zone: UsageZone::Red,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "incident-commander".to_string(),
            inputs: vec!["incident.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("incident run");

    assert_eq!(run.state, "AwaitingApproval");

    let published = service.publish(&run.run_id, None).expect("publish should succeed");
    let leaf = default_publish_leaf(&run.run_id, "incident");
    assert!(published.published_to.ends_with(&format!("docs/incidents/{leaf}")));
    assert!(published.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));
}
