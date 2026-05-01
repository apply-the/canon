use std::fs;
use std::process::Command as ProcessCommand;

use canon_engine::EngineService;
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_supply_chain_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/cli")).expect("src dir");
    fs::write(
        workspace.path().join("Cargo.toml"),
        "[package]\nname = \"bounded-cli\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nclap = \"4\"\nserde = \"1\"\n",
    )
    .expect("manifest");
    fs::write(
        workspace.path().join("deny.toml"),
        "[licenses]\nallow = [\"MIT\", \"Apache-2.0\"]\n",
    )
    .expect("deny config");
    fs::write(
        workspace.path().join("src/cli/main.rs"),
        "fn main() { println!(\"bounded cli\"); }\n",
    )
    .expect("main source");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed supply chain repo"]);
}

fn supply_chain_request(input: &str, risk: RiskClass, zone: UsageZone) -> RunRequest {
    RunRequest {
        mode: Mode::SupplyChainAnalysis,
        risk,
        zone,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "release-engineer".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
    format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
}

fn complete_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Licensing Posture\n\n- Mixed OSS distribution with attribution obligations\n\n## Distribution Model\n\n- shipped as source releases and prebuilt binaries\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n- Rust advisory and license policy metadata\n\n## Out Of Scope Components\n\n- external CI marketplace actions\n\n## Scanner Selection Rationale\n\n- use Rust-native dependency and advisory tooling first\n\n## SBOM Outputs\n\n- emit one machine-readable SBOM for the workspace crates\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n- Medium: CLI parsing dependencies should stay current\n- Low: documentation-only tooling drift is tracked separately\n\n## Exploitability Notes\n\n- findings affecting shell execution or persistence matter most\n\n## Triage Decisions\n\n- escalate shell execution findings\n\n## Compatibility Classes\n\n- permissive licenses are acceptable for the current distribution model\n\n## Flagged Incompatibilities\n\n- none confirmed from the authored inputs alone\n\n## Obligations\n\n- preserve notices and attribution for distributed dependencies\n\n## Outdated Dependencies\n\n- review lagging CLI and persistence dependencies first\n\n## End Of Life Signals\n\n- no end-of-life signals are currently confirmed\n\n## Abandonment Signals\n\n- no abandonment signals are currently confirmed\n\n## Modernization Slices\n\n1. refresh shell-execution-adjacent crates first\n2. revisit advisory and license tooling after the first pass\n\n## Scanner Decisions\n\n- installed: Rust-native OSS tooling only\n\n## Coverage Gaps\n\n- no non-Rust ecosystem coverage is included in this packet\n\n## Source Inputs\n\n- Cargo.toml\n- deny.toml\n\n## Independent Checks\n\n- focused run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- attach final scanner outputs before external review\n"
}

fn incomplete_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n\n## Triage Decisions\n\n- escalate shell execution findings\n"
}

fn missing_posture_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n- Rust advisory and license policy metadata\n\n## Out Of Scope Components\n\n- external CI marketplace actions\n\n## Scanner Selection Rationale\n\n- use Rust-native dependency and advisory tooling first\n\n## SBOM Outputs\n\n- emit one machine-readable SBOM for the workspace crates\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n\n## Exploitability Notes\n\n- findings affecting shell execution or persistence matter most\n\n## Triage Decisions\n\n- escalate shell execution findings\n\n## Compatibility Classes\n\n- permissive licenses are acceptable once posture is confirmed\n\n## Flagged Incompatibilities\n\n- none confirmed from the authored inputs alone\n\n## Obligations\n\n- preserve notices and attribution for distributed dependencies\n\n## Outdated Dependencies\n\n- review lagging CLI and persistence dependencies first\n\n## End Of Life Signals\n\n- no end-of-life signals are currently confirmed\n\n## Abandonment Signals\n\n- no abandonment signals are currently confirmed\n\n## Modernization Slices\n\n1. refresh shell-execution-adjacent crates first\n\n## Scanner Decisions\n\n- installed: Rust-native OSS tooling only\n\n## Coverage Gaps\n\n- licensing posture and distribution model are still awaiting maintainer confirmation\n\n## Source Inputs\n\n- Cargo.toml\n- deny.toml\n\n## Independent Checks\n\n- focused run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- attach final scanner outputs before external review\n"
}

fn skipped_scanner_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Licensing Posture\n\n- Mixed OSS distribution with attribution obligations\n\n## Distribution Model\n\n- shipped as source releases and prebuilt binaries\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n- Rust advisory and license policy metadata\n\n## Out Of Scope Components\n\n- external CI marketplace actions\n\n## Scanner Selection Rationale\n\n- use Rust-native dependency and advisory tooling first\n\n## SBOM Outputs\n\n- emit one machine-readable SBOM for the workspace crates\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n\n## Exploitability Notes\n\n- findings affecting shell execution or persistence matter most\n\n## Triage Decisions\n\n- escalate shell execution findings\n\n## Compatibility Classes\n\n- permissive licenses are acceptable for the current distribution model\n\n## Flagged Incompatibilities\n\n- none confirmed from the authored inputs alone\n\n## Obligations\n\n- preserve notices and attribution for distributed dependencies\n\n## Outdated Dependencies\n\n- review lagging CLI and persistence dependencies first\n\n## End Of Life Signals\n\n- no end-of-life signals are currently confirmed\n\n## Abandonment Signals\n\n- no abandonment signals are currently confirmed\n\n## Modernization Slices\n\n1. refresh shell-execution-adjacent crates first\n\n## Scanner Decisions\n\n- skipped: Rust license scanner coverage for Cargo workspace because no approved OSS tool is installed on PATH\n\n## Source Inputs\n\n- Cargo.toml\n- deny.toml\n\n## Independent Checks\n\n- focused run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- attach final scanner outputs before external review\n"
}

