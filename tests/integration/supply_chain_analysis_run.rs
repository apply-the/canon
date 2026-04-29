use std::fs;
use std::process::Command as ProcessCommand;

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

fn init_repo(workspace: &TempDir) {
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

fn complete_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Licensing Posture\n\n- Mixed OSS distribution with attribution obligations\n\n## Distribution Model\n\n- shipped as source releases and prebuilt binaries\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n- Rust advisory and license policy metadata\n\n## Out Of Scope Components\n\n- external CI marketplace actions\n\n## Scanner Selection Rationale\n\n- use Rust-native dependency and advisory tooling first\n\n## SBOM Outputs\n\n- emit one machine-readable SBOM for the workspace crates\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n- Medium: CLI parsing dependencies should stay current\n- Low: documentation-only tooling drift is tracked separately\n\n## Exploitability Notes\n\n- findings affecting shell execution or persistence matter most\n\n## Triage Decisions\n\n- escalate shell execution findings\n\n## Compatibility Classes\n\n- permissive licenses are acceptable for the current distribution model\n\n## Flagged Incompatibilities\n\n- none confirmed from the authored inputs alone\n\n## Obligations\n\n- preserve notices and attribution for distributed dependencies\n\n## Outdated Dependencies\n\n- review lagging CLI and persistence dependencies first\n\n## End Of Life Signals\n\n- no end-of-life signals are currently confirmed\n\n## Abandonment Signals\n\n- no abandonment signals are currently confirmed\n\n## Modernization Slices\n\n1. refresh shell-execution-adjacent crates first\n2. revisit advisory and license tooling after the first pass\n\n## Scanner Decisions\n\n- installed: Rust-native OSS tooling only\n\n## Coverage Gaps\n\n- no non-Rust ecosystem coverage is included in this packet\n\n## Source Inputs\n\n- Cargo.toml\n- deny.toml\n\n## Independent Checks\n\n- focused CLI run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- attach final scanner outputs before external review\n"
}

fn incomplete_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n"
}

fn skipped_scanner_brief() -> &'static str {
    "# Supply Chain Analysis Brief\n\n## Declared Scope\n\n- Cargo workspace dependency and release posture only\n\n## Licensing Posture\n\n- Mixed OSS distribution with attribution obligations\n\n## Distribution Model\n\n- shipped as source releases and prebuilt binaries\n\n## Ecosystems In Scope\n\n- Cargo workspace manifests\n- Rust advisory and license policy metadata\n\n## Out Of Scope Components\n\n- external CI marketplace actions\n\n## Scanner Selection Rationale\n\n- use Rust-native dependency and advisory tooling first\n\n## SBOM Outputs\n\n- emit one machine-readable SBOM for the workspace crates\n\n## Findings By Severity\n\n- High: shell execution dependencies deserve focused review\n\n## Exploitability Notes\n\n- findings affecting shell execution or persistence matter most\n\n## Triage Decisions\n\n- escalate shell execution findings\n\n## Compatibility Classes\n\n- permissive licenses are acceptable for the current distribution model\n\n## Flagged Incompatibilities\n\n- none confirmed from the authored inputs alone\n\n## Obligations\n\n- preserve notices and attribution for distributed dependencies\n\n## Outdated Dependencies\n\n- review lagging CLI and persistence dependencies first\n\n## End Of Life Signals\n\n- no end-of-life signals are currently confirmed\n\n## Abandonment Signals\n\n- no abandonment signals are currently confirmed\n\n## Modernization Slices\n\n1. refresh shell-execution-adjacent crates first\n\n## Scanner Decisions\n\n- skipped: Rust license scanner coverage for Cargo workspace because no approved OSS tool is installed on PATH\n\n## Source Inputs\n\n- Cargo.toml\n- deny.toml\n\n## Independent Checks\n\n- focused CLI run and renderer suites validate the packet surface\n\n## Deferred Verification\n\n- attach final scanner outputs before external review\n"
}

#[test]
fn run_supply_chain_analysis_emits_a_reviewable_packet_and_publishes_while_approval_gated() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), complete_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "supply-chain-analysis",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "release-engineer",
            "--input",
            "supply-chain-analysis.md",
            "--output",
            "json",
        ])
        .assert()
        .code(3)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(run_id)
        .join("supply-chain-analysis");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["artifact_count"], 7);
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Analysis Overview"));
    assert!(
        json["approval_targets"].as_array().is_some_and(|targets| targets
            .iter()
            .any(|target| target.as_str() == Some("gate:risk")))
    );
    assert!(artifact_root.join("analysis-overview.md").exists());
    assert!(artifact_root.join("analysis-evidence.md").exists());

    let overview =
        fs::read_to_string(artifact_root.join("analysis-overview.md")).expect("analysis overview");
    assert!(overview.contains("## Declared Scope"));

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("supply-chain")
            .join(run_id)
            .join("analysis-overview.md")
            .exists()
    );

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "release-engineer",
            "--decision",
            "approve",
            "--rationale",
            "bounded supply-chain packet accepted for governed review",
        ])
        .assert()
        .success();

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
        Some("Analysis Overview")
    );
}

#[test]
fn run_supply_chain_analysis_blocks_when_required_authored_sections_are_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), incomplete_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "supply-chain-analysis",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "release-engineer",
            "--input",
            "supply-chain-analysis.md",
            "--output",
            "json",
        ])
        .assert()
        .code(2)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("json output");
    let run_id = json["run_id"].as_str().expect("run id");
    let artifact_root = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(run_id)
        .join("supply-chain-analysis");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");

    let license = fs::read_to_string(artifact_root.join("license-compliance.md"))
        .expect("license compliance");
    assert!(license.contains("## Missing Authored Body"));
    assert!(license.contains("`## Compatibility Classes`"));
}

#[test]
fn run_supply_chain_analysis_records_a_derived_coverage_gap_for_skipped_scanners() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("supply-chain-analysis.md"), skipped_scanner_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "supply-chain-analysis",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "release-engineer",
            "--input",
            "supply-chain-analysis.md",
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
    let policy = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(run_id)
            .join("supply-chain-analysis")
            .join("policy-decisions.md"),
    )
    .expect("policy decisions");

    assert!(policy.contains("Coverage gap derived from recorded scanner decisions"));
    assert!(policy.contains("skipped: Rust license scanner coverage"));
}
