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

    fs::create_dir_all(workspace.path().join("src/api")).expect("src dir");
    fs::create_dir_all(workspace.path().join("deploy/helm")).expect("deploy dir");
    fs::create_dir_all(workspace.path().join("infra/queue")).expect("infra dir");
    fs::write(
        workspace.path().join("src/api/checkout.rs"),
        "pub fn enqueue_checkout(id: &str) -> String {\n    format!(\"queued:{id}\")\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("deploy/helm/values.yaml"),
        "api:\n  replicas: 2\nworker:\n  replicas: 1\n",
    )
    .expect("deploy file");
    fs::write(
        workspace.path().join("infra/queue/topology.md"),
        "# Queue Topology\n\n- billing-worker consumes checkout events\n",
    )
    .expect("queue file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed system assessment repo"]);
}

fn complete_brief() -> &'static str {
    "# System Assessment Brief\n\n## Assessment Objective\n\n- establish an as-is architecture packet before planning the next bounded change\n\n## Stakeholders\n\n- principal architect\n- platform maintainer\n\n## Primary Concerns\n\n- current component ownership\n- deployment boundaries\n- integration choke points\n\n## Assessment Posture\n\n- read-only and evidence-first\n\n## Stakeholder Concerns\n\n- boundary clarity for downstream architecture work\n- confidence in deployment assumptions\n\n## Assessed Views\n\n- functional\n- component\n- deployment\n- technology\n- integration\n\n## Partial Or Skipped Coverage\n\n- production queue topology remains partially assessed\n\n## Confidence By Surface\n\n- component view: high\n- deployment view: medium\n\n## Assessed Assets\n\n- api service\n- worker service\n- postgres datastore\n\n## Critical Dependencies\n\n- stripe webhook delivery\n- redis job queue\n\n## Boundary Notes\n\n- worker crosses the queue boundary to reach the datastore\n\n## Ownership Signals\n\n- platform team owns the deployment manifests\n\n## Responsibilities\n\n- api validates inbound requests and schedules jobs\n\n## Primary Flows\n\n- inbound checkout events enter the api and fan out to workers\n\n## Observed Boundaries\n\n- queue and datastore boundaries are explicit in repository config\n\n## Components\n\n- api handlers\n- billing worker\n- persistence gateway\n\n## Interfaces\n\n- http handlers call the persistence gateway through the billing worker\n\n## Confidence Notes\n\n- component boundaries are explicit in source and deployment inputs\n\n## Execution Environments\n\n- containers on a shared kubernetes cluster\n\n## Network And Runtime Boundaries\n\n- ingress to api\n- api to redis\n- worker to postgres\n\n## Deployment Signals\n\n- helm values declare api and worker replicas separately\n\n## Coverage Gaps\n\n- background cron topology is not represented in the repository\n\n## Technology Stack\n\n- rust services\n- postgres\n- redis\n\n## Platform Dependencies\n\n- kubernetes\n- github actions\n\n## Version Or Lifecycle Signals\n\n- redis image pinning is present but postgres lifecycle policy is not explicit\n\n## Evidence Gaps\n\n- no live runtime manifests confirm the cluster namespace layout\n\n## Integrations\n\n- stripe webhooks\n- email provider callback\n\n## Data Exchanges\n\n- checkout events enter via http and persist through the billing worker\n\n## Trust And Failure Boundaries\n\n- inbound webhook trust ends at the api boundary\n\n## Inference Notes\n\n- the repository suggests one worker pool, but runtime shard count is inferred\n\n## Observed Risks\n\n- queue retry policy could amplify billing reprocessing\n\n## Risk Triggers\n\n- retry configuration is partly declared and partly inferred\n\n## Impact Notes\n\n- duplicate billing work could reach downstream reconciliation\n\n## Likely Follow-On Modes\n\n- architecture\n- change\n\n## Observed Findings\n\n- the repository contains separate api and worker deployment inputs\n\n## Inferred Findings\n\n- one queue-backed worker pool appears to own async billing execution\n\n## Assessment Gaps\n\n- no repository evidence confirms production queue partitioning\n\n## Evidence Sources\n\n- src/api/checkout.rs\n- deploy/helm/values.yaml\n- infra/queue/topology.md\n"
}

fn incomplete_brief() -> &'static str {
    "# System Assessment Brief\n\n## Assessment Objective\n\n- capture integration posture only\n\n## Integrations\n\n- stripe webhooks\n\n## Data Exchanges\n\n- checkout events enter via http\n"
}

#[test]
fn run_system_assessment_emits_an_as_is_packet_and_publishes_after_risk_approval() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("system-assessment.md"), complete_brief()).expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "system-assessment",
            "--system-context",
            "existing",
            "--risk",
            "systemic-impact",
            "--zone",
            "yellow",
            "--owner",
            "architecture-lead",
            "--input",
            "system-assessment.md",
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
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("system-assessment");

    assert_eq!(json["state"], "AwaitingApproval");
    assert_eq!(json["artifact_count"], 10);
    assert_eq!(json["mode_result"]["execution_posture"].as_str(), Some("recommendation-only"));
    assert_eq!(json["mode_result"]["primary_artifact_title"].as_str(), Some("Assessment Overview"));
    assert!(json["approval_targets"].as_array().is_some_and(|targets| {
        targets.iter().any(|target| target.as_str() == Some("gate:risk"))
    }));
    assert!(artifact_root.join("assessment-overview.md").exists());
    assert!(artifact_root.join("coverage-map.md").exists());
    assert!(artifact_root.join("component-view.md").exists());
    assert!(artifact_root.join("integration-view.md").exists());
    assert!(artifact_root.join("assessment-evidence.md").exists());

    let coverage = fs::read_to_string(artifact_root.join("coverage-map.md")).expect("coverage map");
    assert!(coverage.contains("## Assessed Views"));

    cli_command()
        .current_dir(workspace.path())
        .args([
            "approve",
            "--run",
            run_id,
            "--target",
            "gate:risk",
            "--by",
            "architecture-lead",
            "--decision",
            "approve",
            "--rationale",
            "bounded system assessment packet accepted for governed review",
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

    cli_command().current_dir(workspace.path()).args(["publish", run_id]).assert().success();

    assert!(
        workspace
            .path()
            .join("docs")
            .join("architecture")
            .join("assessments")
            .join(run_id)
            .join("assessment-overview.md")
            .exists()
    );
}

#[test]
fn run_system_assessment_blocks_when_a_required_authored_section_is_missing() {
    let workspace = TempDir::new().expect("temp dir");
    init_repo(&workspace);
    fs::write(workspace.path().join("system-assessment.md"), incomplete_brief())
        .expect("brief file");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "run",
            "--mode",
            "system-assessment",
            "--system-context",
            "existing",
            "--risk",
            "bounded-impact",
            "--zone",
            "yellow",
            "--owner",
            "architecture-lead",
            "--input",
            "system-assessment.md",
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
    let artifact_root =
        workspace.path().join(".canon").join("artifacts").join(run_id).join("system-assessment");

    assert_eq!(json["state"], "Blocked");
    assert_eq!(json["blocking_classification"], "artifact-blocked");

    let integration_view =
        fs::read_to_string(artifact_root.join("integration-view.md")).expect("integration view");
    assert!(integration_view.contains("## Missing Authored Body"));
    assert!(integration_view.contains("`## Trust And Failure Boundaries`"));
}
