use std::process::Command as ProcessCommand;

fn script_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}

fn run_shell(profile: &str, run_id: &str, target: &str) -> String {
    let output = ProcessCommand::new("/bin/bash")
        .arg(script_path(".agents/skills/canon-shared/scripts/render-next-steps.sh"))
        .args(["--profile", profile, "--run-id", run_id, "--target", target])
        .output()
        .expect("run shell renderer");

    assert!(
        output.status.success(),
        "shell renderer failed: stdout=`{}` stderr=`{}`",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("shell output utf8")
}

fn run_powershell(profile: &str, run_id: &str, target: &str) -> String {
    let output = ProcessCommand::new("pwsh")
        .args([
            "-File",
            &script_path(".agents/skills/canon-shared/scripts/render-next-steps.ps1"),
            "-Profile",
            profile,
            "-RunId",
            run_id,
            "-Target",
            target,
        ])
        .output()
        .expect("run powershell renderer");

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

    let shell_output = run_shell("gated", "run-123", "gate:review-disposition");
    assert_eq!(shell_output, expected);

    let powershell_output =
        run_powershell("gated", "run-123", "gate:review-disposition").replace("\r\n", "\n");
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

    let shell_output = run_shell("approval-recorded", "run-456", "");
    assert_eq!(shell_output, expected);

    let powershell_output =
        run_powershell("approval-recorded", "run-456", "").replace("\r\n", "\n");
    assert_eq!(powershell_output, expected);
}
