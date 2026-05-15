//! Adapter crate for Canon.
//!
//! Provides the concrete adapter implementations (filesystem, shell, Copilot CLI,
//! MCP stdio) and the shared capability classification types used by the engine
//! to enforce governance policy on all tool invocations.

/// Filesystem adapter: reads and writes local files.
pub mod capability;
/// Copilot CLI adapter: communicates with the GitHub Copilot CLI.
pub mod copilot_cli;
/// Dispatch helpers: mutation-policy enforcement at the adapter boundary.
pub mod dispatcher;
/// Filesystem adapter implementation.
pub mod filesystem;
/// MCP stdio adapter: invokes structured tools via the Model Context Protocol.
pub mod mcp_stdio;
/// Shell adapter: executes arbitrary local shell commands.
pub mod shell;

pub use capability::{
    AdapterCapability, AdapterInvocation, AdapterKind, AdapterRequest, CapabilityKind,
    InvocationOrientation, LineageClass, MutabilityClass, SideEffectClass, TrustBoundaryKind,
    classify_capability,
};

use thiserror::Error;

/// Errors produced by adapter invocations.
#[derive(Debug, Error)]
pub enum AdapterError {
    /// An adapter invocation was blocked because the current policy does not
    /// permit mutation.
    #[error("mutating adapter capability is blocked by current policy")]
    MutationBlocked,
    /// An underlying filesystem I/O error occurred.
    #[error("filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    /// An error occurred while spawning or waiting for a child process.
    #[error("process error: {0}")]
    Process(std::io::Error),
}
