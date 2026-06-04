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

fn write_backlog_packet(workspace: &TempDir) {
    let packet_root = workspace.path().join("canon-input").join("backlog");
    fs::create_dir_all(&packet_root).expect("backlog packet dir");
    fs::write(
		packet_root.join("brief.md"),
		"# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- tech-docs/changes/auth-session.md\n- tech-docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n\n## Epic Tree\n- Epic AUTH-SESSION: harden revocation and verification boundaries.\n\n## Capability To Epic Map\n- Auth session revocation -> Epic AUTH-SESSION\n\n## Dependency Map\n- [SLICE-AUTH-001] depends on auth-boundary rollback guard rails.\n\n## Delivery Slices\n- [SLICE-AUTH-001] Revoke sessions with rollback-safe persistence boundaries.\n- [SLICE-AUTH-002] Verify revocation evidence reaches downstream operators.\n\n## Sequencing Plan\n1. [SLICE-AUTH-001] first because it establishes the bounded revoke path.\n2. [SLICE-AUTH-002] after the revoke path is stable.\n\n## Acceptance Anchors\n- [SLICE-AUTH-001] Session revoke behavior is bounded and traceable.\n- [SLICE-AUTH-002] Revocation evidence is externally reviewable.\n\n## Planning Risks\n- Hidden auth-boundary coupling can widen rollback scope.\n\n## Execution Handoff\nSelected Slice: SLICE-AUTH-001\nRationale: deliver the first rollback-safe bounded slice before evidence expansion.\n\n## Implementation Artifact References\n- src/auth/session.rs\n- tech-docs/changes/auth-session.md\n\n## Dependency Prerequisites\n- auth-boundary rollback guard rails remain intact.\n\n## Independent Verification Anchors\n- contract test proves an existing session revokes without widening mutation scope.\n\n## Blocked Assumptions\n- No additional login UI changes are required for the first slice.\n",
	)
	.expect("brief");
    fs::write(
		packet_root.join("priorities.md"),
		"# Priorities\n\n- Ship the rollback-safe slice first.\n- Keep dependency blockers explicit.\n",
	)
	.expect("priorities");
    fs::write(
		packet_root.join("context-links.md"),
		"# Context Links\n\n## Source References\n- tech-docs/changes/auth-session.md\n- tech-docs/architecture/auth-boundary.md\n",
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
            .join("tech-docs")
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
        format!(".canon/artifacts/{run_id}/backlog/01-backlog-overview.md"),
        format!(".canon/artifacts/{run_id}/backlog/02-epic-tree.md"),
        format!(".canon/artifacts/{run_id}/backlog/03-capability-to-epic-map.md"),
        format!(".canon/artifacts/{run_id}/backlog/04-dependency-map.md"),
        format!(".canon/artifacts/{run_id}/backlog/05-delivery-slices.md"),
        format!(".canon/artifacts/{run_id}/backlog/06-sequencing-plan.md"),
        format!(".canon/artifacts/{run_id}/backlog/07-acceptance-anchors.md"),
        format!(".canon/artifacts/{run_id}/backlog/08-planning-risks.md"),
        format!(".canon/artifacts/{run_id}/backlog/09-execution-handoff.md"),
        format!(".canon/artifacts/{run_id}/backlog/packet-metadata.json"),
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
        .stdout(contains("governed execution handoff"))
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
            .join("tech-docs")
            .join("planning")
            .join(default_publish_leaf(&run_id, "backlog"))
            .join("05-delivery-slices.md")
            .exists()
    );
    assert!(
        workspace
            .path()
            .join("tech-docs")
            .join("planning")
            .join(default_publish_leaf(&run_id, "backlog"))
            .join("packet-metadata.json")
            .exists()
    );
    assert!(
        workspace
            .path()
            .join("tech-docs")
            .join("planning")
            .join(default_publish_leaf(&run_id, "backlog"))
            .join("09-execution-handoff.md")
            .exists()
    );

    let slices = read_published_file(&workspace, &run_id, "05-delivery-slices.md");
    let dependencies = read_published_file(&workspace, &run_id, "04-dependency-map.md");
    let sequencing = read_published_file(&workspace, &run_id, "06-sequencing-plan.md");
    let anchors = read_published_file(&workspace, &run_id, "07-acceptance-anchors.md");
    let handoff = read_published_file(&workspace, &run_id, "09-execution-handoff.md");

    assert!(slices.contains("## Dependency Links"));
    assert!(slices.contains("SLICE-AUTH-001"));
    assert!(slices.contains("tech-docs/changes/auth-session.md"));
    assert!(dependencies.contains("## Blocking Edges"));
    assert!(dependencies.contains("SLICE-AUTH-001"));
    assert!(sequencing.contains("## Ordering Rationale"));
    assert!(sequencing.contains("SLICE-AUTH-001"));
    assert!(anchors.contains("## Source Trace Links"));
    assert!(anchors.contains("SLICE-AUTH-001"));
    assert!(anchors.contains("tech-docs/architecture/auth-boundary.md"));
    assert!(handoff.contains("## Selected Slice"));
    assert!(handoff.contains("SLICE-AUTH-001"));
    assert!(handoff.contains("src/auth/session.rs"));
    assert!(handoff.contains("## Independent Verification Anchors"));
    assert!(!slices.contains("story points"));
    assert!(!anchors.contains("ticket"));
}
