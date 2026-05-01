use std::fs;

const CARGO_TOML: &str = "Cargo.toml";
const CARGO_LOCK: &str = "Cargo.lock";
const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const AGENTS_PATH: &str = "AGENTS.md";
const GETTING_STARTED_PATH: &str = "docs/guides/getting-started.md";
const MODES_GUIDE_PATH: &str = "docs/guides/modes.md";
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
fn release_029_version_surfaces_are_aligned() {
    let cargo_toml = read(CARGO_TOML);
    assert!(
        cargo_toml.contains("version = \"0.29.0\""),
        "Cargo.toml must expose workspace version 0.29.0"
    );

    let cargo_lock = read(CARGO_LOCK);
    for package in [
        "name = \"canon-adapters\"\nversion = \"0.29.0\"",
        "name = \"canon-cli\"\nversion = \"0.29.0\"",
        "name = \"canon-engine\"\nversion = \"0.29.0\"",
        "name = \"canon-workspace\"\nversion = \"0.29.0\"",
    ] {
        assert!(
            cargo_lock.contains(package),
            "Cargo.lock missing aligned workspace package block: {package}"
        );
    }

    let runtime_compat_source = read(RUNTIME_COMPAT_SOURCE);
    let runtime_compat_mirror = read(RUNTIME_COMPAT_MIRROR);
    assert!(
        runtime_compat_source.contains("expected_workspace_version = \"0.29.0\""),
        "embedded runtime compatibility reference must target 0.29.0"
    );
    assert_eq!(
        runtime_compat_source, runtime_compat_mirror,
        "runtime compatibility mirror must match embedded source"
    );

    let readme = read(README_PATH);
    assert!(
        readme.contains("The current delivery line in this repository targets Canon `0.29.0`."),
        "README must advertise the 0.29.0 delivery line"
    );

    let changelog = read(CHANGELOG_PATH);
    assert!(
        changelog.contains("## [0.29.0] - 2026-05-01"),
        "CHANGELOG must record the 0.29.0 release entry"
    );
    assert!(
        changelog.contains("`029` - Publish Destinations"),
        "CHANGELOG must name the delivered 029 spec"
    );

    let agents = read(AGENTS_PATH);
    assert!(
        agents.contains("029-publish-destinations"),
        "AGENTS.md must include the 029 feature context"
    );
}

#[test]
fn release_029_docs_describe_structured_publish_contract() {
    let readme = read(README_PATH);
    assert!(
        readme.contains("date-prefixed") && readme.contains("packet-metadata.json"),
        "README must describe structured publish folders and metadata sidecars"
    );

    let getting_started = read(GETTING_STARTED_PATH);
    assert!(
        getting_started.contains("docs/reviews/prs/<YYYY-MM-DD>-<descriptor>/")
            && getting_started.contains("packet-metadata.json"),
        "getting-started guide must describe the structured publish default"
    );

    let guide = read(MODES_GUIDE_PATH);
    for expected in [
        "`specs/<YYYY-MM-DD>-<descriptor>/`",
        "`docs/reviews/prs/<YYYY-MM-DD>-<descriptor>/`",
        "`packet-metadata.json`",
    ] {
        assert!(
            guide.contains(expected),
            "modes guide missing structured publish contract marker {expected}"
        );
    }

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("## Delivered Feature: 029 Structured External Publish Destinations"),
        "ROADMAP must record the delivered 029 publish slice"
    );
    assert!(
        roadmap.contains("<YYYY-MM-DD>-<descriptor>") && roadmap.contains("packet-metadata.json"),
        "ROADMAP must describe the structured publish path and metadata sidecar"
    );
}
