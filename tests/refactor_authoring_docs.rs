use std::fs;

const CONTRACT_PATH: &str =
    "specs/019-authoring-specialization-remaining/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-refactor/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-refactor/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/refactor.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/refactor-auth-session-cleanup.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Preserved Behavior",
    "## Approved Exceptions",
    "## Refactor Scope",
    "## Allowed Paths",
    "## Structural Rationale",
    "## Untouched Surface",
    "## Safety-Net Evidence",
    "## Regression Findings",
    "## Contract Drift",
    "## Reviewer Notes",
    "## Feature Audit",
    "## Decision",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn refactor_contract_skill_template_and_example_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Refactor Required Sections"));

    for heading in CANONICAL_HEADINGS {
        assert!(contract.contains(&heading[3..]), "{CONTRACT_PATH} missing {heading}");
    }

    for path in [SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
        let content = read(path);
        for heading in CANONICAL_HEADINGS {
            assert!(content.contains(heading), "{path} missing {heading}");
        }
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Refactor Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn refactor_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
