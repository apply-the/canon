use std::fs;

const CONTRACT_PATH: &str =
    "specs/016-mode-authoring-specialization/contracts/discovery-authoring.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-discovery/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-discovery/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/discovery.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/discovery-legacy-migration.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";

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
fn discovery_contract_skill_template_and_example_share_canonical_headings() {
    for path in [CONTRACT_PATH, SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
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

    let template = read(TEMPLATE_PATH);
    assert!(
        template.contains("blocked job") && template.contains("Assumption test"),
        "template must teach the follow-on discovery framing"
    );
}

#[test]
fn discovery_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}

#[test]
fn discovery_mode_guide_and_roadmap_document_the_follow_on_slice() {
    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("`## Problem Domain`") && guide.contains("`## Repo Surface`"),
        "mode guide must describe the discovery authored-body contract"
    );
    assert!(
        guide.contains("Opportunity Solution Tree seed")
            && guide.contains("exploratory research lead"),
        "mode guide must describe the 030 discovery shape and persona"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("## Delivered Feature: 030 Industry-Standard Artifact Shapes Follow-On")
            && roadmap.contains("Opportunity Solution Tree")
            && roadmap.contains("Jobs-To-Be-Done"),
        "roadmap must record the delivered 030 discovery follow-on slice"
    );
    assert!(
        roadmap.contains("discovery")
            && roadmap.contains("system-shaping")
            && roadmap.contains("review"),
        "roadmap must keep the 030 follow-on modes in the delivered scope"
    );
}
