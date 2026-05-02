use std::fs;

use assert_cmd::Command;
use canon_engine::persistence::layout::ProjectLayout;
use tempfile::TempDir;

fn cli_command() -> Command {
    if let Some(binary) = std::env::var_os("CARGO_BIN_EXE_canon") {
        return Command::new(binary);
    }

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
        "# Requirements Brief\n\n## Problem\n\n{problem}\n\n## Outcome\n\n{outcome}\n\n## Constraints\n\n- Keep execution local-first\n- Preserve explicit approval checkpoints\n\n## Non-Negotiables\n\n- Persist evidence under `.canon/`\n- Keep named ownership explicit\n\n## Options\n\n1. Deliver the bounded packet first.\n2. Defer broader rollout.\n\n## Recommended Path\n\nDeliver the bounded packet first.\n\n## Tradeoffs\n\n- Structure before speed\n\n## Consequences\n\n- Reviewers can inspect durable artifacts.\n\n## Out of Scope\n\n- No hosted control plane in this slice\n\n## Deferred Work\n\n- Hosted coordination remains later work.\n\n## Decision Checklist\n\n- [x] Scope is explicit\n- [x] Ownership is explicit\n\n## Open Questions\n\n- Which downstream mode should consume the packet first?\n"
    )
}

fn incomplete_requirements_brief() -> &'static str {
    "# Requirements Brief\n\n## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface.\n"
}

fn complete_architecture_brief() -> &'static str {
    "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Risks\n- Context crossings may be hidden inside summary prose.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
}

fn governance_start_request(
    workspace: &TempDir,
    mode: &str,
    system_context: &str,
    risk: &str,
    zone: &str,
    owner: &str,
    input_path: Option<&str>,
) -> serde_json::Value {
    let mut request = serde_json::json!({
        "request_kind": "start",
        "governance_attempt_id": "ga-start-001",
        "stage_key": "analysis",
        "goal": "Create a governed packet",
        "workspace_ref": workspace.path().to_string_lossy().to_string(),
        "mode": mode,
        "system_context": system_context,
        "risk": risk,
        "zone": zone,
        "owner": owner
    });

    if let Some(input_path) = input_path {
        request["input_documents"] = serde_json::json!([{ "path": input_path }]);
    }

    request
}

fn governance_refresh_request(
    workspace: &TempDir,
    run_ref: &str,
    mode: &str,
    system_context: &str,
    risk: &str,
    zone: &str,
    owner: &str,
) -> serde_json::Value {
    serde_json::json!({
        "request_kind": "refresh",
        "governance_attempt_id": "ga-refresh-001",
        "stage_key": "verification",
        "goal": "Refresh the governed packet",
        "workspace_ref": workspace.path().to_string_lossy().to_string(),
        "mode": mode,
        "system_context": system_context,
        "risk": risk,
        "zone": zone,
        "owner": owner,
        "run_ref": run_ref
    })
}

