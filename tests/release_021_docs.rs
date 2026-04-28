use std::fs;

const WORKSPACE_MANIFEST: &str = "Cargo.toml";
const CHANGELOG_PATH: &str = "CHANGELOG.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn workspace_manifest_reports_0220() {
    let manifest = read(WORKSPACE_MANIFEST);
    assert!(
        manifest.contains("version = \"0.22.0\""),
        "workspace manifest must report version 0.22.0"
    );
}

#[test]
fn changelog_records_the_021_release() {
    let changelog = read(CHANGELOG_PATH);
    assert!(
        changelog.contains("## [0.21.0] - 2026-04-27"),
        "changelog must record the 0.21.0 release header"
    );
    assert!(
        changelog.contains("`021` - Industry-Standard Artifact Shapes With Personas"),
        "changelog must record spec 021"
    );
    assert!(
        changelog.contains("persona-aware industry-standard packet shaping")
            && changelog.contains("product lead")
            && changelog.contains("architect")
            && changelog.contains("change owner"),
        "changelog must summarize the first-slice shape-plus-persona release"
    );
}
