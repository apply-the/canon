use canon_adapters::AdapterKind;
use canon_adapters::dispatcher::DispatchDisposition;
use canon_adapters::shell::ShellAdapter;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{
    AdapterPolicyMatrix, GatePolicy, PolicySet, RiskClass, RiskPolicyClass, UsageZone, ZonePolicy,
};
use canon_engine::domain::policy::{InvocationConstraintProfile, ValidationIndependencePolicy};
use canon_engine::domain::verification::VerificationLayer;
use canon_engine::orchestrator::classifier::{MutationPolicy, mutation_policy_for_mode};

fn sample_policy_set() -> PolicySet {
    PolicySet {
        risk_classes: vec![
            RiskPolicyClass {
                name: RiskClass::LowImpact,
                requires_owner: false,
                mutable_execution: true,
                verification_layers: vec![VerificationLayer::SelfCritique],
            },
            RiskPolicyClass {
                name: RiskClass::BoundedImpact,
                requires_owner: true,
                mutable_execution: true,
                verification_layers: vec![
                    VerificationLayer::SelfCritique,
                    VerificationLayer::PeerReview,
                ],
            },
            RiskPolicyClass {
                name: RiskClass::SystemicImpact,
                requires_owner: true,
                mutable_execution: false,
                verification_layers: vec![VerificationLayer::ArchitecturalReview],
            },
        ],
        zones: vec![
            ZonePolicy { name: UsageZone::Green, mutable_execution: true },
            ZonePolicy { name: UsageZone::Yellow, mutable_execution: true },
            ZonePolicy { name: UsageZone::Red, mutable_execution: false },
        ],
        gate_policy: GatePolicy { mandatory_gates: Vec::new() },
        adapter_matrix: vec![
            AdapterPolicyMatrix {
                adapter: AdapterKind::Filesystem,
                capabilities: vec![
                    canon_adapters::CapabilityKind::ReadRepository,
                    canon_adapters::CapabilityKind::ReadArtifact,
                    canon_adapters::CapabilityKind::EmitArtifact,
                ],
            },
            AdapterPolicyMatrix {
                adapter: AdapterKind::Shell,
                capabilities: vec![
                    canon_adapters::CapabilityKind::RunCommand,
                    canon_adapters::CapabilityKind::ValidateWithTool,
                    canon_adapters::CapabilityKind::ExecuteBoundedTransformation,
                ],
            },
            AdapterPolicyMatrix {
                adapter: AdapterKind::CopilotCli,
                capabilities: vec![
                    canon_adapters::CapabilityKind::GenerateContent,
                    canon_adapters::CapabilityKind::CritiqueContent,
                    canon_adapters::CapabilityKind::ProposeWorkspaceEdit,
                ],
            },
        ],
        constraint_profiles: vec![InvocationConstraintProfile {
            id: "requirements-generation".to_string(),
            payload_retention:
                canon_engine::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
            max_payload_bytes: Some(16 * 1024),
            command_profile: Some("requirements-generation".to_string()),
            recommendation_only: false,
            patch_disabled: true,
        }],
        runtime_disabled_adapters: vec![AdapterKind::McpStdio],
        validation_independence: ValidationIndependencePolicy {
            ai_generation_requires_distinct_validation: true,
            human_review_counts_independent: true,
        },
        block_mutation_for_red_or_systemic: true,
    }
}

#[test]
fn shell_adapter_separates_read_only_and_mutating_capabilities() {
    let shell = ShellAdapter;

    let read_only = shell.read_only_request("inspect diff");
    assert_eq!(read_only.capability, canon_adapters::CapabilityKind::RunCommand);
    assert_eq!(read_only.side_effect, canon_adapters::SideEffectClass::ReadOnly);

    let mutating = shell.mutating_request("apply patch");
    assert_eq!(mutating.capability, canon_adapters::CapabilityKind::ExecuteBoundedTransformation);
    assert_eq!(mutating.side_effect, canon_adapters::SideEffectClass::WorkspaceMutation);

    assert_eq!(
        canon_adapters::dispatcher::dispatch_disposition(&mutating, false),
        DispatchDisposition::RecommendationOnly
    );
}

#[test]
fn shell_adapter_blocks_mutating_execution_without_permission() {
    let shell = ShellAdapter;
    let request = shell.mutating_request("attempt mutation");
    let error = shell
        .run(&request, "cargo", &["--version"], None, false)
        .expect_err("mutating command should be blocked");
    assert!(matches!(error, canon_adapters::AdapterError::MutationBlocked));
}

#[test]
fn change_red_or_systemic_work_becomes_recommendation_only() {
    let policy_set = sample_policy_set();

    assert_eq!(
        mutation_policy_for_mode(
            Mode::Change,
            &policy_set,
            RiskClass::SystemicImpact,
            UsageZone::Yellow,
        ),
        MutationPolicy::RecommendationOnly
    );
    assert_eq!(
        mutation_policy_for_mode(
            Mode::Change,
            &policy_set,
            RiskClass::BoundedImpact,
            UsageZone::Red,
        ),
        MutationPolicy::RecommendationOnly
    );
    assert_eq!(
        mutation_policy_for_mode(
            Mode::Requirements,
            &policy_set,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        ),
        MutationPolicy::Execute
    );
}
