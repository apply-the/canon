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

    let staged_modes = [Mode::Incident, Mode::Migration];

    for mode in staged_modes {
        let profile =
            profiles.iter().find(|profile| profile.mode == mode).expect("profile should exist");
        assert!(
            matches!(
                profile.implementation_depth,
                ImplementationDepth::ContractOnly | ImplementationDepth::Skeleton
            ),
            "staged mode `{}` should remain limited in v0.1",
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
        Mode::Architecture,
        Mode::Implementation,
        Mode::Refactor,
        Mode::Verification,
        Mode::Review,
        Mode::PrReview,
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
}
