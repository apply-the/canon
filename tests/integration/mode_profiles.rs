use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::{ImplementationDepth, Mode, all_mode_profiles};

#[test]
fn system_shaping_mode_is_available_only_through_historical_parsing() {
    assert!("system-shaping".parse::<Mode>().is_err());
    assert_eq!(Mode::parse_historical("system-shaping"), Ok(Mode::SystemShaping));
}

#[test]
fn legacy_public_mode_names_fail_with_generic_unsupported_mode_errors() {
    for legacy_name in ["brownfield-change", "brownfield", "greenfield"] {
        assert!(legacy_name.parse::<Mode>().is_err());
    }
}

#[test]
fn all_stable_modes_have_typed_profiles_and_supported_depths_match_runtime_truth() {
    let profiles = all_mode_profiles();
    assert_eq!(profiles.len(), Mode::all().len(), "every mode should have a profile");

    for mode in Mode::all() {
        assert!(
            profiles.iter().any(|profile| profile.mode == *mode),
            "missing typed profile for mode `{}`",
            mode.as_str()
        );
    }

    for mode in Mode::all() {
        let profile =
            profiles.iter().find(|profile| profile.mode == *mode).expect("profile should exist");
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

    for mode in Mode::all() {
        let profile = profiles
            .iter()
            .find(|profile| profile.mode == *mode)
            .expect("deep mode profile should exist");
        assert!(
            matches!(profile.implementation_depth, ImplementationDepth::Full),
            "deep mode `{}` should be fully implemented in v0.1",
            mode.as_str()
        );
    }
}

#[test]
fn stable_profiles_advertise_distinct_governance_artifact_families() {
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

    assert!(
        profiles.iter().all(|profile| profile.mode != Mode::Implementation),
        "implementation must not have a stable Canon profile"
    );

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
}
