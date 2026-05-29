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
fn run_architecture_starts_draft_refinement_and_materializes_working_brief() {
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
    let refinement = json["refinement_state"].as_object().expect("refinement state");
    let working_brief_path = refinement["working_brief_path"].as_str().expect("working brief path");
    let working_brief =
        fs::read_to_string(workspace.path().join(working_brief_path)).expect("working brief");

    assert_eq!(json["state"], "Draft");
    assert_eq!(json["artifact_count"], 0);
    assert_eq!(json["invocations_total"], 0);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(refinement["workflow_family"], "planning");
    assert_eq!(refinement["current_mode"], "architecture");
    assert_eq!(refinement["status"], "active");
    assert_eq!(refinement["explicit_continuation_required"], true);
    assert!(working_brief.contains("# Architecture Brief"));
    assert!(working_brief.contains("## Clarification Provenance"));
    assert!(working_brief.contains("## Readiness Delta"));

    let status_output = cli_command()
        .current_dir(workspace.path())
        .args(["status", "--run", run_id, "--output", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let status_json: serde_json::Value =
        serde_json::from_slice(&status_output).expect("status json");
    assert_eq!(status_json["state"], "Draft");
    assert_eq!(status_json["refinement_state"]["current_mode"], "architecture");
}

#[test]
fn architecture_run_enters_awaiting_approval_for_systemic_and_red_zone_cases() {
    for (risk, zone) in [("systemic-impact", "yellow"), ("bounded-impact", "red")] {
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
                risk,
                "--zone",
                zone,
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
        assert_eq!(json["state"], "Draft");

        cli_command()
            .current_dir(workspace.path())
            .args(["resume", "--run", run_id])
            .assert()
            .code(3);

        let status_output = cli_command()
            .current_dir(workspace.path())
            .args(["status", "--run", run_id, "--output", "json"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        let status_json: serde_json::Value =
            serde_json::from_slice(&status_output).expect("status json");

        assert_eq!(status_json["state"], "AwaitingApproval");
        assert_eq!(status_json["blocking_classification"], "approval-gated");
        assert!(
            status_json["approval_targets"]
                .as_array()
                .is_some_and(|targets| targets.iter().any(|target| target == "gate:risk")),
            "{risk}/{zone} architecture run should surface gate:risk approval"
        );
    }
}
