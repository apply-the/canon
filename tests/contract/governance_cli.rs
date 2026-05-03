use assert_cmd::Command;
use predicates::str::contains;
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

#[test]
fn governance_help_lists_supported_operations() {
    cli_command()
        .args(["governance", "--help"])
        .assert()
        .success()
        .stdout(contains("start"))
        .stdout(contains("refresh"))
        .stdout(contains("capabilities"));
}

#[test]
fn governance_capabilities_reports_v1_machine_contract() {
    let output = cli_command()
        .args(["governance", "capabilities", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("capabilities json");

    assert_eq!(json["supported_schema_versions"], serde_json::json!(["v1"]));
    assert_eq!(json["operations"], serde_json::json!(["start", "refresh", "capabilities"]));
    assert!(json["supported_modes"].as_array().is_some_and(|modes| !modes.is_empty()));
    assert_eq!(
        json["status_values"],
        serde_json::json!([
            "pending_selection",
            "running",
            "governed_ready",
            "awaiting_approval",
            "blocked",
            "completed",
            "failed"
        ])
    );
    assert_eq!(
        json["approval_state_values"],
        serde_json::json!(["not_needed", "requested", "granted", "rejected", "expired"])
    );
    assert_eq!(
        json["packet_readiness_values"],
        serde_json::json!(["pending", "incomplete", "reusable", "rejected"])
    );
    assert_eq!(
        json["compatibility_notes"],
        serde_json::json!([
            "The governance adapter is the machine-facing boundary around the same Canon runtime used by the human CLI.",
            "Canon is not the higher-level orchestrator; requests that omit adapter_schema_version are interpreted as v1 and unknown additive fields are ignored within supported schema versions."
        ])
    );
    assert!(json["canon_version"].as_str().is_some_and(|value| !value.is_empty()));
}

#[test]
fn governance_start_blocks_well_formed_requests_with_missing_domain_fields() {
    let workspace = TempDir::new().expect("temp dir");

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "start", "--json"])
        .write_stdin(
            r#"{
  "request_kind": "start",
  "governance_attempt_id": "ga-001",
  "stage_key": "analysis"
}"#,
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("blocked json");

    assert_eq!(json["adapter_schema_version"], "v1");
    assert_eq!(json["status"], "blocked");
    assert_eq!(json["approval_state"], "not_needed");
    assert_eq!(json["reason_code"], "missing_required_field");
    assert!(json["message"].as_str().is_some_and(|value| value.contains("missing")));
    assert_eq!(
        json["missing_fields"],
        serde_json::json!([
            "goal",
            "workspace_ref",
            "mode",
            "system_context",
            "risk",
            "zone",
            "owner"
        ])
    );
    assert!(json.get("missing_sections").is_none());
}

#[test]
fn governance_refresh_returns_failed_outcome_for_unknown_run_ref() {
    let workspace = TempDir::new().expect("temp dir");
    let workspace_ref = workspace.path().to_string_lossy().into_owned();
    let request = serde_json::json!({
        "request_kind": "refresh",
        "governance_attempt_id": "ga-002",
        "stage_key": "verification",
        "goal": "Refresh the governed packet",
        "workspace_ref": workspace_ref,
        "mode": "verification",
        "system_context": "existing",
        "risk": "bounded-impact",
        "zone": "yellow",
        "owner": "staff-engineer",
        "run_ref": "R-20260502-deadbeef"
    });

    let output = cli_command()
        .current_dir(workspace.path())
        .args(["governance", "refresh", "--json"])
        .write_stdin(request.to_string())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("refresh json");

    assert_eq!(json["adapter_schema_version"], "v1");
    assert_eq!(json["status"], "failed");
    assert_eq!(json["approval_state"], "not_needed");
    assert_eq!(json["reason_code"], "run_not_found");
    assert_eq!(json["run_ref"], "R-20260502-deadbeef");
}
