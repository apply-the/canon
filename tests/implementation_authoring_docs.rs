use std::fs;

const CONTRACT_PATH: &str =
    "specs/019-authoring-specialization-remaining/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-implementation/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-implementation/SKILL.md";
const TEMPLATE_PATH: &str = "docs/templates/canon-input/implementation.md";
const EXAMPLE_PATH: &str = "docs/examples/canon-input/implementation-auth-session-revocation.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";

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
fn implementation_contract_skill_template_and_example_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Implementation Required Sections"));

    for heading in CANONICAL_HEADINGS {
        assert!(contract.contains(&heading[3..]), "{CONTRACT_PATH} missing {heading}");
    }

    for path in [SKILL_SOURCE, TEMPLATE_PATH, EXAMPLE_PATH] {
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

#[test]
fn implementation_mode_guide_and_roadmap_document_the_remaining_shape_slice() {
    let guide = read(MODES_GUIDE);
    assert!(
        guide.contains("task-mapped delivery packet")
            && guide.contains("implementation lead")
            && guide.contains("bounded execution options"),
        "mode guide must describe the implementation packet shape and persona"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("## Delivered Feature: 031 Remaining Industry-Standard Artifact Shapes")
            && roadmap.contains("task-mapped delivery packet")
            && roadmap.contains("implementation lead"),
        "roadmap must record the delivered 031 implementation slice"
    );
    assert!(
        roadmap.contains("Persona guidance remains presentation only"),
        "roadmap must keep the implementation persona boundary explicit"
    );
}
