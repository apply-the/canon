use std::fs;

const CONTRACT_PATH: &str =
    "specs/017-domain-boundary-design/contracts/architecture-context-map.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-architecture/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-architecture/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/architecture.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/architecture-state-management.md";

const CONTRACT_SECTIONS: &[&str] = &[
    "Bounded Contexts",
    "Context Relationships",
    "Integration Seams",
    "Anti-Corruption Candidates",
    "Ownership Boundaries",
    "Shared Invariants",
];

const CANONICAL_HEADINGS: &[&str] = &[
    "## Bounded Contexts",
    "## Context Relationships",
    "## Integration Seams",
    "## Anti-Corruption Candidates",
    "## Ownership Boundaries",
    "## Shared Invariants",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn architecture_contract_skill_template_and_example_share_context_map_sections() {
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
        skill_source.contains("Author Architecture Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn architecture_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
