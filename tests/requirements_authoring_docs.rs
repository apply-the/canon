use std::fs;

const CONTRACT_PATH: &str =
    "specs/016-mode-authoring-specialization/contracts/requirements-authoring.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-requirements/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-requirements/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/requirements.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/requirements-api-v2.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const README_PATH: &str = "README.md";
const GETTING_STARTED_PATH: &str = "docs/guides/getting-started.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Problem",
    "## Outcome",
    "## Constraints",
    "## Non-Negotiables",
    "## Options",
    "## Recommended Path",
    "## Tradeoffs",
    "## Consequences",
    "## Scope Cuts",
    "## Deferred Work",
    "## Decision Checklist",
    "## Open Questions",
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
fn requirements_contract_skill_template_and_example_share_canonical_headings() {
    for path in [CONTRACT_PATH, SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
        let content = read(path);
        assert_lists_headings(path, &content, CANONICAL_HEADINGS);
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Requirements Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
    assert!(
        skill_source.contains("### Authoring Persona")
            && skill_source.contains("product lead")
            && skill_source.contains("Persona guidance is presentation only"),
        "skill source must document the bounded product persona"
    );

    let template = read(TEMPLATE_PATH);
    assert!(
        template.contains("Suggested persona: product lead")
            && template.contains("persona guidance shapes voice only"),
        "requirements template must document the bounded product persona"
    );

    let example = read(EXAMPLE_PATH);
    assert!(
        example.contains("Authored as a product lead"),
        "requirements example must surface the intended persona"
    );
}

#[test]
fn requirements_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}

#[test]
fn requirements_mode_guide_and_roadmap_document_the_delivered_shape_slice() {
    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("`## Problem`") && guide.contains("`## Scope Cuts`"),
        "mode guide must describe the requirements authored-body contract"
    );
    assert!(
        guide
            .contains("The AI companion should author Requirements as if it were the product lead")
            && guide.contains("the persona shapes voice and prioritization only"),
        "mode guide must document the bounded requirements persona"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("`requirements`, `architecture`, and `change` ship the first")
            && roadmap.contains("PRD"),
        "roadmap must record the delivered requirements artifact-shape slice"
    );
    assert!(
        roadmap.contains("requirements"),
        "roadmap must keep requirements in the delivered scope"
    );
    assert!(
        roadmap.contains("product-facing persona")
            && roadmap.contains("Keep personas guidance-only"),
        "roadmap must document the requirements persona layer and its boundary"
    );
}

#[test]
fn readme_and_getting_started_use_canonical_requirements_examples() {
    for path in [README_PATH, GETTING_STARTED_PATH] {
        let content = read(path);
        assert!(
            content.contains("# Requirements Brief")
                && content.contains("## Problem")
                && content.contains("## Outcome"),
            "{path} must show a canonical requirements input example"
        );
        assert!(
            !content.contains("# Idea\n\nDefine requirements for a bounded internal CLI without letting scope drift."),
            "{path} must not show the obsolete freeform requirements example"
        );
        assert!(
            content.contains("product lead, architect, and change owner")
                || content.contains("product lead,\narchitect, and change owner")
                || content.contains("product\nlead, architect, and change-owner")
                || content.contains("product lead, architect, and change-owner"),
            "{path} must mention the delivered requirements persona layer"
        );
    }
}
