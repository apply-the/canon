use std::fs;

const CONTRACT_PATH: &str =
    "specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-migration/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-migration/SKILL.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Current State",
    "## Target State",
    "## Transition Boundaries",
    "## Guaranteed Compatibility",
    "## Temporary Incompatibilities",
    "## Coexistence Rules",
    "## Options Matrix",
    "## Ordered Steps",
    "## Parallelizable Work",
    "## Cutover Criteria",
    "## Rollback Triggers",
    "## Fallback Paths",
    "## Re-Entry Criteria",
    "## Adoption Implications",
    "## Verification Checks",
    "## Residual Risks",
    "## Release Readiness",
    "## Migration Decisions",
    "## Tradeoff Analysis",
    "## Decision Evidence",
    "## Recommendation",
    "## Why Not The Others",
    "## Ecosystem Health",
    "## Deferred Decisions",
    "## Approval Notes",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn migration_contract_and_skill_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Migration Required Sections"));

    for heading in CANONICAL_HEADINGS {
        assert!(contract.contains(&heading[3..]), "{CONTRACT_PATH} missing {heading}");
    }

    for path in [SKILL_SOURCE] {
        let content = read(path);
        for heading in CANONICAL_HEADINGS {
            assert!(content.contains(heading), "{path} missing {heading}");
        }
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Migration Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
}

#[test]
fn migration_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
