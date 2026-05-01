use std::fs;

const CONTRACT_PATH: &str =
    "specs/018-architecture-adr-options/contracts/architecture-decision-shape.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-architecture/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-architecture/SKILL.md";

const DECISION_SECTIONS: &[&str] = &[
    "Decision Drivers",
    "Options Considered",
    "Pros",
    "Cons",
    "Recommendation",
    "Why Not The Others",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn architecture_decision_contract_and_skill_stay_in_sync() {
    let contract = read(CONTRACT_PATH);
    for section in DECISION_SECTIONS {
        assert!(contract.contains(section), "{CONTRACT_PATH} missing {section}");
    }
    assert!(
        contract.contains("Consequences"),
        "{CONTRACT_PATH} must describe the ADR-style consequences output"
    );
    assert!(
        contract.contains("Risks"),
        "{CONTRACT_PATH} must mention the legacy Risks compatibility path"
    );

    for path in [SKILL_SOURCE] {
        let content = read(path);
        for section in DECISION_SECTIONS {
            assert!(content.contains(section), "{path} missing {section}");
        }
        assert!(
            content.contains("Missing Authored Body"),
            "{path} must describe missing-body behavior"
        );
        assert!(
            content.contains("Consequences"),
            "{path} must describe the ADR-style consequences output"
        );
        assert!(
            content.contains("Risks"),
            "{path} must mention the legacy Risks compatibility path"
        );
    }
}

#[test]
fn architecture_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
