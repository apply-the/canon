use std::fs;

const README_PATH: &str = "README.md";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const MODES_GUIDE: &str = "docs/guides/modes.md";
const ROADMAP_PATH: &str = "ROADMAP.md";
const SECURITY_TEMPLATE: &str = "docs/templates/canon-input/security-assessment.md";
const INCIDENT_TEMPLATE: &str = "docs/templates/canon-input/incident.md";
const SECURITY_EXAMPLE: &str = "docs/examples/canon-input/security-assessment-webhook-platform.md";

fn read(path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    fs::read_to_string(format!("{manifest_dir}/{path}"))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

#[test]
fn release_docs_describe_the_022_security_assessment_surface() {
    let readme = read(README_PATH);
    assert!(
        readme.contains("`security-assessment`")
            && readme.contains("recommendation-only")
            && readme.contains("threat")
            && readme.contains("risk")
            && readme.contains("evidence"),
        "README must summarize the 0.22.0 security-assessment surface"
    );

    let changelog = read(CHANGELOG_PATH);
    assert!(changelog.contains("## [0.22.0]"), "CHANGELOG missing 0.22.0 entry");
    assert!(
        changelog.contains("Cybersecurity Risk Assessment Mode"),
        "CHANGELOG must describe feature 023"
    );

    let guide = read(MODES_GUIDE);
    for heading in [
        "security-assessment",
        "docs/security-assessments/<RUN_ID>/",
        "Security risk assessment",
        "system_context=existing",
    ] {
        assert!(guide.contains(heading), "{MODES_GUIDE} missing {heading}");
    }
    assert!(
        guide.contains("approval-gated or")
            && guide.contains("blocked operational packets")
            && guide.contains("canon-input/security-assessment.md"),
        "modes guide must document the security-assessment operational flow and input binding"
    );

    let roadmap = read(ROADMAP_PATH);
    assert!(
        roadmap.contains("`security-assessment`")
            && roadmap.contains("docs/security-assessments/<RUN_ID>/")
            && roadmap.contains("Current end-to-end depth exists"),
        "ROADMAP must reflect the delivered security-assessment mode"
    );
}

#[test]
fn release_scaffolds_exist_for_022_examples_and_templates() {
    let security_template = read(SECURITY_TEMPLATE);
    for heading in [
        "## Assessment Scope",
        "## In-Scope Assets",
        "## Threat Inventory",
        "## Risk Findings",
        "## Evidence Gaps",
    ] {
        assert!(security_template.contains(heading), "{SECURITY_TEMPLATE} missing {heading}");
    }

    let incident_template = read(INCIDENT_TEMPLATE);
    for heading in [
        "## Incident Scope",
        "## Trigger And Current State",
        "## Operational Constraints",
        "## Follow-Up Work",
    ] {
        assert!(incident_template.contains(heading), "{INCIDENT_TEMPLATE} missing {heading}");
    }

    let security_example = read(SECURITY_EXAMPLE);
    for heading in [
        "## Assessment Scope",
        "## In-Scope Assets",
        "## Threat Inventory",
        "## Risk Findings",
        "## Evidence Gaps",
    ] {
        assert!(security_example.contains(heading), "{SECURITY_EXAMPLE} missing {heading}");
    }
}
