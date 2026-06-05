use crate::policy::models::{DraftPolicy, ImpactReport, MigrationPlan};

pub fn generate_migration(_policy: &DraftPolicy, report: &ImpactReport) -> MigrationPlan {
    MigrationPlan {
        waiver_policy: "Default waiver policy".to_string(),
        rollout_phases: vec![
            "Identify affected modules".to_string(),
            "Apply automated fixes where possible".to_string(),
            "Review remaining manual changes".to_string(),
        ],
        debt_created: format!("{} violations require attention", report.total_violations),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_migration() {
        let draft = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec![],
            scope_out: vec![],
            invariants: vec![],
        };
        let report = ImpactReport {
            total_violations: 5,
            affected_modules: 2,
            severity: "High".to_string(),
            migration_risk: "Medium".to_string(),
        };
        let plan = generate_migration(&draft, &report);
        assert_eq!(plan.waiver_policy, "Default waiver policy");
        assert_eq!(plan.debt_created, "5 violations require attention");
        assert_eq!(plan.rollout_phases.len(), 3);
    }
}
