//! Adapter crate for Canon.

pub mod capability;
pub mod copilot_cli;
pub mod dispatcher;
pub mod filesystem;
pub mod mcp_stdio;
pub mod shell;

pub use capability::{
    AdapterInvocation, AdapterKind, AdapterRequest, CapabilityKind, SideEffectClass,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("mutating adapter capability is blocked by current policy")]
    MutationBlocked,
    #[error("filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("process error: {0}")]
    Process(std::io::Error),
}
