use std::fs;

const CONTRACT_PATH: &str =
    "specs/016-mode-authoring-specialization/contracts/requirements-authoring.md";
const SKILL_SOURCE: &str = "defaults/embedded-skills/canon-requirements/skill-source.md";
const SKILL_MIRROR: &str = ".agents/skills/canon-requirements/SKILL.md";

const CANONICAL_HEADINGS: &[&str] = &[
    "## Problem",
    "## Outcome",
    "## Constraints",
    "## Non-Negotiables",
    "## Options",
    "## Recommended Path",
    "## Tradeoffs",
    "## Consequences",
    "## Scope Cuts",
    "## Deferred Work",
    "## Decision Checklist",
    "## Open Questions",
];

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn assert_lists_headings(path: &str, content: &str, headings: &[&str]) {
    for heading in headings {
        assert!(content.contains(heading), "{path} missing {heading}");
    }
}

#[test]
fn requirements_contract_and_skill_share_canonical_headings() {
    for path in [CONTRACT_PATH, SKILL_SOURCE] {
        let content = read(path);
        assert_lists_headings(path, &content, CANONICAL_HEADINGS);
    }

    let skill_source = read(SKILL_SOURCE);
    assert!(
        skill_source.contains("Author Requirements Body Before Invoking Canon"),
        "skill source missing authored-body section"
    );
    assert!(
        skill_source.contains("Missing Authored Body"),
        "skill source must mention the missing-body marker"
    );
    assert!(
        skill_source.contains("### Authoring Persona")
            && skill_source.contains("product lead")
            && skill_source.contains("Persona guidance is presentation only"),
        "skill source must document the bounded product persona"
    );
}

#[test]
fn requirements_skill_mirror_matches_skill_source() {
    assert_eq!(read(SKILL_SOURCE), read(SKILL_MIRROR));
}
