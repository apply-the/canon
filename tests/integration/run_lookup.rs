use std::fs;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use std::sync::{Mutex, MutexGuard, OnceLock};

use assert_cmd::Command;
use tempfile::TempDir;

fn cli_command() -> Command {
    if let Some(binary) = std::env::var_os("CARGO_BIN_EXE_canon") {
        return Command::new(binary);
    }

    let workspace_target = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let candidate =
        workspace_target.join("debug").join(format!("canon{}", std::env::consts::EXE_SUFFIX));
    if candidate.exists() {
        return Command::new(candidate);
    }

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

fn run_lookup_test_guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().expect("run lookup test lock")
}

fn init_workspace() -> TempDir {
    let workspace = tempfile::tempdir().expect("tempdir");
    cli_command()
        .current_dir(workspace.path())
        .args(["init", "--output", "json"])
        .assert()
        .success();
    workspace
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
    git(workspace, &["commit", "-m", "seed implementation repo"]);
}

fn create_requirements_run(workspace: &TempDir) -> String {
    let input_dir = workspace.path().join("canon-input");
    fs::create_dir_all(&input_dir).unwrap();
    let unique = format!("Idea {}", uuid::Uuid::now_v7().as_simple());
    fs::write(
        input_dir.join("idea.md"),
        format!(
            "# Requirements Brief\n\n## Problem\n\n{unique}: bound the work before downstream planning.\n\n## Outcome\n\nOperators can review a complete requirements packet before implementation.\n\n## Constraints\n\n- Keep execution local-first\n- Preserve explicit audit logs\n\n## Non-Negotiables\n\n- Keep human ownership explicit\n- Persist artifacts under `.canon/`\n\n## Options\n\n1. Start with a bounded CLI slice.\n2. Defer broader orchestration work.\n\n## Recommended Path\n\nStart with the bounded CLI slice before expanding scope.\n\n## Tradeoffs\n\n- Governance adds upfront structure.\n- Durable artifacts add overhead but improve reviewability.\n\n## Consequences\n\n- Reviewers can inspect the packet without chat history.\n\n## Scope Cuts\n\n- No GUI in this slice\n\n## Deferred Work\n\n- Hosted rollout remains a later slice.\n\n## Decision Checklist\n\n- [x] Scope is explicit\n- [x] Ownership is explicit\n\n## Open Questions\n\n- Which downstream mode should consume the packet first?\n"
        ),
    )
    .unwrap();

    let assert = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "Owner <owner@example.com>",
            "--input",
            "canon-input/idea.md",
            "--output",
            "json",
        ])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("run json");
    json["run_id"].as_str().expect("run_id").to_string()
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

#[test]
fn run_creation_emits_canonical_display_id() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    assert!(
        run_id.starts_with("R-") && run_id.len() == 19,
        "run_id should look like R-YYYYMMDD-XXXXXXXX, got `{run_id}`"
    );
}

#[test]
fn status_resolves_short_id_prefix() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    let short = &run_id[run_id.len() - 8..];

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", short, "--output", "json"])
        .assert()
        .success();
}

#[test]
fn list_runs_reports_empty_workspace_in_text_format() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();

    let assert =
        cli_command().current_dir(workspace.path()).args(["list", "runs"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert_eq!(stdout.trim(), "(no runs)");
}

#[test]
fn list_runs_returns_structured_json_output() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert = cli_command()
        .current_dir(workspace.path())
        .args(["list", "runs", "--output", "json"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout).to_string();
    let runs: Vec<serde_json::Value> = serde_json::from_str(&stdout).expect("list runs json");

    assert_eq!(runs.len(), 1, "expected one visible run in list output: {stdout}");
    assert_eq!(runs[0]["run_id"], run_id);
    assert_eq!(runs[0]["short_id"].as_str().map(str::len), Some(8));
    assert!(runs[0]["created_at"].as_str().is_some(), "expected created_at string: {stdout}");
    assert!(runs[0]["is_legacy"].is_boolean(), "expected is_legacy boolean: {stdout}");
}

#[test]
fn list_runs_renders_text_table_for_existing_runs() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert =
        cli_command().current_dir(workspace.path()).args(["list", "runs"]).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(stdout.contains("RUN_ID"), "expected header row: {stdout}");
    assert!(stdout.contains("SHORT_ID"), "expected header row: {stdout}");
    assert!(stdout.contains(&run_id), "expected run in table: {stdout}");
}

#[test]
fn list_runs_supports_yaml_output() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    let assert = cli_command()
        .current_dir(workspace.path())
        .args(["list", "runs", "--output", "yaml"])
        .assert()
        .success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);

    assert!(
        stdout.contains(&format!("run_id: {run_id}")),
        "expected run_id in yaml output: {stdout}"
    );
    assert!(stdout.contains("created_at:"), "expected created_at field in yaml output: {stdout}");
}

#[test]
fn publish_accepts_last_alias_and_writes_default_destination() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);

    cli_command().current_dir(workspace.path()).args(["publish", "@last"]).assert().success();

    assert!(
        workspace
            .path()
            .join("specs")
            .join(default_publish_leaf(&run_id, "requirements"))
            .join("problem-statement.md")
            .exists()
    );
}

