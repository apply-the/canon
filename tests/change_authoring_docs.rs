use std::fs;

const CONTRACT_PATH: &str = "specs/016-mode-authoring-specialization/contracts/change-authoring.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-change/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-change/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/change.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/change-add-caching.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## System Slice",
    "## Excluded Areas",
    "## Legacy Invariants",
    "## Forbidden Normalization",
    "## Change Surface",
    "## Ownership",
    "## Implementation Plan",
    "## Sequencing",
    "## Validation Strategy",
    "## Independent Checks",
    "## Decision Record",
    "## Consequences",
    "## Unresolved Questions",
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
fn change_contract_skill_template_and_example_share_canonical_headings() {
    for path in [CONTRACT_PATH, SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
        let content = read(path);
        assert_lists_headings(path, &content, CANONICAL_HEADINGS);
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Change Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn change_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}

#[test]
fn change_mode_guide_and_roadmap_document_the_first_slice() {
    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("`## System Slice`") && guide.contains("`## Ownership`"),
        "mode guide must describe the change authored-body contract"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("requirements, discovery, and change now ship the first slice"),
        "roadmap must record the delivered first slice"
    );
    assert!(roadmap.contains("change"), "roadmap must keep change in the delivered scope");
}
