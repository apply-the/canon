use std::fs;

const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const MIGRATION_TEMPLATE: &str = "docs/templates/canon-input/migration.md";
const INCIDENT_TEMPLATE: &str = "docs/templates/canon-input/incident.md";
const MIGRATION_EXAMPLE: &str = "docs/examples/canon-input/migration-platform-consolidation.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_docs_describe_the_022_decision_and_persona_surface() {
    let readme = read(README_PATH);
    assert!(
        readme.contains("`system-shaping`")
            && readme.contains("`implementation`")
            && readme.contains("`migration`")
            && readme.contains("`review`")
            && readme.contains("`pr-review`")
            && readme.contains("`verification`")
            && readme.contains("`incident`"),
        "README must summarize the 0.22.0 decision-and-persona surface"
    );

    let changelog = read(CHANGELOG_PATH);
    assert!(changelog.contains("## [0.22.0]"), "CHANGELOG missing 0.22.0 entry");
    assert!(
        changelog.contains("Decision Alternatives, Pattern Choices, And Framework Evaluations"),
        "CHANGELOG must describe feature 022"
    );

    let guide = read(MODES_GUIDE);
    for heading in ["Options Matrix", "Recommendation", "Adoption Implications", "Ecosystem Health"]
    {
        assert!(guide.contains(heading), "{MODES_GUIDE} missing {heading}");
    }
    assert!(
        guide.contains("implementation lead") && guide.contains("migration lead"),
        "modes guide must document the new implementation and migration personas"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains(
            "## Next Feature: 022 Decision Alternatives, Pattern Choices, And Framework Evaluations"
        ) && roadmap.contains("## Remaining Roadmap Candidates"),
        "ROADMAP must expose the 022 entry and remaining candidates"
    );
}

#[test]
fn release_scaffolds_exist_for_022_examples_and_templates() {
    let migration_template = read(MIGRATION_TEMPLATE);
    for heading in [
        "## Options Matrix",
        "## Adoption Implications",
        "## Tradeoff Analysis",
        "## Recommendation",
        "## Ecosystem Health",
    ] {
        assert!(migration_template.contains(heading), "{MIGRATION_TEMPLATE} missing {heading}");
    }

    let incident_template = read(INCIDENT_TEMPLATE);
    for heading in [
        "## Incident Scope",
        "## Trigger And Current State",
        "## Operational Constraints",
        "## Follow-Up Work",
    ] {
        assert!(incident_template.contains(heading), "{INCIDENT_TEMPLATE} missing {heading}");
    }

    let migration_example = read(MIGRATION_EXAMPLE);
    for heading in [
        "## Options Matrix",
        "## Adoption Implications",
        "## Tradeoff Analysis",
        "## Recommendation",
        "## Ecosystem Health",
    ] {
        assert!(migration_example.contains(heading), "{MIGRATION_EXAMPLE} missing {heading}");
    }
}
