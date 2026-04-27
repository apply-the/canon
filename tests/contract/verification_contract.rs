use std::fs;

use assert_cmd::Command;
use predicates::str::contains;
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
    "# Verification Brief\n\n## Claims Under Test\n\n- rollback remains bounded and auditable\n- operator evidence remains tied to the rollback boundary\n\n## Invariant Checks\n\n- rollback metadata remains explicit during the bounded flow\n\n## Contract Assumptions\n\n- rollback metadata must remain explicit\n\n## Verification Outcome\n\nStatus: supported\n\n## Challenge Findings\n\n- no additional challenge findings remain beyond the authored packet\n\n## Contradictions\n\n- none recorded\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n- operator evidence remains tied to the rollback boundary\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported\n\nRationale: the current evidence covers the authored claim set.\n\n## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification packet.\n\n## Required Follow-Up\n\n- Keep the verification packet attached to downstream release review.\n"
}

fn blocked_verification_brief() -> &'static str {
    "# Verification Brief\n\n## Claims Under Test\n\n- the rollback guarantee is fully proven without any additional evidence\n- an unresolved contradiction remains between the authored claim and the runtime contract\n\n## Invariant Checks\n\n- the broadest rollback guarantee still needs contradiction review\n\n## Contract Assumptions\n\n- rollback metadata must remain explicit\n\n## Verification Outcome\n\nStatus: unsupported\n\n## Challenge Findings\n\n- look for contradictions between the rollback claim and the runtime contract\n- look for missing proof in operator evidence\n\n## Contradictions\n\n- an unsupported rollback guarantee still lacks concrete proof\n\n## Verified Claims\n\n- rollback metadata remains explicit\n\n## Rejected Claims\n\n- the rollback guarantee is not yet supported by concrete proof\n\n## Overall Verdict\n\nStatus: unsupported\n\nRationale: unresolved contradictions remain against the authored claim set.\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- resolve the contradiction before treating the packet as supported.\n\n## Required Follow-Up\n\n- look for contradictions between the rollback claim and the runtime contract\n- look for missing proof in operator evidence\n"
}

#[test]
fn verification_run_returns_completed_result_for_supported_claims() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("verification.md");
    fs::write(&brief_path, ready_verification_brief()).expect("brief file");

    let run_output = cli_command()
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

    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    let run_id = run_json["run_id"].as_str().expect("run id");
    assert_eq!(run_json["state"], "Completed");
    assert!(run_json["blocking_classification"].is_null());
    assert!(
        run_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()),
        "completed verification runs should not advertise approval targets"
    );
    assert_eq!(run_json["artifact_count"], 5);
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 5),
        "verification runs should expose the full verification packet"
    );
    assert_eq!(run_json["mode_result"]["primary_artifact_title"], "Verification Report");
    assert_eq!(
        run_json["mode_result"]["primary_artifact_path"],
        format!(".canon/artifacts/{run_id}/verification/verification-report.md")
    );
    assert!(run_json["mode_result"]["headline"].as_str().is_some_and(|headline| {
        headline.contains("supported") && headline.contains("2 claim set(s)")
    }));
    assert!(
        run_json["mode_result"]["artifact_packet_summary"]
            .as_str()
            .is_some_and(|summary| summary.contains("2 claim set(s) under test")
                && summary.contains("0 unresolved finding set(s)"))
    );
    assert!(run_json["mode_result"]["result_excerpt"].as_str().is_some_and(|excerpt| {
        excerpt.contains("Status: supported") && excerpt.contains("Rationale:")
    }));
    assert!(run_json["recommended_next_action"].is_null());

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status_output).expect("status");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(status_json["mode_result"]["primary_artifact_title"], "Verification Report");
    assert!(status_json["recommended_next_action"].is_null());
}

#[test]
fn verification_run_blocks_when_unresolved_findings_remain() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("verification.md");
    fs::write(&brief_path, blocked_verification_brief()).expect("brief file");

    let run_output = cli_command()
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
        .stdout(contains("\"state\": \"Blocked\""))
        .get_output()
        .stdout
        .clone();

    let run_json: serde_json::Value = serde_json::from_slice(&run_output).expect("run json");
    let run_id = run_json["run_id"].as_str().expect("run id");
    assert_eq!(run_json["state"], "Blocked");
    assert_eq!(run_json["blocking_classification"], "artifact-blocked");
    assert!(
        run_json["approval_targets"].as_array().is_some_and(|targets| targets.is_empty()),
        "blocked verification runs should not advertise approval targets"
    );
    assert_eq!(run_json["mode_result"]["primary_artifact_title"], "Verification Report");
    assert_eq!(run_json["recommended_next_action"]["action"], "inspect-artifacts");
    assert!(
        run_json["artifact_paths"].as_array().is_some_and(|paths| paths.len() == 5),
        "blocked verification runs should still expose the readable verification packet"
    );
    assert!(
        run_json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|headline| headline.contains("unresolved finding"))
    );
    assert!(
        run_json["mode_result"]["artifact_packet_summary"]
            .as_str()
            .is_some_and(|summary| summary.contains("2 claim set(s) under test"))
    );
    assert!(run_json["mode_result"]["result_excerpt"].as_str().is_some_and(|excerpt| {
        excerpt.contains("Status: unsupported") && excerpt.contains("Rationale:")
    }));

    let blocked_gates = run_json["blocked_gates"].as_array().expect("blocked gates");
    let readiness_gate = blocked_gates
        .iter()
        .find(|gate| gate["gate"] == "release-readiness")
        .expect("release readiness gate");
    assert!(
        readiness_gate["blockers"].as_array().is_some_and(|blockers| blockers.iter().any(
            |blocker| blocker.as_str().is_some_and(|value| value.contains("unresolved findings")
                || value.contains("unsupported verdict"))
        )),
        "verification status should surface concrete readiness blockers"
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value = serde_json::from_slice(&status_output).expect("status");
    assert_eq!(status_json["state"], "Blocked");
    assert_eq!(status_json["recommended_next_action"]["action"], "inspect-artifacts");
}

#[test]
fn verification_run_rejects_empty_authored_input_before_execution() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("verification.md");
    fs::write(&brief_path, "   \n\n").expect("brief file");

    cli_command()
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
        ])
        .assert()
        .failure()
        .stderr(contains("is empty or whitespace-only"));
}
