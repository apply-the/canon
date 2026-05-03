use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
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

#[test]
fn inspect_clarity_surfaces_targeted_questions_for_requirements_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\n\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\n\nOperators can flash firmware safely over USB with explicit logs.\n",
    )
    .expect("requirements brief");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "requirements.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"target\": \"clarity\""))
        .stdout(contains("\"mode\": \"requirements\""))
        .stdout(contains("\"requires_clarification\": true"))
        .stdout(contains("Which constraints are non-negotiable for this work?"))
        .stdout(contains("What is explicitly out of scope or deferred for this packet?"));
}

#[test]
fn inspect_clarity_recurses_directory_inputs_and_accepts_multiple_paths_in_one_group() {
    let workspace = TempDir::new().expect("temp dir");
    let requirements_dir = workspace.path().join("canon-input").join("requirements");
    let support_dir = requirements_dir.join("supporting");
    fs::create_dir_all(&support_dir).expect("requirements dir");
    fs::write(
        requirements_dir.join("brief.md"),
        "# Requirements Brief\n\n## Problem\n\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\n\nOperators can flash firmware safely over USB with explicit logs.\n",
    )
    .expect("brief");
    fs::write(
        requirements_dir.join("constraints.md"),
        "## Constraints\n\n- USB transport only\n\n## Tradeoffs\n\n- Safety over throughput\n",
    )
    .expect("constraints");
    fs::write(
        support_dir.join("scope-and-questions.md"),
        "## Out of Scope\n\n- No Bluetooth flashing in v1\n\n## Open Questions\n\n- How is the Bird identified over USB?\n",
    )
    .expect("scope questions");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "canon-input/requirements",
            "canon-input/requirements/brief.md",
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
    let source_inputs = json["entries"][0]["source_inputs"]
        .as_array()
        .expect("source inputs array")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();

    assert!(source_inputs.contains(&"canon-input/requirements/brief.md"));
    assert!(source_inputs.contains(&"canon-input/requirements/constraints.md"));
    assert!(source_inputs.contains(&"canon-input/requirements/supporting/scope-and-questions.md"));
    assert_eq!(source_inputs.len(), 3, "source inputs should be de-duplicated");
    assert_eq!(json["entries"][0]["requires_clarification"].as_bool(), Some(true));
    assert_eq!(
        json["entries"][0]["authoring_lifecycle"]["packet_shape"].as_str(),
        Some("directory-backed")
    );
    assert_eq!(
        json["entries"][0]["authoring_lifecycle"]["authority_status"].as_str(),
        Some("explicit-authoritative-brief")
    );
    assert_eq!(
        json["entries"][0]["authoring_lifecycle"]["authoritative_inputs"][0].as_str(),
        Some("canon-input/requirements/brief.md")
    );
    assert!(text.contains("Answer the remaining clarification questions"));
    assert!(text.contains("How is the Bird identified over USB?"));
}

#[test]
fn inspect_clarity_keeps_ambiguous_directory_packets_explicit() {
    let workspace = TempDir::new().expect("temp dir");
    let implementation_dir = workspace.path().join("canon-input").join("implementation");
    fs::create_dir_all(&implementation_dir).expect("implementation dir");
    fs::write(
        implementation_dir.join("source-map.md"),
        "# Source Map\n\n## Upstream Sources\n\n- docs/changes/auth-session.md\n",
    )
    .expect("source map");
    fs::write(
        implementation_dir.join("notes.md"),
        "# Notes\n\n## Supporting Context\n\n- Reuse auth session revocation helper.\n",
    )
    .expect("notes");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "implementation",
            "--input",
            "canon-input/implementation",
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

    assert_eq!(
        json["entries"][0]["authoring_lifecycle"]["authority_status"].as_str(),
        Some("ambiguous-current-brief")
    );
    assert!(
        json["entries"][0]["authoring_lifecycle"]["authoritative_inputs"]
            .as_array()
            .is_some_and(|items| items.is_empty())
    );
    assert!(text.contains("add `brief.md` or reduce the packet to one clear readiness brief"));
}

#[test]
fn inspect_clarity_surfaces_targeted_questions_for_supply_chain_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("supply-chain-analysis.md"),
        "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n\n## Source Inputs\n\n- Cargo.toml\n",
    )
    .expect("supply-chain brief");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "supply-chain-analysis",
            "--input",
            "supply-chain-analysis.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .stdout(contains("\"target\": \"clarity\""))
        .stdout(contains("\"mode\": \"supply-chain-analysis\""))
        .stdout(contains("\"requires_clarification\": true"))
        .stdout(contains("What licensing posture governs this repository surface"))
        .stdout(contains("Are non-OSS scanner proposals allowed"));
}

fn representative_mode_input(mode: &str) -> &'static str {
    match mode {
        "system-shaping" => "docs/examples/canon-input/system-shaping-billing.md",
        "architecture" => "docs/examples/canon-input/architecture-state-management.md",
        "change" => "docs/examples/canon-input/change-add-caching.md",
        "backlog" => "docs/examples/canon-input/backlog-auth-session-hardening.md",
        "implementation" => "docs/examples/canon-input/implementation-auth-session-revocation.md",
        "refactor" => "docs/examples/canon-input/refactor-auth-session-cleanup.md",
        "migration" => "docs/examples/canon-input/migration-platform-consolidation.md",
        "review" => "canon-input/review.md",
        "verification" => "docs/examples/canon-input/verification-e2e-flakiness.md",
        "incident" => "docs/examples/canon-input/incident/brief.md",
        "security-assessment" => {
            "docs/examples/canon-input/security-assessment-webhook-platform.md"
        }
        "system-assessment" => "docs/examples/canon-input/system-assessment-commerce-platform.md",
        other => panic!("unexpected mode {other}"),
    }
}

