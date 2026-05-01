use std::fs;

const CONTRACT_PATH: &str =
    "specs/019-authoring-specialization-remaining/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-implementation/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-implementation/SKILL.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Task Mapping",
    "## Bounded Changes",
    "## Mutation Bounds",
    "## Allowed Paths",
    "## Executed Changes",
    "## Candidate Frameworks",
    "## Options Matrix",
    "## Decision Evidence",
    "## Recommendation",
    "## Task Linkage",
    "## Completion Evidence",
    "## Adoption Implications",
    "## Remaining Risks",
    "## Ecosystem Health",
    "## Safety-Net Evidence",
    "## Independent Checks",
    "## Rollback Triggers",
    "## Rollback Steps",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn implementation_contract_and_skill_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Implementation Required Sections"));

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
        skill_source.contains("Author Implementation Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
    assert!(
        skill_source.contains("### Packet Shape And Persona"),
        "skill source missing persona section"
    );
    assert!(
        skill_source.contains(
            "Author the packet as an implementation lead comparing bounded execution\noptions"
        ),
        "skill source missing persona role"
    );
    assert!(
        skill_source.contains("task-mapped delivery packet"),
        "skill source missing implementation packet-shape wording"
    );
    assert!(
        skill_source.contains("Persona guidance is presentation only."),
        "skill source missing persona guardrail"
    );
}

#[test]
fn implementation_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
