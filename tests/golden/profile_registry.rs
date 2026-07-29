//! Golden contract for Canon's frozen nine-profile stable registry.

use std::{collections::BTreeSet, process::Command};

use canon_contracts::{Profile, StableProfileRegistry as ContractProfileRegistry};
use canon_engine::{
    EngineService, InspectTarget,
    modes::{ProfileRegistryError, stable_profile_registry},
};
use tempfile::tempdir;

const EXPECTED_IDS: [&str; 9] = [
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

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: &str) -> TestResult {
    if condition { Ok(()) } else { Err(message.to_string().into()) }
}

#[test]
fn stable_registry_is_exact_ordered_unique_and_contract_compatible() -> TestResult {
    let registry = stable_profile_registry();
    let ids = registry.iter().map(|entry| entry.id()).collect::<Vec<_>>();
    require(ids == EXPECTED_IDS, "stable registry IDs or order changed")?;
    require(
        ids.iter().copied().collect::<BTreeSet<_>>().len() == EXPECTED_IDS.len(),
        "stable registry contains a duplicate ID",
    )?;

    let contract_profiles = ContractProfileRegistry::profiles();
    require(
        registry.iter().map(|entry| entry.contract_profile()).eq(contract_profiles.iter().copied()),
        "engine registry diverges from canon-contracts 0.90.0",
    )?;

    let serialized = serde_json::to_string(
        &registry.iter().map(|entry| entry.contract_profile()).collect::<Vec<Profile>>(),
    )?;
    require(
        serialized
            == r#"["discovery","requirements","architecture","backlog","change","refactor","verification","pr-review","incident"]"#,
        "canonical profile serialization changed",
    )
}

#[test]
fn stable_parser_accepts_only_exact_canonical_ids() -> TestResult {
    let registry = stable_profile_registry();
    for expected in EXPECTED_IDS {
        let parsed = registry.parse(expected)?;
        require(parsed.id() == expected, "canonical profile did not round trip")?;
    }

    for rejected in [
        "implementation",
        "Implementation",
        " requirements",
        "requirements ",
        "pr_review",
        "review",
        "unknown",
        "",
    ] {
        let result = registry.parse(rejected);
        require(
            matches!(result, Err(ProfileRegistryError::UnsupportedProfile { .. })),
            "noncanonical or nonstable profile was admitted",
        )?;
    }
    Ok(())
}

#[test]
fn stable_inspection_uses_the_same_registry_and_excludes_implementation() -> TestResult {
    let service = EngineService::new(env!("CARGO_MANIFEST_DIR"));
    let response = service.inspect(InspectTarget::Modes)?;
    let projected = serde_json::to_value(response)?;
    let entries = projected
        .get("entries")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "modes inspection did not contain an entries array".to_string())?;
    let ids = entries
        .iter()
        .map(|entry| entry.as_str().ok_or_else(|| "mode entry was not a string".to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    require(ids == EXPECTED_IDS, "inspection diverges from the stable registry")?;
    require(!ids.contains(&"implementation"), "implementation leaked into stable inspection")
}

#[test]
fn profile_selection_is_governance_only_and_has_no_execution_profile() -> TestResult {
    let registry = stable_profile_registry();
    let change = registry.parse("change")?;
    let verification = registry.parse("verification")?;

    require(
        change.governs_change_intent(),
        "change must govern intent, scope, risk, invariants, and evidence",
    )?;
    require(
        verification.governs_evidence_requirements(),
        "verification must govern evidence requirements",
    )?;
    require(
        registry.iter().all(|entry| !entry.executes_implementation()),
        "a Canon stable profile claims implementation authority",
    )
}

#[test]
fn stable_cli_rejects_implementation_without_creating_runtime_state() -> TestResult {
    let workspace = tempdir()?;
    let output = Command::new("cargo")
        .current_dir(workspace.path())
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
            "run",
            "--mode",
            "implementation",
            "--risk",
            "low-impact",
            "--zone",
            "green",
            "--owner",
            "profile-test",
        ])
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;

    require(!output.status.success(), "implementation was admitted as a new stable run")?;
    require(
        stderr.contains("unsupported stable profile: implementation"),
        "implementation rejection did not use the stable registry reason",
    )?;
    require(
        !workspace.path().join(".canon").exists(),
        "rejected implementation admission created runtime state",
    )
}

#[test]
fn stable_run_help_does_not_advertise_implementation_or_legacy_profiles() -> TestResult {
    let output = Command::new("cargo")
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
            "run",
            "--help",
        ])
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;

    require(output.status.success(), "stable run help failed")?;
    for forbidden in ["implementation", "system-shaping", "migration"] {
        require(!stdout.contains(forbidden), "stable run help advertised a nonstable profile")?;
    }
    Ok(())
}
