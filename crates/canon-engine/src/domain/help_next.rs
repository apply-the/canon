//! Canon help-next domain types.
//!
//! This module defines the state model, diagnostic collector, and
//! recommendation builder used by the `canon help-next` command.
//! All diagnostics are deterministic inspections of typed Canon state.

use serde::{Deserialize, Serialize};

/// Detectable Canon governance states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonHelpNextState {
    /// No `.canon/` directory exists — Canon not initialized.
    NotInitialized,
    /// `.canon/` exists but no active run.
    NoActiveRun,
    /// Active run with no packet yet.
    NoPacket,
    /// Packet exists but required documents are missing.
    IncompleteDocuments,
    /// All documents present but no evidence recorded.
    PendingEvidence,
    /// Awaiting approval.
    PendingApproval,
    /// Ready for completion or publication.
    Ready,
    /// Promotion is blocked (lineage, approval, policy).
    BlockedPromotion,
    /// Run in a terminal or failed state.
    Failed,
}

impl CanonHelpNextState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotInitialized => "not-initialized",
            Self::NoActiveRun => "no-active-run",
            Self::NoPacket => "no-packet",
            Self::IncompleteDocuments => "incomplete-documents",
            Self::PendingEvidence => "pending-evidence",
            Self::PendingApproval => "pending-approval",
            Self::Ready => "ready",
            Self::BlockedPromotion => "blocked-promotion",
            Self::Failed => "failed",
        }
    }
}

/// Severity of a canonical diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonDiagnosticSeverity {
    Info = 0,
    Warning = 1,
    Blocking = 2,
}

/// A single actionable diagnostic finding for Canon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonHelpNextDiagnostic {
    pub key: String,
    pub severity: CanonDiagnosticSeverity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    pub docs_key: String,
}

/// The resolved next action for a Canon workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonHelpNextRecommendation {
    pub state: CanonHelpNextState,
    pub blockers_found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_issue: Option<CanonHelpNextDiagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_issues: Vec<CanonHelpNextDiagnostic>,
    pub additional_count: u64,
    pub recommended_action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_command: Option<String>,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<String>,
}

impl CanonHelpNextRecommendation {
    /// Build a recommendation from collected diagnostics.
    #[must_use]
    pub fn from_diagnostics(
        state: CanonHelpNextState,
        mut diagnostics: Vec<CanonHelpNextDiagnostic>,
        docs_link: Option<String>,
    ) -> Self {
        diagnostics.sort_by_key(|d| std::cmp::Reverse(d.severity));
        let primary = diagnostics.first().cloned();
        let additional: Vec<_> = diagnostics.iter().skip(1).cloned().collect();
        let additional_count = additional.len() as u64;
        let blockers_found =
            primary.as_ref().is_some_and(|d| d.severity == CanonDiagnosticSeverity::Blocking);

        let (recommended_action, recommended_command, reason) = match state {
            CanonHelpNextState::NotInitialized => (
                "initialize Canon workspace".into(),
                Some("canon init".into()),
                "no .canon/ directory found — initialization is required".into(),
            ),
            CanonHelpNextState::NoActiveRun => (
                "start a new run".into(),
                Some("canon run".into()),
                "no active run — start a new governed run in a supported mode".into(),
            ),
            CanonHelpNextState::NoPacket => (
                "author the first document".into(),
                Some("canon run --mode <mode>".into()),
                "no packet exists for the active run — author the first required document".into(),
            ),
            CanonHelpNextState::IncompleteDocuments => (
                "complete missing documents".into(),
                primary.as_ref().and_then(|d| d.command.clone()),
                "packet is incomplete — some required documents are missing".into(),
            ),
            _ => (
                "continue the governed workflow".into(),
                Some("canon run".into()),
                "proceed with the current governed workflow".into(),
            ),
        };

        Self {
            state,
            blockers_found,
            primary_issue: primary,
            additional_issues: additional,
            additional_count,
            recommended_action,
            recommended_command,
            reason,
            docs_link,
        }
    }

