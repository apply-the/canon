use crate::policy::models::{DraftPolicy, ImpactReport};

#[derive(Default)]
pub struct PolicyEvaluator {
    // Adapter context might be added here
}

impl PolicyEvaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, _draft_policy: &DraftPolicy) -> Result<ImpactReport, String> {
        // T010: Orchestrate .agents/skills using CopilotCliAdapter or similar.
        // For now, this is a stub that returns a dummy report.

        Ok(ImpactReport {
            total_violations: 0,
            affected_modules: 0,
            severity: "Low".to_string(),
            migration_risk: "None".to_string(),
        })
    }
}
