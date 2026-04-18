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

#[test]
fn init_without_ai_only_materializes_runtime_state() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).arg("init").assert().success();

    assert!(workspace.path().join(".canon").is_dir(), ".canon should be created by init");
    assert!(
        !workspace.path().join(".agents").exists(),
        ".agents should not be created unless --ai codex/copilot is requested"
    );
    assert!(
        !workspace.path().join(".claude").exists(),
        ".claude should not be created unless --ai claude is requested"
    );
    assert!(
        !workspace.path().join("CLAUDE.md").exists(),
        "CLAUDE.md should not be created unless --ai claude is requested"
    );
}

#[test]
fn init_with_codex_materializes_agents_skills_only() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let skills = workspace.path().join(".agents").join("skills");
    assert!(skills.is_dir(), ".agents/skills should be created by init");
    assert!(
        skills.join("canon-init").join("SKILL.md").exists(),
        "canon-init/SKILL.md should exist in .agents/skills"
    );
    assert!(
        skills.join("canon-inspect-clarity").join("SKILL.md").exists(),
        "canon-inspect-clarity/SKILL.md should exist in .agents/skills"
    );
    assert!(
        skills.join("canon-requirements").join("SKILL.md").exists(),
        "canon-requirements/SKILL.md should exist in .agents/skills"
    );
    assert!(
        skills.join("canon-shared").join("scripts").join("check-runtime.sh").exists(),
        "check-runtime.sh should exist in .agents/skills"
    );
    assert!(
        !workspace.path().join(".claude").exists(),
        ".claude should not be created for --ai codex"
    );
    assert!(
        !workspace.path().join("CLAUDE.md").exists(),
        "CLAUDE.md should not be created for --ai codex"
    );
}

#[test]
fn init_with_claude_materializes_claude_skills_only() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).args(["init", "--ai", "claude"]).assert().success();

    let claude_skills = workspace.path().join(".claude").join("skills");
    assert!(claude_skills.is_dir(), ".claude/skills should be created by init");
    assert!(
        claude_skills.join("canon-init").join("SKILL.md").exists(),
        "canon-init/SKILL.md should exist in .claude/skills"
    );
    assert!(
        claude_skills.join("canon-inspect-clarity").join("SKILL.md").exists(),
        "canon-inspect-clarity/SKILL.md should exist in .claude/skills"
    );
    assert!(
        claude_skills.join("canon-requirements").join("SKILL.md").exists(),
        "canon-requirements/SKILL.md should exist in .claude/skills"
    );
    assert!(
        claude_skills.join("canon-shared").join("scripts").join("check-runtime.sh").exists(),
        "check-runtime.sh should exist in .claude/skills"
    );

    let claude_md = workspace.path().join("CLAUDE.md");
    assert!(claude_md.exists(), "CLAUDE.md should be created by init");
    let contents = std::fs::read_to_string(&claude_md).expect("read CLAUDE.md");
    assert!(contents.contains("@AGENTS.md"), "CLAUDE.md should import AGENTS.md");

    assert!(
        !workspace.path().join(".agents").exists(),
        ".agents should not be created for --ai claude"
    );
}

#[test]
fn init_is_idempotent_for_codex_skills() {
    let workspace = TempDir::new().expect("temp dir");

    let mut first = cli_command();
    first.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let skill_path =
        workspace.path().join(".agents").join("skills").join("canon-init").join("SKILL.md");
    assert!(skill_path.exists(), "skill should exist after first init");

    let original = std::fs::read_to_string(&skill_path).expect("read skill");
    std::fs::write(&skill_path, "modified").expect("write");

    let mut second = cli_command();
    second.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let after_second = std::fs::read_to_string(&skill_path).expect("read skill after second init");
    assert_eq!(after_second, "modified", "init should not overwrite existing skill");

    let mut update = cli_command();
    update
        .current_dir(workspace.path())
        .args(["skills", "update", "--ai", "codex"])
        .assert()
        .success();

    let after_update = std::fs::read_to_string(&skill_path).expect("read skill after update");
    assert_eq!(after_update, original, "skills update should restore embedded content");
}

