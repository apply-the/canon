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
fn architecture_publish_uses_next_non_conflicting_adr_number_and_keeps_registry_fixed() {
    let workspace = TempDir::new().expect("temp dir");
    let brief_path = workspace.path().join("architecture.md");
    fs::write(&brief_path, architecture_brief()).expect("brief file");

    let adr_dir = workspace.path().join("docs").join("adr");
    fs::create_dir_all(&adr_dir).expect("adr dir");
    fs::write(adr_dir.join("ADR-0001-existing-decision.md"), "# ADR 0001: Existing\n")
        .expect("existing adr 1");
    fs::write(adr_dir.join("ADR-0003-existing-gap.md"), "# ADR 0003: Existing gap\n")
        .expect("existing adr 3");

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
            "architecture.md",
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
    let override_dir = workspace.path().join("docs").join("published").join("architecture-packet");

    let publish_output = cli_command()
        .current_dir(workspace.path())
        .args(["publish", run_id, "--to", "docs/published/architecture-packet"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let publish_text = String::from_utf8(publish_output).expect("utf8 publish output");
    let mut adr_names = fs::read_dir(&adr_dir)
        .expect("adr registry dir")
        .map(|entry| entry.expect("adr dir entry").file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    adr_names.sort();

    assert_eq!(adr_names.len(), 3);
    assert!(adr_names.iter().any(|name| name == "ADR-0001-existing-decision.md"));
    assert!(adr_names.iter().any(|name| name == "ADR-0003-existing-gap.md"));
    let generated =
        adr_names.iter().find(|name| name.starts_with("ADR-0004-")).expect("generated adr number");

    assert!(override_dir.join("01-architecture-overview.md").exists());
    assert!(publish_text.contains(&format!("docs/adr/{generated}")));
    assert!(!override_dir.join(generated).exists());
}
