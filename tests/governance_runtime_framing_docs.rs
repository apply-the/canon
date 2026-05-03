use std::fs;

const README: &str = "README.md";
const GETTING_STARTED: &str = "docs/guides/getting-started.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const GOVERNANCE_ADAPTER_GUIDE: &str = "docs/integration/governance-adapter.md";

fn read(path: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
        .unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn opening_docs_frame_canon_as_governed_runtime_with_a_human_happy_path() {
    let readme = read(README);
    let getting_started = read(GETTING_STARTED);

    assert!(
        readme.contains("governed packet runtime for AI-assisted engineering"),
        "README should frame Canon as the governed packet runtime"
    );
    assert!(
        readme.contains("not a generic agent framework"),
        "README should keep the non-goal explicit"
    );
    assert!(
        getting_started.contains("inspect clarity"),
        "getting started should include inspect clarity in the human happy path"
    );
    assert!(
        getting_started.contains("canon publish <RUN_ID>"),
        "getting started should keep publish in the happy path"
    );
}

#[test]
fn governance_adapter_docs_define_the_machine_boundary_and_examples() {
    let readme = read(README);
    let modes_guide = read(MODES_GUIDE);
    let adapter_guide = read(GOVERNANCE_ADAPTER_GUIDE);

    assert!(
        readme.contains("docs/integration/governance-adapter.md"),
        "README should link to the dedicated governance adapter guide"
    );
    assert!(
        modes_guide.contains("docs/integration/governance-adapter.md"),
        "modes guide should link to the dedicated governance adapter guide"
    );

    for command in [
        "canon governance capabilities --json",
        "canon governance start --json < request.json",
        "canon governance refresh --json < request.json",
    ] {
        assert!(
            adapter_guide.contains(command),
            "adapter guide should include command `{command}`"
        );
    }

    for field in
        ["status", "approval_state", "packet_readiness", "reason_code", "workspace-relative"]
    {
        assert!(adapter_guide.contains(field), "adapter guide should document `{field}`");
    }

    for example_mode in ["change", "implementation", "verification", "pr-review"] {
        assert!(
            adapter_guide.contains(example_mode),
            "adapter guide should include an example for `{example_mode}`"
        );
    }

    assert!(
        adapter_guide.contains("same runtime"),
        "adapter guide should say it wraps the same runtime"
    );
    assert!(
        adapter_guide.contains("not the higher-level orchestrator"),
        "adapter guide should keep the boundary with external orchestrators explicit"
    );
}