    /// Build a ready-state recommendation.
    #[must_use]
    pub fn ready(docs_link: Option<String>) -> Self {
        Self {
            state: CanonHelpNextState::Ready,
            blockers_found: false,
            primary_issue: None,
            additional_issues: Vec::new(),
            additional_count: 0,
            recommended_action: "publish or complete the packet".into(),
            recommended_command: Some("canon publish".into()),
            reason: "all required documents and evidence are present — ready for publication"
                .into(),
            docs_link,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_initialized_suggests_init() {
        let rec = CanonHelpNextRecommendation::from_diagnostics(
            CanonHelpNextState::NotInitialized,
            vec![],
            None,
        );
        assert_eq!(rec.state, CanonHelpNextState::NotInitialized);
        assert_eq!(rec.recommended_command.as_deref(), Some("canon init"));
        assert!(!rec.blockers_found);
    }

    #[test]
    fn no_active_run_suggests_canon_run() {
        let rec = CanonHelpNextRecommendation::from_diagnostics(
            CanonHelpNextState::NoActiveRun,
            vec![],
            None,
        );
        assert_eq!(rec.state, CanonHelpNextState::NoActiveRun);
        assert_eq!(rec.recommended_command.as_deref(), Some("canon run"));
    }

    #[test]
    fn no_packet_suggests_authoring() {
        let rec = CanonHelpNextRecommendation::from_diagnostics(
            CanonHelpNextState::NoPacket,
            vec![],
            None,
        );
        assert_eq!(rec.state, CanonHelpNextState::NoPacket);
        assert!(rec.recommended_command.unwrap().contains("canon run"));
    }

    #[test]
    fn incomplete_documents_suggests_completion() {
        let diag = CanonHelpNextDiagnostic {
            key: "doc_missing".into(),
            severity: CanonDiagnosticSeverity::Blocking,
            message: "missing readme".into(),
            source: None,
            command: Some("canon run --mode review".into()),
            docs_key: "fallback".into(),
        };
        let rec = CanonHelpNextRecommendation::from_diagnostics(
            CanonHelpNextState::IncompleteDocuments,
            vec![diag],
            None,
        );
        assert_eq!(rec.state, CanonHelpNextState::IncompleteDocuments);
        assert!(rec.blockers_found);
        assert!(rec.primary_issue.is_some());
    }

    #[test]
    fn default_state_continues_workflow() {
        for state in [
            CanonHelpNextState::PendingEvidence,
            CanonHelpNextState::PendingApproval,
            CanonHelpNextState::BlockedPromotion,
            CanonHelpNextState::Failed,
        ] {
            let rec = CanonHelpNextRecommendation::from_diagnostics(state, vec![], None);
            assert!(rec.recommended_action.contains("continue"));
        }
    }

    #[test]
    fn diagnostics_sorted_by_severity_desc() {
        let d_info = CanonHelpNextDiagnostic {
            key: "info".into(),
            severity: CanonDiagnosticSeverity::Info,
            message: "i".into(),
            source: None,
            command: None,
            docs_key: "fallback".into(),
        };
        let d_block = CanonHelpNextDiagnostic {
            key: "block".into(),
            severity: CanonDiagnosticSeverity::Blocking,
            message: "b".into(),
            source: None,
            command: None,
            docs_key: "fallback".into(),
        };
        let rec = CanonHelpNextRecommendation::from_diagnostics(
            CanonHelpNextState::IncompleteDocuments,
            vec![d_info, d_block],
            None,
        );
        assert_eq!(rec.primary_issue.unwrap().severity, CanonDiagnosticSeverity::Blocking);
        assert_eq!(rec.additional_count, 1);
        assert!(rec.blockers_found);
    }

    #[test]
    fn ready_state_is_blocker_free() {
        let rec = CanonHelpNextRecommendation::ready(Some("wiki/canon-help".into()));
        assert_eq!(rec.state, CanonHelpNextState::Ready);
        assert!(!rec.blockers_found);
        assert!(rec.primary_issue.is_none());
        assert_eq!(rec.additional_count, 0);
        assert_eq!(rec.recommended_command.as_deref(), Some("canon publish"));
    }

    #[test]
    fn ready_with_none_docs_link() {
        let rec = CanonHelpNextRecommendation::ready(None);
        assert!(rec.docs_link.is_none());
    }

    #[test]
    fn diagnostic_severity_ordering() {
        assert!(CanonDiagnosticSeverity::Blocking > CanonDiagnosticSeverity::Warning);
        assert!(CanonDiagnosticSeverity::Warning > CanonDiagnosticSeverity::Info);
    }

    #[test]
    fn all_state_labels_are_non_empty() {
        let states = [
            CanonHelpNextState::NotInitialized,
            CanonHelpNextState::NoActiveRun,
            CanonHelpNextState::NoPacket,
            CanonHelpNextState::IncompleteDocuments,
            CanonHelpNextState::PendingEvidence,
            CanonHelpNextState::PendingApproval,
            CanonHelpNextState::Ready,
            CanonHelpNextState::BlockedPromotion,
            CanonHelpNextState::Failed,
        ];
        for s in states {
            assert!(!s.label().is_empty());
        }
    }

    #[test]
    fn diagnostic_serialization_roundtrip() {
        let d = CanonHelpNextDiagnostic {
            key: "test".into(),
            severity: CanonDiagnosticSeverity::Warning,
            message: "msg".into(),
            source: Some("file".into()),
            command: Some("cmd".into()),
            docs_key: "fallback".into(),
        };
        let json = serde_json::to_string(&d).unwrap();
        let parsed: CanonHelpNextDiagnostic = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.key, "test");
        assert_eq!(parsed.severity, CanonDiagnosticSeverity::Warning);
    }

    #[test]
    fn recommendation_serialization_roundtrip() {
        let rec = CanonHelpNextRecommendation::ready(Some("docs".into()));
        let json = serde_json::to_string(&rec).unwrap();
        let parsed: CanonHelpNextRecommendation = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.state, CanonHelpNextState::Ready);
    }

    #[test]
    fn state_serialization_roundtrip() {
        for state in [
            CanonHelpNextState::NotInitialized,
            CanonHelpNextState::Ready,
            CanonHelpNextState::Failed,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let parsed: CanonHelpNextState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, parsed);
        }
    }
}
