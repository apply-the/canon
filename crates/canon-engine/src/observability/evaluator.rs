use crate::observability::TelemetryPlan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Failed to parse LLM response: {0}")]
    ParseError(String),
    #[error("LLM inference failed: {0}")]
    InferenceError(String),
}

/// Parses the JSON output from the LLM into a strongly typed TelemetryPlan.
pub fn parse_llm_boundary_response(raw_json: &str) -> Result<TelemetryPlan, EvaluatorError> {
    // Attempt to extract JSON if the LLM wrapped it in markdown code blocks.
    let s = raw_json.trim();
    let cleaned = s
        .strip_prefix("```json\n")
        .or_else(|| s.strip_prefix("```json"))
        .or_else(|| s.strip_prefix("```\n"))
        .or_else(|| s.strip_prefix("```"))
        .unwrap_or(s)
        .strip_suffix("```")
        .unwrap_or(s)
        .trim();

    serde_json::from_str(cleaned).map_err(|e| EvaluatorError::ParseError(e.to_string()))
}

/// Evaluates the given architecture document and infers observability boundaries.
/// In a real implementation, this would call the canon_adapters LLM interface.
pub fn evaluate_architecture(
    _architecture_content: &str,
    interactive: bool,
) -> Result<TelemetryPlan, EvaluatorError> {
    if interactive {
        // In a real CLI, we would use dialoguer or rustyline here to prompt the user
        // For the MVP, we just return the interactive mock response directly
        println!("Interactive mode: Prompting user for boundaries...");
        let interactive_json = r#"{
            "boundaries": [
                {
                    "boundary_name": "Interactive Boundary",
                    "failure_domain": "manual",
                    "consumer": "human",
                    "signals": []
                }
            ],
            "global_constraints": []
        }"#;
        return parse_llm_boundary_response(interactive_json);
    }

    // Default automated LLM Prompting logic goes here.
    let mock_json = r#"{
        "boundaries": [
            {
                "boundary_name": "Inferred Backend",
                "failure_domain": "service",
                "consumer": "internal",
                "signals": []
            }
        ],
        "global_constraints": []
    }"#;

    parse_llm_boundary_response(mock_json)
}

/// Generates SLOs and Runbooks based on the inferred TelemetryPlan.
pub fn generate_slos_and_runbooks(
    _plan: &TelemetryPlan,
) -> Result<
    (Vec<crate::observability::SloAlert>, Vec<crate::observability::RunbookStub>),
    EvaluatorError,
> {
    let slos = vec![crate::observability::SloAlert {
        sli_name: "Inferred Backend Latency".to_string(),
        threshold: "P99 < 500ms".to_string(),
        alert_destination: "#backend-oncall".to_string(),
    }];
    let runbooks = vec![crate::observability::RunbookStub {
        alert_trigger: "High Inferred Backend Latency".to_string(),
        action_items: vec!["Check database metrics".to_string()],
        escalation_path: "Backend Engineering Team".to_string(),
    }];
    Ok((slos, runbooks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_llm_boundary_response_strips_json_blocks() {
        let raw = "```json\n{\"boundaries\":[{\"boundary_name\":\"B1\",\"signals\":[],\"failure_domain\":\"D\",\"consumer\":\"C\"}],\"global_constraints\":[]}\n```";
        let res = parse_llm_boundary_response(raw).unwrap();
        assert_eq!(res.boundaries.len(), 1);
    }

    #[test]
    fn test_parse_llm_boundary_response_strips_bare_blocks() {
        let raw = "```\n{\"boundaries\":[],\"global_constraints\":[\"GC1\"]}\n```";
        let res = parse_llm_boundary_response(raw).unwrap();
        assert_eq!(res.global_constraints.len(), 1);
    }

    #[test]
    fn test_evaluate_architecture_automated_mock() {
        let plan = evaluate_architecture("arch", false).unwrap();
        assert_eq!(plan.boundaries[0].boundary_name, "Inferred Backend");
    }

    #[test]
    fn test_evaluate_architecture_interactive_mock() {
        let plan = evaluate_architecture("arch", true).unwrap();
        assert_eq!(plan.boundaries[0].boundary_name, "Interactive Boundary");
    }

    #[test]
    fn test_generate_slos_and_runbooks_returns_defaults() {
        let plan =
            crate::observability::TelemetryPlan { boundaries: vec![], global_constraints: vec![] };
        let (slos, runbooks) = generate_slos_and_runbooks(&plan).unwrap();
        assert_eq!(slos[0].sli_name, "Inferred Backend Latency");
        assert_eq!(runbooks[0].alert_trigger, "High Inferred Backend Latency");
    }
}