#[test]
fn supply_chain_analysis_direct_run_exercises_service_summary_and_publish_paths() {
    let workspace = TempDir::new().expect("temp dir");
    init_supply_chain_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), complete_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(supply_chain_request(
            "supply-chain-analysis.md",
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
        ))
        .expect("supply chain run");

    assert_eq!(summary.state, "AwaitingApproval");
    assert_eq!(summary.artifact_count, 7);
    assert!(summary.approval_targets.iter().any(|target| target == "gate:risk"));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("analysis-overview.md")));
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("analysis-evidence.md")));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Analysis Overview");
    assert_eq!(mode_result.headline, "Supply-chain-analysis packet ready for governed review.");
    assert!(
        mode_result
            .artifact_packet_summary
            .contains("2 ecosystem set(s), 3 finding set(s), and 2 modernization slice set(s)")
    );
    assert!(
        mode_result.primary_artifact_path.ends_with("supply-chain-analysis/analysis-overview.md")
    );

    let published = service
        .publish(&summary.run_id, None)
        .expect("publish should succeed before risk approval");
    let leaf = default_publish_leaf(&summary.run_id, "supply-chain-analysis");
    assert!(published.published_to.ends_with(&format!("docs/supply-chain/{leaf}")));
    assert!(published.published_files.iter().any(|path| path.ends_with("analysis-overview.md")));
    assert!(published.published_files.iter().any(|path| path.ends_with("packet-metadata.json")));

    let published_overview =
        workspace.path().join("docs").join("supply-chain").join(&leaf).join("analysis-overview.md");
    assert!(published_overview.exists());
    let overview_contents = fs::read_to_string(published_overview).expect("published overview");
    assert!(overview_contents.contains("## Declared Scope"));

    let approval = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "release-engineer",
            ApprovalDecision::Approve,
            "bounded supply-chain packet accepted for governed review",
        )
        .expect("gate approval");
    assert_eq!(approval.state, "Completed");

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Completed");
    assert!(status.approval_targets.is_empty());
    assert_eq!(
        status.mode_result.as_ref().map(|result| result.primary_artifact_title.as_str()),
        Some("Analysis Overview")
    );
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
}

#[test]
fn supply_chain_analysis_direct_run_exposes_blocked_gate_and_missing_body_markers() {
    let workspace = TempDir::new().expect("temp dir");
    init_supply_chain_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), incomplete_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(supply_chain_request(
            "supply-chain-analysis.md",
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ))
        .expect("supply chain run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));
    assert!(summary.blocked_gates.iter().any(|gate| gate.gate == "risk"));

    let mode_result = summary.mode_result.as_ref().expect("mode result");
    assert_eq!(mode_result.execution_posture.as_deref(), Some("recommendation-only"));
    assert_eq!(mode_result.primary_artifact_title, "Analysis Overview");
    assert!(mode_result.headline.contains("explicit missing-context marker(s)"));
    assert!(mode_result.artifact_packet_summary.contains("missing-context marker(s)"));

    let license = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("supply-chain-analysis")
            .join("license-compliance.md"),
    )
    .expect("license compliance artifact");
    assert!(license.contains("## Missing Authored Body"));
    assert!(license.contains("`## Compatibility Classes`"));

    let status = service.status(&summary.run_id).expect("status");
    assert_eq!(status.state, "Blocked");
    assert!(status.blocked_gates.iter().any(|gate| gate.gate == "risk"));
    assert_eq!(
        status.mode_result.as_ref().and_then(|result| result.execution_posture.as_deref()),
        Some("recommendation-only")
    );
}

#[test]
fn supply_chain_analysis_direct_run_surfaces_missing_authored_decision_markers() {
    let workspace = TempDir::new().expect("temp dir");
    init_supply_chain_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), missing_posture_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(supply_chain_request(
            "supply-chain-analysis.md",
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ))
        .expect("supply chain run");

    assert_eq!(summary.state, "Completed");

    let overview = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("supply-chain-analysis")
            .join("analysis-overview.md"),
    )
    .expect("analysis overview");
    assert!(overview.contains("## Missing Authored Decision"));
    assert!(overview.contains("`## Licensing Posture`"));
    assert!(overview.contains("`## Distribution Model`"));
}

#[test]
fn supply_chain_analysis_direct_run_derives_coverage_gap_from_skipped_scanner_decision() {
    let workspace = TempDir::new().expect("temp dir");
    init_supply_chain_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), skipped_scanner_brief())
        .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(supply_chain_request(
            "supply-chain-analysis.md",
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ))
        .expect("supply chain run");

    assert_eq!(summary.state, "Completed");

    let policy = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("supply-chain-analysis")
            .join("policy-decisions.md"),
    )
    .expect("policy decisions artifact");
    assert!(policy.contains("Coverage gap derived from recorded scanner decisions"));
    assert!(policy.contains("skipped: Rust license scanner coverage"));
    assert!(policy.contains("Next action: install the missing scanner or document an approved replacement and rerun the packet."));
}
