//! Frozen contract tests for Canon's stable human and one-shot machine surfaces.

use std::io::Write;
use std::process::{Command, Output, Stdio};

use canon_contracts::{OneShotOperation, RecordOutcomeResponse};
use serde_json::{Value, json};
use tempfile::tempdir;

#[path = "stable_surface/fixture.rs"]
mod fixture;

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
const HISTORICAL_ONE_SHOT_OPERATIONS: [(OneShotOperation, &str); 6] = [
    (OneShotOperation::Capabilities, "capabilities"),
    (OneShotOperation::Start, "start"),
    (OneShotOperation::Refresh, "refresh"),
    (OneShotOperation::Approve, "approve"),
    (OneShotOperation::Inspect, "inspect"),
    (OneShotOperation::Publish, "publish"),
];
const ONE_SHOT_OPERATIONS: [(OneShotOperation, &str); 7] = [
    (OneShotOperation::Capabilities, "capabilities"),
    (OneShotOperation::Start, "start"),
    (OneShotOperation::Refresh, "refresh"),
    (OneShotOperation::Approve, "approve"),
    (OneShotOperation::Inspect, "inspect"),
    (OneShotOperation::Publish, "publish"),
    (OneShotOperation::RecordOutcome, "record_outcome"),
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
fn assistant_help_exposes_only_install() -> TestResult {
    let output = run(&["assistant", "--help"])?;
    require(output.status.success(), "assistant help failed")?;
    let stdout = utf8(output.stdout, "stdout")?;
    require(
        root_help_commands(&stdout) == ["install"],
        format!("stable assistant command inventory drifted:\n{stdout}"),
    )
}

#[test]
fn published_one_shot_registry_is_exact_and_independently_golden() -> TestResult {
    let actual = ONE_SHOT_OPERATIONS
        .iter()
        .map(|(operation, _)| serde_json::to_value(operation))
        .collect::<Result<Vec<_>, _>>()?;
    let expected = serde_json::from_str::<Vec<Value>>(
        r#"["capabilities","start","refresh","approve","inspect","publish","record_outcome"]"#,
    )?;
    require(actual == expected, "published one-shot operation registry drifted")?;
    require(
        ONE_SHOT_OPERATIONS.iter().map(|(_, wire)| *wire).collect::<Vec<_>>()
            == expected.iter().filter_map(Value::as_str).collect::<Vec<_>>(),
        "independent one-shot golden fixture drifted",
    )?;
    require(
        HISTORICAL_ONE_SHOT_OPERATIONS.iter().map(|(_, wire)| *wire).collect::<Vec<_>>()
            == ["capabilities", "start", "refresh", "approve", "inspect", "publish"],
        "historical 0.90 operation inventory drifted",
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
            == json!([
                "capabilities",
                "start",
                "refresh",
                "approve",
                "inspect",
                "publish",
                "record_outcome"
            ]),
        "capabilities operation inventory drifted",
    )?;
    require(
        response["result"]["operation_capabilities"]
            == json!([
                {"operation": "capabilities", "available": true, "reason_code": null},
                {"operation": "start", "available": true, "reason_code": null},
                {"operation": "refresh", "available": true, "reason_code": null},
                {"operation": "approve", "available": true, "reason_code": null},
                {"operation": "inspect", "available": true, "reason_code": null},
                {"operation": "publish", "available": true, "reason_code": null},
                {
                    "operation": "record_outcome",
                    "available": false,
                    "reason_code": "unsupported_operation"
                }
            ]),
        "capabilities did not expose typed record_outcome readiness",
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
        if !input.is_empty() {
            require(
                !utf8(output.stderr, "stderr")?.contains(input.escape_ascii().to_string().as_str()),
                format!("{label} diagnostics echoed the request"),
            )?;
        }
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

#[test]
fn historical_six_rpc_operations_keep_their_deterministic_handlers() -> TestResult {
    let fixture = fixture::RpcFixture::new()?;
    let draft = fixture::governance_draft("bundle-rpc-six", 1);

    let capabilities = fixture.invoke("req-capabilities", "capabilities", json!({}))?;
    require(
        capabilities["result"]["operations"]
            == json!([
                "capabilities",
                "start",
                "refresh",
                "approve",
                "inspect",
                "publish",
                "record_outcome"
            ]),
        "capabilities did not expose the additive seven-operation registry",
    )?;

    let started = fixture.invoke(
        "bundle-rpc-six",
        "start",
        json!({"bundle": serde_json::to_value(&draft)?}),
    )?;
    require(started["result"]["terminal_status"] == "accepted", "start was not accepted")?;
    require(
        started["result"]["execution_audit"]
            == json!({
                "process_invocations": 0,
                "network_invocations": 0,
                "provider_credential_reads": 0,
                "model_calls": 0,
                "semantic_evidence_created": 0
            }),
        "start violated the zero-execution audit",
    )?;

    let snapshot_before_reads = fixture.snapshot_bytes()?;
    for operation in ["refresh", "inspect", "publish"] {
        let response = fixture.invoke(&format!("req-{operation}"), operation, json!({}))?;
        require(
            response["result"]["terminal_status"] == "accepted",
            format!("{operation} did not return the accepted terminal projection"),
        )?;
        require(
            fixture.snapshot_bytes()? == snapshot_before_reads,
            format!("read-only operation `{operation}` mutated decision memory"),
        )?;
    }

    let approval_fixture = fixture::RpcFixture::new()?;
    let approved = approval_fixture.invoke(
        "bundle-rpc-approve",
        "approve",
        json!({"bundle": fixture::governance_draft("bundle-rpc-approve", 1)}),
    )?;
    require(approved["result"]["terminal_status"] == "accepted", "approve was not accepted")
}

#[test]
fn record_outcome_is_discoverable_but_typed_unavailable_before_t059() -> TestResult {
    let fixture = fixture::RpcFixture::new()?;
    let request = fixture::record_outcome_request()?;
    let event_id = request.event_id.clone();
    let event_digest = request.event_digest.clone();
    let (success, response) = fixture.invoke_with_status(
        event_id.as_str(),
        "record_outcome",
        json!({"outcome": request}),
    )?;
    require(!success, "pre-T059 record_outcome returned process success")?;
    let typed = serde_json::from_value::<RecordOutcomeResponse>(response["result"].clone())?;
    require(typed.event_id == event_id, "typed rejection changed event identity")?;
    require(typed.event_digest == event_digest, "typed rejection changed event digest")?;
    require(
        typed.disposition == canon_contracts::RecordOutcomeDisposition::Rejected,
        "pre-T059 disposition was not rejected",
    )?;
    require(
        typed.reason_code
            == Some(canon_contracts::RecordOutcomeRejectionReason::UnsupportedOperation),
        "pre-T059 rejection reason drifted",
    )?;
    require(
        typed.decision_memory_revision.is_none() && typed.decision_memory_digest.is_none(),
        "pre-T059 rejection invented decision memory",
    )?;
    require(!fixture.has_snapshot(), "pre-T059 rejection created a decision-memory snapshot")
}

#[test]
fn mutation_replay_is_idempotent_and_digest_conflict_fails_closed() -> TestResult {
    let fixture = fixture::RpcFixture::new()?;
    let draft = fixture::governance_draft("bundle-rpc-replay", 1);
    let payload = json!({"bundle": serde_json::to_value(&draft)?});

    let first = fixture.invoke("bundle-rpc-replay", "start", payload.clone())?;
    let snapshot = fixture.snapshot_bytes()?;
    let replay = fixture.invoke("bundle-rpc-replay", "start", payload)?;
    require(first["result"]["graph_digest"] == replay["result"]["graph_digest"], "replay drifted")?;
    require(replay["result"]["replayed"] == true, "matching retry was not marked replayed")?;
    require(fixture.snapshot_bytes()? == snapshot, "matching retry rewrote decision memory")?;

    let changed = fixture::governance_draft("bundle-rpc-replay", 2);
    let conflict =
        fixture.invoke_rejected("bundle-rpc-replay", "start", json!({"bundle": changed}))?;
    require(
        conflict["result"]["reason_code"] == "identity_digest_conflict",
        "changed digest did not fail with identity_digest_conflict",
    )?;
    require(fixture.snapshot_bytes()? == snapshot, "conflict changed the original state")
}

#[test]
fn rpc_rejects_unknown_fields_operations_and_request_identity_mismatch() -> TestResult {
    let fixture = fixture::RpcFixture::new()?;
    let unknown = fixture.invoke_rejected("req-unknown", "execute", json!({}))?;
    require(
        unknown["result"]["reason_code"] == "unsupported_operation",
        "unknown operation did not use the stable unsupported reason",
    )?;

    let extra = fixture.invoke_raw_rejected(json!({
        "contract_version": "1.0",
        "request_id": "req-extra",
        "operation": "capabilities",
        "payload": {},
        "authority": "inferred"
    }))?;
    require(
        extra["result"]["reason_code"] == "invalid_input",
        "unknown envelope field did not fail strict decoding",
    )?;

    let mismatched = fixture.invoke_rejected(
        "different-request-id",
        "start",
        json!({"bundle": fixture::governance_draft("bundle-identity", 1)}),
    )?;
    require(
        mismatched["result"]["reason_code"] == "identity_digest_conflict",
        "mutation request identity was not bound to its bundle",
    )
}

#[test]
fn cli_and_rpc_share_terminal_and_decision_memory_projections() -> TestResult {
    let cli_first = fixture::RpcFixture::new()?;
    let draft = fixture::governance_draft("bundle-cli-first", 1);
    let cli_result = cli_first.cli_run(&draft)?;
    let rpc_inspect = cli_first.invoke("req-inspect-cli-first", "inspect", json!({}))?;
    require(
        cli_result["terminal_status"] == rpc_inspect["result"]["terminal_status"],
        "CLI and RPC terminal status diverged",
    )?;
    require(
        cli_result["graph_digest"] == rpc_inspect["result"]["graph_digest"],
        "CLI and RPC graph digest diverged",
    )?;
    require(
        cli_result["decision_memory"] == rpc_inspect["result"]["decision_memory"],
        "CLI and RPC decision-memory projections diverged",
    )?;

    let rpc_first = fixture::RpcFixture::new()?;
    let rpc_result = rpc_first.invoke(
        "bundle-rpc-first",
        "start",
        json!({"bundle": fixture::governance_draft("bundle-rpc-first", 1)}),
    )?;
    let cli_inspect = rpc_first.cli_inspect()?;
    require(
        rpc_result["result"]["graph_digest"] == cli_inspect["graph_digest"],
        "RPC mutation was not visible to CLI inspect",
    )
}

#[test]
fn rpc_rejects_oversized_invalid_utf8_and_semantic_execution_requests() -> TestResult {
    let oversized = vec![b'x'; 1_048_577];
    let output = run_with_stdin(&["rpc", "--stdio"], &oversized)?;
    require(!output.status.success(), "oversized input unexpectedly succeeded")?;
    require(
        parse_single_json(&output.stdout)?["result"]["reason_code"] == "invalid_input",
        "oversized input did not use invalid_input",
    )?;

    let invalid_utf8 = [0xff, 0xfe, 0xfd];
    let output = run_with_stdin(&["rpc", "--stdio"], &invalid_utf8)?;
    require(!output.status.success(), "invalid UTF-8 unexpectedly succeeded")?;
    require(
        parse_single_json(&output.stdout)?["result"]["reason_code"] == "invalid_input",
        "invalid UTF-8 did not use invalid_input",
    )?;

    let fixture = fixture::RpcFixture::new()?;
    let rejected = fixture.invoke_raw_rejected(json!({
        "contract_version": "1.0",
        "request_id": "req-semantic",
        "operation": "start",
        "payload": {
            "semantic_execution": true,
            "bundle": fixture::governance_draft("req-semantic", 1)
        }
    }))?;
    require(
        rejected["result"]["reason_code"] == "invalid_input",
        "semantic execution request did not fail strict decoding",
    )
}

#[test]
fn terminal_governance_failures_use_frozen_non_success_exit_codes() -> TestResult {
    let authority_fixture = fixture::RpcFixture::new()?;
    let mut missing_authority = fixture::governance_draft("bundle-no-authority", 1);
    missing_authority.approvals.clear();
    missing_authority.risk_acceptances.clear();
    let (success, blocked) = authority_fixture.invoke_with_status(
        "bundle-no-authority",
        "start",
        json!({"bundle": missing_authority}),
    )?;
    require(!success, "authority-denied governance returned process success")?;
    require(
        blocked["result"]["terminal_status"] == "blocked",
        "authority denial did not preserve the kernel status",
    )?;

    let evidence_fixture = fixture::RpcFixture::new()?;
    let mut missing_evidence = fixture::governance_draft("bundle-no-evidence", 1);
    if let Some(evidence) = missing_evidence.provided_evidence.first_mut() {
        evidence.challenge_tier = canon_contracts::ChallengeTier::Tier1;
    }
    let (success, required_missing) = evidence_fixture.invoke_with_status(
        "bundle-no-evidence",
        "start",
        json!({"bundle": missing_evidence}),
    )?;
    require(!success, "missing evidence returned process success")?;
    require(
        required_missing["result"]["terminal_status"] == "required_missing",
        format!("missing evidence did not preserve the kernel status: {required_missing}"),
    )
}

#[test]
fn persistence_failure_is_typed_and_records_no_terminal_success() -> TestResult {
    let fixture = fixture::RpcFixture::new()?;
    fixture.obstruct_state_root()?;
    let (success, response) = fixture.invoke_with_status(
        "bundle-persistence-failure",
        "start",
        json!({"bundle": fixture::governance_draft("bundle-persistence-failure", 1)}),
    )?;
    require(!success, "persistence failure returned process success")?;
    require(
        response["result"]["reason_code"] == "persistence_failure",
        format!("persistence failure reason drifted: {response}"),
    )
}
