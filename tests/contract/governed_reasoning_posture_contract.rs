use std::error::Error;
use std::fs;

const CANON_MANIFEST_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
const STABLE_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/docs/integration/governed-reasoning-posture-contract.md");
const FEATURE_LOCAL_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md"
);
const SUPPORTED_BOUNDLINE_VERSION: &str = "0.62.0";
const SUPPORTED_BOUNDLINE_WINDOW: &str = "0.62.x";
const SUPPORTED_BOUNDLINE_MAX_EXCLUSIVE: &str = "0.63.0";
const SUPPORTED_CANON_VERSION: &str = "0.58.0";
const SUPPORTED_CANON_WINDOW: &str = "0.58.x";
const SUPPORTED_CANON_MAX_EXCLUSIVE: &str = "0.59.0";
const SUPPORTED_CONTRACT_LINE: &str = "governed_reasoning_posture_v1";
const SUPPORTED_REQUIRED_PROFILE_FAMILIES: [&str; 5] =
    ["self_consistency", "blind_review", "heterogeneous_review", "reflexion", "debate_enabled"];
const SUPPORTED_REQUIRED_PROFILE_IDS: [&str; 4] = [
    "bounded_self_consistency",
    "independent_pair_review",
    "heterogeneous_security_review",
    "bounded_reflexion",
];
const SUPPORTED_ADMISSION_PRIORITIES: [&str; 3] =
    ["advisory", "required_before_continue", "required_before_acceptance"];
const REQUIRED_MINIMUM_INDEPENDENCE_KEYS: [&str; 5] = [
    "route_distinct",
    "provider_distinct",
    "context_distinct",
    "prompt_pattern_distinct",
    "minimum_participants",
];

fn read_text(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

fn assert_contains(document: &str, expected: &str, context: &str) {
    assert!(document.contains(expected), "{context}: expected to find `{expected}`");
}

#[test]
fn governed_reasoning_posture_feature_brief_declares_supported_release_pair()
-> Result<(), Box<dyn Error>> {
    let brief = read_text(FEATURE_LOCAL_CONTRACT_PATH)?;

    assert_contains(
        &brief,
        SUPPORTED_CONTRACT_LINE,
        "Feature-local Canon brief should declare the shared contract line",
    );
    assert_contains(
        &brief,
        SUPPORTED_BOUNDLINE_WINDOW,
        "Feature-local Canon brief should declare the supported Boundline window",
    );
    assert_contains(
        &brief,
        SUPPORTED_CANON_WINDOW,
        "Feature-local Canon brief should declare the supported Canon window",
    );

    Ok(())
}

#[test]
fn governed_reasoning_posture_alignment_matches_workspace_version_and_exact_contract_pair()
-> Result<(), Box<dyn Error>> {
    let canon_manifest = read_text(CANON_MANIFEST_PATH)?;
    let contract = read_text(STABLE_CONTRACT_PATH)?;
    let canon_version_entry = format!("version = \"{SUPPORTED_CANON_VERSION}\"");
    let boundline_min_entry = format!("boundline_min = \"{SUPPORTED_BOUNDLINE_VERSION}\"");
    let boundline_max_entry =
        format!("boundline_max_exclusive = \"{SUPPORTED_BOUNDLINE_MAX_EXCLUSIVE}\"");
    let canon_min_entry = format!("canon_min = \"{SUPPORTED_CANON_VERSION}\"");
    let canon_max_entry = format!("canon_max_exclusive = \"{SUPPORTED_CANON_MAX_EXCLUSIVE}\"");

    assert_contains(
        &canon_manifest,
        canon_version_entry.as_str(),
        "Canon manifest should carry the planned workspace version",
    );
    assert_contains(
        &contract,
        boundline_min_entry.as_str(),
        "Canonical reasoning posture contract should publish the supported Boundline minimum version",
    );
    assert_contains(
        &contract,
        boundline_max_entry.as_str(),
        "Canonical reasoning posture contract should publish the supported Boundline max-exclusive version",
    );
    assert_contains(
        &contract,
        canon_min_entry.as_str(),
        "Canonical reasoning posture contract should publish the supported Canon minimum version",
    );
    assert_contains(
        &contract,
        canon_max_entry.as_str(),
        "Canonical reasoning posture contract should publish the supported Canon max-exclusive version",
    );

    Ok(())
}

#[test]
fn governed_reasoning_posture_contract_publishes_supported_line_and_window()
-> Result<(), Box<dyn Error>> {
    let contract = read_text(STABLE_CONTRACT_PATH)?;

    assert_contains(
        &contract,
        SUPPORTED_CONTRACT_LINE,
        "Canonical reasoning posture contract should publish the supported contract line",
    );
    assert_contains(
        &contract,
        SUPPORTED_BOUNDLINE_WINDOW,
        "Canonical reasoning posture contract should publish the supported Boundline window",
    );
    assert_contains(
        &contract,
        SUPPORTED_CANON_WINDOW,
        "Canonical reasoning posture contract should publish the supported Canon window",
    );

    Ok(())
}

#[test]
fn governed_reasoning_posture_contract_publishes_supported_vocabulary() -> Result<(), Box<dyn Error>>
{
    let contract = read_text(STABLE_CONTRACT_PATH)?;

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
            "Canonical reasoning posture contract should publish every supported explicit profile id",
        );
    }
    for admission_priority in SUPPORTED_ADMISSION_PRIORITIES {
        assert_contains(
            &contract,
            admission_priority,
            "Canonical reasoning posture contract should publish every supported admission priority",
        );
    }
    for independence_key in REQUIRED_MINIMUM_INDEPENDENCE_KEYS {
        assert_contains(
            &contract,
            independence_key,
            "Canonical reasoning posture contract should publish the full minimum_independence shape",
        );
    }

    Ok(())
}

#[test]
fn governed_reasoning_posture_contract_matches_feature_brief_window_and_fields()
-> Result<(), Box<dyn Error>> {
    let contract = read_text(STABLE_CONTRACT_PATH)?;
    let brief = read_text(FEATURE_LOCAL_CONTRACT_PATH)?;

    for expected in [
        SUPPORTED_CONTRACT_LINE,
        SUPPORTED_BOUNDLINE_WINDOW,
        SUPPORTED_CANON_WINDOW,
        "required_profile_family",
        "required_profile_id",
        "admission_priority",
        "confidence_handoff_required",
        "provenance_ref",
    ] {
        assert_contains(
            &contract,
            expected,
            "Canonical reasoning posture contract should stay aligned with the feature-local Canon brief",
        );
        assert_contains(
            &brief,
            expected,
            "Feature-local Canon brief should acknowledge the same provider window and required fields",
        );
    }

    Ok(())
}
