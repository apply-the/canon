use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::gate::GateKind;
use canon_engine::domain::mode::Mode;

#[test]
fn architecture_contract_includes_pragmatic_c4_and_machine_readable_view_artifacts() {
    let contract = contract_for_mode(Mode::Architecture);

    let by_name = |name: &str| {
        contract
            .artifact_requirements
            .iter()
            .find(|requirement| requirement.slug() == name)
            .unwrap_or_else(|| panic!("missing C4 artifact requirement: {name}"))
    };

    let system_context = by_name("system-context.md");
    assert_eq!(system_context.required_sections, vec!["System Context"]);
    assert_eq!(system_context.gates, vec![GateKind::Architecture, GateKind::Exploration]);
    assert!(system_context.required);

    let system_context_mermaid = by_name("system-context.mmd");
    assert!(system_context_mermaid.required_sections.is_empty());
    assert_eq!(system_context_mermaid.gates, vec![GateKind::Architecture, GateKind::Exploration]);
    assert!(system_context_mermaid.required);

    let container_view = by_name("container-view.md");
    assert_eq!(container_view.required_sections, vec!["Containers"]);
    assert_eq!(container_view.gates, vec![GateKind::Architecture]);
    assert!(container_view.required);

    let deployment_view = by_name("deployment-view.md");
    assert_eq!(deployment_view.required_sections, vec!["Deployment"]);
    assert_eq!(deployment_view.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
    assert!(deployment_view.required);

    let component_view = by_name("component-view.md");
    assert_eq!(component_view.required_sections, vec!["Components"]);
    assert_eq!(component_view.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
    assert!(!component_view.required);

    let dynamic_view = by_name("dynamic-view.md");
    assert_eq!(dynamic_view.required_sections, vec!["Dynamic View"]);
    assert_eq!(dynamic_view.gates, vec![GateKind::Architecture, GateKind::ReleaseReadiness]);
    assert!(!dynamic_view.required);
}

#[test]
fn architecture_artifact_set_size_grows_to_nineteen_with_overview_and_visual_sidecars() {
    let contract = contract_for_mode(Mode::Architecture);
    assert_eq!(contract.artifact_requirements.len(), 19);
}
