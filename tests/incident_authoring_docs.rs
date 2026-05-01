use std::fs;

const CONTRACT_PATH: &str =
    "specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-incident/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-incident/SKILL.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Incident Scope",
    "## Trigger And Current State",
    "## Operational Constraints",
    "## Known Facts",
    "## Working Hypotheses",
    "## Evidence Gaps",
    "## Impacted Surfaces",
    "## Propagation Paths",
    "## Confidence And Unknowns",
    "## Immediate Actions",
    "## Ordered Sequence",
    "## Stop Conditions",
    "## Decision Points",
    "## Approved Actions",
    "## Deferred Actions",
    "## Verification Checks",
    "## Release Readiness",
    "## Follow-Up Work",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn incident_contract_and_skill_share_canonical_headings() {
    let contract = read(CONTRACT_PATH);
    assert!(contract.contains("Incident Required Sections"));

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
        skill_source.contains("Author Incident Body Before Invoking Canon"),
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
        skill_source.contains("Author the packet as an incident commander"),
        "skill source missing persona role"
    );
    assert!(
        skill_source.contains("Persona guidance is presentation only."),
        "skill source missing persona guardrail"
    );
}

#[test]
fn incident_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
