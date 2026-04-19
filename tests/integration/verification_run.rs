use std::fs;

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

fn ready_verification_brief() -> &'static str {
    "# Verification Brief\n\n## Claims Under Test\n- rollback remains bounded and auditable\n- operator evidence remains tied to the rollback boundary\n\n## Evidence Basis\n- current contract notes\n- repository checks\n- operator logs\n\n## Contract Surface\n- rollback metadata must remain explicit\n\n## Risk Boundary\n- contradictions or missing evidence on rollback scope should block readiness\n"
}

fn blocked_verification_brief() -> &'static str {
    "# Verification Brief\n\n## Claims Under Test\n- the rollback guarantee is fully proven without any additional evidence\n- an unresolved contradiction remains between the authored claim and the runtime contract\n\n## Evidence Basis\n- an unsupported rollback guarantee still lacks concrete proof\n\n## Contract Surface\n- rollback metadata must remain explicit\n\n## Challenge Focus\n- look for contradictions between the rollback claim and the runtime contract\n- look for missing proof in operator evidence\n"
}

#[test]
fn run_verification_persists_verification_packet_and_evidence_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("verification.md");
    fs::write(&brief_path, ready_verification_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "verification",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("verification");

    for artifact in [
        "invariants-checklist.md",
        "contract-matrix.md",
        "adversarial-review.md",
        "verification-report.md",
        "unresolved-findings.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the verification bundle"
        );
    }

    let report = fs::read_to_string(artifact_root.join("verification-report.md"))
        .expect("verification report artifact");
    let contract = fs::read_to_string(artifact_root.join("contract-matrix.md"))
        .expect("contract matrix artifact");
    let adversarial = fs::read_to_string(artifact_root.join("adversarial-review.md"))
        .expect("adversarial review artifact");
    let unresolved = fs::read_to_string(artifact_root.join("unresolved-findings.md"))
        .expect("unresolved findings artifact");
    assert!(report.contains("Status: supported"));
    assert!(report.contains("- rollback remains bounded and auditable"));
    assert!(contract.contains("rollback metadata must remain explicit"));
    assert!(adversarial.contains("Validation fell back to local workspace scan"));
    assert!(adversarial.contains("Fallback surfaces: verification.md"));
    assert!(unresolved.contains("Status: no-open-findings"));
    assert!(!report.contains("Challenge the verification packet for claim support strength"));

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let inspect_json: serde_json::Value =
        serde_json::from_slice(&inspect_output).expect("inspect json");
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    assert_eq!(entries.len(), 5);
    assert!(entries.iter().any(|entry| {
        entry.as_str().is_some_and(|path| path.ends_with("/verification/verification-report.md"))
    }));

    let evidence_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value =
        serde_json::from_slice(&evidence_output).expect("evidence json");
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(entry["artifact_provenance_links"].as_array().is_some_and(|paths| !paths.is_empty()));

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
    assert_eq!(status_json["validation_independence_satisfied"], true);
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Verification Report")
    );
    assert!(
        status_json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|headline| headline.contains("2 claim set(s)"))
    );
}

#[test]
fn run_verification_surfaces_blocked_readiness_for_unresolved_findings() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("verification.md");
    fs::write(&brief_path, blocked_verification_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "verification",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            brief_path.file_name().expect("file name").to_str().expect("utf8"),
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("verification");

    let report = fs::read_to_string(artifact_root.join("verification-report.md"))
        .expect("verification report artifact");
    let contract = fs::read_to_string(artifact_root.join("contract-matrix.md"))
        .expect("contract matrix artifact");
    let unresolved = fs::read_to_string(artifact_root.join("unresolved-findings.md"))
        .expect("unresolved findings artifact");
    assert!(report.contains("Status: unsupported"));
    assert!(report.contains("Still unsupported from the current packet"));
    assert!(report.contains("Rationale:"));
    assert!(contract.contains("rollback metadata must remain explicit"));
    assert!(unresolved.contains("Status: unresolved-findings-open"));
    assert!(
        unresolved.contains(
            "look for contradictions between the rollback claim and the runtime contract"
        )
    );
    assert!(!report.contains("Challenge the verification packet for claim support strength"));

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let inspect_json: serde_json::Value =
        serde_json::from_slice(&inspect_output).expect("inspect json");
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    assert_eq!(entries.len(), 5);

    let evidence_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value =
        serde_json::from_slice(&evidence_output).expect("evidence json");
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));
    assert!(entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()));

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
    assert_eq!(status_json["state"], "Blocked");
    assert_eq!(status_json["blocking_classification"], "artifact-blocked");
    assert!(status_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()));
    assert_eq!(
        status_json["recommended_next_action"]["action"].as_str(),
        Some("inspect-artifacts")
    );
    assert!(status_json["mode_result"]["result_excerpt"].as_str().is_some_and(|excerpt| {
        excerpt.contains("Status: unsupported") && excerpt.contains("Rationale:")
    }));
}
