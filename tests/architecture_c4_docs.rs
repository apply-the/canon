use std::fs;

const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-architecture/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-architecture/SKILL.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
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
fn architecture_skill_mirror_matches_skill_source() {
    let source = read(SKILL_SOURCE);
    let mirror = read(SKILL_MIRROR);
    assert_eq!(
        source, mirror,
        "`.agents/skills/canon-architecture/SKILL.md` must mirror `defaults/embedded-skills/canon-architecture/skill-source.md`"
    );
}
