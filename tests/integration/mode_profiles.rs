use canon_engine::domain::mode::{ImplementationDepth, Mode, all_mode_profiles};

#[test]
fn all_modes_have_typed_profiles_and_non_mvp_modes_remain_staged() {
    let profiles = all_mode_profiles();
    assert_eq!(profiles.len(), Mode::all().len(), "every mode should have a profile");

    for mode in Mode::all() {
        assert!(
            profiles.iter().any(|profile| profile.mode == *mode),
            "missing typed profile for mode `{}`",
            mode.as_str()
        );
    }

    let non_mvp_modes = [
        Mode::Discovery,
        Mode::Greenfield,
        Mode::Architecture,
        Mode::Implementation,
        Mode::Refactor,
        Mode::Verification,
        Mode::Review,
        Mode::Incident,
        Mode::Migration,
    ];

    for mode in non_mvp_modes {
        let profile =
            profiles.iter().find(|profile| profile.mode == mode).expect("profile should exist");
        assert!(
            matches!(
                profile.implementation_depth,
                ImplementationDepth::ContractOnly | ImplementationDepth::Skeleton
            ),
            "non-MVP mode `{}` should remain staged in v0.1",
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

    for mode in [Mode::Requirements, Mode::BrownfieldChange, Mode::PrReview] {
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
