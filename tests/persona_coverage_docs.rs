use std::fs;

const MODES: &[(&str, &str)] = &[
    ("canon-system-shaping", "Author the packet as a system shaper"),
    ("canon-architecture", "Author the packet as an architect"),
    ("canon-change", "Author the packet as a change owner"),
    ("canon-implementation", "Author the packet as an implementation lead"),
    ("canon-refactor", "Author the packet as a preservation-focused maintainer"),
    ("canon-migration", "Author the packet as a migration lead"),
    ("canon-review", "Author the packet as a skeptical reviewer"),
    ("canon-pr-review", "Author the packet as a code reviewer"),
    ("canon-verification", "Author the packet as an adversarial verifier"),
    ("canon-incident", "Author the packet as an incident commander"),
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn persona_guidance_exists_for_runtime_targeted_and_review_like_modes() {
    for (mode, role_line) in MODES {
        let source_path = format!("defaults/embedded-skills/{mode}/skill-source.md");
        let content = read(&source_path);

        assert!(
            content.contains("### Packet Shape And Persona"),
            "{source_path} missing persona heading"
        );
        assert!(content.contains(role_line), "{source_path} missing role line {role_line}");
        assert!(
            content.contains("Persona guidance is presentation only."),
            "{source_path} missing persona guardrail"
        );
    }
}

#[test]
fn persona_skill_mirrors_match_embedded_sources() {
    for (mode, _) in MODES {
        let source_path = format!("defaults/embedded-skills/{mode}/skill-source.md");
        let mirror_path = format!(".agents/skills/{mode}/SKILL.md");

        assert_eq!(read(&source_path), read(&mirror_path), "mirror drift for {mode}");
    }
}
