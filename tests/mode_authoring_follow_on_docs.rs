use std::fs;

const ROADMAP_PATH: &str = "ROADMAP.md";
const MODES_GUIDE_PATH: &str = "docs/guides/modes.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn roadmap_records_the_completed_authoring_specialization_slice() {
    let roadmap = read(ROADMAP_PATH);

    assert!(
        roadmap.contains(
            "`review`, `verification`, `incident`, and `migration` now share canonical authored H2 contracts"
        ),
        "roadmap must record the delivered 020 completion slice"
    );
    assert!(
        roadmap.contains(
            "Mode Authoring Specialization is now complete for the currently modeled governed modes."
        ),
        "roadmap must record that the modeled-mode rollout is complete"
    );
}

#[test]
fn modes_guide_describes_the_canonical_h2_contract_and_blocking_behavior_for_targeted_modes() {
    let guide = read(MODES_GUIDE_PATH);

    for heading in [
        "`## Review Target`",
        "`## Final Disposition`",
        "`## Claims Under Test`",
        "`## Overall Verdict`",
        "`## Incident Scope`",
        "`## Follow-Up Work`",
        "`## Current State`",
        "`## Re-Entry Criteria`",
    ] {
        assert!(guide.contains(heading), "mode guide missing {heading}");
    }

    assert!(
        guide.contains("emit `## Missing Authored Body` naming the missing canonical heading and block the packet honestly"),
        "mode guide must describe the missing-body plus blocked-packet behavior"
    );
}
