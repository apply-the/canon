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
