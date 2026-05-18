#[path = "contract/delight_provider_contract.rs"]
mod delight_provider_contract;

use std::fs;

const STABLE_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/docs/integration/delight-provider-contract.md");
const FEATURE_BRIEF_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/057-s7-delight-provider/contracts/delight-provider-contract.md"
);

fn assert_identity_block(contract: &str) {
    assert!(contract.contains("- `owner`: `canon`"));
    assert!(contract.contains("- `current_contract_line`: `delight-provider-v1`"));
    assert!(contract.contains("- `stable_doc`: `docs/integration/delight-provider-contract.md`"));
    assert!(contract.contains("- `primary_consumer`: `boundline`"));
}

fn assert_authorized_classes(contract: &str) {
    for class_name in [
        "packets",
        "approval-states",
        "readiness-signals",
        "security-findings",
        "audit-findings",
        "promotion-references",
    ] {
        assert!(
            contract.contains(&format!("### `{class_name}`")),
            "contract should include the `{class_name}` class"
        );
    }

    for legacy_name in [
        "authority-governance",
        "adaptive-governance",
        "semantic-artifact",
        "expertise-input",
        "project-memory-promotion",
    ] {
        assert!(
            !contract.contains(&format!("### `{legacy_name}`")),
            "contract should not keep legacy type-centric class `{legacy_name}` as a first-class heading"
        );
    }

    assert_eq!(
        contract.matches("### `").count(),
        6,
        "contract should expose exactly six first-class artifact classes"
    );
}

fn assert_required_metadata_inventory(contract: &str) {
    assert_eq!(
        contract.matches("- **Required metadata fields**:").count(),
        6,
        "each contract class should expose one required metadata section"
    );
    for marker in [
        "- `mode` — Canon mode that produced the packet",
        "- `approval_state` — `not-needed`, `requested`, `granted`, `rejected`, or `expired`",
        "- `packet_readiness` — `pending`, `incomplete`, `reusable`, or `rejected`",
        "- `finding_kind` — Canon-owned security finding family or category",
        "- `finding_kind` — review, verification, or audit-oriented Canon finding category",
        "- `promotion_ref` — Canon-owned promotion reference identifier or path",
    ] {
        assert!(
            contract.contains(marker),
            "contract should include required metadata marker: {marker}"
        );
    }
}

fn assert_compatibility_signaling(contract: &str) {
    assert!(contract.contains("## Compatibility Signaling"));
    for signal in [
        "| `available` | Present, promoted, within contracted schema | Consume normally |",
        "| `stale` | Present but promotion epoch is outdated or gate is pending | Surface staleness; do not fabricate certainty |",
        "| `incompatible` | Contract line or schema version outside contracted range | Do not consume; surface incompatibility |",
        "| `absent` | Artifact class not present in workspace | Continue with Boundline-only evidence |",
        "| `contradicted` | Artifact contradicts a co-present canonical signal | Surface contradiction; do not merge conflicting signals |",
    ] {
        assert!(
            contract.contains(signal),
            "contract should include compatibility signal row: {signal}"
        );
    }
    assert_eq!(
        contract.matches("- **Degradation conditions**:").count(),
        6,
        "each class should expose degradation conditions"
    );
}

fn assert_schema_versioning_rules(contract: &str) {
    assert!(contract.contains("## Schema Versioning"));
    assert!(
        contract.contains("requires a new contract line"),
        "contract should require a new contract line for breaking changes"
    );
    assert!(
        contract
            .contains("accommodated within the current contract line with a documented amendment"),
        "contract should allow additive changes within the current contract line via amendment"
    );
}

fn assert_amendment_and_deprecation_rules(contract: &str) {
    assert!(contract.contains("## Amendment Procedure"));
    for marker in [
        "Canon proposes the amendment",
        "Once both teams acknowledge",
        "removal_epoch",
        "Canon MUST provide advance notice and fallback guidance",
        "## Deprecated Classes",
        "*None at contract-line `delight-provider-v1`.*",
    ] {
        assert!(
            contract.contains(marker),
            "contract should include amendment/deprecation marker: {marker}"
        );
    }
}

#[test]
fn delight_provider_contract_identity_block_stays_in_sync() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_identity_block(&stable);
    assert_identity_block(&brief);
}

#[test]
fn delight_provider_contract_lists_only_the_six_authorized_classes() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_authorized_classes(&stable);
    assert_authorized_classes(&brief);
}

#[test]
fn delight_provider_contract_exposes_required_metadata_for_each_class() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_required_metadata_inventory(&stable);
    assert_required_metadata_inventory(&brief);
}

#[test]
fn delight_provider_contract_defines_all_compatibility_signals() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_compatibility_signaling(&stable);
    assert_compatibility_signaling(&brief);
}

#[test]
fn delight_provider_contract_makes_schema_versioning_rules_explicit() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_schema_versioning_rules(&stable);
    assert_schema_versioning_rules(&brief);
}

#[test]
fn delight_provider_contract_requires_explicit_amendment_and_deprecation_rules() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_amendment_and_deprecation_rules(&stable);
    assert_amendment_and_deprecation_rules(&brief);
}

#[test]
fn delight_provider_contract_stable_doc_and_feature_brief_do_not_drift() {
    let stable = fs::read_to_string(STABLE_CONTRACT_PATH).expect("stable contract");
    let brief = fs::read_to_string(FEATURE_BRIEF_PATH).expect("feature brief");

    assert_eq!(
        stable, brief,
        "stable doc and feature brief should remain byte-for-byte aligned for this contract line"
    );
}
