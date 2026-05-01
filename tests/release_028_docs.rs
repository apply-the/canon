use std::fs;

const CHANGELOG_PATH: &str = "CHANGELOG.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const AGENTS_PATH: &str = "AGENTS.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const MIGRATION_TEMPLATE: &str = "docs/templates/canon-input/migration.md";
const MIGRATION_EXAMPLE: &str = "docs/examples/canon-input/migration-platform-consolidation.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_028_history_remains_documented() {
    let changelog = read(CHANGELOG_PATH);
    assert!(
        changelog.contains("## [0.28.0] - 2026-05-01"),
        "CHANGELOG must record the 0.28.0 release entry"
    );
    assert!(
        changelog.contains("`028` - Decision Alternatives"),
        "CHANGELOG must name the delivered 028 spec"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains(
            "## Delivered Feature: 028 Decision Alternatives, Pattern Choices, And Framework Evaluations"
        ),
        "ROADMAP must retain the delivered 028 decision-support slice"
    );

    let agents = read(AGENTS_PATH);
    assert!(
        agents.contains("028-decision-alternatives"),
        "AGENTS.md must retain the 028 feature context"
    );
}

#[test]
fn release_028_docs_describe_the_shipped_decision_support_contracts() {
    let guide = read(MODES_GUIDE);
    for expected in [
        "`## Why Not The Others`",
        "`## Decision Drivers`",
        "`## Candidate Frameworks`",
        "`## Decision Evidence`",
    ] {
        assert!(
            guide.contains(expected),
            "modes guide missing shipped decision-support heading {expected}"
        );
    }

    let migration_template = read(MIGRATION_TEMPLATE);
    assert!(
        migration_template.contains("## Decision Evidence")
            && migration_template.contains("## Why Not The Others"),
        "migration template must include the delivered decision-evidence headings"
    );

    let migration_example = read(MIGRATION_EXAMPLE);
    assert!(
        migration_example.contains("## Decision Evidence")
            && migration_example.contains("## Why Not The Others"),
        "migration example must include the delivered decision-evidence headings"
    );
}
