use time::OffsetDateTime;

use crate::domain::execution::{
    ExecutionAdapterDescriptor, InvocationConstraintSet, InvocationPolicyDecision,
    InvocationRequest, PolicyDecisionKind, ToolOutcome, ToolOutcomeKind,
};
use crate::domain::policy::PolicySet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChangeMutationScopeStatus {
    Missing,
    Bounded,
    Expanded,
}

pub fn placeholder_descriptor() -> ExecutionAdapterDescriptor {
    ExecutionAdapterDescriptor {
        adapter: canon_adapters::AdapterKind::Filesystem,
        trust_boundary: canon_adapters::TrustBoundaryKind::LocalDeterministic,
        available: true,
        supported_capabilities: vec![canon_adapters::CapabilityKind::ReadRepository],
    }
}

pub fn placeholder_decision(request: &InvocationRequest) -> InvocationPolicyDecision {
    InvocationPolicyDecision {
        kind: PolicyDecisionKind::Allow,
        constraints: InvocationConstraintSet::default(),
        requires_approval: false,
        rationale: format!("placeholder decision for {}", request.request_id),
        policy_refs: vec!["defaults/policies/adapters.toml".to_string()],
        decided_at: OffsetDateTime::now_utc(),
    }
}

pub fn placeholder_outcome(summary: &str) -> ToolOutcome {
    ToolOutcome {
        kind: ToolOutcomeKind::Succeeded,
        summary: summary.to_string(),
        exit_code: Some(0),
        payload_refs: Vec::new(),
        candidate_artifacts: Vec::new(),
        recorded_at: OffsetDateTime::now_utc(),
    }
}

pub fn evaluate_request_policy(
    request: &InvocationRequest,
    policy_set: &PolicySet,
) -> InvocationPolicyDecision {
    let mut decision = placeholder_decision(request);

    if !policy_set.runtime_adapter_enabled(request.adapter) {
        decision.kind = PolicyDecisionKind::Deny;
        decision.rationale = format!(
            "adapter {:?} is modeled in policy but disabled for runtime execution in this tranche",
            request.adapter
        );
        return decision;
    }

    if !policy_set.adapter_supports(request.adapter, request.capability) {
        decision.kind = PolicyDecisionKind::Deny;
        decision.rationale = format!(
            "adapter {:?} does not declare capability {:?}",
            request.adapter, request.capability
        );
        return decision;
    }

    if request.mode == "requirements"
        && matches!(request.capability, canon_adapters::CapabilityKind::ProposeWorkspaceEdit)
    {
        decision.kind = PolicyDecisionKind::Deny;
        decision.rationale =
            "requirements mode may analyze and generate content, but it may not mutate the workspace"
                .to_string();
        decision.requires_approval = false;
        return decision;
    }

    if matches!(request.capability, canon_adapters::CapabilityKind::ExecuteBoundedTransformation)
        && execution_mutation_profile_id(request.mode.as_str()).is_some()
    {
        let profile_id = execution_mutation_profile_id(request.mode.as_str())
            .expect("profile id should exist for execution mutation modes");
        match classify_change_mutation_scope(&request.requested_scope) {
            ChangeMutationScopeStatus::Missing => {
                decision.kind = PolicyDecisionKind::Deny;
                decision.requires_approval = false;
                decision.rationale = missing_execution_scope_rationale(request.mode.as_str());
            }
            ChangeMutationScopeStatus::Expanded => {
                decision.kind = PolicyDecisionKind::NeedsApproval;
                decision.requires_approval = true;
                decision.constraints.allowed_paths = request.requested_scope.clone();
                decision.rationale = expanded_execution_scope_rationale(
                    request.mode.as_str(),
                    &request.requested_scope,
                );
            }
            ChangeMutationScopeStatus::Bounded => {
                decision.kind = PolicyDecisionKind::AllowConstrained;
                decision.constraints = policy_set
                    .constraint_profile_with_allowed_paths(
                        profile_id,
                        request.requested_scope.clone(),
                    )
                    .unwrap_or_default();
                decision.rationale = bounded_execution_scope_rationale(
                    request.mode.as_str(),
                    &request.requested_scope,
                );
            }
        }
        return decision;
    }

    let approval_required = matches!(
        (request.mode.as_str(), request.capability),
        ("requirements" | "change", canon_adapters::CapabilityKind::GenerateContent)
    ) && (!policy_set.allow_mutation(request.risk, request.zone)
        || matches!(request.risk, crate::domain::policy::RiskClass::SystemicImpact));

    if approval_required {
        decision.kind = PolicyDecisionKind::NeedsApproval;
        decision.requires_approval = true;
        decision.rationale =
            "consequential generation requires explicit approval in systemic or mutation-blocked contexts"
                .to_string();
        return decision;
    }

    if let Some(profile_id) = constraint_profile_id(request) {
        decision.kind = PolicyDecisionKind::AllowConstrained;
        decision.constraints = policy_set.constraint_profile(profile_id).unwrap_or_default();
        decision.rationale = constrained_rationale(request).to_string();
    }

    decision
}

