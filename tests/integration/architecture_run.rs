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
    "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Risks\n- Context crossings may be hidden inside summary prose.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
}

#[test]
fn run_architecture_persists_a_completed_run_and_artifact_bundle() {
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
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("architecture");

    assert_eq!(json["state"], "Completed");
    assert_eq!(json["invocations_total"], 3);
    assert_eq!(json["invocations_pending_approval"], 0);
    assert_eq!(
        json["mode_result"]["headline"],
        "Architecture packet ready for downstream implementation or review."
    );
    assert_eq!(
        json["mode_result"]["result_excerpt"],
        "Use a dedicated context map to make architecture boundaries reviewable."
    );
    assert_eq!(
        json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Architecture Decisions")
    );
    assert!(
        json["mode_result"]["primary_artifact_path"]
            .as_str()
            .is_some_and(|value| value.ends_with("/architecture/architecture-decisions.md"))
    );

    for artifact in [
        "architecture-decisions.md",
        "invariants.md",
        "tradeoff-matrix.md",
        "boundary-map.md",
        "context-map.md",
        "readiness-assessment.md",
    ] {
        assert!(
            artifact_root.join(artifact).exists(),
            "{artifact} should exist in the architecture bundle"
        );
    }

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
    assert_eq!(status_json["state"], "Completed");
    assert_eq!(
        status_json["mode_result"]["primary_artifact_title"].as_str(),
        Some("Architecture Decisions")
    );

    let context_map =
        fs::read_to_string(artifact_root.join("context-map.md")).expect("context map");
    assert!(context_map.starts_with("# Context Map\n\n## Summary\n\nDecision focus:"));
    assert!(context_map.contains("## Bounded Contexts"));
    assert!(context_map.contains("## Shared Invariants"));
    assert!(
        !context_map.contains("# Architecture Brief"),
        "context-map.md should render canonical sections instead of dumping the full authored brief"
    );

    let decisions = fs::read_to_string(artifact_root.join("architecture-decisions.md"))
        .expect("architecture decisions");
    assert!(decisions.starts_with("# Architecture Decisions\n\n## Summary\n\nDecision focus:"));
    assert!(decisions.contains("## Decision"));
    assert!(decisions.contains("## Decision Drivers"));
    assert!(decisions.contains("## Recommendation"));
    assert!(decisions.contains("## Consequences"));
    assert!(!decisions.contains("# Architecture Brief"));

    let tradeoff_matrix =
        fs::read_to_string(artifact_root.join("tradeoff-matrix.md")).expect("tradeoff matrix");
    assert!(tradeoff_matrix.starts_with("# Tradeoff Matrix\n\n## Summary\n\nDecision focus:"));
    assert!(tradeoff_matrix.contains("## Options Considered"));
    assert!(tradeoff_matrix.contains("## Pros"));
    assert!(tradeoff_matrix.contains("## Cons"));
    assert!(tradeoff_matrix.contains("## Why Not The Others"));
    assert!(!tradeoff_matrix.contains("# Architecture Brief"));
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
            .code(3)
            .get_output()
            .stdout
            .clone();

        let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
        assert_eq!(json["state"], "AwaitingApproval");
        assert_eq!(json["blocking_classification"], "approval-gated");
        assert!(
            json["approval_targets"]
                .as_array()
                .is_some_and(|targets| targets.iter().any(|target| target == "gate:risk")),
            "{risk}/{zone} architecture run should surface gate:risk approval"
        );
    }
}
