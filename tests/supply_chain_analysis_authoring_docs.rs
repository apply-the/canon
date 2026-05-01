use std::fs;

const MODES_GUIDE: &str = "docs/guides/modes.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/supply-chain-analysis.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/supply-chain-analysis-rust-workspace.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-supply-chain-analysis/SKILL.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn supply_chain_authoring_docs_describe_clarification_and_publish_surface() {
    let guide = read(MODES_GUIDE);
    for needle in [
        "## Mode: supply-chain-analysis",
        "docs/supply-chain/<YYYY-MM-DD>-<descriptor>/",
        "canon-input/supply-chain-analysis.md",
        "missing scanner coverage or deferred verification",
    ] {
        assert!(guide.contains(needle), "{MODES_GUIDE} missing {needle}");
    }
}

#[test]
fn supply_chain_template_and_example_preserve_canonical_sections() {
    let template = read(TEMPLATE_PATH);
    let example = read(EXAMPLE_PATH);

    for heading in [
        "## Declared Scope",
        "## Licensing Posture",
        "## Distribution Model",
        "## Scanner Decisions",
        "## Coverage Gaps",
        "## Source Inputs",
        "## Deferred Verification",
    ] {
        assert!(template.contains(heading), "{TEMPLATE_PATH} missing {heading}");
        assert!(example.contains(heading), "{EXAMPLE_PATH} missing {heading}");
    }
}

#[test]
fn supply_chain_skill_source_and_mirror_stay_in_sync() {
    let source = read(SKILL_SOURCE);
    let mirror = read(SKILL_MIRROR);
    assert_eq!(source, mirror, "supply-chain skill source and mirror should match exactly");
}