#[test]
fn init_is_idempotent_for_claude_files() {
    let workspace = TempDir::new().expect("temp dir");

    let mut first = cli_command();
    first.current_dir(workspace.path()).args(["init", "--ai", "claude"]).assert().success();

    let skill_path =
        workspace.path().join(".claude").join("skills").join("canon-init").join("SKILL.md");
    let claude_md = workspace.path().join("CLAUDE.md");
    assert!(skill_path.exists(), "Claude skill should exist after first init");
    assert!(claude_md.exists(), "CLAUDE.md should exist after first init");

    let original = std::fs::read_to_string(&skill_path).expect("read skill");
    std::fs::write(&skill_path, "modified").expect("write skill");
    std::fs::write(&claude_md, "custom user content\n").expect("write CLAUDE.md");

    let mut second = cli_command();
    second.current_dir(workspace.path()).args(["init", "--ai", "claude"]).assert().success();

    let after_second = std::fs::read_to_string(&skill_path).expect("read skill after second init");
    assert_eq!(after_second, "modified", "init should not overwrite existing Claude skill");

    let claude_after =
        std::fs::read_to_string(&claude_md).expect("read CLAUDE.md after second init");
    assert_eq!(
        claude_after, "custom user content\n",
        "init should not overwrite existing CLAUDE.md"
    );

    let mut update = cli_command();
    update
        .current_dir(workspace.path())
        .args(["skills", "update", "--ai", "claude"])
        .assert()
        .success();

    let after_update = std::fs::read_to_string(&skill_path).expect("read skill after update");
    assert_eq!(after_update, original, "skills update should restore embedded content");

    let claude_after_update =
        std::fs::read_to_string(&claude_md).expect("read CLAUDE.md after update");
    assert_eq!(
        claude_after_update, "custom user content\n",
        "skills update should not overwrite existing CLAUDE.md"
    );
}

#[test]
fn skills_install_for_codex_works_without_canon_dir() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command
        .current_dir(workspace.path())
        .args(["skills", "install", "--ai", "codex"])
        .assert()
        .success();

    let skills = workspace.path().join(".agents").join("skills");
    assert!(skills.is_dir(), ".agents/skills should be created by skills install");
    assert!(
        skills.join("canon-init").join("SKILL.md").exists(),
        "canon-init/SKILL.md should exist in .agents/skills"
    );
    assert!(
        !workspace.path().join(".claude").exists(),
        ".claude should not be created by codex skills install"
    );
    assert!(
        !workspace.path().join(".canon").exists(),
        ".canon/ should not be created by skills install"
    );
}

#[test]
fn skills_install_for_claude_works_without_canon_dir() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command
        .current_dir(workspace.path())
        .args(["skills", "install", "--ai", "claude"])
        .assert()
        .success();

    let claude_skills = workspace.path().join(".claude").join("skills");
    assert!(claude_skills.is_dir(), ".claude/skills should be created by skills install");
    assert!(
        claude_skills.join("canon-init").join("SKILL.md").exists(),
        "canon-init/SKILL.md should exist in .claude/skills"
    );
    assert!(workspace.path().join("CLAUDE.md").exists(), "CLAUDE.md should be created");
    assert!(
        !workspace.path().join(".agents").exists(),
        ".agents should not be created by claude skills install"
    );
    assert!(
        !workspace.path().join(".canon").exists(),
        ".canon/ should not be created by skills install"
    );
}