#[test]
fn publish_accepts_short_id_prefix_and_explicit_destination() {
    let _guard = run_lookup_test_guard();
    let workspace = init_workspace();
    let run_id = create_requirements_run(&workspace);
    let short = &run_id[run_id.len() - 8..];

    cli_command()
        .current_dir(workspace.path())
        .args(["publish", short, "--to", "docs/public/prd"])
        .assert()
        .success();

    assert!(workspace.path().join("docs/public/prd").join("problem-statement.md").exists());
}

#[test]
fn recommendation_only_implementation_runs_remain_resolvable_via_last_alias() {
    let _guard = run_lookup_test_guard();
    let workspace = tempfile::tempdir().expect("tempdir");
    init_existing_repo(&workspace);
    fs::write(
        workspace.path().join("implementation.md"),
        "# Implementation Brief\n\n## Task Mapping\n\n1. Add bounded auth session repository helpers.\n2. Thread the helper through the revocation service without expanding the public API.\n3. Record implementation notes for operator review and rollback.\n\n## Bounded Changes\n\n- Auth session repository helper wiring.\n- Revocation service internal composition.\n\n## Mutation Bounds\n\nsrc/auth/session.rs and src/auth/repository.rs only.\n\n## Allowed Paths\n\n- src/auth/session.rs\n- src/auth/repository.rs\n\n## Executed Changes\n\n- Add the bounded repository helper and thread it through the revocation service without widening the public API.\n\n## Candidate Frameworks\n\n- Candidate 1 keeps the helper local to the auth-session slice.\n- Candidate 2 introduces a shared auth abstraction before the bounded slice is proven.\n\n## Options Matrix\n\n- Option 1 keeps the helper inside the auth-session slice.\n- Option 2 introduces a shared auth abstraction before the bounded slice is proven.\n\n## Decision Evidence\n\n- Existing auth-session rollback posture already aligns with the local helper approach.\n- Focused implementation suites already guard the bounded packet contract.\n\n## Recommendation\n\n- Start with the local helper and defer broader abstraction until a later change proves it necessary.\n\n## Task Linkage\n\n- Step 1 adds the helper.\n- Step 2 rewires the service behind the existing external contract.\n- Step 3 records the resulting packet and rollback posture.\n\n## Completion Evidence\n\n- The emitted implementation packet and focused tests confirm the bounded slice is ready for operator review.\n\n## Adoption Implications\n\n- Operators can adopt the helper in the auth-session slice without widening the pattern across the rest of auth.\n\n## Remaining Risks\n\n- Repository wiring could still drift into adjacent auth modules if the bounded paths expand during review.\n\n## Ecosystem Health\n\n- The surrounding auth subsystem is stable enough for a local helper, but shared abstraction pressure is still low.\n\n## Safety-Net Evidence\n\nContract coverage protects revocation formatting and audit ordering before mutation.\n\n## Independent Checks\n\n- cargo test --test session_contract\n\n## Rollback Triggers\n\nRevocation output drifts or audit ordering becomes unstable.\n\n## Rollback Steps\n\n1. Revert the bounded auth-session patch.\n2. Redeploy the previous build.\n",
    )
    .expect("implementation brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "implementation",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
            "--input",
            "implementation.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run_id");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:execution",
            "--by",
            "maintainer",
            "--decision",
            "approve",
            "--rationale",
            "approved bounded execution",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", run_id])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", "@last", "--output", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("\"run\": \"{run_id}\"")));

    cli_command().current_dir(workspace.path()).args(["publish", "@last"]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("implementation")
            .join(default_publish_leaf(run_id, "implementation"))
            .join("task-mapping.md")
            .exists()
    );
}

#[test]
fn backlog_runs_remain_publishable_via_last_alias_and_short_id() {
    let _guard = run_lookup_test_guard();
    let workspace = tempfile::tempdir().expect("tempdir");
    init_existing_repo(&workspace);
    fs::create_dir_all(workspace.path().join("canon-input").join("backlog"))
        .expect("backlog packet dir");
    fs::write(
        workspace.path().join("canon-input").join("backlog").join("brief.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded delivery backlog for auth session hardening.\n\n## Desired Granularity\nepic-plus-slice\n\n## Planning Horizon\nnext two releases\n\n## Source References\n- docs/changes/auth-session.md\n- docs/architecture/auth-boundary.md\n\n## Constraints\n- Keep the output above task level.\n\n## Out of Scope\n- Login UI redesign\n",
    )
    .expect("backlog brief");
    fs::write(
        workspace.path().join("canon-input").join("backlog").join("priorities.md"),
        "# Priorities\n\n- Ship the rollback-safe slice first.\n",
    )
    .expect("backlog priorities");

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
            "maintainer",
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
    let json: serde_json::Value = serde_json::from_slice(&output).expect("run json");
    let run_id = json["run_id"].as_str().expect("run_id");
    let short = &run_id[run_id.len() - 8..];

    cli_command().current_dir(workspace.path()).args(["publish", "@last"]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("planning")
            .join(default_publish_leaf(run_id, "backlog"))
            .join("backlog-overview.md")
            .exists()
    );

    cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", short, "--output", "json"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("\"run\": \"{run_id}\"")));
}
