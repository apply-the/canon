/// Atomic file-write helpers.
pub mod atomic;
/// Invocation persistence: persisted requests, decisions, and attempts.
pub mod invocations;
/// Workspace directory layout helpers.
pub mod layout;
/// Run ID lookup and resolution.
pub mod lookup;
/// Run manifest persistence: run, state, context, and artifact manifests.
pub mod manifests;
/// Run ID slug normalization.
pub mod slug;
/// Workspace store: the primary persistence API for Canon runs.
pub mod store;
/// Trace event persistence for adapter invocation audit logs.
pub mod traces;
