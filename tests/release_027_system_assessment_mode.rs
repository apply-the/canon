use std::fs;

const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const AGENTS_PATH: &str = "AGENTS.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_docs_describe_the_027_system_assessment_surface() {
    let readme = read(README_PATH);
    assert!(
        readme.contains("`system-assessment`")
            && readme.contains("ISO 42010")
            && readme.contains("observed findings")
            && readme.contains("assessment\ngaps")
            && readme.contains("as-is architecture packet"),
        "README must summarize the 0.26.0 system-assessment surface"
    );

    let changelog = read(CHANGELOG_PATH);
    assert!(changelog.contains("## [0.26.0]"), "CHANGELOG missing 0.26.0 entry");
    assert!(changelog.contains("System Assessment Mode"), "CHANGELOG must describe feature 027");

    let guide = read(MODES_GUIDE);
    for heading in [
        "system-assessment",
        "docs/architecture/assessments/<RUN_ID>/",
        "As-is system assessment",
        "system_context=existing",
        "canon-input/system-assessment.md",
        "## Mode: system-assessment",
    ] {
        assert!(guide.contains(heading), "{MODES_GUIDE} missing {heading}");
    }
    assert!(
        guide.contains("ISO 42010-style")
            && guide.contains("## Observed Findings")
            && guide.contains("## Assessment Gaps")
            && guide.contains("approval-gated or blocked"),
        "modes guide must document the system-assessment packet and publish flow"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("`system-assessment`")
            && roadmap.contains("docs/architecture/assessments/<RUN_ID>/")
            && roadmap.contains("ISO 42010-style coverage")
            && roadmap.contains("Current end-to-end depth exists"),
        "ROADMAP must reflect the delivered system-assessment mode"
    );

    let agents = read(AGENTS_PATH);
    assert!(
        agents.contains("`canon-system-assessment`"),
        "AGENTS must list the delivered canon-system-assessment skill"
    );
}
