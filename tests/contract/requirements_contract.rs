use std::fs;

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

fn complete_requirements_brief(problem: &str, outcome: &str) -> String {
    format!(
        "# Requirements Brief\n\n## Problem\n\n{problem}\n\n## Outcome\n\n{outcome}\n\n## Constraints\n\n- USB-only execution path\n- Preserve explicit audit logs\n\n## Non-Negotiables\n\n- Keep human ownership explicit\n- Persist artifacts under `.canon/`\n\n## Options\n\n1. Start with a bounded CLI slice.\n2. Defer broader orchestration work.\n\n## Recommended Path\n\nStart with the bounded CLI slice before expanding scope.\n\n## Tradeoffs\n\n- Governance adds upfront structure.\n- Durable artifacts add overhead but improve reviewability.\n\n## Consequences\n\n- Reviewers can inspect the packet without chat history.\n\n## Out of Scope\n\n- No GUI in this slice\n\n## Deferred Work\n\n- Hosted rollout remains a later slice.\n\n## Decision Checklist\n\n- [x] Scope is explicit\n- [x] Ownership is explicit\n\n## Open Questions\n\n- How is bootloader mode entered?\n"
    )
}

fn requirements_generation_approval_target(run_id: &str) -> String {
    format!("invocation:{run_id}-generate")
}

fn complete_requirements_flow(workspace: &TempDir) -> String {
    let idea_path = workspace.path().join("idea.md");
    fs::write(
        &idea_path,
        complete_requirements_brief(
            "Bound AI-assisted engineering work with explicit governance.",
            "Operators can review a complete packet before downstream planning.",
        ),
    )
    .expect("idea file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "product-lead",
            "--input",
            idea_path.file_name().expect("file name").to_str().expect("utf8"),
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
    let run_id = json["run_id"].as_str().expect("run id").to_string();

    cli_command().current_dir(workspace.path()).args(["resume", "--run", &run_id]).assert().code(3);

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            &run_id,
            "--target",
            &requirements_generation_approval_target(&run_id),
            "--by",
            "principal-engineer",
            "--decision",
            "approve",
            "--rationale",
            "Requirements generation may proceed after review.",
        ])
        .assert()
        .success();

    cli_command()
        .current_dir(workspace.path())
        .args(["resume", "--run", &run_id])
        .assert()
        .success();

    run_id
}

#[test]
fn inspect_artifacts_lists_the_requirements_bundle() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = complete_requirements_flow(&workspace);

    let inspect_output = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id, "--output", "json"])
        .assert()
        .success()
        .stdout(contains("prd.md"))
        .stdout(contains("decision-checklist.md"))
        .get_output()
        .stdout
        .clone();

    let inspect_text = String::from_utf8(inspect_output).expect("utf8 stdout");
    let inspect_json: serde_json::Value = serde_json::from_str(&inspect_text).expect("json output");
    let entries = inspect_json["entries"].as_array().expect("artifact entries");
    let actual_paths =
        entries.iter().map(|entry| entry.as_str().expect("artifact path")).collect::<Vec<_>>();
    let expected_paths = vec![
        format!(".canon/artifacts/{run_id}/requirements/01-problem-statement.md"),
        format!(".canon/artifacts/{run_id}/requirements/02-constraints.md"),
        format!(".canon/artifacts/{run_id}/requirements/03-options.md"),
        format!(".canon/artifacts/{run_id}/requirements/04-tradeoffs.md"),
        format!(".canon/artifacts/{run_id}/requirements/05-scope-cuts.md"),
        format!(".canon/artifacts/{run_id}/requirements/06-decision-checklist.md"),
        format!(".canon/artifacts/{run_id}/requirements/07-prd.md"),
        format!(".canon/artifacts/{run_id}/requirements/packet-metadata.json"),
    ];
    assert_eq!(actual_paths, expected_paths);

    cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "artifacts", "--run", &run_id])
        .assert()
        .success()
        .stdout(contains("# artifacts"))
        .stdout(contains(format!("Run ID: {run_id}")))
        .stdout(contains(format!(".canon/artifacts/{run_id}/requirements/07-prd.md")))
        .stdout(contains(format!(
            ".canon/artifacts/{run_id}/requirements/01-problem-statement.md"
        )));

    let contract_path = canon_engine::persistence::layout::ProjectLayout::new(workspace.path())
        .run_dir(&run_id)
        .join("artifact-contract.toml");
    let contract_toml = fs::read_to_string(contract_path).expect("artifact contract");
    assert_eq!(
        contract_toml.trim(),
        include_str!("snapshots/requirements_artifact_contract.toml").trim()
    );
}

#[test]
fn run_requirements_markdown_surfaces_draft_summary_without_raw_json() {
    let workspace = TempDir::new().expect("temp dir");
    let idea_path = workspace.path().join("idea.md");
    fs::write(
        &idea_path,
        complete_requirements_brief(
            "Build a bounded USB flashing CLI for the Bird device.",
            "Operators can flash firmware safely over USB with explicit logs.",
        ),
    )
    .expect("idea file");

    cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "requirements",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "product-lead",
            "--input",
            idea_path.file_name().expect("file name").to_str().expect("utf8"),
            "--output",
            "markdown",
        ])
        .assert()
        .success()
        .stdout(contains("# run"))
        .stdout(contains("Mode: requirements"))
        .stdout(contains("State: Draft"))
        .stdout(predicates::str::contains("## Result").not())
        .stdout(predicates::str::contains("Primary Artifact:").not())
        .stdout(predicates::str::contains("## Recommended Next Step").not())
        .stdout(predicates::str::is_match("\\\"run_id\\\"").expect("regex").not());
}
