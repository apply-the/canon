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

fn blocked_brief() -> &'static str {
    "# Change Brief\n\nSystem Slice: auth session boundary and persistence layer.\nImplementation Plan: keep the external auth API stable while tightening the persistence boundary.\n"
}

fn complete_brief() -> &'static str {
    "# Change Brief\n\nSystem Slice: auth session boundary and persistence layer.\nLegacy Invariants: session revocation remains eventually consistent and audit log ordering stays stable.\nChange Surface: session repository, auth service, and token cleanup job.\nImplementation Plan: add bounded repository methods and preserve the public auth contract.\nValidation Strategy: contract tests, invariant checks, and rollback rehearsal.\nDecision Record: prefer additive change over normalization to preserve operator expectations.\n"
}

fn markdown_heading_brief() -> &'static str {
    "# Change Brief: Debug Logging for Null Arguments\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for every public function argument that is null. This will help diagnose edge cases and support troubleshooting in the schema validation pipeline.\n\n## Legacy Invariants\n- API compatibility must be maintained\n- Existing behavior and requirements must not change\n- Output format must remain unchanged\n\n## Change Surface\n- Public functions in schema validation module\n- Debug-level log statements only (non-intrusive)\n- No changes to function signatures or return types\n\n## Validation Strategy\n- Unit tests using JUnit5 and Mockito\n- Verify debug logs are emitted for null arguments\n- Ensure no performance degradation\n- Confirm null argument handling remains correct\n\n## Owner\nLead Eng\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n"
}

#[test]
fn run_change_change_blocks_when_preservation_artifacts_are_incomplete() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("change.md");
    fs::write(&brief_path, blocked_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
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

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let run_root =
        canon_engine::persistence::layout::ProjectLayout::new(workspace.path()).run_dir(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("change");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Change Surface"));
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/change/change-surface.md"))
    );
    assert!(
        json["mode_result"]["headline"]
            .as_str()
            .is_some_and(|value| value.contains("missing-context marker"))
    );
    assert!(run_root.join("run.toml").exists(), "run manifest should exist");
    assert!(run_root.join("artifact-contract.toml").exists(), "artifact contract should exist");
    assert!(
        run_root.join("gates").join("change-preservation.toml").exists(),
        "change preservation gate should be persisted"
    );
    assert!(
        artifact_root.join("legacy-invariants.md").exists(),
        "legacy invariants artifact should exist"
    );
    assert!(
        artifact_root.join("change-surface.md").exists(),
        "change surface artifact should exist"
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
        serde_json::from_slice(&status_output).expect("status json output");
    assert_eq!(status_json["state"], "Blocked");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Change Surface")
    );
}

#[test]
fn run_change_change_completes_when_context_is_fully_described() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("change.md");
    fs::write(&brief_path, complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "maintainer",
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

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let run_root =
        canon_engine::persistence::layout::ProjectLayout::new(workspace.path()).run_dir(run_id);
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("change");

    assert_eq!(json["state"], "Completed");
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Change Surface"));
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/change/change-surface.md"))
    );
    assert!(run_root.join("inputs").is_dir(), "input snapshot directory should exist");
    assert!(
        run_root.join("inputs").join("input-00-change.md").exists(),
        "authored input snapshot should exist"
    );

    for artifact in [
        "system-slice.md",
        "legacy-invariants.md",
        "change-surface.md",
        "implementation-plan.md",
        "validation-strategy.md",
        "decision-record.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the change bundle"
        );
    }

    let context_toml = fs::read_to_string(run_root.join("context.toml")).expect("context file");
    let context: toml::Value = toml::from_str(&context_toml).expect("context toml");
    let fingerprint = context["input_fingerprints"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("input fingerprint");
    let expected_snapshot_ref = format!("runs/{run_id}/inputs/input-00-change.md");
    assert!(
        fingerprint["content_digest_sha256"].as_str().is_some_and(|value| !value.is_empty()),
        "input fingerprint should include a content digest"
    );
    assert_eq!(
        fingerprint["snapshot_ref"].as_str(),
        Some(expected_snapshot_ref.as_str()),
        "input fingerprint should reference the persisted snapshot"
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
        serde_json::from_slice(&status_output).expect("status json output");
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Change Surface")
    );
}

#[test]
fn run_change_change_preserves_markdown_brief_structure() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("change.md");
    fs::write(&brief_path, markdown_heading_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "change",
            "--system-context",
            "existing",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "Lead Eng",
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

    let text = String::from_utf8(output).expect("utf8 stdout");
    let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("change");

    let system_slice =
        fs::read_to_string(artifact_root.join("system-slice.md")).expect("system slice artifact");
    assert!(
        system_slice.contains("## Summary\n\n- Bounded slice: `Schema validation`"),
        "summary should be compact instead of repeating the whole brief"
    );
    assert!(
        system_slice.contains(
            "- Intended change: Add debug logging for every public function argument that is null."
        ),
        "summary should include a short statement of the intended change"
    );
    assert!(
        system_slice.contains("- Owner / risk / zone: `Lead Eng` / `low-impact` / `green`"),
        "summary should keep ownership and risk metadata concise"
    );
    assert!(
        system_slice.contains("- Details: [legacy-invariants.md](legacy-invariants.md), [change-surface.md](change-surface.md), [implementation-plan.md](implementation-plan.md), [validation-strategy.md](validation-strategy.md), [decision-record.md](decision-record.md)"),
        "summary should point to the other detail files in the change bundle"
    );
    assert!(
        system_slice.contains("## System Slice\n\nSchema validation"),
        "system slice should be extracted from markdown headings"
    );
    assert!(
        !system_slice.contains("## Summary\n\n# Change Brief: Debug Logging for Null Arguments"),
        "summary should no longer inline the full brief"
    );

    let legacy_invariants = fs::read_to_string(artifact_root.join("legacy-invariants.md"))
        .expect("legacy invariants artifact");
    assert!(
        legacy_invariants.contains("## Legacy Invariants\n\n- API compatibility must be maintained\n- Existing behavior and requirements must not change\n- Output format must remain unchanged"),
        "legacy invariants should preserve bullet lines without collapsing them into a single paragraph"
    );
    assert!(
        !legacy_invariants.contains("## Legacy Invariants\n\n- - API compatibility"),
        "renderer should not prepend an extra bullet to an already bulleted section"
    );

    let change_surface = fs::read_to_string(artifact_root.join("change-surface.md"))
        .expect("change surface artifact");
    assert!(
        change_surface.contains("## Change Surface\n\n- Public functions in schema validation module\n- Debug-level log statements only (non-intrusive)\n- No changes to function signatures or return types"),
        "change surface should preserve multi-line markdown bullets"
    );

    let validation_strategy = fs::read_to_string(artifact_root.join("validation-strategy.md"))
        .expect("validation strategy artifact");
    assert!(
        validation_strategy.contains("## Validation Strategy\n\n- Unit tests using JUnit5 and Mockito\n- Verify debug logs are emitted for null arguments\n- Ensure no performance degradation\n- Confirm null argument handling remains correct"),
        "validation strategy should preserve markdown bullets from the input brief"
    );
}
