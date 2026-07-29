//! Frozen contract tests for Canon's stable human and one-shot machine surfaces.

use std::io::Write;
use std::process::{Command, Output, Stdio};

use canon_contracts::OneShotOperation;
use serde_json::{Value, json};
use tempfile::tempdir;

const STABLE_COMMANDS: [&str; 9] =
    ["init", "run", "resume", "status", "approve", "inspect", "publish", "assistant", "rpc"];
const STABLE_PROFILES: [&str; 9] = [
    "discovery",
    "requirements",
    "architecture",
    "backlog",
    "change",
    "refactor",
    "verification",
    "pr-review",
    "incident",
];
const ONE_SHOT_OPERATIONS: [(OneShotOperation, &str); 6] = [
    (OneShotOperation::Capabilities, "capabilities"),
    (OneShotOperation::Start, "start"),
    (OneShotOperation::Refresh, "refresh"),
    (OneShotOperation::Approve, "approve"),
    (OneShotOperation::Inspect, "inspect"),
    (OneShotOperation::Publish, "publish"),
];

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: impl Into<String>) -> TestResult {
    if condition { Ok(()) } else { Err(message.into().into()) }
}

fn canon_command() -> Result<Command, Box<dyn std::error::Error>> {
    let workspace = tempdir()?;
    let workspace_path = workspace.keep();
    let mut command = Command::new("cargo");
    command
        .args([
            "run",
            "--quiet",
            "--manifest-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
            "-p",
            "canon-cli",
            "--bin",
            "canon",
            "--",
        ])
        .current_dir(workspace_path);
    Ok(command)
}

fn run(args: &[&str]) -> Result<Output, Box<dyn std::error::Error>> {
    Ok(canon_command()?.args(args).output()?)
}

fn run_with_stdin(args: &[&str], request: &[u8]) -> Result<Output, Box<dyn std::error::Error>> {
    let mut child = canon_command()?
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin =
        child.stdin.take().ok_or_else(|| "Canon child stdin was unavailable".to_string())?;
    stdin.write_all(request)?;
    drop(stdin);
    Ok(child.wait_with_output()?)
}

fn utf8(bytes: Vec<u8>, channel: &str) -> Result<String, Box<dyn std::error::Error>> {
    String::from_utf8(bytes).map_err(|error| format!("{channel} was not UTF-8: {error}").into())
}

fn root_help_commands(help: &str) -> Vec<&str> {
    help.lines()
        .skip_while(|line| *line != "Commands:")
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .filter_map(|line| line.split_whitespace().next())
        .filter(|command| *command != "help")
        .collect()
}

fn parse_single_json(stdout: &[u8]) -> Result<Value, Box<dyn std::error::Error>> {
    let mut stream = serde_json::Deserializer::from_slice(stdout).into_iter::<Value>();
    let value = stream.next().ok_or_else(|| "stdout contained no JSON value".to_string())??;
    require(stream.next().is_none(), "stdout contained more than one JSON value")?;
    Ok(value)
}

#[test]
fn stable_root_help_is_exact_ordered_and_contains_no_aliases() -> TestResult {
    let output = run(&["--help"])?;
    require(output.status.success(), "root help failed")?;
    let stdout = utf8(output.stdout, "stdout")?;
    require(
        root_help_commands(&stdout) == STABLE_COMMANDS,
        format!("stable root command inventory drifted:\n{stdout}"),
    )?;
    for removed in [
        "verify",
        "skills",
        "governance",
        "pr-review",
        "list",
        "policy-shaping",
        "observability-design",
        "help-next",
    ] {
        require(
            !root_help_commands(&stdout).contains(&removed),
            format!("non-stable command `{removed}` leaked into root help"),
        )?;
    }
    Ok(())
}