fn representative_mode_workspace(mode: &str) -> &'static str {
    match mode {
        "review" => concat!(env!("CARGO_MANIFEST_DIR"), "/docs/templates"),
        _ => env!("CARGO_MANIFEST_DIR"),
    }
}

#[test]
fn inspect_clarity_supports_all_file_backed_governed_modes_with_reasoning_signals() {
    let modes = [
        "system-shaping",
        "architecture",
        "change",
        "backlog",
        "implementation",
        "refactor",
        "migration",
        "review",
        "verification",
        "incident",
        "security-assessment",
        "system-assessment",
    ];

    for mode in modes {
        let input_path = representative_mode_input(mode);

        let output = cli_command()
            .current_dir(representative_mode_workspace(mode))
            .args(["inspect", "clarity", "--mode", mode, "--input", input_path, "--output", "json"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let text = String::from_utf8(output).expect("utf8 stdout");
        let json: serde_json::Value = serde_json::from_str(&text).expect("json output");
        assert_eq!(json["entries"][0]["mode"].as_str(), Some(mode));
        assert!(
            json["entries"][0]["summary"]
                .as_str()
                .is_some_and(|summary| !summary.trim().is_empty()),
            "summary missing for {mode}: {text}"
        );
        assert!(
            json["entries"][0]["reasoning_signals"]
                .as_array()
                .is_some_and(|signals| !signals.is_empty()),
            "reasoning signals missing for {mode}: {text}"
        );
    }
}

#[test]
fn inspect_clarity_marks_materially_closed_architecture_briefs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("architecture.md"),
        "# Architecture Brief\n\n## Decision\n\nSeparate review posture from fallback rendering.\n\n## Constraints\n\n- Preserve existing .canon schema\n\n## Recommendation\n\nUse a shared runtime posture helper.\n\n## Decision Drivers\n\n- Shared runtime behavior beats prompt-by-prompt wording patches\n",
    )
    .expect("architecture brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "architecture",
            "--input",
            "architecture.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).expect("utf8 stdout");
    assert!(
        text.contains("materially closes the decision"),
        "expected material closure signal: {text}"
    );
}

#[test]
fn inspect_clarity_reports_output_quality_posture_for_weak_and_strong_packets() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\n\nBuild a bounded USB flashing CLI for the Bird device.\n\n## Outcome\n\nOperators can flash firmware safely over USB with explicit logs.\n",
    )
    .expect("requirements brief");
    fs::write(
        workspace.path().join("architecture.md"),
        "# Architecture Brief\n\n## Decision\n\nSeparate review posture from fallback rendering.\n\n## Constraints\n\n- Preserve existing .canon schema\n\n## Recommendation\n\nUse a shared runtime posture helper.\n\n## Decision Drivers\n\n- Shared runtime behavior beats prompt-by-prompt wording patches\n",
    )
    .expect("architecture brief");

    let weak_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "requirements",
            "--input",
            "requirements.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let weak_json: serde_json::Value =
        serde_json::from_slice(&weak_output).expect("weak clarity json");
    assert_eq!(
        weak_json["entries"][0]["output_quality"]["posture"].as_str(),
        Some("structurally-complete")
    );
    assert!(
        weak_json["entries"][0]["output_quality"]["downgrade_reasons"]
            .as_array()
            .is_some_and(|reasons| !reasons.is_empty())
    );

    let strong_output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "architecture",
            "--input",
            "architecture.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let strong_json: serde_json::Value =
        serde_json::from_slice(&strong_output).expect("strong clarity json");
    assert_eq!(
        strong_json["entries"][0]["output_quality"]["posture"].as_str(),
        Some("publishable")
    );
    assert_eq!(
        strong_json["entries"][0]["output_quality"]["materially_closed"].as_bool(),
        Some(true)
    );
}

#[test]
fn inspect_clarity_rejects_pr_review_mode_as_unsupported() {
    // pr-review is a diff-based mode and clarity inspection does not apply to it
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "clarity", "--mode", "pr-review", "--input", "HEAD~1", "--input", "HEAD"])
        .assert()
        .failure();
}

#[test]
fn inspect_clarity_rejects_empty_input_list() {
    // clarity inspect with no --input and no --input-text should be rejected
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "clarity", "--mode", "requirements"])
        .assert()
        .failure();
}

#[test]
fn inspect_clarity_rejects_nonexistent_input_path_for_requirements() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "clarity", "--mode", "requirements", "--input", "nonexistent-brief.md"])
        .assert()
        .failure();
}

#[test]
fn inspect_clarity_rejects_nonexistent_input_path_for_discovery() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "clarity", "--mode", "discovery", "--input", "nonexistent-discovery.md"])
        .assert()
        .failure();
}

#[test]
fn inspect_risk_zone_rejects_pr_review_with_inline_text() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "risk-zone",
            "--mode",
            "pr-review",
            "--input-text",
            "some inline content",
        ])
        .assert()
        .failure();
}

#[test]
fn inspect_risk_zone_rejects_pr_review_with_single_input_ref() {
    let workspace = TempDir::new().expect("temp dir");

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "risk-zone", "--mode", "pr-review", "--input", "HEAD~1"])
        .assert()
        .failure();
}
