use time::OffsetDateTime;

use crate::domain::execution::{
    ExecutionAdapterDescriptor, InvocationConstraintSet, InvocationPolicyDecision,
    InvocationRequest, PolicyDecisionKind, ToolOutcome, ToolOutcomeKind,
};
use crate::domain::policy::PolicySet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrownfieldMutationScopeStatus {
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

    if request.mode == "brownfield-change"
        && matches!(
            request.capability,
            canon_adapters::CapabilityKind::ExecuteBoundedTransformation
        )
    {
        match classify_brownfield_mutation_scope(&request.requested_scope) {
            BrownfieldMutationScopeStatus::Missing => {
                decision.kind = PolicyDecisionKind::Deny;
                decision.requires_approval = false;
                decision.rationale =
                    "brownfield mutation requires a closed named change surface before execution can be recommended"
                        .to_string();
            }
            BrownfieldMutationScopeStatus::Expanded => {
                decision.kind = PolicyDecisionKind::NeedsApproval;
                decision.requires_approval = true;
                decision.constraints.allowed_paths = request.requested_scope.clone();
                decision.rationale = format!(
                    "brownfield mutation scope broadens beyond a closed change surface and requires explicit approval before it can be recommended: {}",
                    request.requested_scope.join(", ")
                );
            }
            BrownfieldMutationScopeStatus::Bounded => {
                decision.kind = PolicyDecisionKind::AllowConstrained;
                decision.constraints =
                    policy_set.constraint_profile("brownfield-mutation").unwrap_or_default();
                decision.constraints.allowed_paths = request.requested_scope.clone();
                decision.rationale = format!(
                    "brownfield mutation remains recommendation-only within the declared change surface: {}",
                    request.requested_scope.join(", ")
                );
            }
        }
        return decision;
    }

    let approval_required = matches!(
        (request.mode.as_str(), request.capability),
        ("requirements" | "brownfield-change", canon_adapters::CapabilityKind::GenerateContent)
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
        ("brownfield-change", canon_adapters::CapabilityKind::ReadRepository) => {
            Some("brownfield-context")
        }
        ("brownfield-change", canon_adapters::CapabilityKind::GenerateContent) => {
            Some("brownfield-generation")
        }
        ("brownfield-change", canon_adapters::CapabilityKind::ValidateWithTool) => {
            Some("brownfield-validation")
        }
        ("brownfield-change", canon_adapters::CapabilityKind::ExecuteBoundedTransformation) => {
            Some("brownfield-mutation")
        }
        ("pr-review", canon_adapters::CapabilityKind::InspectDiff) => Some("pr-review-diff"),
        ("pr-review", canon_adapters::CapabilityKind::CritiqueContent) => {
            Some("pr-review-critique")
        }
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
        ("brownfield-change", canon_adapters::CapabilityKind::ReadRepository) => {
            "brownfield repository analysis is bounded to the named system slice and current workspace"
        }
        ("brownfield-change", canon_adapters::CapabilityKind::GenerateContent) => {
            "brownfield generation is bounded and must remain traceable to repository context"
        }
        ("brownfield-change", canon_adapters::CapabilityKind::ValidateWithTool) => {
            "brownfield validation must challenge generated change framing through a separate non-generative path"
        }
        ("brownfield-change", canon_adapters::CapabilityKind::ExecuteBoundedTransformation) => {
            "brownfield mutation remains recommendation-only until a later execution tranche"
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

fn classify_brownfield_mutation_scope(scope: &[String]) -> BrownfieldMutationScopeStatus {
    let normalized = scope
        .iter()
        .map(|entry| entry.trim())
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<_>>();

    if normalized.is_empty() {
        return BrownfieldMutationScopeStatus::Missing;
    }

    if normalized.iter().any(|entry| looks_like_expanded_brownfield_scope(entry)) {
        BrownfieldMutationScopeStatus::Expanded
    } else {
        BrownfieldMutationScopeStatus::Bounded
    }
}

fn looks_like_expanded_brownfield_scope(entry: &str) -> bool {
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
                    id: "brownfield-context".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("brownfield-context".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "brownfield-generation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(32 * 1024),
                    command_profile: Some("brownfield-generation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "brownfield-validation".to_string(),
                    payload_retention:
                        crate::domain::execution::PayloadRetentionLevel::SummaryWithDigest,
                    max_payload_bytes: Some(16 * 1024),
                    command_profile: Some("brownfield-validation".to_string()),
                    recommendation_only: false,
                    patch_disabled: true,
                },
                InvocationConstraintProfile {
                    id: "brownfield-mutation".to_string(),
                    payload_retention: crate::domain::execution::PayloadRetentionLevel::SummaryOnly,
                    max_payload_bytes: Some(4 * 1024),
                    command_profile: Some("brownfield-mutation".to_string()),
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
    fn brownfield_mutation_requests_become_recommendation_only() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "brownfield-change",
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
    fn brownfield_mutation_requests_without_a_named_change_surface_are_denied() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "brownfield-change",
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
    fn expanded_brownfield_mutation_scope_requires_approval() {
        let decision = evaluate_request_policy(
            &request_with_scope(
                "brownfield-change",
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
    fn systemic_brownfield_generation_requires_approval() {
        let decision = evaluate_request_policy(
            &request(
                "brownfield-change",
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
