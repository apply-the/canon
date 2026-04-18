use canon_engine::domain::mode::{ImplementationDepth, Mode, all_mode_profiles};

#[test]
fn greenfield_alias_is_no_longer_accepted_for_mode_parsing() {
    assert_eq!("system-shaping".parse::<Mode>(), Ok(Mode::Greenfield));
    assert!("greenfield".parse::<Mode>().is_err());
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

    let staged_modes = [
        Mode::Implementation,
        Mode::Refactor,
        Mode::Verification,
        Mode::Review,
        Mode::Incident,
        Mode::Migration,
    ];

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
        Mode::Greenfield,
        Mode::BrownfieldChange,
        Mode::Architecture,
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
