use crate::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRequirement, artifact_slug, is_packet_sidecar,
    prefixed_artifact_name,
};
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::run::{ClosureAssessment, ClosureDecompositionScope};
use crate::domain::verification::VerificationLayer;

/// Return the full [`ArtifactContract`] for the given [`Mode`].
///
/// The contract lists every expected body artifact in canonical delivery order,
/// the required Markdown sections each artifact must contain, and the gate kinds
/// that must be satisfied before the artifact is accepted. Sidecar artifacts
/// (`view-manifest.json`, `packet-metadata.json`) are appended last and excluded
/// from ordering and primary-artifact resolution by [`is_packet_sidecar`].
pub fn contract_for_mode(mode: Mode) -> ArtifactContract {
    let mut files = match mode {
        Mode::Requirements => vec![
            requirement(
                "problem-statement.md",
                &["Summary", "Problem", "Outcome"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "constraints.md",
                &["Summary", "Constraints", "Non-Negotiables"],
                &[GateKind::Exploration, GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "options.md",
                &["Summary", "Options", "Recommended Path"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "tradeoffs.md",
                &["Summary", "Tradeoffs", "Consequences"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "scope-cuts.md",
                &["Summary", "Scope Cuts", "Deferred Work"],
                &[GateKind::Exploration, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-checklist.md",
                &["Summary", "Decision Checklist", "Open Questions"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "prd.md",
                &[
                    "Summary",
                    "Problem",
                    "Outcome",
                    "Constraints",
                    "Recommended Path",
                    "Tradeoffs",
                    "Scope Cuts",
                    "Decision Checklist",
                ],
                &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Discovery => vec![
            requirement(
                "problem-map.md",
                &[
                    "Summary",
                    "Problem Domain",
                    "Repo Surface",
                    "Immediate Tensions",
                    "Downstream Handoff",
                ],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "unknowns-and-assumptions.md",
                &["Summary", "Unknowns", "Assumptions", "Validation Targets", "Confidence Levels"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "context-boundary.md",
                &[
                    "Summary",
                    "In-Scope Context",
                    "Repo Surface",
                    "Out-of-Scope Context",
                    "Translation Trigger",
                ],
                &[GateKind::Exploration, GateKind::ReleaseReadiness],
            ),
            requirement(
                "exploration-options.md",
                &["Summary", "Options", "Constraints", "Recommended Direction", "Next-Phase Shape"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "decision-pressure-points.md",
                &[
                    "Summary",
                    "Pressure Points",
                    "Blocking Decisions",
                    "Open Questions",
                    "Recommended Owner",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::SystemShaping => vec![
            requirement(
                "system-shape.md",
                &["Summary", "System Shape", "Boundary Decisions", "Domain Responsibilities"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "domain-model.md",
                &[
                    "Summary",
                    "Candidate Bounded Contexts",
                    "Core And Supporting Domain Hypotheses",
                    "Ubiquitous Language",
                    "Domain Invariants",
                    "Boundary Risks And Open Questions",
                ],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "architecture-outline.md",
                &[
                    "Summary",
                    "Structural Options",
                    "Selected Boundaries",
                    "Rationale",
                    "Why Not The Others",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "capability-map.md",
                &["Summary", "Capabilities", "Dependencies", "Gaps"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "delivery-options.md",
                &["Summary", "Delivery Phases", "Sequencing Rationale", "Risk per Phase"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "risk-hotspots.md",
                &["Summary", "Hotspots", "Mitigation Status", "Unresolved Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Change => vec![
            requirement(
                "system-slice.md",
                &["Summary", "System Slice", "Domain Slice", "Excluded Areas"],
                &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "legacy-invariants.md",
                &["Summary", "Legacy Invariants", "Domain Invariants", "Forbidden Normalization"],
                &[GateKind::ChangePreservation, GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "change-surface.md",
                &["Summary", "Change Surface", "Ownership", "Cross-Context Risks"],
                &[GateKind::ChangePreservation, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "implementation-plan.md",
                &["Summary", "Implementation Plan", "Sequencing"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "validation-strategy.md",
                &["Summary", "Validation Strategy", "Independent Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-record.md",
                &[
                    "Summary",
                    "Decision Record",
                    "Decision Drivers",
                    "Options Considered",
                    "Decision Evidence",
                    "Boundary Tradeoffs",
                    "Recommendation",
                    "Why Not The Others",
                    "Consequences",
                    "Unresolved Questions",
                ],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Backlog => vec![
            requirement(
                "backlog-overview.md",
                &[
                    "Summary",
                    "Scope",
                    "Planning Horizon",
                    "Source Inputs",
                    "Delivery Intent",
                    "Decomposition Posture",
                ],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "epic-tree.md",
                &["Summary", "Epic Tree", "Scope Boundaries", "Source Trace Links"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "capability-to-epic-map.md",
                &["Summary", "Capability Mapping", "Source Trace Links", "Planning Gaps"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "dependency-map.md",
                &["Summary", "Dependencies", "Blocking Edges", "External Dependencies"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "delivery-slices.md",
                &["Summary", "Delivery Slices", "Slice Boundaries", "Dependency Links"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "sequencing-plan.md",
                &["Summary", "Sequencing", "Ordering Rationale", "Readiness Signals"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "acceptance-anchors.md",
                &["Summary", "Acceptance Anchors", "Source Trace Links", "Deferred Detail"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "planning-risks.md",
                &["Summary", "Closure Findings", "Planning Risks", "Follow-Up Triggers"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Incident => vec![
            requirement(
                "incident-frame.md",
                &[
                    "Summary",
                    "Incident Scope",
                    "Trigger And Current State",
                    "Operational Constraints",
                ],
                &[GateKind::Risk, GateKind::IncidentContainment, GateKind::Architecture],
            ),
            requirement(
                "hypothesis-log.md",
                &["Summary", "Known Facts", "Working Hypotheses", "Evidence Gaps"],
                &[GateKind::IncidentContainment, GateKind::Risk],
            ),
            requirement(
                "blast-radius-map.md",
                &["Summary", "Impacted Surfaces", "Propagation Paths", "Confidence And Unknowns"],
                &[GateKind::IncidentContainment, GateKind::Architecture],
            ),
            requirement(
                "containment-plan.md",
                &["Summary", "Immediate Actions", "Ordered Sequence", "Stop Conditions"],
                &[GateKind::IncidentContainment, GateKind::ReleaseReadiness],
            ),
            requirement(
                "incident-decision-record.md",
                &["Summary", "Decision Points", "Approved Actions", "Deferred Actions"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "follow-up-verification.md",
                &["Summary", "Verification Checks", "Release Readiness", "Follow-Up Work"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::SecurityAssessment => vec![
            requirement(
                "assessment-overview.md",
                &[
                    "Summary",
                    "Assessment Scope",
                    "In-Scope Assets",
                    "Trust Boundaries",
                    "Out Of Scope",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "threat-model.md",
                &["Summary", "Threat Inventory", "Attacker Goals", "Boundary Threats"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "risk-register.md",
                &["Summary", "Risk Findings", "Likelihood And Impact", "Proposed Owners"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "mitigations.md",
                &["Summary", "Recommended Controls", "Tradeoffs", "Sequencing Notes"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "assumptions-and-gaps.md",
                &["Summary", "Assumptions", "Evidence Gaps", "Unobservable Surfaces"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "compliance-anchors.md",
                &["Summary", "Applicable Standards", "Control Families", "Scope Limits"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "assessment-evidence.md",
                &["Summary", "Source Inputs", "Independent Checks", "Deferred Verification"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Implementation => vec![
            requirement(
                "task-mapping.md",
                &["Summary", "Task Mapping", "Bounded Changes"],
                &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            requirement(
                "mutation-bounds.md",
                &["Summary", "Mutation Bounds", "Allowed Paths"],
                &[GateKind::Risk, GateKind::ImplementationReadiness],
            ),
            requirement(
                "implementation-notes.md",
                &[
                    "Summary",
                    "Executed Changes",
                    "Candidate Frameworks",
                    "Options Matrix",
                    "Decision Evidence",
                    "Recommendation",
                    "Task Linkage",
                ],
                &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            requirement(
                "completion-evidence.md",
                &["Summary", "Completion Evidence", "Adoption Implications", "Remaining Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "validation-hooks.md",
                &["Summary", "Ecosystem Health", "Safety-Net Evidence", "Independent Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "rollback-notes.md",
                &["Summary", "Rollback Triggers", "Rollback Steps"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Migration => vec![
            requirement(
                "source-target-map.md",
                &["Summary", "Current State", "Target State", "Transition Boundaries"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "compatibility-matrix.md",
                &[
                    "Summary",
                    "Guaranteed Compatibility",
                    "Temporary Incompatibilities",
                    "Coexistence Rules",
                    "Options Matrix",
                ],
                &[GateKind::Architecture, GateKind::MigrationSafety],
            ),
            requirement(
                "sequencing-plan.md",
                &["Summary", "Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
                &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
            ),
            requirement(
                "fallback-plan.md",
                &[
                    "Summary",
                    "Rollback Triggers",
                    "Fallback Paths",
                    "Re-Entry Criteria",
                    "Adoption Implications",
                ],
                &[GateKind::MigrationSafety, GateKind::Risk],
            ),
            requirement(
                "migration-verification-report.md",
                &["Summary", "Verification Checks", "Residual Risks", "Release Readiness"],
                &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-record.md",
                &[
                    "Summary",
                    "Migration Decisions",
                    "Tradeoff Analysis",
                    "Decision Evidence",
                    "Recommendation",
                    "Why Not The Others",
                    "Ecosystem Health",
                    "Deferred Decisions",
                    "Approval Notes",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
        ],
        Mode::SupplyChainAnalysis => vec![
            requirement(
                "analysis-overview.md",
                &[
                    "Summary",
                    "Declared Scope",
                    "Licensing Posture",
                    "Distribution Model",
                    "Ecosystems In Scope",
                    "Out Of Scope Components",
                ],
                &[GateKind::Risk],
            ),
            requirement(
                "sbom-bundle.md",
                &[
                    "Summary",
                    "Scanner Selection Rationale",
                    "SBOM Outputs",
                    "Scanner Decisions",
                    "Coverage Gaps",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "vulnerability-triage.md",
                &["Summary", "Findings By Severity", "Exploitability Notes", "Triage Decisions"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "license-compliance.md",
                &["Summary", "Compatibility Classes", "Flagged Incompatibilities", "Obligations"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "legacy-posture.md",
                &[
                    "Summary",
                    "Outdated Dependencies",
                    "End Of Life Signals",
                    "Abandonment Signals",
                    "Modernization Slices",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "policy-decisions.md",
                &["Summary", "Scanner Decisions", "Coverage Gaps", "Deferred Verification"],
                &[GateKind::ReleaseReadiness],
            ),
            requirement(
                "analysis-evidence.md",
                &["Summary", "Source Inputs", "Independent Checks", "Deferred Verification"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Refactor => vec![
            requirement(
                "preserved-behavior.md",
                &["Summary", "Preserved Behavior", "Approved Exceptions"],
                &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
            requirement(
                "refactor-scope.md",
                &["Summary", "Refactor Scope", "Allowed Paths"],
                &[GateKind::ChangePreservation, GateKind::Risk],
            ),
            requirement(
                "structural-rationale.md",
                &["Summary", "Structural Rationale", "Untouched Surface"],
                &[GateKind::Exploration, GateKind::ChangePreservation],
            ),
            requirement(
                "regression-evidence.md",
                &["Summary", "Safety-Net Evidence", "Regression Findings"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "contract-drift-check.md",
                &["Summary", "Contract Drift", "Reviewer Notes"],
                &[GateKind::Architecture, GateKind::ChangePreservation],
            ),
            requirement(
                "no-feature-addition.md",
                &["Summary", "Feature Audit", "Decision"],
                &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Architecture => vec![
            requirement(
                "architecture-overview.md",
                &[
                    "Summary",
                    "Primary Decision",
                    "Key Constraints",
                    "Included Views",
                    "Omitted Views",
                    "Review Guidance",
                ],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "architecture-decisions.md",
                &[
                    "Summary",
                    "Decision",
                    "Constraints",
                    "Decision Drivers",
                    "Recommendation",
                    "Consequences",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "invariants.md",
                &["Summary", "Invariants", "Rationale", "Verification Hooks"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "tradeoff-matrix.md",
                &[
                    "Summary",
                    "Options Considered",
                    "Evaluation Criteria",
                    "Pros",
                    "Cons",
                    "Why Not The Others",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "boundary-map.md",
                &["Summary", "Boundaries", "Ownership", "Crossing Rules"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "context-map.md",
                &[
                    "Summary",
                    "Bounded Contexts",
                    "Context Relationships",
                    "Integration Seams",
                    "Anti-Corruption Candidates",
                    "Ownership Boundaries",
                    "Shared Invariants",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "readiness-assessment.md",
                &[
                    "Summary",
                    "Readiness Status",
                    "Working Assumptions",
                    "Unresolved Questions",
                    "Blockers",
                    "Accepted Risks",
                    "Recommended Next Mode",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "system-context.md",
                &["System Context"],
                &[GateKind::Architecture, GateKind::Exploration],
            ),
            requirement_with_format(
                "system-context.mmd",
                ArtifactFormat::Markdown,
                &[],
                &[GateKind::Architecture, GateKind::Exploration],
            ),
            requirement("container-view.md", &["Containers"], &[GateKind::Architecture]),
            requirement_with_format(
                "container-view.mmd",
                ArtifactFormat::Markdown,
                &[],
                &[GateKind::Architecture],
            ),
            requirement(
                "deployment-view.md",
                &["Deployment"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement_with_format(
                "deployment-view.mmd",
                ArtifactFormat::Markdown,
                &[],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            optional_requirement(
                "component-view.md",
                &["Components"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            optional_requirement_with_format(
                "component-view.mmd",
                ArtifactFormat::Markdown,
                &[],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            optional_requirement(
                "dynamic-view.md",
                &["Dynamic View"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            optional_requirement_with_format(
                "dynamic-view.mmd",
                ArtifactFormat::Markdown,
                &[],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement_with_format(
                "view-manifest.json",
                ArtifactFormat::Json,
                &[],
                &[GateKind::ReleaseReadiness],
            ),
            requirement_with_format(
                "packet-metadata.json",
                ArtifactFormat::Json,
                &[],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::SystemAssessment => vec![
            requirement(
                "assessment-overview.md",
                &[
                    "Summary",
                    "Assessment Objective",
                    "Stakeholders",
                    "Primary Concerns",
                    "Assessment Posture",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "coverage-map.md",
                &[
                    "Summary",
                    "Stakeholder Concerns",
                    "Assessed Views",
                    "Partial Or Skipped Coverage",
                    "Confidence By Surface",
                ],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "asset-inventory.md",
                &[
                    "Summary",
                    "Assessed Assets",
                    "Critical Dependencies",
                    "Boundary Notes",
                    "Ownership Signals",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "functional-view.md",
                &[
                    "Summary",
                    "Responsibilities",
                    "Primary Flows",
                    "Observed Boundaries",
                    "Confidence Notes",
                ],
                &[GateKind::Architecture],
            ),
            requirement(
                "component-view.md",
                &["Summary", "Components", "Responsibilities", "Interfaces", "Confidence Notes"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "deployment-view.md",
                &[
                    "Summary",
                    "Execution Environments",
                    "Network And Runtime Boundaries",
                    "Deployment Signals",
                    "Coverage Gaps",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "technology-view.md",
                &[
                    "Summary",
                    "Technology Stack",
                    "Platform Dependencies",
                    "Version Or Lifecycle Signals",
                    "Evidence Gaps",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "integration-view.md",
                &[
                    "Summary",
                    "Integrations",
                    "Data Exchanges",
                    "Trust And Failure Boundaries",
                    "Inference Notes",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "risk-register.md",
                &[
                    "Summary",
                    "Observed Risks",
                    "Risk Triggers",
                    "Impact Notes",
                    "Likely Follow-On Modes",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "assessment-evidence.md",
                &[
                    "Summary",
                    "Observed Findings",
                    "Inferred Findings",
                    "Assessment Gaps",
                    "Evidence Sources",
                ],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Review => vec![
            requirement(
                "review-brief.md",
                &["Summary", "Review Target", "Evidence Basis"],
                &[GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "boundary-assessment.md",
                &["Summary", "Boundary Findings", "Ownership Notes"],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "missing-evidence.md",
                &["Summary", "Missing Evidence", "Collection Priorities"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-impact.md",
                &["Summary", "Decision Impact", "Reversibility Concerns"],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "review-disposition.md",
                &["Summary", "Final Disposition", "Accepted Risks"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Verification => vec![
            requirement(
                "invariants-checklist.md",
                &["Summary", "Claims Under Test", "Invariant Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "contract-matrix.md",
                &["Summary", "Contract Assumptions", "Verification Outcome"],
                &[GateKind::ReleaseReadiness],
            ),
            requirement(
                "adversarial-review.md",
                &["Summary", "Challenge Findings", "Contradictions"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "verification-report.md",
                &["Summary", "Verified Claims", "Rejected Claims", "Overall Verdict"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "unresolved-findings.md",
                &["Summary", "Open Findings", "Required Follow-Up"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::PrReview => vec![
            requirement(
                "pr-analysis.md",
                &[
                    "Summary",
                    "Scope Summary",
                    "Changed Modules",
                    "Inferred Intent",
                    "Surprising Surface Area",
                ],
                &[GateKind::Risk, GateKind::ReviewDisposition],
            ),
            requirement(
                "boundary-check.md",
                &[
                    "Summary",
                    "Boundary Findings",
                    "Ownership Breaks",
                    "Unauthorized Structural Impact",
                ],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "conventional-comments.md",
                &["Summary", "Evidence Posture", "Conventional Comments", "Traceability"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "duplication-check.md",
                &["Summary", "Duplicate Behavior", "Canonical Owner Conflicts"],
                &[GateKind::ReviewDisposition],
            ),
            requirement(
                "contract-drift.md",
                &["Summary", "Interface Drift", "Compatibility Concerns"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "missing-tests.md",
                &[
                    "Summary",
                    "Missing Invariant Checks",
                    "Missing Contract Checks",
                    "Weak or Mirrored Tests",
                ],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-impact.md",
                &[
                    "Summary",
                    "Implied Decisions",
                    "Absent Decision Records",
                    "Reversibility Concerns",
                ],
                &[GateKind::Risk, GateKind::ReviewDisposition],
            ),
            requirement(
                "review-summary.md",
                &[
                    "Summary",
                    "Severity",
                    "Must-Fix Findings",
                    "Accepted Risks",
                    "Final Disposition",
                ],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::DomainLanguage => vec![
            requirement(
                "language-overview.md",
                &[
                    "Summary",
                    "Domain Scope",
                    "Language Maturity",
                    "Upstream Sources",
                    "Downstream Consumers",
                ],
                &[GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "domain-glossary.md",
                &["Summary", "Glossary Entries", "Source References", "Open Gaps"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "preferred-language.md",
                &["Summary", "Canonical Terms", "Deprecated Synonyms", "Migration Notes"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "language-conflicts.md",
                &["Summary", "Conflict Inventory", "Resolution Status", "Escalation Triggers"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "contextual-meanings.md",
                &["Summary", "Context-Dependent Terms", "Disambiguation Rules", "Usage Examples"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "business-language-rules.md",
                &["Summary", "Naming Conventions", "Domain Boundaries", "Enforcement Guidance"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "code-and-api-vocabulary.md",
                &["Summary", "Code Naming Patterns", "API Surface Terms", "Alignment Gaps"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "downstream-language-guidance.md",
                &["Summary", "Consumer Modes", "Handoff Expectations", "Adoption Risks"],
                &[GateKind::ReleaseReadiness],
            ),
            requirement(
                "language-decision-record.md",
                &[
                    "Summary",
                    "Decision Drivers",
                    "Options Considered",
                    "Decision Evidence",
                    "Recommendation",
                    "Consequences",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "ai-provenance.md",
                &["Summary", "Generation Lineage", "Human Authored Sections", "Confidence Posture"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::DomainModel => vec![
            requirement(
                "model-overview.md",
                &[
                    "Summary",
                    "Domain Scope",
                    "Model Maturity",
                    "Upstream Sources",
                    "Downstream Consumers",
                ],
                &[GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "concept-catalog.md",
                &["Summary", "Concepts", "Ownership Boundaries", "Open Gaps"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "relationship-map.md",
                &["Summary", "Relationships", "Cardinality Rules", "Boundary Crossings"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "bounded-context-map.md",
                &["Summary", "Bounded Contexts", "Context Relationships", "Integration Seams"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "lifecycle-and-state-model.md",
                &["Summary", "Entity Lifecycles", "State Transitions", "Invariant Guards"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "domain-invariants.md",
                &["Summary", "Invariants", "Enforcement Points", "Violation Consequences"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "policy-and-constraint-rules.md",
                &["Summary", "Business Policies", "Constraint Rules", "Exception Handling"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "feature-impact-rules.md",
                &["Summary", "Impact Rules", "Affected Concepts", "Downstream Effects"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "code-data-alignment.md",
                &["Summary", "Code Mapping", "Data Store Mapping", "Alignment Gaps"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "model-gaps-and-risks.md",
                &["Summary", "Model Gaps", "Risk Signals", "Recommended Follow-Ups"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "downstream-model-guidance.md",
                &["Summary", "Consumer Modes", "Handoff Expectations", "Adoption Risks"],
                &[GateKind::ReleaseReadiness],
            ),
            requirement_with_format(
                "domain-model.json",
                ArtifactFormat::Json,
                &[],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "ai-provenance.md",
                &["Summary", "Generation Lineage", "Human Authored Sections", "Confidence Posture"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
    };

    if !files
        .iter()
        .any(|requirement| artifact_slug(&requirement.file_name) == "packet-metadata.json")
    {
        files.push(requirement_with_format(
            "packet-metadata.json",
            ArtifactFormat::Json,
            &[],
            &[GateKind::ReleaseReadiness],
        ));
    }

    let mut reader_facing_index = 0;
    let prefixed_files = files
        .into_iter()
        .map(|mut req| {
            let bare_name = artifact_slug(&req.file_name).to_string();
            req.file_name = if is_packet_sidecar(&bare_name) {
                bare_name
            } else {
                reader_facing_index += 1;
                prefixed_artifact_name(reader_facing_index, &bare_name)
            };
            req
        })
        .collect();

    ArtifactContract {
        version: 1,
        artifact_requirements: prefixed_files,
        required_verification_layers: vec![VerificationLayer::SelfCritique],
    }
}

pub fn backlog_contract_for_closure(
    contract: &ArtifactContract,
    closure_assessment: &ClosureAssessment,
) -> ArtifactContract {
    if matches!(closure_assessment.decomposition_scope, ClosureDecompositionScope::RiskOnlyPacket) {
        let mut filtered = contract.clone();
        filtered.artifact_requirements.retain(|requirement| {
            matches!(
                crate::domain::artifact::artifact_slug(&requirement.file_name),
                "backlog-overview.md" | "planning-risks.md"
            ) || is_packet_sidecar(&requirement.file_name)
        });
        filtered
    } else {
        contract.clone()
    }
}

pub fn architecture_contract_for_context(
    contract: &ArtifactContract,
    context_summary: &str,
) -> ArtifactContract {
    let mut filtered = contract.clone();
    filtered.artifact_requirements.retain(|requirement| {
        requirement.required
            || crate::artifacts::markdown::architecture_artifact_enabled(
                &requirement.file_name,
                context_summary,
            )
    });

    let mut reader_facing_index = 0;
    for requirement in &mut filtered.artifact_requirements {
        let bare_name = artifact_slug(&requirement.file_name).to_string();
        requirement.file_name = if is_packet_sidecar(&bare_name) {
            bare_name
        } else {
            reader_facing_index += 1;
            prefixed_artifact_name(reader_facing_index, &bare_name)
        };
    }

    filtered
}

pub fn validate_artifact(requirement: &ArtifactRequirement, contents: &str) -> Vec<String> {
    let mut blockers = Vec::new();

    for section in &requirement.required_sections {
        if !contains_required_heading(contents, section) {
            blockers
                .push(format!("{} is missing required section `{section}`", requirement.file_name));
        }
    }

    blockers
}

fn contains_required_heading(contents: &str, section: &str) -> bool {
    let expected = format!("## {section}");
    contents.lines().any(|line| line.trim() == expected)
}

pub fn validate_release_bundle(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
) -> Vec<String> {
    let mut blockers = Vec::new();

    for requirement in &contract.artifact_requirements {
        match artifacts.iter().find(|(file_name, _)| file_name == &requirement.file_name) {
            Some((_, contents)) => blockers.extend(validate_artifact(requirement, contents)),
            None if requirement.required => {
                blockers.push(format!("missing required artifact `{}`", requirement.file_name))
            }
            None => {}
        }
    }

    blockers
}

fn requirement(
    file_name: &str,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    requirement_with_format(file_name, ArtifactFormat::Markdown, required_sections, gates)
}

fn requirement_with_format(
    file_name: &str,
    format: ArtifactFormat,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format,
        required_sections: required_sections.iter().map(ToString::to_string).collect(),
        gates: gates.to_vec(),
        required: true,
    }
}

fn optional_requirement(
    file_name: &str,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    optional_requirement_with_format(file_name, ArtifactFormat::Markdown, required_sections, gates)
}

fn optional_requirement_with_format(
    file_name: &str,
    format: ArtifactFormat,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format,
        required_sections: required_sections.iter().map(ToString::to_string).collect(),
        gates: gates.to_vec(),
        required: false,
    }
}

#[cfg(test)]
mod tests {
    use super::{architecture_contract_for_context, contract_for_mode};
    use crate::domain::mode::Mode;

    #[test]
    fn architecture_contract_for_context_excludes_unmentioned_optional_views() {
        let contract = contract_for_mode(Mode::Architecture);

        let filtered = architecture_contract_for_context(
            &contract,
            "# Architecture Brief\n\nDecision focus: bounded analytics CLI.\nConstraint: preserve Canon runtime contracts.\n",
        );

        let slugs = filtered
            .artifact_requirements
            .iter()
            .map(|requirement| requirement.slug())
            .collect::<Vec<_>>();

        assert!(!slugs.contains(&"component-view.md"));
        assert!(!slugs.contains(&"component-view.mmd"));
        assert!(!slugs.contains(&"dynamic-view.md"));
        assert!(!slugs.contains(&"dynamic-view.mmd"));
        assert_eq!(slugs.last(), Some(&"packet-metadata.json"));
        assert!(
            filtered
                .artifact_requirements
                .iter()
                .any(|requirement| requirement.file_name == "view-manifest.json")
        );
        assert!(
            filtered
                .artifact_requirements
                .iter()
                .any(|requirement| requirement.file_name == "packet-metadata.json")
        );
    }
}
