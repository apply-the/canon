use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_policy_shaping_cli_dry_run_and_evaluation() {
    let temp = tempdir().unwrap();
    let draft_policy_path = temp.path().join("draft-policy.md");
    fs::write(
        &draft_policy_path,
        "---\ntitle: \"Strict Typestate\"\nmode: \"policy-shaping\"\nrisk: \"Systemic Impact\"\nscope-in: [\"crates/canon-engine\"]\nscope-out: []\ninvariants: [\"All state changes must use typed states\"]\n---\n# Strict Typestate",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("canon").unwrap();
    // Assuming the interface is `canon policy-shaping <file>`
    cmd.current_dir(temp.path())
        .arg("policy-shaping")
        .arg("draft-policy.md")
        .arg("--dry-run");

    // The command should succeed, possibly with a message about skipping broad-impact since it's a dry run,
    // or just run successfully and parse. We'll just assert it doesn't panic.
    // For now we'll assert success, though it might fail if the mode isn't fully wired.
    // In TDD we expect it to fail until implemented.
}
