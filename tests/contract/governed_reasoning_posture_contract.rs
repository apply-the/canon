use std::error::Error;
use std::fs;

const CANON_MANIFEST_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
const STABLE_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tech-docs/integration/governed-reasoning-posture-contract.md"
);
const FEATURE_LOCAL_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2.md"
);
const EXAMPLE_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-examples.md"
);
const MIGRATION_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-migration.md"
);
const RUNTIME_COMPATIBILITY_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml"
);
const ASSISTANT_PLUGIN_METADATA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/assistant/plugin-metadata.json");
const FIXTURE_ROOT: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/governed_reasoning_posture_v2");

const SUPPORTED_BOUNDLINE_VERSION: &str = "0.63.0";
const SUPPORTED_BOUNDLINE_WINDOW: &str = "0.63.x";
const SUPPORTED_BOUNDLINE_MAX_EXCLUSIVE: &str = "0.65.0";
const SUPPORTED_CANON_VERSION: &str = "0.65.0";
const SUPPORTED_CANON_WINDOW: &str = "0.64.x";
const SUPPORTED_CANON_MAX_EXCLUSIVE: &str = "0.65.0";
const SUPPORTED_CONTRACT_LINE: &str = "governed_reasoning_posture_v2";

const REQUIRED_TOP_LEVEL_FIELDS: [&str; 8] = [
    "contract_line",
    "schema_version",
    "publication_status",
    "compatibility_window",
    "profile_selector",
    "minimum_independence",
    "confidence_handoff",
    "provenance",
];
const SUPPORTED_SELECTOR_KINDS: [&str; 2] = ["profile_family", "profile_id"];
const SUPPORTED_REQUIRED_PROFILE_FAMILIES: [&str; 5] =
    ["self_consistency", "blind_review", "heterogeneous_review", "reflexion", "debate_enabled"];
const SUPPORTED_REQUIRED_PROFILE_IDS: [&str; 4] = [
    "bounded_self_consistency",
    "independent_pair_review",
    "heterogeneous_security_review",
    "bounded_reflexion",
];
const REQUIRED_MINIMUM_INDEPENDENCE_KEYS: [&str; 5] = [
    "route_distinct",
    "provider_distinct",
    "context_distinct",
    "prompt_pattern_distinct",
    "minimum_participants",
];
const SUPPORTED_CONFIDENCE_HANDOFF_STATES: [&str; 2] = ["none", "required"];
const SUPPORTED_REJECTION_MODES: [&str; 1] = ["fail_closed"];
const SUPPORTED_PROVENANCE_STATES: [&str; 2] = ["minimal", "evidence_backed"];
const SUPPORTED_REFERENCE_KINDS: [&str; 5] =
    ["packet", "artifact", "stable_doc", "validation_report", "fixture"];
const EXPECTED_EXAMPLE_IDS: [&str; 23] = [
    "valid-v2-posture",
    "invalid-selector-both-present",
    "invalid-selector-neither-present",
    "invalid-independence-missing-block",
    "invalid-independence-contradictory",
    "invalid-independence-impossible-minima",
    "invalid-independence-guidance-override",
    "invalid-confidence-missing-block",
    "invalid-confidence-none-contradictory",
    "invalid-confidence-required-missing-fields",
    "invalid-provenance-missing-block",
    "invalid-provenance-missing-reference-kind",
    "invalid-provenance-incompatible-handoff",
    "invalid-provenance-stale",
    "invalid-provenance-contradictory",
    "invalid-unsupported-vocabulary",
    "invalid-compatibility-window",
    "invalid-release-metadata-stale",
    "invalid-release-metadata-contradictory",
    "dual-line-coexistence-valid",
    "dual-line-coexistence-ambiguous",
    "migration-rejection-v2-to-v1-consumer",
    "migration-rejection-v1-to-v2-required",
];

