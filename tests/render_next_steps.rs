use std::process::Command as ProcessCommand;

fn script_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}

fn run_shell(profile: &str, run_id: &str, target: &str, primary_artifact_path: &str) -> String {
    let mut command = ProcessCommand::new("/bin/bash");
    command.arg(script_path(".agents/skills/canon-shared/scripts/render-next-steps.sh")).args([
        "--profile",
        profile,
        "--run-id",
        run_id,
        "--target",
        target,
    ]);
    if !primary_artifact_path.is_empty() {
        command.args(["--primary-artifact-path", primary_artifact_path]);
    }

    let output = command.output().expect("run shell renderer");

    assert!(
        output.status.success(),
        "shell renderer failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("shell output utf8")
}

fn run_powershell(
    profile: &str,
    run_id: &str,
    target: &str,
    primary_artifact_path: &str,
) -> String {
    let mut command = ProcessCommand::new("pwsh");
    command.args([
        "-File",
        &script_path(".agents/skills/canon-shared/scripts/render-next-steps.ps1"),
        "-Profile",
        profile,
        "-RunId",
        run_id,
        "-Target",
        target,
    ]);
    if !primary_artifact_path.is_empty() {
        command.args(["-PrimaryArtifactPath", primary_artifact_path]);
    }

    let output = command.output().expect("run powershell renderer");

    assert!(
        output.status.success(),
        "powershell renderer failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("powershell output utf8")
}

#[test]
fn gated_renderers_emit_recommended_then_possible_actions_in_order() {
    let expected = concat!(
        "Recommended Next Step:\n",
        "- Use $canon-inspect-evidence for run run-123 before recording approval.\n",
        "\n",
        "Possible Actions:\n",
        "- Use $canon-approve for target gate:review-disposition on run run-123 after review.\n",
        "- Use $canon-status after approval, or $canon-resume if Canon still requires continuation.\n"
    );

    let shell_output = run_shell("gated", "run-123", "gate:review-disposition", "");
    assert_eq!(shell_output, expected);

    let powershell_output =
        run_powershell("gated", "run-123", "gate:review-disposition", "").replace("\r\n", "\n");
    assert_eq!(powershell_output, expected);
}

#[test]
fn approval_recorded_renderers_prioritize_resume_over_status() {
    let expected = concat!(
        "Recommended Next Step:\n",
        "- Use $canon-resume for run run-456 only if Canon still requires continuation.\n",
        "\n",
        "Possible Actions:\n",
        "- Use $canon-status to confirm the post-approval run state.\n"
    );

    let shell_output = run_shell("approval-recorded", "run-456", "", "");
    assert_eq!(shell_output, expected);

    let powershell_output =
        run_powershell("approval-recorded", "run-456", "", "").replace("\r\n", "\n");
    assert_eq!(powershell_output, expected);
}

#[test]
fn completed_renderers_do_not_force_evidence_follow_up() {
    let expected = concat!(
        "Recommended Next Step:\n",
        "- None. The run result is already readable for run run-789.\n",
        "\n",
        "Possible Actions:\n",
        "- Open the primary artifact at .canon/artifacts/run-789/requirements/problem-statement.md directly when your host supports it.\n",
        "- Use $canon-inspect-artifacts for the full emitted packet on run run-789.\n",
        "- Use $canon-inspect-evidence only if you need lineage or policy rationale for run run-789.\n"
    );

    let shell_output = run_shell(
        "status-completed",
        "run-789",
        "",
        ".canon/artifacts/run-789/requirements/problem-statement.md",
    );
    assert_eq!(shell_output, expected);

    let powershell_output = run_powershell(
        "status-completed",
        "run-789",
        "",
        ".canon/artifacts/run-789/requirements/problem-statement.md",
    )
    .replace("\r\n", "\n");
    assert_eq!(powershell_output, expected);
}
