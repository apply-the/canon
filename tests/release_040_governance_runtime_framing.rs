use std::fs;
use std::path::PathBuf;

const VERSION: &str = "0.42.0";

#[test]
fn publish_and_release_surfaces_align_on_0_42_0_delivery() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let cargo_manifest = fs::read_to_string(repo_root.join("Cargo.toml")).expect("read Cargo.toml");
    assert!(
        cargo_manifest.contains(&format!("version = \"{VERSION}\"")),
        "workspace manifest should be bumped to {VERSION}"
    );

    for compatibility_ref in [
        repo_root.join(".agents/skills/canon-shared/references/runtime-compatibility.toml"),
        repo_root
            .join("defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml"),
    ] {
        let content =
            fs::read_to_string(&compatibility_ref).expect("read runtime compatibility reference");
        assert!(
            content.contains(&format!("expected_workspace_version = \"{VERSION}\"")),
            "runtime compatibility reference should point at {VERSION}: {}",
            compatibility_ref.display()
        );
    }

    let readme = fs::read_to_string(repo_root.join("README.md")).expect("read README");
    let modes_guide =
        fs::read_to_string(repo_root.join("docs/guides/modes.md")).expect("read modes guide");
    let adapter_guide =
        fs::read_to_string(repo_root.join("docs/integration/governance-adapter.md"))
            .expect("read adapter guide");
    let roadmap = fs::read_to_string(repo_root.join("ROADMAP.md")).expect("read roadmap");
    let changelog = fs::read_to_string(repo_root.join("CHANGELOG.md")).expect("read changelog");

    assert!(
        readme.contains(&format!(
            "The current delivery line in this repository targets Canon `{VERSION}`."
        )),
        "README should advertise Canon {VERSION}"
    );
    assert!(
        readme.contains("Generated packet files land under `.canon/artifacts/<RUN_ID>/...` first."),
        "README should explain the pre-publish artifact location"
    );
    assert!(
        readme.contains("`prd.md`") && readme.contains("$canon-publish"),
        "README should mention the consolidated requirements PRD and chat publish skill"
    );
    assert!(
        readme.contains("architecture-overview.md") && readme.contains("Mermaid `.mmd` sources"),
        "README should mention the overview-first architecture packet and Mermaid sidecars"
    );
    assert!(
        readme.contains("docs/integration/governance-adapter.md"),
        "README should link to the governance adapter guide"
    );
    assert!(
        modes_guide.contains("- `prd.md`")
            && modes_guide.contains("published folder includes `prd.md`, the sectional packet files, and `packet-metadata.json`"),
        "modes guide should describe the requirements prd publish surface"
    );
    assert!(
        adapter_guide.contains("same runtime"),
        "adapter guide should keep the same-runtime boundary explicit"
    );
    assert!(
        roadmap.contains("Delivered work through `041`"),
        "roadmap should retain the delivered 041 summary"
    );
    assert!(
        roadmap.contains("Delivered work through `042`")
            && roadmap.contains("architecture-overview.md"),
        "roadmap should describe the delivered 042 architecture packet feature"
    );
    assert!(
        changelog.contains(&format!("## [{VERSION}]")),
        "changelog should record the {VERSION} release"
    );
    assert!(
        changelog.contains("Pragmatic C4 Architecture Packets And Visual Artifacts"),
        "changelog should name the 042 feature"
    );
}