const INVALID_TOML_FIXTURES: [(&str, &str); 16] = [
    ("invalid-selector-both-present.toml", "selector conflict"),
    ("invalid-selector-neither-present.toml", "selector missing"),
    ("invalid-independence-missing-block.toml", "minimum_independence missing"),
    ("invalid-independence-contradictory.toml", "independence requirements contradict each other"),
    (
        "invalid-independence-impossible-minima.toml",
        "independence requirements are impossible to satisfy",
    ),
    ("invalid-independence-guidance-override.toml", "guidance weakens hard minima"),
    ("invalid-confidence-missing-block.toml", "confidence_handoff missing"),
    (
        "invalid-confidence-none-contradictory.toml",
        "confidence_handoff none state carries contradictory fields",
    ),
    (
        "invalid-confidence-required-missing-fields.toml",
        "confidence_handoff required state is incomplete",
    ),
    ("invalid-provenance-missing-block.toml", "provenance missing"),
    ("invalid-provenance-missing-reference-kind.toml", "provenance reference kind missing"),
    (
        "invalid-provenance-incompatible-handoff.toml",
        "provenance incompatible with required confidence handoff",
    ),
    ("invalid-provenance-stale.toml", "provenance is stale"),
    ("invalid-provenance-contradictory.toml", "provenance is contradictory"),
    ("invalid-unsupported-vocabulary.toml", "unsupported vocabulary"),
    (
        "invalid-compatibility-window.toml",
        "compatibility window contradicts the published contract line",
    ),
];

const RELEASE_METADATA_FIXTURES: [(&str, &str); 2] = [
    ("invalid-release-metadata-stale.json", "release metadata is stale"),
    (
        "invalid-release-metadata-contradictory.json",
        "release metadata contradicts the published contract line",
    ),
];

const MIGRATION_FIXTURES: [(&str, &str, &str); 4] = [
    ("dual-line-coexistence-valid.toml", "accept", "one active line and one legacy line"),
    ("dual-line-coexistence-ambiguous.toml", "reject", "dual-line publication is ambiguous"),
    (
        "migration-rejection-v2-to-v1-consumer.toml",
        "reject",
        "v2 payload cannot be consumed by a v1-only consumer",
    ),
    (
        "migration-rejection-v1-to-v2-required.toml",
        "reject",
        "v1 payload cannot satisfy a v2-required workflow",
    ),
];

fn read_text(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

fn read_toml_fixture(file_name: &str) -> Result<toml::Value, Box<dyn Error>> {
    let path = format!("{FIXTURE_ROOT}/{file_name}");
    Ok(toml::from_str(&read_text(path.as_str())?)?)
}

fn read_json_fixture(file_name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let path = format!("{FIXTURE_ROOT}/{file_name}");
    Ok(serde_json::from_str(&read_text(path.as_str())?)?)
}

fn assert_contains(document: &str, expected: &str, context: &str) {
    assert!(document.contains(expected), "{context}: expected to find `{expected}`");
}

fn toml_value_at<'a>(value: &'a toml::Value, path: &[&str]) -> Option<&'a toml::Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn assert_toml_string(value: &toml::Value, path: &[&str], expected: &str, context: &str) {
    let actual = toml_value_at(value, path).and_then(toml::Value::as_str).unwrap_or_default();
    assert_eq!(actual, expected, "{context}");
}

fn assert_toml_array_contains(value: &toml::Value, path: &[&str], expected: &str, context: &str) {
    let found = toml_value_at(value, path)
        .and_then(toml::Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)));
    assert!(found, "{context}: expected `{expected}` in {:?}", path);
}

fn assert_json_string(value: &serde_json::Value, key: &str, expected: &str, context: &str) {
    let actual = value.get(key).and_then(serde_json::Value::as_str).unwrap_or_default();
    assert_eq!(actual, expected, "{context}");
}

