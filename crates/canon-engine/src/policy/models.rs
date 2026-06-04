use serde::{Deserialize, Serialize};

/// Represents the proposed governance rule change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DraftPolicy {
    pub title: String,
    pub mode: String,
    pub risk: String,
    pub scope_in: Vec<String>,
    pub scope_out: Vec<String>,
    pub invariants: Vec<String>,
}

/// Evidence describing current codebase violations against the draft policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImpactReport {
    pub total_violations: usize,
    pub affected_modules: usize,
    pub severity: String,
    pub migration_risk: String,
}

/// Strategy for transitioning legacy areas to compliance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MigrationPlan {
    pub waiver_policy: String,
    pub rollout_phases: Vec<String>,
    pub debt_created: String,
}

/// Semantic changes to the existing constitution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PolicyDiff {
    pub semantic_changes: Vec<String>,
}
