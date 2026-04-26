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

fn complete_requirements_brief(problem: &str, outcome: &str) -> String {
    format!(
        "# Requirements Brief\n\n## Problem\n\n{problem}\n\n## Outcome\n\n{outcome}\n\n## Constraints\n\n- Keep evidence durable\n- Preserve explicit ownership\n\n## Non-Negotiables\n\n- Persist artifacts under `.canon/`\n- Keep review inputs local-first\n\n## Options\n\n1. Deliver the bounded packet first.\n2. Defer broader orchestration.\n\n## Recommended Path\n\nDeliver the bounded packet first.\n\n## Tradeoffs\n\n- Structure before speed\n\n## Consequences\n\n- Evidence remains inspectable.\n\n## Out of Scope\n\n- No hosted control plane\n\n## Deferred Work\n\n- Hosted coordination can wait.\n\n## Decision Checklist\n\n- [x] Owner is explicit\n\n## Open Questions\n\n- Which mode should consume this packet next?\n"
    )
}

#[test]
fn requirements_artifacts_record_provenance_and_link_to_evidence() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("idea.md"),
        complete_requirements_brief(
            "Leave durable evidence for requirements framing.",
            "Maintainers can inspect provenance-linked requirements artifacts.",
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
            "idea.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: serde_json::Value = serde_json::from_slice(&output).expect("json");
    let run_id = json["run_id"].as_str().expect("run id");

    let manifest = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("requirements")
            .join("manifest.toml"),
    )
    .expect("artifact manifest");
    assert!(manifest.contains("provenance"), "artifact manifest should carry provenance");
    assert!(
        manifest.contains(&format!("runs/{run_id}/evidence.toml")),
        "artifact provenance should link back to the run evidence bundle"
    );

    let evidence = cli_command()
        .current_dir(workspace.path())
        .args(["inspect", "evidence", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let evidence_json: serde_json::Value = serde_json::from_slice(&evidence).expect("json");
    let entry = evidence_json["entries"]
        .as_array()
        .and_then(|entries| entries.first())
        .expect("evidence entry");
    assert!(
        entry["generation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "generation paths should be inspectable"
    );
    assert!(
        entry["validation_paths"].as_array().is_some_and(|paths| !paths.is_empty()),
        "validation paths should be inspectable"
    );
}
