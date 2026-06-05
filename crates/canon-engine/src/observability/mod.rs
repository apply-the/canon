//! Observability design module.
//! Handles generating telemetry contracts, SLI/SLO alerts, and runbook stubs.

use serde::{Deserialize, Serialize};

pub mod evaluator;
pub mod generators;

/// The type of observability signal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SignalType {
    /// Logging signal
    Log,
    /// Metric signal
    Metric,
    /// Tracing signal
    Trace,
}

/// A specific telemetry signal planned for a boundary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signal {
    /// The type of the signal (Log, Metric, Trace).
    pub signal_type: SignalType,
    /// The name of the signal.
    pub name: String,
    /// Description of what the signal captures.
    pub description: String,
}

/// Maps a specific system boundary to its corresponding signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoundarySignalMap {
    /// The name of the system boundary.
    pub boundary_name: String,
    /// The signals associated with this boundary.
    pub signals: Vec<Signal>,
    /// The failure domain this boundary belongs to.
    pub failure_domain: String,
    /// The expected consumer of these signals.
    pub consumer: String,
}

/// Represents the overall mapping of system boundaries to observability signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelemetryPlan {
    /// List of boundary signal maps.
    pub boundaries: Vec<BoundarySignalMap>,
    /// Global observability constraints.
    pub global_constraints: Vec<String>,
}

/// Represents an actionable Service Level Indicator and its threshold.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SloAlert {
    /// Name of the SLI.
    pub sli_name: String,
    /// The alert threshold (e.g., "> 200ms").
    pub threshold: String,
    /// Where the alert should be routed.
    pub alert_destination: String,
}

/// An actionable playbook for first responders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunbookStub {
    /// The alert that triggers this runbook.
    pub alert_trigger: String,
    /// The steps a responder should take.
    pub action_items: Vec<String>,
    /// The path to escalate if unresolved.
    pub escalation_path: String,
}
