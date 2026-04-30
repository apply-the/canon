use std::fs;
use std::path::Path;

#[test]
fn system_assessment_method_skill_template_and_example_stay_aligned() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let method = fs::read_to_string(repo_root.join("defaults/methods/system-assessment.toml"))
        .expect("read method");
    assert!(method.contains("mode = \"system-assessment\""));
    assert!(method.contains("\"coverage-map.md\""));
    assert!(method.contains("\"assessment-evidence.md\""));

    let source_skill = fs::read_to_string(
        repo_root.join("defaults/embedded-skills/canon-system-assessment/skill-source.md"),
    )
    .expect("read source skill");
    let materialized_skill =
        fs::read_to_string(repo_root.join(".agents/skills/canon-system-assessment/SKILL.md"))
            .expect("read materialized skill");
    assert_eq!(
        source_skill, materialized_skill,
        "materialized skill should mirror embedded source"
    );
    assert!(source_skill.contains("available-now"));
    assert!(source_skill.contains("Author System Assessment Body Before Invoking Canon"));
    assert!(source_skill.contains("canon-input/system-assessment.md"));
    assert!(source_skill.contains("--system-context existing"));
    assert!(source_skill.contains("## Missing Authored Body"));
    assert!(source_skill.contains("## Observed Findings"));
    assert!(source_skill.contains("## Inferred Findings"));
    assert!(source_skill.contains("## Assessment Gaps"));

    let template =
        fs::read_to_string(repo_root.join("docs/templates/canon-input/system-assessment.md"))
            .expect("read template");
    assert!(template.contains("# System Assessment Brief"));
    assert!(template.contains("## Assessed Views"));
    assert!(template.contains("## Confidence By Surface"));
    assert!(template.contains("## Observed Findings"));
    assert!(template.contains("## Inferred Findings"));
    assert!(template.contains("## Assessment Gaps"));

    let example = fs::read_to_string(
        repo_root.join("docs/examples/canon-input/system-assessment-commerce-platform.md"),
    )
    .expect("read example");
    assert!(example.contains("Commerce Checkout Platform"));
    assert!(example.contains("## Assessed Views"));
    assert!(example.contains("## Observed Risks"));
    assert!(example.contains("## Likely Follow-On Modes"));
    assert!(example.contains("`security-assessment`"));
}
