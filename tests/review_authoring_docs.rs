use std::fs;

const CONTRACT_PATH: &str =
    "specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-review/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-review/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/review.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/review-db-migration.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Review Target",
    "## Evidence Basis",
    "## Boundary Findings",
    "## Ownership Notes",
    "## Missing Evidence",
    "## Collection Priorities",
    "## Decision Impact",
    "## Reversibility Concerns",
    "## Final Disposition",
    "## Accepted Risks",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn review_contract_skill_template_and_example_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Review Required Sections"));
    assert!(
        contract.contains("findings-first review bundle")
            && contract.contains("severity, location, rationale, and recommended change"),
        "contract must describe the 030 review shape"
    );

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
        skill_source.contains("Author Review Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
    assert!(
        skill_source.contains("### Packet Shape And Persona"),
        "skill source missing persona section"
    );
    assert!(
        skill_source.contains("Author the packet as a skeptical reviewer")
            && skill_source.contains("findings-first review")
            && skill_source.contains("severity, location")
            && skill_source.contains("recommended change"),
        "skill source missing persona role"
    );
    assert!(
        skill_source.contains("Persona guidance is presentation only."),
        "skill source missing persona guardrail"
    );

    let template = read(TEMPLATE_PATH);
    assert!(
        template.contains("Severity: low | medium | high")
            && template.contains("Location:")
            && template.contains("Recommended Change:"),
        "review template must teach reviewer-native findings structure"
    );

    let example = read(EXAMPLE_PATH);
    assert!(
        example.contains("Severity: high") && example.contains("Recommended Change:"),
        "review example must demonstrate reviewer-native findings structure"
    );

    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("findings-first")
            && guide.contains("severity, location, rationale, and recommended change"),
        "mode guide must describe the 030 review shape"
    );
}

#[test]
fn review_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
