//! The stable profile registry deliberately excludes implementation execution.

use serde::{Deserialize, Serialize};

/// Stable Canon 1.0 governance profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Profile {
    /// Explore a problem space and record relevant observations.
    Discovery,
    /// Define required outcomes and constraints.
    Requirements,
    /// Govern system structure and cross-boundary design.
    Architecture,
    /// Shape ordered, admitted work without executing it.
    Backlog,
    /// Govern change intent, scope, risk, invariants, and evidence.
    Change,
    /// Govern behavior-preserving structural improvement.
    Refactor,
    /// Govern deterministic checks and external verification evidence.
    Verification,
    /// Govern pull-request review claims and findings.
    PrReview,
    /// Govern incident facts, decisions, and recovery obligations.
    Incident,
}

const STABLE_PROFILES: [Profile; 9] = [
    Profile::Discovery,
    Profile::Requirements,
    Profile::Architecture,
    Profile::Backlog,
    Profile::Change,
    Profile::Refactor,
    Profile::Verification,
    Profile::PrReview,
    Profile::Incident,
];

/// Read-only registry for the exact Canon 1.0 stable profile set.
pub struct StableProfileRegistry;

impl StableProfileRegistry {
    /// Returns the exact ordered stable profile set.
    pub const fn profiles() -> &'static [Profile; 9] {
        &STABLE_PROFILES
    }
}
