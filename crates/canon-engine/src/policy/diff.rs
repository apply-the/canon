use crate::policy::models::{DraftPolicy, PolicyDiff};

pub fn generate_diff(old_policy: Option<&DraftPolicy>, new_policy: &DraftPolicy) -> PolicyDiff {
    if let Some(old) = old_policy {
        let mut changes = vec![];
        if old.scope_in != new_policy.scope_in || old.scope_out != new_policy.scope_out {
            changes.push("Scope changed".to_string());
        }
        for i in &new_policy.invariants {
            if !old.invariants.contains(i) {
                changes.push(format!("Added invariant: {}", i));
            }
        }
        for i in &old.invariants {
            if !new_policy.invariants.contains(i) {
                changes.push(format!("Removed invariant: {}", i));
            }
        }
        PolicyDiff { semantic_changes: changes }
    } else {
        PolicyDiff { semantic_changes: vec!["Initial policy draft".to_string()] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::models::DraftPolicy;

    #[test]
    fn test_generate_diff_initial() {
        let new_policy = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec![],
            scope_out: vec![],
            invariants: vec![],
        };
        let diff = generate_diff(None, &new_policy);
        assert_eq!(diff.semantic_changes, vec!["Initial policy draft".to_string()]);
    }

    #[test]
    fn test_generate_diff_with_changes() {
        let old_policy = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec!["a".to_string()],
            scope_out: vec![],
            invariants: vec!["inv1".to_string(), "inv2".to_string()],
        };
        let new_policy = DraftPolicy {
            title: "Draft".to_string(),
            mode: "policy-shaping".to_string(),
            risk: "low-impact".to_string(),
            scope_in: vec!["b".to_string()],
            scope_out: vec![],
            invariants: vec!["inv2".to_string(), "inv3".to_string()],
        };
        let diff = generate_diff(Some(&old_policy), &new_policy);
        assert!(diff.semantic_changes.contains(&"Scope changed".to_string()));
        assert!(diff.semantic_changes.contains(&"Added invariant: inv3".to_string()));
        assert!(diff.semantic_changes.contains(&"Removed invariant: inv1".to_string()));
    }
}
