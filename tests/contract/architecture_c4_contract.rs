use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn architecture_contract_includes_three_c4_artifacts_with_expected_gates() {
    let contract = contract_for_mode(Mode::Architecture);

    let by_name = |name: &str| {
        contract
            .artifact_requirements
            .iter()
            .find(|requirement| requirement.file_name == name)
            .unwrap_or_else(|| panic!("missing C4 artifact requirement: {name}"))
    };

    let system_context = by_name("system-context.md");
    assert_eq!(system_context.gates, vec![GateKind::Architecture, GateKind::Exploration]);

    let container_view = by_name("container-view.md");
    assert_eq!(container_view.gates, vec![GateKind::Architecture]);

    let component_view = by_name("component-view.md");
    assert_eq!(component_view.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
}

#[test]
fn architecture_artifact_set_size_grows_to_eight_with_c4_extension() {
    let contract = contract_for_mode(Mode::Architecture);
    assert_eq!(contract.artifact_requirements.len(), 8);
}
