use canon_adapters::{AdapterInvocation, AdapterKind, CapabilityKind};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::execution::{PolicyDecisionKind, ToolOutcomeKind};

/// A discrete event in the adapter invocation audit trail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceEventKind {
    /// A new invocation request was persisted.
    RequestPersisted,
    /// A policy decision was persisted for an invocation request.
    DecisionPersisted,
    /// Approval is required before the invocation may proceed.
    ApprovalRequired,
    /// An approval record was persisted for an invocation request.
    ApprovalRecorded,
    /// The adapter dispatch for an invocation was started.
    DispatchStarted,
    /// A tool outcome was recorded for an invocation attempt.
    OutcomeRecorded,
    /// The evidence bundle was updated after an invocation.
    EvidenceBundleUpdated,
    /// The adapter dispatch was denied at runtime by the policy.
    RuntimeDenied,
}

/// A single audit trace event for an adapter invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceEvent {
    /// The run ID associated with this event.
    pub run_id: String,
    /// The invocation request ID, if applicable.
    pub request_id: Option<String>,
    /// The adapter that executed the invocation.
    pub adapter: Option<AdapterKind>,
    /// The capability exercised by the invocation.
    pub capability: Option<CapabilityKind>,
    /// The kind of event.
    pub event: TraceEventKind,
    /// Human-readable summary of the event.
    pub summary: String,
    /// The policy decision applied, if applicable.
    pub policy_decision: Option<PolicyDecisionKind>,
    /// The tool outcome kind, if applicable.
    pub outcome: Option<ToolOutcomeKind>,
    /// When this event was recorded.
    pub recorded_at: OffsetDateTime,
}

impl TraceEvent {
    /// Constructs a trace event from an adapter invocation record.
    pub fn from_adapter_invocation(run_id: &str, invocation: &AdapterInvocation) -> Self {
        Self {
            run_id: run_id.to_string(),
            request_id: None,
            adapter: Some(invocation.adapter),
            capability: Some(invocation.capability),
            event: TraceEventKind::OutcomeRecorded,
            summary: invocation.purpose.clone(),
            policy_decision: if invocation.allowed {
                Some(PolicyDecisionKind::Allow)
            } else {
                Some(PolicyDecisionKind::Deny)
            },
            outcome: Some(if invocation.allowed {
                ToolOutcomeKind::Succeeded
            } else {
                ToolOutcomeKind::Denied
            }),
            recorded_at: invocation.occurred_at,
        }
    }
}

/// Parses newline-delimited JSON trace events from the given string.
pub fn parse_trace_events(contents: &str) -> Result<Vec<TraceEvent>, serde_json::Error> {
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(serde_json::from_str::<TraceEvent>)
        .collect()
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::{TraceEvent, TraceEventKind, parse_trace_events};
    use crate::domain::execution::{PolicyDecisionKind, ToolOutcomeKind};
    use canon_adapters::{AdapterInvocation, AdapterKind, CapabilityKind, SideEffectClass};

    #[test]
    fn from_adapter_invocation_maps_allowed_and_denied_outcomes() {
        let allowed = TraceEvent::from_adapter_invocation(
            "run-1",
            &AdapterInvocation {
                adapter: AdapterKind::Filesystem,
                capability: CapabilityKind::ReadRepository,
                purpose: "capture context".to_string(),
                side_effect: SideEffectClass::ReadOnly,
                allowed: true,
                occurred_at: OffsetDateTime::UNIX_EPOCH,
            },
        );
        assert_eq!(allowed.event, TraceEventKind::OutcomeRecorded);
        assert_eq!(allowed.policy_decision, Some(PolicyDecisionKind::Allow));
        assert_eq!(allowed.outcome, Some(ToolOutcomeKind::Succeeded));

        let denied = TraceEvent::from_adapter_invocation(
            "run-1",
            &AdapterInvocation {
                adapter: AdapterKind::Shell,
                capability: CapabilityKind::RunCommand,
                purpose: "blocked mutation".to_string(),
                side_effect: SideEffectClass::WorkspaceMutation,
                allowed: false,
                occurred_at: OffsetDateTime::UNIX_EPOCH,
            },
        );
        assert_eq!(denied.policy_decision, Some(PolicyDecisionKind::Deny));
        assert_eq!(denied.outcome, Some(ToolOutcomeKind::Denied));
    }

    #[test]
    fn parse_trace_events_ignores_blank_lines() {
        let contents = format!(
            "{}\n\n{}\n",
            serde_json::to_string(&TraceEvent {
                run_id: "run-1".to_string(),
                request_id: Some("req-1".to_string()),
                adapter: Some(AdapterKind::Filesystem),
                capability: Some(CapabilityKind::ReadRepository),
                event: TraceEventKind::RequestPersisted,
                summary: "stored request".to_string(),
                policy_decision: None,
                outcome: None,
                recorded_at: OffsetDateTime::UNIX_EPOCH,
            })
            .expect("serialize event"),
            serde_json::to_string(&TraceEvent {
                run_id: "run-1".to_string(),
                request_id: Some("req-2".to_string()),
                adapter: Some(AdapterKind::Shell),
                capability: Some(CapabilityKind::RunCommand),
                event: TraceEventKind::OutcomeRecorded,
                summary: "stored outcome".to_string(),
                policy_decision: Some(PolicyDecisionKind::Allow),
                outcome: Some(ToolOutcomeKind::Succeeded),
                recorded_at: OffsetDateTime::UNIX_EPOCH,
            })
            .expect("serialize event")
        );

        let events = parse_trace_events(&contents).expect("trace parsing should succeed");

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].request_id.as_deref(), Some("req-1"));
        assert_eq!(events[1].request_id.as_deref(), Some("req-2"));
    }
}
