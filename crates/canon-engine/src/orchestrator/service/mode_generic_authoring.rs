use super::EngineService;
use super::authoring_workflow::AuthoringWorkflowSpec;
use super::*;

impl EngineService {
    pub(super) fn run_generic_authoring(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let (spec, is_brainstorming) = match request.mode {
            Mode::PolicyShaping => (
                AuthoringWorkflowSpec {
                    mode: Mode::PolicyShaping,
                    context_summary: "capture policy-shaping context",
                    generation_summary: "generate policy-shaping impact report",
                    critique_summary: "critique policy-shaping impact report",
                },
                false,
            ),
            Mode::Brainstorming => (
                AuthoringWorkflowSpec {
                    mode: Mode::Brainstorming,
                    context_summary: "capture brainstorming context and problem space",
                    generation_summary: "generate bounded brainstorming analysis",
                    critique_summary: "critique bounded brainstorming analysis",
                },
                true,
            ),
            _ => return Err(EngineError::UnsupportedMode(request.mode.as_str().to_string())),
        };

        self.execute_authoring_workflow(
            store,
            request,
            policy_set,
            spec,
            |file_name, context_summary, generation_summary, critique_summary| {
                if is_brainstorming {
                    crate::artifacts::markdown::render_brainstorming_artifact(
                        file_name,
                        context_summary,
                        generation_summary,
                        critique_summary,
                    )
                } else {
                    crate::artifacts::markdown::render_markdown(file_name, context_summary)
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn run_brainstorming_executes_autonomous_flow() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());
        let service = EngineService::new(workspace.path());

        let request = RunRequest {
            mode: Mode::Brainstorming,
            risk: crate::domain::policy::RiskClass::LowImpact,
            zone: crate::domain::policy::UsageZone::Green,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "tester".to_string(),
            inputs: Vec::new(),
            inline_inputs: vec!["Some brainstorming content.".to_string()],
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let policy_set = crate::domain::policy::PolicySet {
            risk_classes: vec![],
            zones: vec![],
            gate_policy: crate::domain::policy::GatePolicy { mandatory_gates: vec![] },
            adapter_matrix: vec![],
            constraint_profiles: vec![],
            runtime_disabled_adapters: vec![],
            validation_independence: crate::domain::policy::ValidationIndependencePolicy {
                ai_generation_requires_distinct_validation: false,
                human_review_counts_independent: true,
            },
            block_mutation_for_red_or_systemic: false,
        };
        let summary = service
            .run_generic_authoring(&store, request, policy_set)
            .expect("run generic authoring");
        assert_eq!(summary.mode, Mode::Brainstorming.as_str());
        assert!(summary.artifact_count > 0);
    }

    #[test]
    fn run_policy_shaping_executes_autonomous_flow() {
        let workspace = TempDir::new().expect("temp dir");
        let store = WorkspaceStore::new(workspace.path());
        let service = EngineService::new(workspace.path());

        let request = RunRequest {
            mode: Mode::PolicyShaping,
            risk: crate::domain::policy::RiskClass::LowImpact,
            zone: crate::domain::policy::UsageZone::Green,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "tester".to_string(),
            inputs: Vec::new(),
            inline_inputs: vec!["Some policy shaping content.".to_string()],
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let policy_set = crate::domain::policy::PolicySet {
            risk_classes: vec![],
            zones: vec![],
            gate_policy: crate::domain::policy::GatePolicy { mandatory_gates: vec![] },
            adapter_matrix: vec![],
            constraint_profiles: vec![],
            runtime_disabled_adapters: vec![],
            validation_independence: crate::domain::policy::ValidationIndependencePolicy {
                ai_generation_requires_distinct_validation: false,
                human_review_counts_independent: true,
            },
            block_mutation_for_red_or_systemic: false,
        };
        let summary = service
            .run_generic_authoring(&store, request, policy_set)
            .expect("run generic authoring for policy shaping");
        assert_eq!(summary.mode, Mode::PolicyShaping.as_str());
        assert!(summary.artifact_count > 0);
    }
}
