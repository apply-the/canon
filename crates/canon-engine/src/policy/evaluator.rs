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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::models::DraftPolicy;

    #[test]
    fn test_evaluator_new_and_evaluate() {
        let evaluator = PolicyEvaluator::new();
        let draft = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec![],
            scope_out: vec![],
            invariants: vec![],
        };
        let report = evaluator.evaluate(&draft).unwrap();
        assert_eq!(report.total_violations, 0);
        assert_eq!(report.severity, "Low");
    }

    #[test]
    fn test_evaluator_default() {
        let evaluator = PolicyEvaluator::default();
        let draft = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec![],
            scope_out: vec![],
            invariants: vec![],
        };
        assert!(evaluator.evaluate(&draft).is_ok());
    }
}
