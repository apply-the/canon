use std::fs;
use std::process::Command as ProcessCommand;

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

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(["-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
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

fn init_existing_repo(workspace: &TempDir) {
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

fn write_closure_limited_backlog_packet(workspace: &TempDir) {
    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("backlog packet dir");
    fs::write(
		packet_root.join("brief.md"),
		"# Backlog Brief\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Constraints\n- Keep the output above task level.\n",
	)
	.expect("brief");
}

fn write_downgraded_backlog_packet(workspace: &TempDir) {
    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("backlog packet dir");
    fs::write(
		packet_root.join("brief.md"),
		"# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- tech-docs/changes/auth-session.md\n- tech-docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n\n## Epic Tree\n- Epic AUTH-SESSION: harden revocation and verification boundaries.\n\n## Capability To Epic Map\n- Auth session revocation -> Epic AUTH-SESSION\n\n## Dependency Map\n- [SLICE-AUTH-001] depends on auth-boundary rollback guard rails.\n\n## Delivery Slices\n- [SLICE-AUTH-001] Revoke sessions with rollback-safe persistence boundaries.\n\n## Sequencing Plan\n1. [SLICE-AUTH-001] first because it establishes the bounded revoke path.\n\n## Acceptance Anchors\n- [SLICE-AUTH-001] Session revoke behavior is bounded and traceable.\n\n## Planning Risks\n- Verification posture is still underspecified for downstream execution.\n",
	)
	.expect("brief");
    fs::write(
        packet_root.join("priorities.md"),
        "# Priorities\n\n- Ship the rollback-safe slice first.\n",
    )
    .expect("priorities");
}

#[test]
fn closure_blocked_backlog_run_emits_only_risk_packet_artifacts() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    write_closure_limited_backlog_packet(&workspace);

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
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    assert_eq!(json["mode"].as_str(), Some("backlog"));
    assert_eq!(json["state"].as_str(), Some("Blocked"));
    assert!(json["mode_result"]["headline"].as_str().is_some_and(|headline| {
        headline.contains("closure-limited") || headline.contains("blocked")
    }));

    let run_id = json["run_id"].as_str().expect("run id");
    let artifacts_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("backlog-overview.md"))
        .stdout(contains("planning-risks.md"))
        .get_output()
        .stdout
        .clone();
    let artifacts_json: serde_json::Value =
        serde_json::from_slice(&artifacts_output).expect("json output");
    let entries = artifacts_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    let expected_paths = vec![
        format!(".canon/artifacts/{run_id}/backlog/01-backlog-overview.md"),
        format!(".canon/artifacts/{run_id}/backlog/08-planning-risks.md"),
        format!(".canon/artifacts/{run_id}/backlog/packet-metadata.json"),
    ];
    assert_eq!(actual_paths, expected_paths);
    assert!(
        !actual_paths
            .iter()
            .any(|path| path.ends_with("epic-tree.md") || path.ends_with("delivery-slices.md")),
        "closure-limited backlog runs must not emit full decomposition artifacts: {actual_paths:?}"
    );
}

#[test]
fn downgraded_backlog_run_completes_with_a_risk_only_packet() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    write_downgraded_backlog_packet(&workspace);

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

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    assert_eq!(json["mode"].as_str(), Some("backlog"));
    assert_eq!(json["state"].as_str(), Some("Completed"));
    assert_eq!(json["artifact_count"].as_u64(), Some(9));
    assert!(json["mode_result"]["headline"].as_str().is_some_and(|headline| {
        headline.contains("handoff unavailable") || headline.contains("planning risks")
    }));

    let run_id = json["run_id"].as_str().expect("run id");
    let artifacts_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let artifacts_json: serde_json::Value =
        serde_json::from_slice(&artifacts_output).expect("json output");
    let entries = artifacts_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    assert_eq!(
        actual_paths,
        vec![
            format!(".canon/artifacts/{run_id}/backlog/01-backlog-overview.md"),
            format!(".canon/artifacts/{run_id}/backlog/02-epic-tree.md"),
            format!(".canon/artifacts/{run_id}/backlog/03-capability-to-epic-map.md"),
            format!(".canon/artifacts/{run_id}/backlog/04-dependency-map.md"),
            format!(".canon/artifacts/{run_id}/backlog/05-delivery-slices.md"),
            format!(".canon/artifacts/{run_id}/backlog/06-sequencing-plan.md"),
            format!(".canon/artifacts/{run_id}/backlog/07-acceptance-anchors.md"),
            format!(".canon/artifacts/{run_id}/backlog/08-planning-risks.md"),
            format!(".canon/artifacts/{run_id}/backlog/packet-metadata.json"),
        ]
    );

    let overview = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("backlog")
            .join("01-backlog-overview.md"),
    )
    .expect("read overview");
    assert!(overview.contains("handoff unavailable"));
    assert!(
        !workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("backlog")
            .join("09-execution-handoff.md")
            .exists()
    );

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("json output");
    assert_eq!(status_json["closure_status"].as_str(), Some("sufficient"));
    assert_eq!(status_json["decomposition_scope"].as_str(), Some("full-packet"));
}
