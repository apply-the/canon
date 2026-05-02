use std::fs;
use std::process::Command as ProcessCommand;

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
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

fn write_backlog_packet(workspace: &TempDir) {
    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("backlog packet dir");
    fs::write(
		packet_root.join("brief.md"),
		"# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n",
	)
	.expect("brief");
    fs::write(
		packet_root.join("priorities.md"),
		"# Priorities\n\n- Ship the rollback-safe slice first.\n- Keep dependency blockers explicit.\n",
	)
	.expect("priorities");
    fs::write(
		packet_root.join("context-links.md"),
		"# Context Links\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n",
	)
	.expect("context links");
}

fn run_backlog_flow(workspace: &TempDir) -> String {
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
    json["run_id"].as_str().expect("run id").to_string()
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

fn read_published_file(workspace: &TempDir, run_id: &str, file_name: &str) -> String {
    fs::read_to_string(
        workspace
            .path()
            .join("docs")
            .join("planning")
            .join(default_publish_leaf(run_id, "backlog"))
            .join(file_name),
    )
    .expect("published file")
}

#[test]
fn inspect_artifacts_lists_the_full_backlog_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    write_backlog_packet(&workspace);
    let run_id = run_backlog_flow(&workspace);

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("backlog-overview.md"))
        .stdout(contains("planning-risks.md"))
        .get_output()
        .stdout
        .clone();

    let inspect_json: serde_json::Value =
        serde_json::from_slice(&inspect_output).expect("json output");
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    let expected_paths = vec![
        format!(".canon/artifacts/{run_id}/backlog/acceptance-anchors.md"),
        format!(".canon/artifacts/{run_id}/backlog/backlog-overview.md"),
        format!(".canon/artifacts/{run_id}/backlog/capability-to-epic-map.md"),
        format!(".canon/artifacts/{run_id}/backlog/delivery-slices.md"),
        format!(".canon/artifacts/{run_id}/backlog/dependency-map.md"),
        format!(".canon/artifacts/{run_id}/backlog/epic-tree.md"),
        format!(".canon/artifacts/{run_id}/backlog/planning-risks.md"),
        format!(".canon/artifacts/{run_id}/backlog/sequencing-plan.md"),
    ];
    assert_eq!(actual_paths, expected_paths);

    let invocations_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "invocations", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let invocations_json: serde_json::Value =
        serde_json::from_slice(&invocations_output).expect("json output");
    let entries = invocations_json["entries"].as_array().expect("invocation entries");
    assert_eq!(
        entries.len(),
        4,
        "backlog should persist read, generate, critique, and validation requests"
    );
    assert!(entries.iter().any(|entry| entry["capability"] == "ReadRepository"));
    assert!(entries.iter().any(|entry| entry["capability"] == "GenerateContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "CritiqueContent"));
    assert!(entries.iter().any(|entry| entry["capability"] == "ValidateWithTool"));
}

#[test]
fn run_backlog_markdown_surfaces_primary_result_without_task_level_sections() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    write_backlog_packet(&workspace);

    cli_command()
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
            "markdown",
        ])
        .assert()
        .success()
        .stdout(contains("# run"))
        .stdout(contains("Primary Artifact: .canon/artifacts/"))
        .stdout(contains("Backlog packet is publishable for downstream execution planning."))
        .stdout(contains("## Task Mapping").not())
        .stdout(contains("task-mapping.md").not());
}

#[test]
fn publish_backlog_packet_preserves_handoff_context_without_hidden_runtime_state() {
    let workspace = TempDir::new().expect("temp dir");
    init_existing_repo(&workspace);
    write_backlog_packet(&workspace);
    let run_id = run_backlog_flow(&workspace);

    cli_command().current_dir(workspace.path()).args(["publish", &run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("planning")
            .join(default_publish_leaf(&run_id, "backlog"))
            .join("delivery-slices.md")
            .exists()
    );
    assert!(
        workspace
            .path()
            .join("docs")
            .join("planning")
            .join(default_publish_leaf(&run_id, "backlog"))
            .join("packet-metadata.json")
            .exists()
    );

    let slices = read_published_file(&workspace, &run_id, "delivery-slices.md");
    let dependencies = read_published_file(&workspace, &run_id, "dependency-map.md");
    let sequencing = read_published_file(&workspace, &run_id, "sequencing-plan.md");
    let anchors = read_published_file(&workspace, &run_id, "acceptance-anchors.md");

    assert!(slices.contains("## Dependency Links"));
    assert!(slices.contains("docs/changes/auth-session.md"));
    assert!(dependencies.contains("## Blocking Edges"));
    assert!(sequencing.contains("## Ordering Rationale"));
    assert!(anchors.contains("## Source Trace Links"));
    assert!(anchors.contains("docs/architecture/auth-boundary.md"));
    assert!(!slices.contains("story points"));
    assert!(!anchors.contains("ticket"));
}
