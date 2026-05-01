use std::fs;

const CARGO_TOML: &str = "Cargo.toml";
const CARGO_LOCK: &str = "Cargo.lock";
const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const AGENTS_PATH: &str = "AGENTS.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const MIGRATION_TEMPLATE: &str = "docs/templates/canon-input/migration.md";
const MIGRATION_EXAMPLE: &str = "docs/examples/canon-input/migration-platform-consolidation.md";
const RUNTIME_COMPAT_SOURCE: &str =
    "defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml";
const RUNTIME_COMPAT_MIRROR: &str =
    ".agents/skills/canon-shared/references/runtime-compatibility.toml";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_028_version_surfaces_are_aligned() {
    let cargo_toml = read(CARGO_TOML);
    assert!(
        cargo_toml.contains("version = \"0.28.0\""),
        "Cargo.toml must expose workspace version 0.28.0"
    );

    let cargo_lock = read(CARGO_LOCK);
    for package in [
        "name = \"canon-adapters\"\nversion = \"0.28.0\"",
        "name = \"canon-cli\"\nversion = \"0.28.0\"",
        "name = \"canon-engine\"\nversion = \"0.28.0\"",
        "name = \"canon-workspace\"\nversion = \"0.28.0\"",
    ] {
        assert!(
            cargo_lock.contains(package),
            "Cargo.lock missing aligned workspace package block: {package}"
        );
    }

    let runtime_compat_source = read(RUNTIME_COMPAT_SOURCE);
    let runtime_compat_mirror = read(RUNTIME_COMPAT_MIRROR);
    assert!(
        runtime_compat_source.contains("expected_workspace_version = \"0.28.0\""),
        "embedded runtime compatibility reference must target 0.28.0"
    );
    assert_eq!(
        runtime_compat_source, runtime_compat_mirror,
        "runtime compatibility mirror must match embedded source"
    );

    let readme = read(README_PATH);
    assert!(
        readme.contains("The current delivery line in this repository targets Canon `0.28.0`."),
        "README must advertise the 0.28.0 delivery line"
    );

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
        "ROADMAP must record the delivered 028 decision-support slice"
    );

    let agents = read(AGENTS_PATH);
    assert!(
        agents.contains("028-decision-alternatives"),
        "AGENTS.md must include the 028 feature context"
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