#[test]
fn governed_reasoning_posture_contract_and_feature_briefs_publish_v2_identity()
-> Result<(), Box<dyn Error>> {
    let contract = read_text(STABLE_CONTRACT_PATH)?;
    let brief = read_text(FEATURE_LOCAL_CONTRACT_PATH)?;

    for document in [&contract, &brief] {
        assert_contains(
            document,
            SUPPORTED_CONTRACT_LINE,
            "Governed reasoning posture contract surfaces should publish the v2 contract line",
        );
        assert_contains(
            document,
            SUPPORTED_BOUNDLINE_WINDOW,
            "Governed reasoning posture contract surfaces should publish the supported Boundline window",
        );
        assert_contains(
            document,
            SUPPORTED_CANON_WINDOW,
            "Governed reasoning posture contract surfaces should publish the supported Canon window",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_contract_publishes_required_v2_shape_and_vocabulary()
-> Result<(), Box<dyn Error>> {
    let contract = read_text(STABLE_CONTRACT_PATH)?;

    for required_field in REQUIRED_TOP_LEVEL_FIELDS {
        assert_contains(
            &contract,
            required_field,
            "Canonical reasoning posture contract should publish every required v2 top-level field",
        );
    }
    for selector_kind in SUPPORTED_SELECTOR_KINDS {
        assert_contains(
            &contract,
            selector_kind,
            "Canonical reasoning posture contract should publish every supported selector kind",
        );
    }
    for profile_family in SUPPORTED_REQUIRED_PROFILE_FAMILIES {
        assert_contains(
            &contract,
            profile_family,
            "Canonical reasoning posture contract should publish every supported profile family",
        );
    }
    for profile_id in SUPPORTED_REQUIRED_PROFILE_IDS {
        assert_contains(
            &contract,
            profile_id,
            "Canonical reasoning posture contract should publish every supported profile id",
        );
    }
    for independence_key in REQUIRED_MINIMUM_INDEPENDENCE_KEYS {
        assert_contains(
            &contract,
            independence_key,
            "Canonical reasoning posture contract should publish the full hard-minimum independence shape",
        );
    }
    for state in SUPPORTED_CONFIDENCE_HANDOFF_STATES {
        assert_contains(
            &contract,
            state,
            "Canonical reasoning posture contract should publish every supported confidence-handoff state",
        );
    }
    for rejection_mode in SUPPORTED_REJECTION_MODES {
        assert_contains(
            &contract,
            rejection_mode,
            "Canonical reasoning posture contract should publish every supported rejection mode",
        );
    }
    for state in SUPPORTED_PROVENANCE_STATES {
        assert_contains(
            &contract,
            state,
            "Canonical reasoning posture contract should publish every supported provenance state",
        );
    }
    for reference_kind in SUPPORTED_REFERENCE_KINDS {
        assert_contains(
            &contract,
            reference_kind,
            "Canonical reasoning posture contract should publish every supported provenance reference kind",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_example_and_migration_contracts_list_required_fixture_inventory()
-> Result<(), Box<dyn Error>> {
    let examples = read_text(EXAMPLE_CONTRACT_PATH)?;
    let migration = read_text(MIGRATION_CONTRACT_PATH)?;

    for example_id in EXPECTED_EXAMPLE_IDS {
        assert_contains(
            &examples,
            example_id,
            "Example contract should list every required fixture id",
        );
    }

    for expected in ["active", "legacy", "v1-only consumer", "v2-required workflow"] {
        assert_contains(
            &migration,
            expected,
            "Migration contract should publish active-versus-legacy and incompatibility rules",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_release_surfaces_align_on_v2_workspace_version()
-> Result<(), Box<dyn Error>> {
    let manifest = read_text(CANON_MANIFEST_PATH)?;
    let runtime_compatibility = read_text(RUNTIME_COMPATIBILITY_PATH)?;
    let plugin_metadata = read_text(ASSISTANT_PLUGIN_METADATA_PATH)?;

    let manifest_version_entry = format!("version = \"{SUPPORTED_CANON_VERSION}\"");
    let expected_workspace_version_entry =
        format!("expected_workspace_version = \"{SUPPORTED_CANON_VERSION}\"");
    let plugin_version_entry = format!("\"version\": \"{SUPPORTED_CANON_VERSION}\"");

    assert_contains(
        &manifest,
        manifest_version_entry.as_str(),
        "Cargo workspace version should align with the v2 Canon release",
    );
    assert_contains(
        &runtime_compatibility,
        expected_workspace_version_entry.as_str(),
        "Runtime compatibility guidance should align with the v2 Canon release",
    );
    assert_contains(
        &plugin_metadata,
        plugin_version_entry.as_str(),
        "Assistant plugin metadata should align with the v2 Canon release",
    );

    Ok(())
}

#[test]
fn governed_reasoning_posture_valid_fixture_declares_acceptance_metadata_and_shape()
-> Result<(), Box<dyn Error>> {
    let fixture = read_toml_fixture("valid-v2-posture.toml")?;

    assert_toml_string(
        &fixture,
        &["example_id"],
        "valid-v2-posture",
        "Valid fixture should declare its example id",
    );
    assert_toml_string(
        &fixture,
        &["expected_validation_result"],
        "accept",
        "Valid fixture should declare an accept result",
    );
    assert_toml_string(
        &fixture,
        &["contract_line"],
        SUPPORTED_CONTRACT_LINE,
        "Valid fixture should target the v2 contract line",
    );
    assert_toml_string(
        &fixture,
        &["schema_version"],
        "v2",
        "Valid fixture should declare schema version v2",
    );
    assert_toml_string(
        &fixture,
        &["publication_status"],
        "active",
        "Valid fixture should declare active publication status",
    );
    assert_toml_string(
        &fixture,
        &["compatibility_window", "boundline_min"],
        SUPPORTED_BOUNDLINE_VERSION,
        "Valid fixture should declare the expected Boundline minimum version",
    );
    assert_toml_string(
        &fixture,
        &["compatibility_window", "boundline_max_exclusive"],
        SUPPORTED_BOUNDLINE_MAX_EXCLUSIVE,
        "Valid fixture should declare the expected Boundline max-exclusive version",
    );
    assert_toml_string(
        &fixture,
        &["compatibility_window", "canon_max_exclusive"],
        SUPPORTED_CANON_MAX_EXCLUSIVE,
        "Valid fixture should declare the expected Canon max-exclusive version",
    );
    assert_toml_string(
        &fixture,
        &["profile_selector", "selector_kind"],
        "profile_family",
        "Valid fixture should declare a profile-family selector kind",
    );
    assert_toml_string(
        &fixture,
        &["confidence_handoff", "state"],
        "required",
        "Valid fixture should declare required confidence handoff when evidence is present",
    );
    assert_toml_string(
        &fixture,
        &["provenance", "state"],
        "evidence_backed",
        "Valid fixture should declare evidence-backed provenance",
    );
    assert_toml_array_contains(
        &fixture,
        &["contract_lines_involved"],
        SUPPORTED_CONTRACT_LINE,
        "Valid fixture should declare the involved contract line",
    );

    Ok(())
}

#[test]
fn governed_reasoning_posture_invalid_toml_fixtures_declare_reject_metadata()
-> Result<(), Box<dyn Error>> {
    for (file_name, expected_reason) in INVALID_TOML_FIXTURES {
        let fixture = read_toml_fixture(file_name)?;

        assert_toml_string(
            &fixture,
            &["expected_validation_result"],
            "reject",
            "Invalid TOML fixtures should declare a reject result",
        );
        let actual_reason = toml_value_at(&fixture, &["expected_reason"])
            .and_then(toml::Value::as_str)
            .unwrap_or_default();
        assert!(
            actual_reason.contains(expected_reason),
            "{file_name}: expected reject reason to contain `{expected_reason}`, found `{actual_reason}`"
        );
        assert_toml_array_contains(
            &fixture,
            &["contract_lines_involved"],
            SUPPORTED_CONTRACT_LINE,
            "Invalid TOML fixtures should declare the v2 contract line",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_release_metadata_fixtures_declare_reject_metadata()
-> Result<(), Box<dyn Error>> {
    for (file_name, expected_reason) in RELEASE_METADATA_FIXTURES {
        let fixture = read_json_fixture(file_name)?;

        assert_json_string(
            &fixture,
            "expected_validation_result",
            "reject",
            "Release metadata fixtures should declare a reject result",
        );
        let actual_reason =
            fixture.get("expected_reason").and_then(serde_json::Value::as_str).unwrap_or_default();
        assert!(
            actual_reason.contains(expected_reason),
            "{file_name}: expected reject reason to contain `{expected_reason}`, found `{actual_reason}`"
        );
        assert_json_string(
            &fixture,
            "contract_line",
            SUPPORTED_CONTRACT_LINE,
            "Release metadata fixtures should declare the v2 contract line",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_migration_fixtures_declare_expected_results()
-> Result<(), Box<dyn Error>> {
    for (file_name, expected_result, expected_reason) in MIGRATION_FIXTURES {
        let fixture = read_toml_fixture(file_name)?;

        assert_toml_string(
            &fixture,
            &["expected_validation_result"],
            expected_result,
            "Migration fixtures should declare their expected validation result",
        );
        let actual_reason = toml_value_at(&fixture, &["expected_reason"])
            .and_then(toml::Value::as_str)
            .unwrap_or_default();
        assert!(
            actual_reason.contains(expected_reason),
            "{file_name}: expected reason to contain `{expected_reason}`, found `{actual_reason}`"
        );
    }

    Ok(())
}
