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
    "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Excluded Areas\n\n- payment settlement\n- billing reports\n\n## Implementation Plan\n\nKeep the external auth API stable while tightening the persistence boundary.\n\n## Sequencing\n\n1. Add repository methods behind the existing contract.\n2. Shift callers after invariants stay visible.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- should the cleanup job roll out in the same slice?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n"
}

fn complete_brief() -> &'static str {
    "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Domain Slice\n\nSession lifecycle and cleanup semantics within the auth domain.\n\n## Excluded Areas\n\n- payment settlement\n- billing reports\n\n## Intended Change\n\nAdd bounded repository methods while preserving the public auth contract.\n\n## Legacy Invariants\n\n- session revocation remains eventually consistent\n- audit log ordering stays stable\n\n## Domain Invariants\n\n- a revoked session must never become active again through cleanup retries\n- audit trails must preserve causal order across repository updates\n\n## Forbidden Normalization\n\n- Do not collapse audit-ordering quirks that operators still rely on.\n\n## Change Surface\n\n- session repository\n- auth service\n- token cleanup job\n\n## Ownership\n\n- primary owner: maintainer\n\n## Cross-Context Risks\n\n- cleanup scheduling can leak into notification flows if repository boundaries widen\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n\n## Sequencing\n\n1. Add bounded repository methods.\n2. Switch callers behind the preserved contract.\n\n## Validation Strategy\n\n- contract tests\n- invariant checks\n\n## Independent Checks\n\n- rollback rehearsal by a separate operator\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Decision Drivers\n\n- Preserve operator expectations.\n- Keep the auth contract stable during the bounded repository change.\n\n## Options Considered\n\n- Option 1 keeps the additive repository helper inside the auth boundary.\n- Option 2 normalizes scheduling and cleanup behavior in the same slice.\n\n## Decision Evidence\n\n- Existing operator workflows still depend on the current auth cleanup ordering.\n- Contract tests already guard the preserved API surface.\n\n## Boundary Tradeoffs\n\n- keep cleanup logic inside the auth boundary even if that duplicates some scheduling code\n\n## Recommendation\n\n- Start with the additive repository helper and defer normalization to a later slice.\n\n## Why Not The Others\n\n- Normalizing cleanup behavior now would widen the change surface beyond the bounded auth slice.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Unresolved Questions\n\n- should the cleanup job roll out in the same slice?\n\nOwner: maintainer\nRisk Level: bounded-impact\nZone: yellow\n"
}

fn markdown_heading_brief() -> &'static str {
    "# Change Brief: Debug Logging for Null Arguments\n\n## System Slice\nSchema validation\n\n## Domain Slice\nNull-handling and diagnostic visibility within the schema validation boundary.\n\n## Excluded Areas\n- parser changes\n- output format changes\n\n## Intended Change\nAdd debug logging for every public function argument that is null. This will help diagnose edge cases and support troubleshooting in the schema validation pipeline.\n\n## Legacy Invariants\n- API compatibility must be maintained\n- Existing behavior and requirements must not change\n- Output format must remain unchanged\n\n## Domain Invariants\n- Null arguments must still follow the same validation and error semantics after logging is added.\n\n## Forbidden Normalization\n- Do not normalize away required null-handling semantics.\n\n## Change Surface\n- Public functions in schema validation module\n- Debug-level log statements only (non-intrusive)\n- No changes to function signatures or return types\n\n## Ownership\n- Primary owner: Lead Eng\n\n## Cross-Context Risks\n- Debug logging must not leak into parser or output-format boundaries.\n\n## Implementation Plan\nAdd debug logging only within the schema validation module.\n\n## Sequencing\n1. Add logging behind the existing API.\n2. Validate logs before rollout.\n\n## Validation Strategy\n- Unit tests using JUnit5 and Mockito\n- Verify debug logs are emitted for null arguments\n- Ensure no performance degradation\n- Confirm null argument handling remains correct\n\n## Independent Checks\n- Separate log assertions from behavior assertions.\n\n## Decision Record\nPrefer additive logging over behavioral changes.\n\n## Decision Drivers\n- Preserve API compatibility while increasing diagnostic visibility.\n- Keep schema-validation diagnostics inside the existing module boundary.\n\n## Options Considered\n- Option 1 adds debug logging only to the schema-validation public functions.\n- Option 2 broadens the slice into parser or output-format changes.\n\n## Decision Evidence\n- Existing bug reports point to null-handling ambiguity in schema validation.\n- The current contract already depends on stable output semantics.\n\n## Boundary Tradeoffs\n- Keep diagnostics inside schema validation even if that duplicates some logging patterns used elsewhere.\n\n## Recommendation\n- Add bounded debug logging inside schema validation and defer broader diagnostics work.\n\n## Why Not The Others\n- Expanding into parser or output-format changes would widen the slice beyond the bounded diagnostics need.\n\n## Consequences\n- The public contract stays stable while diagnostics improve.\n\n## Unresolved Questions\n- Should logging stay debug-only in production?\n\n## Owner\nLead Eng\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n"
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

    let system_slice =
        fs::read_to_string(artifact_root.join("system-slice.md")).expect("system slice artifact");
    assert!(system_slice.contains("## Domain Slice"));

    let legacy_invariants = fs::read_to_string(artifact_root.join("legacy-invariants.md"))
        .expect("legacy invariants artifact");
    assert!(legacy_invariants.contains("## Domain Invariants"));

    let change_surface = fs::read_to_string(artifact_root.join("change-surface.md"))
        .expect("change surface artifact");
    assert!(change_surface.contains("## Cross-Context Risks"));

    let decision_record = fs::read_to_string(artifact_root.join("decision-record.md"))
        .expect("decision record artifact");
    assert!(decision_record.contains("## Boundary Tradeoffs"));
    assert!(decision_record.contains("## Decision Drivers"));
    assert!(decision_record.contains("## Options Considered"));
    assert!(decision_record.contains("## Decision Evidence"));
    assert!(decision_record.contains("## Recommendation"));
    assert!(decision_record.contains("## Why Not The Others"));
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
