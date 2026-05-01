use std::fs;

const CONTRACT_PATH: &str =
    "specs/016-mode-authoring-specialization/contracts/discovery-authoring.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-discovery/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-discovery/SKILL.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Problem Domain",
    "## Repo Surface",
    "## Immediate Tensions",
    "## Downstream Handoff",
    "## Unknowns",
    "## Assumptions",
    "## Validation Targets",
    "## Confidence Levels",
    "## In-Scope Context",
    "## Out-of-Scope Context",
    "## Translation Trigger",
    "## Options",
    "## Constraints",
    "## Recommended Direction",
    "## Next-Phase Shape",
    "## Pressure Points",
    "## Blocking Decisions",
    "## Open Questions",
    "## Recommended Owner",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn assert_lists_headings(path: &str, content: &str, headings: &[&str]) {
    for heading in headings {
        assert!(content.contains(heading), "{path} missing {heading}");
    }
}

#[test]
fn discovery_contract_and_skill_share_canonical_headings() {
    for path in [CONTRACT_PATH, SKILL_SOURCE] {
        let content = read(path);
        assert_lists_headings(path, &content, CANONICAL_HEADINGS);
    }

    let contract = read(CONTRACT_PATH);
    assert!(
        contract.contains("Opportunity Solution Tree seed")
            && contract.contains("exploratory research lead"),
        "contract must describe the 030 discovery shape and persona"
    );

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Discovery Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
    assert!(
        skill_source.contains("Opportunity Solution Tree")
            && skill_source.contains("Jobs-To-Be-Done")
            && skill_source.contains("exploratory research lead"),
        "skill source must describe the 030 discovery shape and persona"
    );
}

#[test]
fn discovery_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
