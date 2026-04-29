use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_supply_chain_analysis_artifact,
};

const FULL_BRIEF: &str = r#"# Supply Chain Analysis Brief

## Declared Scope

- Rust workspace dependencies and release manifests only.

## Licensing Posture

- Mixed OSS distribution with attribution obligations.

## Distribution Model

- Shipped as source releases and prebuilt CLI binaries.

## Ecosystems In Scope

- Cargo workspace crates.
- Rust advisory and license policy metadata.

## Out Of Scope Components

- External CI marketplace actions.

## Scanner Selection Rationale

- Use Rust-native dependency and advisory tooling first.

## SBOM Outputs

- Emit one machine-readable SBOM for the workspace crates.

## Findings By Severity

- High: shell invocation dependencies deserve focused review.
- Medium: CLI parsing dependencies should stay current.
- Low: documentation-only tooling drift is tracked separately.

## Exploitability Notes

- Findings affecting shell execution or artifact persistence matter most.

## Triage Decisions

- Escalate shell execution findings.

## Compatibility Classes

- Permissive licenses are acceptable for the current distribution model.

## Flagged Incompatibilities

- None confirmed from the authored inputs alone.

## Obligations

- Preserve notices and attribution for distributed dependencies.

## Outdated Dependencies

- Review lagging CLI and persistence dependencies first.

## End Of Life Signals

- No end-of-life signals are currently confirmed.

## Abandonment Signals

- No abandonment signals are currently confirmed.

## Modernization Slices

1. Refresh shell-execution-adjacent crates first.
2. Revisit advisory and license tooling after the first pass.

## Scanner Decisions

- installed: Rust-native OSS tooling only.

## Coverage Gaps

- No non-Rust ecosystem coverage is included in this packet.

## Source Inputs

- Cargo.toml
- deny.toml

## Independent Checks

- cargo test --test supply_chain_analysis_direct_runtime

## Deferred Verification

- Attach final scanner outputs before external review.
"#;

const MISSING_COVERAGE_GAPS_BRIEF: &str = r#"# Supply Chain Analysis Brief

## Scanner Selection Rationale

- Use Rust-native dependency and advisory tooling first.

## SBOM Outputs

- Emit one machine-readable SBOM for the workspace crates.

## Scanner Decisions

- installed: Rust-native OSS tooling only.
"#;

const NEAR_MISS_BRIEF: &str = r#"# Supply Chain Analysis Brief

## Scanner Decisions

- installed: Rust-native OSS tooling only.

## Coverage Gap

This near-miss heading should not satisfy the canonical contract.
"#;

#[test]
fn supply_chain_renderer_preserves_authored_sections_verbatim() {
    let overview = render_supply_chain_analysis_artifact("analysis-overview.md", FULL_BRIEF);
    let vulnerability =
        render_supply_chain_analysis_artifact("vulnerability-triage.md", FULL_BRIEF);
    let license = render_supply_chain_analysis_artifact("license-compliance.md", FULL_BRIEF);
    let legacy = render_supply_chain_analysis_artifact("legacy-posture.md", FULL_BRIEF);
    let policy = render_supply_chain_analysis_artifact("policy-decisions.md", FULL_BRIEF);

    assert!(overview.contains(
        "## Declared Scope\n\n- Rust workspace dependencies and release manifests only."
    ));
    assert!(overview.contains("## Ecosystems In Scope\n\n- Cargo workspace crates.\n- Rust advisory and license policy metadata."));
    assert!(!overview.contains(MISSING_AUTHORED_BODY_MARKER));

    assert!(vulnerability.contains(
        "## Findings By Severity\n\n- High: shell invocation dependencies deserve focused review."
    ));
    assert!(vulnerability.contains("## Triage Decisions\n\n- Escalate shell execution findings."));

    assert!(license.contains("## Compatibility Classes\n\n- Permissive licenses are acceptable for the current distribution model."));
    assert!(license.contains(
        "## Obligations\n\n- Preserve notices and attribution for distributed dependencies."
    ));

    assert!(
        legacy.contains(
            "## Modernization Slices\n\n1. Refresh shell-execution-adjacent crates first."
        )
    );
    assert!(
        legacy.contains(
            "## End Of Life Signals\n\n- No end-of-life signals are currently confirmed."
        )
    );

    assert!(policy.contains("## Scanner Decisions\n\n- installed: Rust-native OSS tooling only."));
    assert!(policy.contains(
        "## Coverage Gaps\n\n- No non-Rust ecosystem coverage is included in this packet."
    ));
}

#[test]
fn supply_chain_renderer_emits_missing_body_marker_for_missing_heading() {
    let rendered =
        render_supply_chain_analysis_artifact("sbom-bundle.md", MISSING_COVERAGE_GAPS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Coverage Gaps`"));
}

#[test]
fn supply_chain_renderer_treats_near_miss_heading_as_missing() {
    let rendered = render_supply_chain_analysis_artifact("policy-decisions.md", NEAR_MISS_BRIEF);

    assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
    assert!(rendered.contains("`## Coverage Gaps`"));
    assert!(!rendered.contains(
        "## Coverage Gap\n\nThis near-miss heading should not satisfy the canonical contract."
    ));
}
