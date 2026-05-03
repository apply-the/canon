use std::fs;

const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-inspect-clarity/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-inspect-clarity/SKILL.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const TEMPLATE_README: &str = "docs/templates/canon-input/README.md";
const CARRY_FORWARD_GUIDE: &str = "docs/examples/canon-input/carry-forward-packets.md";

fn read(path: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
        .unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn inspect_clarity_skill_source_and_mirror_stay_in_sync() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}

#[test]
fn shared_authoring_surfaces_document_the_same_lifecycle_and_authority_rules() {
    let skill_source = read(SKILL_SOURCE);
    let modes_guide = read(MODES_GUIDE);
    let template_readme = read(TEMPLATE_README);
    let carry_forward = read(CARRY_FORWARD_GUIDE);

    for content in [&skill_source, &modes_guide, &template_readme, &carry_forward] {
        assert!(
            content.contains("inspect clarity"),
            "shared authoring surface should mention inspect clarity: {content}"
        );
        assert!(
            content.contains("brief.md"),
            "shared authoring surface should mention brief.md authority: {content}"
        );
    }

    assert!(
        skill_source
            .contains("packet shape, authoritative inputs, supporting inputs, and readiness delta"),
        "inspect clarity skill should describe the new output contract"
    );
    assert!(
        skill_source.contains("author or tighten the packet, inspect clarity")
            && skill_source.contains("critique the emitted packet")
            && skill_source.contains("publish only when"),
        "inspect clarity skill should describe the full lifecycle"
    );
    assert!(
        modes_guide.contains("Shared Authoring Lifecycle"),
        "modes guide should expose the shared lifecycle section"
    );
    assert!(
        template_readme.contains("Authority Rules"),
        "template README should explain packet authority rules"
    );
    assert!(
        carry_forward.contains("Critique And Publish"),
        "carry-forward guide should keep critique and publish explicit"
    );
}
