use std::fs;

const CONTRACT_PATH: &str =
    "specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-security-assessment/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-security-assessment/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/security-assessment.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/security-assessment-webhook-platform.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Assessment Scope",
    "## In-Scope Assets",
    "## Trust Boundaries",
    "## Out Of Scope",
    "## Threat Inventory",
    "## Attacker Goals",
    "## Boundary Threats",
    "## Risk Findings",
    "## Likelihood And Impact",
    "## Proposed Owners",
    "## Recommended Controls",
    "## Tradeoffs",
    "## Sequencing Notes",
    "## Assumptions",
    "## Evidence Gaps",
    "## Unobservable Surfaces",
    "## Applicable Standards",
    "## Control Families",
    "## Scope Limits",
    "## Source Inputs",
    "## Independent Checks",
    "## Deferred Verification",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn security_assessment_contract_skill_template_and_example_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Security Assessment Required Sections"));

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
        skill_source.contains("Author Security Assessment Body Before Invoking Canon"),
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
        skill_source.contains("Persona guidance is presentation only."),
        "skill source missing persona guardrail"
    );
}

#[test]
fn security_assessment_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
