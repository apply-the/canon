/// Risk/zone classification and mutation policy.
pub mod classifier;
/// Evidence bundle construction and independence assessment.
pub mod evidence;
/// Gate evaluation functions for all governed modes.
pub mod gatekeeper;
/// Invocation policy evaluation and adapter decision helpers.
pub mod invocation;
/// Publish routing for Canon run artifacts.
pub mod publish;
/// Run resume logic.
pub mod resume;
/// Engine service: the primary API surface for governing Canon runs.
pub mod service;
/// Verification runner for Canon runs.
pub mod verification_runner;
