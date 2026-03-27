//! Core engine crate for Canon.

pub mod artifacts;
pub mod domain;
pub mod modes;
pub mod orchestrator;
pub mod persistence;
pub mod review;

pub use orchestrator::service::{
    ApprovalSummary, EngineError, EngineService, InitSummary, InspectResponse, InspectTarget,
    RunRequest, RunSummary, StatusSummary,
};
