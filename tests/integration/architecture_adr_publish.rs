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

fn architecture_brief() -> &'static str {
    "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Risks\n- Context crossings may be hidden inside summary prose.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Deployment\n- `canon-cli` runs on developer laptops and CI runners.\n- `canon-engine` shares the same Rust process boundary as the CLI.\n- `.canon/` remains the local runtime store on the active workspace filesystem.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
}

#[test]
fn architecture_publish_emits_a_standard_adr_by_default() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("architecture.md");
    fs::write(&brief_path, architecture_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "architecture",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "staff-architect",
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

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");

    let publish_output = cli_command()
        .current_dir(workspace.path())
        .args(["publish", run_id])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let publish_text = String::from_utf8(publish_output).expect("utf8 publish output");
    let adr_dir = workspace.path().join("docs").join("adr");
    let adr_entry = fs::read_dir(&adr_dir)
        .expect("adr registry dir")
        .next()
        .expect("adr entry")
        .expect("adr dir entry");
    let adr_name = adr_entry.file_name().to_string_lossy().to_string();
    let adr_path = adr_entry.path();
    let adr_text = fs::read_to_string(&adr_path).expect("generated adr");

    assert!(adr_name.starts_with("ADR-0001-"));
    assert!(publish_text.contains(&format!("docs/adr/{adr_name}")));
    assert!(adr_text.starts_with(
        "# ADR 0001: Use a dedicated context map to make architecture boundaries reviewable."
    ));
    assert!(adr_text.contains("**Date:** "));
    assert!(adr_text.contains("**Status:** Accepted"));
    assert!(adr_text.contains("## Context"));
    assert!(adr_text.contains(
        "Decision focus: map boundaries and tradeoffs for governed analysis-mode expansion."
    ));
    assert!(adr_text.contains("## Decision"));
    assert!(
        adr_text
            .contains("Use a dedicated context map to make architecture boundaries reviewable.")
    );
    assert!(adr_text.contains("## Consequences"));
    assert!(adr_text.contains(
        "The emitted packet records the chosen option and rejected alternatives explicitly."
    ));
    assert!(adr_text.contains("## Alternatives Considered"));
    assert!(adr_text.contains("Keep the current generic decision summary."));
}
