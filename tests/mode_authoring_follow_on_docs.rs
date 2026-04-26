use std::fs;

const ROADMAP_PATH: &str = "ROADMAP.md";
const MODES_GUIDE_PATH: &str = "docs/guides/modes.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn roadmap_records_the_delivered_follow_on_slice_and_keeps_remaining_scope_explicit() {
    let roadmap = read(ROADMAP_PATH);

    assert!(
        roadmap.contains(
            "`system-shaping`, `implementation`, and `refactor` now share canonical authored H2 contracts"
        ),
        "roadmap must record the delivered 019 follow-on slice"
    );
    assert!(
        roadmap.contains("The remaining authoring-specialization rollout is now limited to `review`, `verification`, `incident`, and `migration`."),
        "roadmap must keep the remaining scope explicit after delivering the 019 slice"
    );
}

#[test]
fn modes_guide_describes_the_canonical_h2_contract_and_blocking_behavior_for_targeted_modes() {
    let guide = read(MODES_GUIDE_PATH);

    for heading in [
        "`## System Shape`",
        "`## Boundary Decisions`",
        "`## Delivery Phases`",
        "`## Hotspots`",
        "`## Task Mapping`",
        "`## Rollback Steps`",
        "`## Preserved Behavior`",
        "`## Decision`",
    ] {
        assert!(guide.contains(heading), "mode guide missing {heading}");
    }

    assert!(
        guide.contains("emit `## Missing Authored Body` naming the missing canonical heading and block the packet honestly"),
        "mode guide must describe the missing-body plus blocked-packet behavior"
    );
}
