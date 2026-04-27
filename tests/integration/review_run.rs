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

fn ready_review_brief() -> &'static str {
    "# Review Brief\n\n## Review Target\n\n- bounded service boundary package.\n\n## Evidence Basis\n\n- owned interfaces, current tests, and rollback notes.\n\n## Boundary Findings\n\n- no boundary expansion beyond the authored review target was detected.\n\n## Ownership Notes\n\n- reviewer remains accountable for downstream acceptance.\n\n## Missing Evidence\n\nStatus: evidence-bounded\n\n- No critical evidence gaps remain from the authored review packet.\n\n## Collection Priorities\n\n- preserve the current evidence bundle for later inspection.\n\n## Decision Impact\n\n- downstream implementation remains reversible within the bounded package.\n\n## Reversibility Concerns\n\n- stop before broader rollout if the packet changes materially.\n\n## Final Disposition\n\nStatus: ready-with-review-notes\n\nRationale: the review packet is bounded enough for downstream inspection.\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.\n"
}

fn gated_review_brief() -> &'static str {
    "# Review Brief\n\n## Review Target\n\n- release boundary package with must-fix follow-up.\n\n## Evidence Basis\n\n- rollback rehearsal and sign-off evidence remain incomplete.\n\n## Boundary Findings\n\n- release boundary package needs explicit review disposition before acceptance.\n\n## Ownership Notes\n\n- reviewer remains accountable for the final acceptance decision.\n\n## Missing Evidence\n\nStatus: missing-evidence-open\n\n- rollback rehearsal and owner sign-off are still missing.\n\n## Collection Priorities\n\n- capture rollback rehearsal evidence before release readiness.\n\n## Decision Impact\n\n- unresolved concerns keep the release boundary in a reversible holding state.\n\n## Reversibility Concerns\n\n- downstream work should stop until explicit disposition is recorded.\n\n## Final Disposition\n\nStatus: awaiting-disposition\n\nRationale: explicit human disposition is still required before readiness can pass.\n\n## Accepted Risks\n\n- No accepted risks recorded while disposition remains pending.\n"
}

fn write_review_brief(workspace: &TempDir, contents: &str) {
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).expect("canon-input dir");
    fs::write(input_dir.join("review.md"), contents).expect("review brief");
}

#[test]
fn run_review_persists_review_packet_and_evidence_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    write_review_brief(&workspace, ready_review_brief());

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            "canon-input/review.md",
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("review");

    for artifact in [
        "review-brief.md",
        "boundary-assessment.md",
        "missing-evidence.md",
        "decision-impact.md",
        "review-disposition.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the review bundle"
        );
    }

    let disposition = fs::read_to_string(artifact_root.join("review-disposition.md"))
        .expect("review disposition artifact");
    let review_brief =
        fs::read_to_string(artifact_root.join("review-brief.md")).expect("review brief artifact");
    assert!(
        disposition.contains("Status: ready-with-review-notes"),
        "review-disposition should record the completed review posture"
    );
    assert!(review_brief.contains("## Review Target\n\n- bounded service boundary package."));
    assert!(
        review_brief.contains(
            "## Evidence Basis\n\n- owned interfaces, current tests, and rollback notes."
        )
    );
    assert!(!review_brief.contains("## Missing Authored Body"));

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
        entry.as_str().is_some_and(|path| path.ends_with("/review/review-disposition.md"))
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
        Some("Review Disposition")
    );
}

#[test]
fn run_review_preserves_gate_target_and_packet_when_disposition_is_pending() {
    let workspace = TempDir::new().expect("temp dir");
    write_review_brief(&workspace, gated_review_brief());

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "review",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "reviewer",
            "--input",
            "canon-input/review.md",
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
        workspace.path().join(".canon").join("artifacts").join(run_id).join("review");

    let disposition = fs::read_to_string(artifact_root.join("review-disposition.md"))
        .expect("review disposition artifact");
    let missing_evidence = fs::read_to_string(artifact_root.join("missing-evidence.md"))
        .expect("missing evidence artifact");
    assert!(disposition.contains("Status: awaiting-disposition"));
    assert!(missing_evidence.contains("Status: missing-evidence-open"));
    assert!(missing_evidence.contains("rollback rehearsal and owner sign-off are still missing"));

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
    assert_eq!(status_json["state"], "AwaitingApproval");
    assert_eq!(status_json["approval_targets"][0].as_str(), Some("gate:review-disposition"));
    assert_eq!(
        status_json["recommended_next_action"]["action"].as_str(),
        Some("inspect-artifacts")
    );
}
