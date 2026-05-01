use std::fs;

const TEMPLATE_PATH: &str = "docs/templates/canon-input/architecture.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/architecture-state-management.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-architecture/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-architecture/SKILL.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn architecture_template_includes_three_canonical_c4_headings() {
    let content = read(TEMPLATE_PATH);
    assert!(content.contains("\n## System Context"), "template missing `## System Context`");
    assert!(content.contains("\n## Containers"), "template missing `## Containers`");
    assert!(content.contains("\n## Components"), "template missing `## Components`");
    assert!(
        content.contains("Suggested persona: architect")
            && content.contains("persona guidance shapes framing only"),
        "template must document the bounded architecture persona"
    );
}

#[test]
fn architecture_example_authors_all_three_c4_sections() {
    let content = read(EXAMPLE_PATH);
    assert!(content.contains("\n## System Context"), "example missing System Context");
    assert!(content.contains("\n## Containers"), "example missing Containers");
    assert!(content.contains("\n## Components"), "example missing Components");
    assert!(
        content.contains("billing-service"),
        "example should ground System Context in a named system"
    );
    assert!(
        content.contains("Authored as an architect"),
        "example must surface the intended architecture persona"
    );
}

#[test]
fn architecture_skill_source_documents_authored_c4_requirement() {
    let content = read(SKILL_SOURCE);
    assert!(
        content.contains("Author Architecture Body Before Invoking Canon"),
        "skill source missing authored-C4 section header"
    );
    assert!(
        content.contains("`## System Context`")
            && content.contains("`## Containers`")
            && content.contains("`## Components`"),
        "skill source must enumerate the three canonical headings"
    );
    assert!(
        content.contains("Missing Authored Body"),
        "skill source must explain the missing-body marker"
    );
    assert!(
        content.contains("### Packet Shape And Persona")
            && content.contains("architect writing a combined C4 plus ADR decision")
            && content.contains("packet for reviewers and downstream implementers")
            && content.contains("Persona guidance is presentation only"),
        "skill source must document the bounded architecture persona"
    );
}

#[test]
fn architecture_mode_guide_and_roadmap_document_persona_guidance() {
    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("The AI companion should author Architecture as if it were the architect")
            && guide.contains("the persona shapes framing only"),
        "mode guide must document the bounded architecture persona"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("explicit architecture-decision persona")
            && roadmap.contains("Keep personas guidance-only"),
        "roadmap must document the architecture persona layer and its boundary"
    );
}

#[test]
fn architecture_skill_mirror_matches_skill_source() {
    let source = read(SKILL_SOURCE);
    let mirror = read(SKILL_MIRROR);
    assert_eq!(
        source, mirror,
        "`.agents/skills/canon-architecture/SKILL.md` must mirror `defaults/embedded-skills/canon-architecture/skill-source.md`"
    );
}
