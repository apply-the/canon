use clap::Args;
use tracing::info;

/// Evaluates a proposed policy change and surfaces migration impact.
#[derive(Debug, Args)]
pub struct PolicyShapingArgs {
    /// Path to the draft policy document to evaluate.
    #[arg(required = true)]
    pub draft_policy_path: String,

    /// Explicitly acknowledge broad migration impact and bypass the fail-safe.
    #[arg(long)]
    pub acknowledge_broad_impact: bool,

    /// Provide explicit Systemic Impact sign-off for the proposed policy.
    #[arg(long)]
    pub approve: bool,
}

pub fn handle(args: &PolicyShapingArgs) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running policy-shaping for {}", args.draft_policy_path);

    // T017: Enforce explicit Systemic Impact broad-impact approval gate.
    // In a real implementation this would evaluate the policy and get the impact radius.
    // For now we mock the broad impact condition based on the acknowledge flag and logic.
    let is_broad_impact = true; // Mocked condition for demonstration

    if is_broad_impact && !args.acknowledge_broad_impact {
        return Err("Broad impact detected. You must explicitly pass --acknowledge-broad-impact to proceed.".into());
    }

    if !args.approve {
        return Err("Systemic Impact approval required. You must explicitly pass --approve to finalize the policy shaping run.".into());
    }

    // T016: Integrate evaluator, report, migration, and diff modules into the CLI handler.
    // (mocked out success)

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_missing_acknowledge() {
        let args = PolicyShapingArgs {
            draft_policy_path: "draft.md".to_string(),
            acknowledge_broad_impact: false,
            approve: true,
        };
        let err = handle(&args).unwrap_err();
        assert!(err.to_string().contains("--acknowledge-broad-impact"));
    }

    #[test]
    fn test_handle_missing_approve() {
        let args = PolicyShapingArgs {
            draft_policy_path: "draft.md".to_string(),
            acknowledge_broad_impact: true,
            approve: false,
        };
        let err = handle(&args).unwrap_err();
        assert!(err.to_string().contains("--approve"));
    }

    #[test]
    fn test_handle_success() {
        let args = PolicyShapingArgs {
            draft_policy_path: "draft.md".to_string(),
            acknowledge_broad_impact: true,
            approve: true,
        };
        assert!(handle(&args).is_ok());
    }
}
