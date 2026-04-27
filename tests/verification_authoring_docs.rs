use std::fs;

const CONTRACT_PATH: &str =
    "specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-verification/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-verification/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/verification.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/verification-e2e-flakiness.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Claims Under Test",
    "## Invariant Checks",
    "## Contract Assumptions",
    "## Verification Outcome",
    "## Challenge Findings",
    "## Contradictions",
    "## Verified Claims",
    "## Rejected Claims",
    "## Overall Verdict",
    "## Open Findings",
    "## Required Follow-Up",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn verification_contract_skill_template_and_example_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Verification Required Sections"));

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
        skill_source.contains("Author Verification Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn verification_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
