use std::fs;

const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-pr-review/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-pr-review/SKILL.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn pr_review_skill_exposes_diff_review_persona_guidance() {
    let skill_source = read(SKILL_SOURCE);

    assert!(
        skill_source.contains("## Canon Command Contract"),
        "skill source missing command contract"
    );
    assert!(
        skill_source.contains("### Packet Shape And Persona"),
        "skill source missing persona section"
    );
    assert!(
        skill_source.contains("Author the packet as a code reviewer"),
        "skill source missing persona role"
    );
    assert!(
        skill_source.contains("Persona guidance is presentation only."),
        "skill source missing persona guardrail"
    );
}

#[test]
fn pr_review_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
