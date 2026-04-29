use std::fs;

const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/supply-chain-analysis.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/supply-chain-analysis-rust-workspace.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_docs_describe_the_024_supply_chain_surface() {
    let readme = read(README_PATH);
    assert!(
        readme.contains("`supply-chain-analysis`")
            && readme.contains("SBOM")
            && readme.contains("license")
            && readme.contains("vulnerability")
            && readme.contains("legacy"),
        "README must summarize the 0.24.0 supply-chain-analysis surface"
    );

    let changelog = read(CHANGELOG_PATH);
    assert!(changelog.contains("## [0.24.0]"), "CHANGELOG missing 0.24.0 entry");
    assert!(
        changelog.contains("Supply Chain And Legacy Analysis Mode"),
        "CHANGELOG must describe feature 024"
    );

    let guide = read(MODES_GUIDE);
    for heading in [
        "supply-chain-analysis",
        "docs/supply-chain/<RUN_ID>/",
        "Supply chain",
        "system_context=existing",
        "canon-input/supply-chain-analysis.md",
    ] {
        assert!(guide.contains(heading), "{MODES_GUIDE} missing {heading}");
    }

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("`supply-chain-analysis`")
            && roadmap.contains("docs/supply-chain/<RUN_ID>/")
            && roadmap.contains("Current end-to-end depth exists"),
        "ROADMAP must reflect the delivered supply-chain-analysis mode"
    );
}

#[test]
fn release_scaffolds_exist_for_024_examples_and_templates() {
    let template = read(TEMPLATE_PATH);
    for heading in [
        "## Declared Scope",
        "## Licensing Posture",
        "## Ecosystems In Scope",
        "## Triage Decisions",
        "## Coverage Gaps",
    ] {
        assert!(template.contains(heading), "{TEMPLATE_PATH} missing {heading}");
    }

    let example = read(EXAMPLE_PATH);
    for heading in [
        "## Declared Scope",
        "## Licensing Posture",
        "## Ecosystems In Scope",
        "## Triage Decisions",
        "## Coverage Gaps",
    ] {
        assert!(example.contains(heading), "{EXAMPLE_PATH} missing {heading}");
    }
}