#[test]
fn skills_list_returns_all_embedded_skills() {
    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    let output = command
        .current_dir(workspace.path())
        .args(["skills", "list", "--output", "json"])
        .output()
        .expect("run skills list");

    assert!(output.status.success(), "skills list should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries: Vec<serde_json::Value> =
        serde_json::from_str(&stdout).expect("parse JSON skill list");

    assert!(entries.len() >= 20, "should have at least 20 embedded canon skills");

    let names: Vec<&str> = entries.iter().filter_map(|entry| entry.get("name")?.as_str()).collect();

    assert!(names.contains(&"canon-init"), "should list canon-init");
    assert!(names.contains(&"canon-inspect-clarity"), "should list canon-inspect-clarity");
    assert!(names.contains(&"canon-requirements"), "should list canon-requirements");
    assert!(names.contains(&"canon-brownfield"), "should list canon-brownfield");
    assert!(names.contains(&"canon-pr-review"), "should list canon-pr-review");
    assert!(names.contains(&"canon-discovery"), "should list canon-discovery");
}

#[cfg(unix)]
#[test]
fn materialized_shell_scripts_are_executable() {
    use std::os::unix::fs::PermissionsExt;

    let workspace = TempDir::new().expect("temp dir");

    let mut command = cli_command();
    command.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let script = workspace
        .path()
        .join(".agents")
        .join("skills")
        .join("canon-shared")
        .join("scripts")
        .join("check-runtime.sh");

    assert!(script.exists(), "check-runtime.sh should exist");

    let perms = std::fs::metadata(&script).expect("stat").permissions();
    assert!(perms.mode() & 0o111 != 0, "shell scripts should be executable");
}

#[cfg(unix)]
#[test]
fn pr_review_preflight_accepts_remote_tracking_refs() {
    use std::os::unix::fs::PermissionsExt;

    let workspace = TempDir::new().expect("temp dir");
    let remote = TempDir::new().expect("remote temp dir");

    let remote_output = ProcessCommand::new("git")
        .args(["init", "--bare", "--initial-branch=main"])
        .arg(remote.path())
        .output()
        .expect("git init bare");
    assert!(
        remote_output.status.success(),
        "git init bare failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&remote_output.stdout),
        String::from_utf8_lossy(&remote_output.stderr)
    );

    git(&workspace, &["init", "-b", "main"]);
    git(&workspace, &["config", "user.name", "Canon Test"]);
    git(&workspace, &["config", "user.email", "canon@example.com"]);

    std::fs::write(workspace.path().join("README.md"), "base\n").expect("write base file");
    git(&workspace, &["add", "."]);
    git(&workspace, &["commit", "-m", "base"]);

    git(&workspace, &["remote", "add", "origin", remote.path().to_str().expect("remote path")]);
    git(&workspace, &["push", "-u", "origin", "main"]);
    git(&workspace, &["checkout", "-b", "feature/pr-review"]);

    std::fs::write(workspace.path().join("README.md"), "feature\n").expect("write feature file");
    git(&workspace, &["add", "."]);
    git(&workspace, &["commit", "-m", "feature"]);

    let mut init = cli_command();
    init.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let bin_dir = workspace.path().join("bin");
    std::fs::create_dir_all(&bin_dir).expect("bin dir");
    let wrapper = bin_dir.join("canon");
    std::fs::write(
        &wrapper,
        format!(
            "#!/usr/bin/env bash\nexec cargo run --quiet --manifest-path '{}' -p canon-cli --bin canon -- \"$@\"\n",
            concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")
        ),
    )
    .expect("write canon wrapper");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755))
        .expect("chmod wrapper");

    let script = workspace
        .path()
        .join(".agents")
        .join("skills")
        .join("canon-shared")
        .join("scripts")
        .join("check-runtime.sh");

    let path = format!("{}:{}", bin_dir.display(), std::env::var("PATH").expect("PATH"));

    let output = ProcessCommand::new("/bin/bash")
        .arg(&script)
        .args([
            "--command",
            "pr-review",
            "--repo-root",
            workspace.path().to_str().expect("workspace path"),
            "--require-init",
            "--owner",
            "reviewer",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--ref",
            "origin/main",
            "--ref",
            "HEAD",
        ])
        .env("PATH", path)
        .current_dir(workspace.path())
        .output()
        .expect("run preflight script");

    assert!(
        output.status.success(),
        "preflight failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("STATUS=ready"), "preflight should be ready: {stdout}");
    assert!(
        stdout.contains("NORMALIZED_REF_1=refs/remotes/origin/main"),
        "preflight should normalize remote ref: {stdout}"
    );
    assert!(stdout.contains("NORMALIZED_REF_2=HEAD"), "preflight should keep HEAD: {stdout}");
}

#[cfg(unix)]
#[test]
fn discovery_preflight_rejects_inputs_under_canon_artifacts() {
    use std::os::unix::fs::PermissionsExt;

    let workspace = TempDir::new().expect("temp dir");
    git(&workspace, &["init", "-b", "main"]);

    let mut init = cli_command();
    init.current_dir(workspace.path()).args(["init", "--ai", "codex"]).assert().success();

    let generated_input = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join("seed-run")
        .join("discovery")
        .join("decision-pressure-points.md");
    std::fs::create_dir_all(generated_input.parent().expect("artifact parent"))
        .expect("artifact dir");
    std::fs::write(
        &generated_input,
        "# Generated Discovery Artifact\n\nThis should not pass preflight.\n",
    )
    .expect("generated artifact file");

    let bin_dir = workspace.path().join("bin");
    std::fs::create_dir_all(&bin_dir).expect("bin dir");
    let wrapper = bin_dir.join("canon");
    std::fs::write(
        &wrapper,
        format!(
            "#!/usr/bin/env bash\nexec cargo run --quiet --manifest-path '{}' -p canon-cli --bin canon -- \"$@\"\n",
            concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")
        ),
    )
    .expect("write canon wrapper");
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755))
        .expect("chmod wrapper");

    let script = workspace
        .path()
        .join(".agents")
        .join("skills")
        .join("canon-shared")
        .join("scripts")
        .join("check-runtime.sh");

    let path = format!("{}:{}", bin_dir.display(), std::env::var("PATH").expect("PATH"));

    let output = ProcessCommand::new("/bin/bash")
        .arg(&script)
        .args([
            "--command",
            "discovery",
            "--repo-root",
            workspace.path().to_str().expect("workspace path"),
            "--require-init",
            "--owner",
            "researcher",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--input",
            ".canon/artifacts/seed-run/discovery/decision-pressure-points.md",
        ])
        .env("PATH", path)
        .current_dir(workspace.path())
        .output()
        .expect("run preflight script");

    assert_eq!(output.status.code(), Some(17), "preflight should fail with invalid-input");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("STATUS=invalid-input"),
        "preflight should reject .canon input: {stdout}"
    );
    assert!(
        stdout.contains("cannot be used as authored input for discovery"),
        "preflight should explain the .canon restriction: {stdout}"
    );
}
