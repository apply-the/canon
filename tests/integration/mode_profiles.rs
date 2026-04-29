use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::{ImplementationDepth, Mode, all_mode_profiles};

#[test]
fn system_shaping_mode_parses_through_the_public_name() {
    assert_eq!("system-shaping".parse::<Mode>(), Ok(Mode::SystemShaping));
}

#[test]
fn legacy_public_mode_names_fail_with_generic_unsupported_mode_errors() {
    for legacy_name in ["brownfield-change", "brownfield", "greenfield"] {
        assert_eq!(legacy_name.parse::<Mode>(), Err(format!("unsupported mode: {legacy_name}")));
    }
}

#[test]
fn all_modes_have_typed_profiles_and_supported_depths_match_runtime_truth() {
    let profiles = all_mode_profiles();
    assert_eq!(profiles.len(), Mode::all().len(), "every mode should have a profile");

    for mode in Mode::all() {
        assert!(
            profiles.iter().any(|profile| profile.mode == *mode),
            "missing typed profile for mode `{}`",
            mode.as_str()
        );
    }

    for mode in
        [Mode::Incident, Mode::SecurityAssessment, Mode::Migration, Mode::SupplyChainAnalysis]
    {
        let profile =
            profiles.iter().find(|profile| profile.mode == mode).expect("profile should exist");
        assert!(
            matches!(profile.implementation_depth, ImplementationDepth::Full),
            "operational mode `{}` should be fully implemented once High-Risk Operational Programs lands",
            mode.as_str()
        );
        assert!(
            !profile.gate_profile.is_empty(),
            "mode `{}` should declare at least one gate",
            mode.as_str()
        );
        assert!(
            !profile.artifact_families.is_empty(),
            "mode `{}` should declare artifact families",
            mode.as_str()
        );
        assert!(
            !profile.allowed_adapters.is_empty(),
            "mode `{}` should declare allowed adapters",
            mode.as_str()
        );
    }

    for mode in [
        Mode::Requirements,
        Mode::Discovery,
        Mode::SystemShaping,
        Mode::Change,
        Mode::Backlog,
        Mode::Architecture,
        Mode::Implementation,
        Mode::Refactor,
        Mode::Verification,
        Mode::Review,
        Mode::PrReview,
        Mode::Incident,
        Mode::SecurityAssessment,
        Mode::Migration,
        Mode::SupplyChainAnalysis,
    ] {
        let profile = profiles
            .iter()
            .find(|profile| profile.mode == mode)
            .expect("deep mode profile should exist");
        assert!(
            matches!(profile.implementation_depth, ImplementationDepth::Full),
            "deep mode `{}` should be fully implemented in v0.1",
            mode.as_str()
        );
    }
}

#[test]
fn promoted_execution_modes_advertise_distinct_artifact_families() {
    let profiles = all_mode_profiles();

    let backlog =
        profiles.iter().find(|profile| profile.mode == Mode::Backlog).expect("backlog profile");
    assert_eq!(
        backlog.artifact_families,
        vec![
            "backlog overview",
            "epic tree",
            "capability map",
            "dependency map",
            "delivery slices",
            "sequencing plan",
            "acceptance anchors",
            "planning risks",
        ]
    );
    assert!(matches!(backlog.implementation_depth, ImplementationDepth::Full));

    let implementation = profiles
        .iter()
        .find(|profile| profile.mode == Mode::Implementation)
        .expect("implementation profile");
    assert_eq!(
        implementation.artifact_families,
        vec![
            "task mapping",
            "mutation bounds",
            "implementation notes",
            "completion evidence",
            "validation hooks",
            "rollback notes",
        ]
    );
    assert!(matches!(implementation.implementation_depth, ImplementationDepth::Full));

    let refactor =
        profiles.iter().find(|profile| profile.mode == Mode::Refactor).expect("refactor profile");
    assert_eq!(
        refactor.artifact_families,
        vec![
            "preserved behavior",
            "refactor scope",
            "structural rationale",
            "regression evidence",
            "contract drift check",
            "no feature addition",
        ]
    );
    assert!(matches!(refactor.implementation_depth, ImplementationDepth::Full));

    let incident =
        profiles.iter().find(|profile| profile.mode == Mode::Incident).expect("incident profile");
    assert_eq!(
        incident.artifact_families,
        vec![
            "incident frame",
            "hypothesis log",
            "blast radius map",
            "containment plan",
            "incident decision record",
            "follow-up verification",
        ]
    );
    assert_eq!(
        incident.gate_profile,
        vec![
            GateKind::Risk,
            GateKind::IncidentContainment,
            GateKind::Architecture,
            GateKind::ReleaseReadiness,
        ]
    );
    assert!(matches!(incident.implementation_depth, ImplementationDepth::Full));

    let migration =
        profiles.iter().find(|profile| profile.mode == Mode::Migration).expect("migration profile");
    assert_eq!(
        migration.artifact_families,
        vec![
            "source-target map",
            "compatibility matrix",
            "sequencing plan",
            "fallback plan",
            "migration verification report",
            "decision record",
        ]
    );
    assert_eq!(
        migration.gate_profile,
        vec![
            GateKind::Exploration,
            GateKind::Architecture,
            GateKind::MigrationSafety,
            GateKind::Risk,
            GateKind::ReleaseReadiness,
        ]
    );
    assert!(matches!(migration.implementation_depth, ImplementationDepth::Full));

    let security_assessment = profiles
        .iter()
        .find(|profile| profile.mode == Mode::SecurityAssessment)
        .expect("security-assessment profile");
    assert_eq!(
        security_assessment.artifact_families,
        vec![
            "assessment overview",
            "threat model",
            "risk register",
            "mitigations",
            "assumptions and gaps",
            "assessment evidence",
        ]
    );
    assert_eq!(
        security_assessment.gate_profile,
        vec![GateKind::Risk, GateKind::Architecture, GateKind::ReleaseReadiness,]
    );
    assert!(matches!(security_assessment.implementation_depth, ImplementationDepth::Full));

    let supply_chain = profiles
        .iter()
        .find(|profile| profile.mode == Mode::SupplyChainAnalysis)
        .expect("supply-chain-analysis profile");
    assert_eq!(
        supply_chain.artifact_families,
        vec![
            "analysis overview",
            "sbom bundle",
            "vulnerability triage",
            "license compliance",
            "legacy posture",
            "policy decisions",
            "analysis evidence",
        ]
    );
    assert_eq!(supply_chain.gate_profile, vec![GateKind::Risk, GateKind::ReleaseReadiness]);
    assert!(matches!(supply_chain.implementation_depth, ImplementationDepth::Full));
}
