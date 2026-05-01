use std::fs;

const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-supply-chain-analysis/SKILL.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn supply_chain_skill_source_and_mirror_stay_in_sync() {
    let source = read(SKILL_SOURCE);
    let mirror = read(SKILL_MIRROR);
    assert_eq!(source, mirror, "supply-chain skill source and mirror should match exactly");
}
