use std::fs;
use std::process::Command as ProcessCommand;

use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::SystemContext;
use tempfile::TempDir;

#[path = "support/historical_run.rs"]
mod historical_run;

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

fn init_debugging_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    fs::write(workspace.path().join("src/main.rs"), "fn main() { println!(\"Hello\"); }\n")
        .expect("source file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed debugging repo"]);
}

fn debugging_brief() -> &'static str {
    "# Debugging Brief\n\n## Context Map\n\nDefect in validation.\n\n## Defect Description\n\nApp crashes on null input.\n\n## Stakeholder Impact\n\nUser can't login.\n\n## Reproduction Harness\n\nSteps:\n1. Run with null.\n2. Crash.\n\n## Red State Verification\n\nVerified it fails.\n\n## Root Cause Isolation\n\nMissing null check.\n\n## Fault Chain\n\nNull passed to unwrap.\n\n## Isolation Proof\n\nStack trace shows it.\n\n## Fix Application\n\nAdd if null.\n\n## Bounded Changes\n\nOnly in validator.rs.\n\n## Invariant Preservation\n\nNo other changes.\n\n## Verification Summary\n\nTests pass.\n\n## Green State\n\nWorks with null.\n\n## No Regression Evidence\n\nAll existing tests pass.\n\nOwner: maintainer\nRisk Level: low-impact\nZone: green\n"
}

#[test]
fn run_debugging_completes_when_context_is_fully_described() {
    let workspace = TempDir::new().expect("temp dir");
    init_debugging_repo(&workspace);
    let brief_path = workspace.path().join("debugging.md");
    fs::write(&brief_path, debugging_brief()).expect("brief file");

    let summary = historical_run::start(
        workspace.path(),
        Mode::Debugging,
        RiskClass::LowImpact,
        UsageZone::Green,
        SystemContext::Existing,
        "maintainer",
        "debugging.md",
    )
    .expect("internal historical debugging run");
    let json = serde_json::to_value(summary).expect("json output");
    assert_eq!(json["state"], "Completed");
}

#[test]
fn run_debugging_blocks_when_required_sections_are_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_debugging_repo(&workspace);
    let brief_path = workspace.path().join("debugging.md");
    // Missing body for Defect Description
    let bad_brief = "# Debugging Brief\n\n## Context Map\n\nDefect in validation.\n\n## Defect Description\n\n## Stakeholder Impact\n\nUser can't login.\n\n## Reproduction Harness\n\nSteps:\n1. Run with null.\n2. Crash.\n\n## Red State Verification\n\nVerified it fails.\n\n## Root Cause Isolation\n\nMissing null check.\n\n## Fault Chain\n\nNull passed to unwrap.\n\n## Isolation Proof\n\nStack trace shows it.\n\n## Fix Application\n\nAdd if null.\n\n## Bounded Changes\n\nOnly in validator.rs.\n\n## Invariant Preservation\n\nNo other changes.\n\n## Verification Summary\n\nTests pass.\n\n## Green State\n\nWorks with null.\n\n## No Regression Evidence\n\nAll existing tests pass.\n\nOwner: maintainer\nRisk Level: low-impact\nZone: green\n";
    fs::write(&brief_path, bad_brief).expect("brief file");

    let summary = historical_run::start(
        workspace.path(),
        Mode::Debugging,
        RiskClass::LowImpact,
        UsageZone::Green,
        SystemContext::Existing,
        "maintainer",
        "debugging.md",
    )
    .expect("internal historical debugging run");
    let json = serde_json::to_value(summary).expect("json output");
    assert_eq!(json["state"], "Blocked");
}

#[test]
fn run_debugging_blocks_when_missing_other_artifacts() {
    let workspace = TempDir::new().expect("temp dir");
    init_debugging_repo(&workspace);
    let brief_path = workspace.path().join("debugging.md");
    // Missing all artifacts except context map by removing their headings
    let bad_brief = "# Debugging Brief\n\n## Context Map\n\nDefect in validation.\n\n## Defect Description\n\nApp crashes on null input.\n\n## Stakeholder Impact\n\nUser can't login.\n\nOwner: maintainer\nRisk Level: low-impact\nZone: green\n";
    fs::write(&brief_path, bad_brief).expect("brief file");

    let summary = historical_run::start(
        workspace.path(),
        Mode::Debugging,
        RiskClass::LowImpact,
        UsageZone::Green,
        SystemContext::Existing,
        "maintainer",
        "debugging.md",
    )
    .expect("internal historical debugging run");
    let json = serde_json::to_value(summary).expect("json output");
    assert_eq!(json["state"], "Blocked");
}
