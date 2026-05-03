use std::fs;
use std::path::PathBuf;

const VERSION: &str = "0.40.0";

#[test]
fn governance_runtime_framing_release_surfaces_align_on_0_40_0_delivery() {
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
        readme.contains("docs/integration/governance-adapter.md"),
        "README should link to the governance adapter guide"
    );
    assert!(
        adapter_guide.contains("same runtime"),
        "adapter guide should keep the same-runtime boundary explicit"
    );
    assert!(
        roadmap.contains("040-governance-runtime-framing"),
        "roadmap should name the delivered 040 feature"
    );
    assert!(
        roadmap.contains(
            "There are no other active roadmap entries beyond the delivered `040` slice."
        ),
        "roadmap should make the delivered 040 scope explicit"
    );
    assert!(
        changelog.contains(&format!("## [{VERSION}]")),
        "changelog should record the {VERSION} release"
    );
    assert!(
        changelog.contains("Governance Runtime Framing"),
        "changelog should name the 040 feature"
    );
}
