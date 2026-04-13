use canon_adapters::{AdapterInvocation, AdapterKind, CapabilityKind};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::execution::{PolicyDecisionKind, ToolOutcomeKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceEventKind {
    RequestPersisted,
    DecisionPersisted,
    ApprovalRequired,
    ApprovalRecorded,
    DispatchStarted,
    OutcomeRecorded,
    EvidenceBundleUpdated,
    RuntimeDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceEvent {
    pub run_id: String,
    pub request_id: Option<String>,
    pub adapter: Option<AdapterKind>,
    pub capability: Option<CapabilityKind>,
    pub event: TraceEventKind,
    pub summary: String,
    pub policy_decision: Option<PolicyDecisionKind>,
    pub outcome: Option<ToolOutcomeKind>,
    pub recorded_at: OffsetDateTime,
}

impl TraceEvent {
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
