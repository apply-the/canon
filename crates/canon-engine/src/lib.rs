//! Core engine crate for Canon.

/// Artifact rendering and contract types.
pub mod artifacts;
/// Core domain model types.
pub mod domain;
/// Mode-specific execution logic.
pub mod modes;
/// Orchestration layer: classification, gating, execution, and service.
pub mod orchestrator;
/// Persistence layer: manifests, invocation records, and workspace store.
pub mod persistence;
/// Policy definition and evaluation.
pub mod policy;
/// Review domain: findings, critique, and summary types.
pub mod review;

pub use orchestrator::publish::PublishSummary;
pub use orchestrator::service::{
    ActionChip, AiTool, ApprovalSummary, AuthoringLifecycleSummary, ClarificationQuestionSummary,
    ClarityInspectSummary, ClassificationInspectSummary, EngineError, EngineService,
    GateInspectSummary, InitSummary, InspectResponse, InspectTarget, ModeResultSummary,
    PossibleActionSummary, RecommendedActionSummary, ResultActionSummary, RunRequest, RunSummary,
    SkillEntry, SkillsSummary, StatusSummary,
};