fn constraint_profile_id(request: &InvocationRequest) -> Option<&'static str> {
    match (request.mode.as_str(), request.capability) {
        ("requirements", canon_adapters::CapabilityKind::ReadRepository) => {
            Some("requirements-context")
        }
        ("requirements", canon_adapters::CapabilityKind::GenerateContent) => {
            Some("requirements-generation")
        }
        ("requirements", canon_adapters::CapabilityKind::CritiqueContent) => {
            Some("requirements-validation")
        }
        ("change", canon_adapters::CapabilityKind::ReadRepository) => Some("change-context"),
        ("change", canon_adapters::CapabilityKind::GenerateContent) => Some("change-generation"),
        ("change", canon_adapters::CapabilityKind::ValidateWithTool) => Some("change-validation"),
        ("change", canon_adapters::CapabilityKind::ExecuteBoundedTransformation) => {
            Some("change-mutation")
        }
        ("pr-review", canon_adapters::CapabilityKind::InspectDiff) => Some("pr-review-diff"),
        ("pr-review", canon_adapters::CapabilityKind::CritiqueContent) => {
            Some("pr-review-critique")
        }
        _ => None,
    }
}

fn execution_mutation_profile_id(mode: &str) -> Option<&'static str> {
    match mode {
        "change" => Some("change-mutation"),
        "implementation" => Some("implementation-mutation"),
        "refactor" => Some("refactor-mutation"),
        _ => None,
    }
}

fn constrained_rationale(request: &InvocationRequest) -> &'static str {
    match (request.mode.as_str(), request.capability) {
        ("requirements", canon_adapters::CapabilityKind::GenerateContent)
        | ("requirements", canon_adapters::CapabilityKind::CritiqueContent) => {
            "AI-assisted invocation is allowed only with summary-first retention"
        }
        ("requirements", canon_adapters::CapabilityKind::ReadRepository) => {
            "requirements context capture is bounded by repository and input scope constraints"
        }
        ("change", canon_adapters::CapabilityKind::ReadRepository) => {
            "change repository analysis is bounded to the named system slice and current workspace"
        }
        ("change", canon_adapters::CapabilityKind::GenerateContent) => {
            "change generation is bounded and must remain traceable to repository context"
        }
        ("change", canon_adapters::CapabilityKind::ValidateWithTool) => {
            "change validation must challenge generated change framing through a separate non-generative path"
        }
        ("change", canon_adapters::CapabilityKind::ExecuteBoundedTransformation) => {
            "change mutation remains recommendation-only until a later execution tranche"
        }
        ("pr-review", canon_adapters::CapabilityKind::InspectDiff) => {
            "pr-review diff inspection is constrained to summary-first repository evidence"
        }
        ("pr-review", canon_adapters::CapabilityKind::CritiqueContent) => {
            "pr-review critique is allowed only with summary-first retention and preserved review evidence"
        }
        _ => "invocation is constrained by policy",
    }
}

fn missing_execution_scope_rationale(mode: &str) -> String {
    match mode {
        "change" => {
            "change mutation requires a closed named change surface before execution can be recommended"
                .to_string()
        }
        "implementation" => {
            "implementation mutation requires explicit bounded paths before execution can be recommended"
                .to_string()
        }
        "refactor" => {
            "refactor mutation requires an explicit preserved refactor scope before execution can be recommended"
                .to_string()
        }
        _ => "execution mutation requires explicit bounded scope before execution can be recommended"
            .to_string(),
    }
}

