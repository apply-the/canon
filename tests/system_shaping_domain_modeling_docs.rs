use std::fs;

const CONTRACT_PATH: &str =
    "specs/017-domain-boundary-design/contracts/system-shaping-domain-modeling.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-system-shaping/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-system-shaping/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/system-shaping.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/system-shaping-billing.md";

const CONTRACT_SECTIONS: &[&str] = &[
    "Candidate Bounded Contexts",
    "Core And Supporting Domain Hypotheses",
    "Ubiquitous Language",
    "Domain Invariants",
    "Boundary Risks And Open Questions",
];

const CANONICAL_HEADINGS: &[&str] = &[
    "## Candidate Bounded Contexts",
    "## Core And Supporting Domain Hypotheses",
    "## Ubiquitous Language",
    "## Domain Invariants",
    "## Boundary Risks And Open Questions",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn system_shaping_contract_skill_template_and_example_share_domain_modeling_sections() {
    let contract = read(CONTRACT_PATH);
    for section in CONTRACT_SECTIONS {
        assert!(contract.contains(section), "{CONTRACT_PATH} missing {section}");
    }

    for path in [SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
        let content = read(path);
        for heading in CANONICAL_HEADINGS {
            assert!(content.contains(heading), "{path} missing {heading}");
        }
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author System-Shaping Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn system_shaping_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
