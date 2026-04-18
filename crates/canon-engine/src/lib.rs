//! Core engine crate for Canon.

pub mod artifacts;
pub mod domain;
pub mod modes;
pub mod orchestrator;
pub mod persistence;
pub mod review;

pub use orchestrator::service::{
    AiTool, ApprovalSummary, ClarificationQuestionSummary, ClarityInspectSummary,
    ClassificationInspectSummary, EngineError, EngineService, GateInspectSummary, InitSummary,
    InspectResponse, InspectTarget, ModeResultSummary, RecommendedActionSummary,
    ResultActionSummary, RunRequest, RunSummary, SkillEntry, SkillsSummary, StatusSummary,
};