fn expanded_execution_scope_rationale(mode: &str, scope: &[String]) -> String {
    match mode {
        "change" => format!(
            "change mutation scope broadens beyond a closed change surface and requires explicit approval before it can be recommended: {}",
            scope.join(", ")
        ),
        "implementation" => format!(
            "implementation mutation scope broadens beyond the declared bounded execution surface and requires explicit approval before it can be recommended: {}",
            scope.join(", ")
        ),
        "refactor" => format!(
            "refactor mutation scope broadens beyond the declared preservation surface and requires explicit approval before it can be recommended: {}",
            scope.join(", ")
        ),
        _ => format!(
            "execution mutation scope broadens beyond the declared bounded surface and requires explicit approval before it can be recommended: {}",
            scope.join(", ")
        ),
    }
}

fn bounded_execution_scope_rationale(mode: &str, scope: &[String]) -> String {
    match mode {
        "change" => format!(
            "change mutation remains recommendation-only within the declared change surface: {}",
            scope.join(", ")
        ),
        "implementation" => format!(
            "implementation mutation remains recommendation-only within the declared mutation bounds: {}",
            scope.join(", ")
        ),
        "refactor" => format!(
            "refactor mutation remains recommendation-only within the declared refactor scope: {}",
            scope.join(", ")
        ),
        _ => format!(
            "execution mutation remains recommendation-only within the declared bounded surface: {}",
            scope.join(", ")
        ),
    }
}

fn classify_change_mutation_scope(scope: &[String]) -> ChangeMutationScopeStatus {
    let normalized = scope
        .iter()
        .map(|entry| entry.trim())
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<_>>();

    if normalized.is_empty() {
        return ChangeMutationScopeStatus::Missing;
    }

    if normalized.iter().any(|entry| looks_like_expanded_change_scope(entry)) {
        ChangeMutationScopeStatus::Expanded
    } else {
        ChangeMutationScopeStatus::Bounded
    }
}

fn looks_like_expanded_change_scope(entry: &str) -> bool {
    let normalized = entry.trim().to_ascii_lowercase();
    normalized == "."
        || normalized == "/"
        || normalized == "*"
        || normalized.contains("entire repo")
        || normalized.contains("whole repo")
        || normalized.contains("repository-wide")
        || normalized.contains("entire repository")
        || normalized.contains("whole repository")
        || normalized.contains("entire workspace")
        || normalized.contains("whole workspace")
        || normalized.contains("workspace-wide")
        || normalized.contains("all files")
        || normalized.contains("all modules")
        || normalized.contains("adjacent modules")
        || normalized.contains("adjacent slices")
        || normalized.contains("cross-cutting")
        || normalized.contains("shared infrastructure")
        || normalized.contains("global")
        || normalized.contains("everything")
}

#[cfg(test)]
mod tests {
    use canon_adapters::{
        AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, MutabilityClass,
        TrustBoundaryKind,
    };
    use time::OffsetDateTime;

    use crate::domain::execution::InvocationRequest;
    use crate::domain::policy::{
        AdapterPolicyMatrix, GatePolicy, InvocationConstraintProfile, PolicySet, RiskClass,
        RiskPolicyClass, UsageZone, ValidationIndependencePolicy, ZonePolicy,
    };
    use crate::domain::verification::VerificationLayer;

