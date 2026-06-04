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