#[test]
fn governance_start_and_refresh_emit_reusable_requirements_packets() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        complete_requirements_brief(
            "Bound AI-assisted engineering work with explicit governance.",
            "Operators can review a complete requirements packet before downstream planning.",
        ),
    )
    .expect("requirements brief");

    let start_output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "start", "--json"])
        .write_stdin(
            governance_start_request(
                &workspace,
                "requirements",
                "existing",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let start_json: serde_json::Value = serde_json::from_slice(&start_output).expect("start json");
    let run_ref = start_json["run_ref"].as_str().expect("run ref").to_string();
    let packet_ref = format!(".canon/artifacts/{run_ref}/requirements");

    assert_eq!(start_json["status"], "governed_ready");
    assert_eq!(start_json["approval_state"], "not_needed");
    assert_eq!(start_json["packet_readiness"], "reusable");
    assert_eq!(start_json["packet_ref"], packet_ref);
    assert!(start_json["document_refs"].as_array().is_some_and(|refs| !refs.is_empty()));
    assert_eq!(start_json["document_refs"], start_json["expected_document_refs"]);

    let refresh_output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "refresh", "--json"])
        .write_stdin(
            governance_refresh_request(
                &workspace,
                &run_ref,
                "requirements",
                "existing",
                "bounded-impact",
                "yellow",
                "product-lead",
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let refresh_json: serde_json::Value =
        serde_json::from_slice(&refresh_output).expect("refresh json");

    assert_eq!(refresh_json["status"], "governed_ready");
    assert_eq!(refresh_json["approval_state"], "not_needed");
    assert_eq!(refresh_json["packet_readiness"], "reusable");
    assert_eq!(refresh_json["run_ref"], run_ref);
    assert_eq!(refresh_json["packet_ref"], start_json["packet_ref"]);
    assert_eq!(refresh_json["document_refs"], start_json["document_refs"]);
    assert_eq!(refresh_json["expected_document_refs"], start_json["expected_document_refs"]);
}

#[test]
fn governance_start_surfaces_approval_gated_architecture_runs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), complete_architecture_brief())
        .expect("architecture brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "start", "--json"])
        .write_stdin(
            governance_start_request(
                &workspace,
                "architecture",
                "existing",
                "systemic-impact",
                "yellow",
                "staff-architect",
                Some("architecture.md"),
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("approval json");

    assert_eq!(json["status"], "awaiting_approval");
    assert_eq!(json["approval_state"], "requested");
    assert_eq!(json["reason_code"], "approval_required");
    assert!(json["run_ref"].as_str().is_some_and(|value| !value.is_empty()));
}

#[test]
fn governance_start_blocks_rejected_requirements_packets() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("requirements.md"), incomplete_requirements_brief())
        .expect("requirements brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "start", "--json"])
        .write_stdin(
            governance_start_request(
                &workspace,
                "requirements",
                "existing",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("blocked json");

    assert_eq!(json["status"], "blocked");
    assert_eq!(json["packet_readiness"], "rejected");
    assert_eq!(json["reason_code"], "rejected_packet");
    assert!(json["run_ref"].as_str().is_some_and(|value| !value.is_empty()));
    assert!(json["document_refs"].as_array().is_some_and(|refs| !refs.is_empty()));
    assert!(json["missing_sections"].as_array().is_some_and(|sections| {
        sections.iter().any(|section| section == "problem-statement.md")
    }));
}

#[test]
fn governance_refresh_fails_when_artifact_contract_is_corrupted() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        complete_requirements_brief(
            "Bound AI-assisted engineering work with explicit governance.",
            "Operators can review a complete requirements packet before downstream planning.",
        ),
    )
    .expect("requirements brief");

    let start_output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "start", "--json"])
        .write_stdin(
            governance_start_request(
                &workspace,
                "requirements",
                "existing",
                "bounded-impact",
                "yellow",
                "product-lead",
                Some("requirements.md"),
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let start_json: serde_json::Value = serde_json::from_slice(&start_output).expect("start json");
    let run_ref = start_json["run_ref"].as_str().expect("run ref").to_string();

    let layout = ProjectLayout::new(workspace.path());
    fs::write(layout.run_dir(&run_ref).join("artifact-contract.toml"), "artifact_requirements = [")
        .expect("corrupt artifact contract");

    let refresh_output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "refresh", "--json"])
        .write_stdin(
            governance_refresh_request(
                &workspace,
                &run_ref,
                "requirements",
                "existing",
                "bounded-impact",
                "yellow",
                "product-lead",
            )
            .to_string(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let refresh_json: serde_json::Value =
        serde_json::from_slice(&refresh_output).expect("refresh json");

    assert_eq!(refresh_json["status"], "failed");
    assert_eq!(refresh_json["reason_code"], "artifact_contract_unreadable");
    assert_eq!(refresh_json["run_ref"], run_ref);
}