    use super::{evaluate_request_policy, placeholder_decision};

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
                    verification_layers: vec![VerificationLayer::SelfCritique],
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
                        CapabilityKind::ReadRepository,
                        CapabilityKind::ReadArtifact,
                        CapabilityKind::EmitArtifact,
                    ],
                },
                AdapterPolicyMatrix {
                    adapter: AdapterKind::CopilotCli,
                    capabilities: vec![
                        CapabilityKind::GenerateContent,
                        CapabilityKind::CritiqueContent,
                        CapabilityKind::ProposeWorkspaceEdit,
                    ],
                },
                AdapterPolicyMatrix {
                    adapter: AdapterKind::Shell,
                    capabilities: vec![
                        CapabilityKind::ValidateWithTool,
                        CapabilityKind::ExecuteBoundedTransformation,
                        CapabilityKind::InspectDiff,
                        CapabilityKind::RunCommand,
                    ],
                },
            ],
            constraint_profiles: vec![
                InvocationConstraintProfile {
                    id: "requirements-context".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(8 * 1024),
                    command_profile: Some("requirements-context".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "requirements-generation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("requirements-generation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "requirements-validation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("requirements-validation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "change-context".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("change-context".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "change-generation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(32 * 1024),
                    command_profile: Some("change-generation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "change-validation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("change-validation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "change-mutation".to_string(),
                    payload_retention: crate::domain::execution::PayloadRetentionLevel::SummaryOnly,
                    max_payload_bytes: Some(4 * 1024),
                    command_profile: Some("change-mutation".to_string()),
                    recommendation_only: true,
                    patch_disabled: false,
                },
                InvocationConstraintProfile {
                    id: "implementation-mutation".to_string(),
                    payload_retention: crate::domain::execution::PayloadRetentionLevel::SummaryOnly,
                    max_payload_bytes: Some(4 * 1024),
                    command_profile: Some("implementation-mutation".to_string()),
                    recommendation_only: true,
                    patch_disabled: false,
                },
                InvocationConstraintProfile {
                    id: "refactor-mutation".to_string(),
                    payload_retention: crate::domain::execution::PayloadRetentionLevel::SummaryOnly,
                    max_payload_bytes: Some(4 * 1024),
                    command_profile: Some("refactor-mutation".to_string()),
                    recommendation_only: true,
                    patch_disabled: false,
                },
                InvocationConstraintProfile {
                    id: "pr-review-diff".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithRetainedPayload,
                    max_payload_bytes: Some(64 * 1024),
                    command_profile: Some("pr-review-diff".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "pr-review-critique".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(32 * 1024),
                    command_profile: Some("pr-review-critique".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
            ],
            runtime_disabled_adapters: vec![AdapterKind::McpStdio],
            validation_independence: ValidationIndependencePolicy {
                ai_generation_requires_distinct_validation: true,
                human_review_counts_independent: true,
            },
            block_mutation_for_red_or_systemic: true,
        }
    }

    fn request_with_scope(
        mode: &str,
        capability: CapabilityKind,
        risk: RiskClass,
        zone: UsageZone,
        requested_scope: Vec<String>,
    ) -> InvocationRequest {
        InvocationRequest {
            request_id: "req-1".to_string(),
            run_id: "run-1".to_string(),
            mode: mode.to_string(),
            system_context: None,
            risk,
            zone,
            adapter: match capability {
                CapabilityKind::GenerateContent
                | CapabilityKind::CritiqueContent
                | CapabilityKind::ProposeWorkspaceEdit => AdapterKind::CopilotCli,
                CapabilityKind::ValidateWithTool
                | CapabilityKind::ExecuteBoundedTransformation
                | CapabilityKind::InspectDiff
                | CapabilityKind::RunCommand => AdapterKind::Shell,
                _ => AdapterKind::Filesystem,
            },
            capability,
            orientation: match capability {
                CapabilityKind::CritiqueContent | CapabilityKind::ValidateWithTool => {
                    InvocationOrientation::Validation
                }
                CapabilityKind::GenerateContent
                | CapabilityKind::ProposeWorkspaceEdit
                | CapabilityKind::ExecuteBoundedTransformation => InvocationOrientation::Generation,
                _ => InvocationOrientation::Context,
            },
            mutability: match capability {
                CapabilityKind::ProposeWorkspaceEdit => MutabilityClass::BroadWorkspaceWrite,
                CapabilityKind::ExecuteBoundedTransformation => {
                    MutabilityClass::BoundedWorkspaceWrite
                }
                _ => MutabilityClass::ReadOnly,
            },
            trust_boundary: if matches!(
                capability,
                CapabilityKind::GenerateContent
                    | CapabilityKind::CritiqueContent
                    | CapabilityKind::ProposeWorkspaceEdit
            ) {
                TrustBoundaryKind::AiReasoning
            } else if matches!(
                capability,
                CapabilityKind::ValidateWithTool
                    | CapabilityKind::ExecuteBoundedTransformation
                    | CapabilityKind::InspectDiff
                    | CapabilityKind::RunCommand
            ) {
                TrustBoundaryKind::LocalProcess
            } else {
                TrustBoundaryKind::LocalDeterministic
            },
            lineage: if matches!(
                capability,
                CapabilityKind::GenerateContent
                    | CapabilityKind::CritiqueContent
                    | CapabilityKind::ProposeWorkspaceEdit
            ) {
                LineageClass::AiVendorFamily
            } else {
                LineageClass::NonGenerative
            },
            requested_scope,
            owner: Some("product-lead".to_string()),
            summary: "governed invocation".to_string(),
            requested_at: OffsetDateTime::now_utc(),
        }
    }

    fn request(
        mode: &str,
        capability: CapabilityKind,
        risk: RiskClass,
        zone: UsageZone,
    ) -> InvocationRequest {
        request_with_scope(mode, capability, risk, zone, vec!["idea.md".to_string()])
    }

    #[test]
    fn placeholder_decision_is_allow_for_scaffolded_request() {
        let request = request(
            "requirements",
            CapabilityKind::ReadRepository,
            RiskClass::BoundedImpact,
            UsageZone::Yellow,
        );
        let decision = placeholder_decision(&request);
        assert!(matches!(decision.kind, crate::domain::execution::PolicyDecisionKind::Allow));
    }

    #[test]
    fn requirements_workspace_edit_requests_are_denied_before_execution() {
        let decision = evaluate_request_policy(
            &request(
                "requirements",
                CapabilityKind::ProposeWorkspaceEdit,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
            ),
            &sample_policy_set(),
        );

        assert!(matches!(decision.kind, crate::domain::execution::PolicyDecisionKind::Deny));
    }

    #[test]
    fn systemic_generation_requests_require_approval_before_execution() {
        let decision = evaluate_request_policy(
            &request(
                "requirements",
                CapabilityKind::GenerateContent,
                RiskClass::SystemicImpact,
                UsageZone::Yellow,
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::NeedsApproval
        ));
        assert!(decision.requires_approval);
    }

    #[test]
    fn change_mutation_requests_become_recommendation_only() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "change",
                CapabilityKind::ExecuteBoundedTransformation,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
                vec!["session repository".to_string(), "auth service".to_string()],
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::AllowConstrained
        ));
        assert!(decision.constraints.recommendation_only);
        assert_eq!(
            decision.constraints.allowed_paths,
            vec!["session repository".to_string(), "auth service".to_string()]
        );
    }

    #[test]
    fn implementation_mutation_requests_become_recommendation_only_with_bounded_paths() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "implementation",
                CapabilityKind::ExecuteBoundedTransformation,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
                vec!["src/auth".to_string(), "tests/auth".to_string()],
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::AllowConstrained
        ));
        assert_eq!(
            decision.constraints.command_profile.as_deref(),
            Some("implementation-mutation")
        );
        assert!(decision.constraints.recommendation_only);
        assert_eq!(
            decision.constraints.allowed_paths,
            vec!["src/auth".to_string(), "tests/auth".to_string()]
        );
    }

    #[test]
    fn refactor_mutation_requests_become_recommendation_only_with_bounded_paths() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "refactor",
                CapabilityKind::ExecuteBoundedTransformation,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
                vec!["src/reviewer".to_string(), "tests/reviewer".to_string()],
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::AllowConstrained
        ));
        assert_eq!(decision.constraints.command_profile.as_deref(), Some("refactor-mutation"));
        assert!(decision.constraints.recommendation_only);
        assert_eq!(
            decision.constraints.allowed_paths,
            vec!["src/reviewer".to_string(), "tests/reviewer".to_string()]
        );
    }

    #[test]
    fn change_mutation_requests_without_a_named_change_surface_are_denied() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "change",
                CapabilityKind::ExecuteBoundedTransformation,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
                Vec::new(),
            ),
            &sample_policy_set(),
        );

        assert!(matches!(decision.kind, crate::domain::execution::PolicyDecisionKind::Deny));
        assert!(!decision.requires_approval);
    }

    #[test]
    fn expanded_change_mutation_scope_requires_approval() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "change",
                CapabilityKind::ExecuteBoundedTransformation,
                RiskClass::BoundedImpact,
                UsageZone::Yellow,
                vec!["auth service".to_string(), "adjacent modules".to_string()],
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::NeedsApproval
        ));
        assert!(decision.requires_approval);
    }

    #[test]
    fn systemic_change_generation_requires_approval() {
        let decision = evaluate_request_policy(
            &request(
                "change",
                CapabilityKind::GenerateContent,
                RiskClass::SystemicImpact,
                UsageZone::Yellow,
            ),
            &sample_policy_set(),
        );

        assert!(matches!(
            decision.kind,
            crate::domain::execution::PolicyDecisionKind::NeedsApproval
        ));
        assert!(decision.requires_approval);
    }
}
