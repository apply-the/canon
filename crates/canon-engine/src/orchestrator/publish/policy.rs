use super::*;

pub(super) fn evaluate_promotion_policy(
    mode: Mode,
    run_state: &RunState,
    _manifest: &RunManifest,
) -> PromotionState {
    match (mode, run_state) {
        (
            Mode::SystemShaping
            | Mode::Discovery
            | Mode::Brainstorming
            | Mode::Requirements
            | Mode::DomainLanguage
            | Mode::DomainModel
            | Mode::Backlog,
            RunState::Completed,
        ) => PromotionState::Auto,
        (
            Mode::Architecture
            | Mode::Change
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration,
            RunState::Completed,
        ) => PromotionState::AutoIfApproved,
        (
            Mode::Architecture
            | Mode::Change
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration,
            _,
        ) => PromotionState::PendingIndex,
        (
            Mode::Verification
            | Mode::Review
            | Mode::PrReview
            | Mode::SecurityAssessment
            | Mode::SupplyChainAnalysis
            | Mode::SystemAssessment,
            _,
        ) => PromotionState::EvidenceOnly,
        (Mode::Incident, RunState::Completed) => PromotionState::PendingIndex,
        (Mode::Incident, _) => PromotionState::Manual,
        (_, _) => PromotionState::PendingIndex,
    }
}

pub(super) fn default_update_strategy_for(mode: Mode) -> UpdateStrategy {
    match mode {
        Mode::SystemShaping
        | Mode::Architecture
        | Mode::Requirements
        | Mode::Discovery
        | Mode::Brainstorming
        | Mode::Change
        | Mode::Implementation
        | Mode::Refactor
        | Mode::Debugging
        | Mode::DomainLanguage
        | Mode::DomainModel => UpdateStrategy::ManagedBlocks,
        Mode::Incident | Mode::Migration => UpdateStrategy::ProposalFiles,
        Mode::Verification
        | Mode::Review
        | Mode::PrReview
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment
        | Mode::Backlog => UpdateStrategy::AppendOnlyIndex,
    }
}

pub(super) fn resolve_profile_destination(
    repo_root: &Path,
    manifest: &RunManifest,
    promotion: &PromotionState,
) -> PathBuf {
    repo_root.join(canonical_project_memory_surface(manifest.mode, *promotion))
}

pub(super) fn canonical_project_memory_surface(
    mode: Mode,
    promotion: PromotionState,
) -> &'static str {
    if promotion.targets_stable_surface() {
        stable_project_memory_surface(mode)
    } else if promotion.targets_pending_surface() {
        pending_project_memory_surface(mode)
    } else {
        evidence_project_memory_surface(mode)
    }
}

pub(super) fn stable_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Discovery | Mode::Brainstorming => "tech-docs/project/overview.md",
        Mode::Requirements => "tech-docs/project/product-context.md",
        Mode::SystemShaping | Mode::Architecture => "tech-docs/project/architecture-map.md",
        Mode::Change | Mode::Migration => "tech-docs/project/decision-index.md",
        Mode::Backlog | Mode::Implementation | Mode::Refactor | Mode::Debugging => {
            "tech-docs/project/delivery-map.md"
        }
        Mode::DomainLanguage => "tech-docs/project/domain-language.md",
        Mode::DomainModel => "tech-docs/project/domain-model.md",
        Mode::Verification
        | Mode::Review
        | Mode::PrReview
        | Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment => "tech-docs/project/operational-context.md",
    }
}

pub(super) fn pending_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Incident => "tech-docs/project/open-risks.md",
        Mode::Verification | Mode::Review | Mode::PrReview => "tech-docs/project/audit-log.md",
        _ => "tech-docs/project/pending-decisions.md",
    }
}

pub(super) fn evidence_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Review | Mode::PrReview | Mode::Verification => "tech-docs/project/audit-log.md",
        Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment => "tech-docs/project/open-risks.md",
        _ => "tech-docs/project/audit-log.md",
    }
}
