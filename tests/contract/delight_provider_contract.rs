use std::fs;
use std::path::Path;

const STABLE_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tech-docs/integration/delight-provider-contract.md");
const FEATURE_BRIEF_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/specs/057-s7-delight-provider/contracts/delight-provider-contract.md"
);

fn read_contract(path: &str) -> String {
    fs::read_to_string(path).expect("contract markdown")
}

fn stable_contract() -> String {
    read_contract(STABLE_CONTRACT_PATH)
}

fn feature_brief() -> String {
    read_contract(FEATURE_BRIEF_PATH)
}

fn assert_identity_block(contract: &str) {
    assert!(contract.contains("- `owner`: `canon`"), "contract should identify canon as the owner");
    assert!(
        contract.contains("- `current_contract_line`: `delight-provider-v1`"),
        "contract should expose the delight-provider contract line"
    );
    assert!(
        contract.contains("- `stable_doc`: `tech-docs/integration/delight-provider-contract.md`"),
        "contract should expose the stable doc path"
    );
    assert!(
        contract.contains("- `primary_consumer`: `boundline`"),
        "contract should expose boundline as the primary consumer"
    );
}

#[test]
fn delight_provider_contract_files_exist() {
    assert!(
        Path::new(STABLE_CONTRACT_PATH).exists(),
        "stable delight-provider contract should exist"
    );
    assert!(
        Path::new(FEATURE_BRIEF_PATH).exists(),
        "feature-local delight-provider brief should exist"
    );

    let stable = stable_contract();
    let brief = feature_brief();

    assert!(
        stable.contains("# Canon Delight Provider Contract"),
        "stable doc should expose the contract title"
    );
    assert!(
        brief.contains("# Canon Delight Provider Contract"),
        "feature brief should expose the contract title"
    );
}

#[test]
fn delight_provider_contract_identity_block_stays_in_sync() {
    let stable = stable_contract();
    let brief = feature_brief();

    assert_identity_block(&stable);
    assert_identity_block(&brief);
}
