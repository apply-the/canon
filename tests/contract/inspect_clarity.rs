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
    assert!(text.contains("How is the Bird identified over USB?"));
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

fn representative_mode_input(mode: &str) -> (&'static str, &'static str) {
    match mode {
        "system-shaping" => (
            "system-shaping.md",
            "# System Shaping Brief\n\n## System Shape\n\nBound billing into catalog, subscription, and invoice contexts.\n\n## Boundary Decisions\n\n- Catalog owns plans\n- Subscription owns lifecycle\n\n## Structural Options\n\n- Merge rules into one service\n- Keep separate bounded contexts\n\n## Selected Boundaries\n\nKeep the contexts separate.\n\n## Rationale\n\nSeparate billing rules reduce cross-context drift.\n\n## Boundary Risks And Open Questions\n\n- How do credit notes cross invoice ownership?\n",
        ),
        "architecture" => (
            "architecture.md",
            "# Architecture Brief\n\n## Decision\n\nSplit packet reasoning posture from renderer fallback logic.\n\n## Constraints\n\n- Preserve the existing .canon schema\n\n## Options Considered\n\n- Shared posture helper\n- Per-mode wording patches\n\n## Recommendation\n\nUse a shared posture helper.\n\n## Decision Drivers\n\n- Shared runtime consistency\n\n## Why Not The Others\n\n- Per-mode patches drift quickly\n",
        ),
        "change" => (
            "change.md",
            "# Change Brief\n\n## Intended Change\n\nTighten backlog fallback sections so missing reasoning stays explicit.\n\n## Change Surface\n\n- crates/canon-engine/src/artifacts/markdown.rs\n\n## Legacy Invariants\n\n- No .canon schema changes\n\n## Decision Evidence\n\n- Current fallback prose can look like authored reasoning\n\n## Options Considered\n\n- Remove fallback sections entirely\n- Convert them to explicit missing-body language\n\n## Recommendation\n\nConvert fallback sections to explicit missing-body language.\n\n## Boundary Tradeoffs\n\n- Less polished prose\n- More honest packet posture\n\n## Unresolved Questions\n\n- Which packet families still need explicit fallback headings?\n",
        ),
        "backlog" => (
            "backlog.md",
            "# Backlog Brief\n\n## Delivery Intent\n\nDecompose feature 033 into bounded validation-first tasks.\n\n## Desired Granularity\n\n- One task per verifiable change family\n\n## Planning Horizon\n\n- Feature 033 only\n\n## Source References\n\n- specs/033-reasoning-evidence-clarity/plan.md\n\n## Constraints\n\n- Keep coverage and validation explicit\n\n## Out of Scope\n\n- New runtime modes\n",
        ),
        "implementation" => (
            "implementation.md",
            "# Implementation Brief\n\n## Task Mapping\n\nImplement shared authored-mode clarity parsing in inspect.rs and clarity.rs.\n\n## Bounded Changes\n\n- inspect clarity dispatch\n\n## Mutation Bounds\n\n- crates/canon-engine/src/orchestrator/service\n\n## Allowed Paths\n\n- crates/canon-engine/src/orchestrator/service/*\n\n## Candidate Frameworks\n\n- Shared generic helper\n- Per-mode branches\n\n## Decision Evidence\n\n- Existing clarity contract already works for three modes\n\n## Recommendation\n\nUse the shared generic helper.\n\n## Safety-Net Evidence\n\n- tests/inspect_clarity.rs\n\n## Remaining Risks\n\n- review path rules might regress\n",
        ),
        "refactor" => (
            "refactor.md",
            "# Refactor Brief\n\n## Refactor Scope\n\nExtract shared authored-mode clarity helpers.\n\n## Allowed Paths\n\n- crates/canon-engine/src/orchestrator/service/clarity.rs\n\n## Preserved Behavior\n\n- requirements, discovery, and supply-chain clarity output stays unchanged\n\n## Structural Rationale\n\nReduce per-mode drift in inspect clarity.\n\n## Safety-Net Evidence\n\n- cargo test --test inspect_clarity\n\n## Regression Findings\n\n- none yet\n\n## Decision\n\nKeep mode-specific summaries but share parsing.\n",
        ),
        "migration" => (
            "migration.md",
            "# Migration Brief\n\n## Current State\n\nClarity exists only for three governed modes.\n\n## Target State\n\nAll file-backed governed modes expose inspect clarity.\n\n## Transition Boundaries\n\n- Do not change .canon storage\n\n## Guaranteed Compatibility\n\n- Existing CLI output schema remains stable\n\n## Options Matrix\n\n- Shared authored brief\n- Separate mode branches\n\n## Decision Evidence\n\n- Shared contract already exists\n\n## Recommendation\n\nUse the shared authored brief.\n\n## Verification Checks\n\n- tests/inspect_clarity.rs\n\n## Deferred Decisions\n\n- Tune per-mode wording later if needed\n",
        ),
        "review" => (
            "canon-input/review.md",
            "# Review Brief\n\n## Review Target\n\nReview feature 033 for reasoning-honesty regressions.\n\n## Evidence Basis\n\n- spec.md\n- plan.md\n- runtime code diffs\n\n## Boundary Findings\n\n- focus on file-backed clarity and packet posture only\n\n## Missing Evidence\n\n- No full regression run yet\n\n## Collection Priorities\n\n- tests/inspect_clarity.rs\n\n## Final Disposition\n\nNeeds more evidence before approval.\n\n## Accepted Risks\n\n- wording drift across docs\n",
        ),
        "verification" => (
            "verification.md",
            "# Verification Brief\n\n## Claims Under Test\n\n- All file-backed modes support inspect clarity\n\n## Invariant Checks\n\n- Existing honesty markers stay explicit\n\n## Contract Assumptions\n\n- CLI output schema stays stable\n\n## Verification Outcome\n\nPartially supported pending targeted tests.\n\n## Challenge Findings\n\n- review and migration still need contract coverage\n\n## Contradictions\n\n- None found yet\n\n## Verified Claims\n\n- requirements, discovery, and supply-chain analysis already support clarity\n\n## Overall Verdict\n\nNeeds targeted implementation validation.\n\n## Open Findings\n\n- New mode summaries are not verified end to end yet\n",
        ),
        "incident" => (
            "incident.md",
            "# Incident Brief\n\n## Incident Scope\n\nClarity output misrepresents weak authored packets as complete.\n\n## Trigger And Current State\n\nReview of feature 033 found unsupported inspect targets.\n\n## Operational Constraints\n\n- Do not change run persistence\n\n## Known Facts\n\n- inspect clarity handles only three modes today\n\n## Evidence Gaps\n\n- No packet-level posture audit yet\n\n## Impacted Surfaces\n\n- inspect.rs\n- clarity.rs\n\n## Immediate Actions\n\n- Expand the clarity dispatcher\n\n## Confidence And Unknowns\n\n- Need targeted tests for review and migration families\n",
        ),
        "security-assessment" => (
            "security-assessment.md",
            "# Security Assessment Brief\n\n## Assessment Scope\n\nAssess reasoning-evidence honesty for feature 033 runtime surfaces.\n\n## In-Scope Assets\n\n- inspect clarity responses\n- rendered packets\n\n## Trust Boundaries\n\n- file-backed inputs versus diff-backed pr-review\n\n## Risk Findings\n\n- heading presence may be mistaken for support\n\n## Control Families\n\n- honesty markers\n- explicit missing evidence posture\n\n## Tradeoffs\n\n- More blunt output language\n\n## Evidence Gaps\n\n- Need coverage across all mode families\n\n## Scope Limits\n\n- no external adapters\n",
        ),
        "system-assessment" => (
            "system-assessment.md",
            "# System Assessment Brief\n\n## Assessment Objective\n\nAssess current reasoning-evidence posture across Canon modes.\n\n## Assessed Views\n\n- clarity\n- packet rendering\n\n## Boundary Notes\n\n- no schema changes\n\n## Observed Findings\n\n- only three modes emit reasoning_signals today\n\n## Inferred Findings\n\n- fallback prose can look authored\n\n## Assessment Gaps\n\n- docs and skills are not yet aligned\n\n## Evidence Sources\n\n- inspect.rs\n- clarity.rs\n\n## Likely Follow-On Modes\n\n- change\n- verification\n",
        ),
        other => panic!("unexpected mode {other}"),
    }
}

#[test]
fn inspect_clarity_supports_all_file_backed_governed_modes_with_reasoning_signals() {
    let workspace = TempDir::new().expect("temp dir");
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
        let (input_path, content) = representative_mode_input(mode);
        let path = workspace.path().join(input_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent dir");
        }
        fs::write(&path, content).expect("mode input");

        let output = cli_command()
            .current_dir(workspace.path())
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