#[test]
fn run_help_freezes_profile_spelling_and_excludes_implementation() -> TestResult {
    let output = run(&["run", "--help"])?;
    require(output.status.success(), "run help failed")?;
    let stdout = utf8(output.stdout, "stdout")?;
    require(stdout.contains("--profile <PROFILE>"), "run help did not expose --profile")?;
    require(!stdout.contains("--mode"), "legacy --mode leaked into stable run help")?;
    for profile in STABLE_PROFILES {
        require(stdout.contains(profile), format!("stable profile `{profile}` was absent"))?;
    }
    require(!stdout.contains("implementation"), "implementation leaked into stable run help")
}

#[test]
fn published_one_shot_registry_is_exact_and_independently_golden() -> TestResult {
    let actual = ONE_SHOT_OPERATIONS
        .iter()
        .map(|(operation, _)| serde_json::to_value(operation))
        .collect::<Result<Vec<_>, _>>()?;
    let expected = serde_json::from_str::<Vec<Value>>(
        r#"["capabilities","start","refresh","approve","inspect","publish"]"#,
    )?;
    require(actual == expected, "published one-shot operation registry drifted")?;
    require(
        ONE_SHOT_OPERATIONS.iter().map(|(_, wire)| *wire).collect::<Vec<_>>()
            == expected.iter().filter_map(Value::as_str).collect::<Vec<_>>(),
        "independent one-shot golden fixture drifted",
    )
}

#[test]
fn rpc_capabilities_emits_one_clean_response_and_terminates() -> TestResult {
    let request = serde_json::to_vec(&json!({
        "contract_version": "1.0",
        "request_id": "req-capabilities",
        "operation": "capabilities",
        "payload": {}
    }))?;
    let output = run_with_stdin(&["rpc", "--stdio"], &request)?;
    require(output.status.success(), format!("RPC failed with status {}", output.status))?;
    require(output.stderr.is_empty(), "successful RPC wrote diagnostics to stderr")?;
    let response = parse_single_json(&output.stdout)?;
    require(response["contract_version"] == "1.0", "response contract version drifted")?;
    require(response["request_id"] == "req-capabilities", "request identity was not preserved")?;
    require(
        response["result"]["operations"]
            == json!(["capabilities", "start", "refresh", "approve", "inspect", "publish"]),
        "capabilities operation inventory drifted",
    )?;
    let stdout = utf8(output.stdout, "stdout")?;
    for forbidden in ["\u{1b}[", "Debug", "/Users/", "token", "secret"] {
        require(
            !stdout.contains(forbidden),
            format!("machine stdout leaked forbidden content `{forbidden}`"),
        )?;
    }
    Ok(())
}

#[test]
fn rpc_framing_failures_are_one_typed_response_without_state_success() -> TestResult {
    for (label, input) in [
        ("empty", b"".as_slice()),
        ("whitespace", b" \n\t".as_slice()),
        ("malformed", br#"{"contract_version":"1.0""#.as_slice()),
        (
            "two-values",
            br#"{"contract_version":"1.0","request_id":"a","operation":"capabilities","payload":{}} {"contract_version":"1.0","request_id":"b","operation":"capabilities","payload":{}}"#
                .as_slice(),
        ),
        ("array", br#"[{"operation":"capabilities"}]"#.as_slice()),
    ] {
        let output = run_with_stdin(&["rpc", "--stdio"], input)?;
        require(!output.status.success(), format!("{label} framing unexpectedly succeeded"))?;
        let response = parse_single_json(&output.stdout)?;
        require(
            response["result"]["status"] == "rejected",
            format!("{label} framing did not return a typed rejection"),
        )?;
        require(
            !utf8(output.stderr, "stderr")?.contains(input.escape_ascii().to_string().as_str()),
            format!("{label} diagnostics echoed the request"),
        )?;
    }
    Ok(())
}

#[test]
fn mcp_transport_is_not_registered_by_t056() -> TestResult {
    let output = run(&["mcp", "--stdio"])?;
    require(!output.status.success(), "MCP transport was registered by T056")?;
    let stderr = utf8(output.stderr, "stderr")?;
    require(
        stderr.contains("unrecognized subcommand"),
        "MCP absence did not use the deterministic parser rejection",
    )
}
