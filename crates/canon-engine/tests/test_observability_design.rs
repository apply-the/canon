use canon_engine::observability::SignalType;
use canon_engine::observability::evaluator::parse_llm_boundary_response;

#[test]
fn test_parse_valid_telemetry_plan_json() {
    let raw_response = r#"{
        "boundaries": [
            {
                "boundary_name": "API Gateway",
                "failure_domain": "edge",
                "consumer": "frontend",
                "signals": [
                    {
                        "signal_type": "Metric",
                        "name": "http_requests_total",
                        "description": "Total HTTP requests"
                    }
                ]
            }
        ],
        "global_constraints": ["Ensure 100% trace sampling on errors"]
    }"#;

    let result = parse_llm_boundary_response(raw_response).expect("Should parse valid JSON");
    assert_eq!(result.boundaries.len(), 1);
    assert_eq!(result.boundaries[0].boundary_name, "API Gateway");
    assert_eq!(result.boundaries[0].signals.len(), 1);
    assert_eq!(result.boundaries[0].signals[0].signal_type, SignalType::Metric);
    assert_eq!(result.global_constraints.len(), 1);
}

#[test]
fn test_parse_invalid_json_returns_error() {
    let raw_response = "I am an AI and I think this: {\"boundaries\": []";
    let result = parse_llm_boundary_response(raw_response);
    assert!(result.is_err());
}

#[test]
fn test_slo_generator_creates_markdown() {
    use canon_engine::observability::{SloAlert, generators};
    let slos = vec![SloAlert {
        sli_name: "API Latency".to_string(),
        threshold: "P99 < 200ms".to_string(),
        alert_destination: "#oncall".to_string(),
    }];
    let markdown = generators::generate_slo_alerts(&slos);
    assert!(markdown.contains("# SLO Alerts"));
    assert!(markdown.contains("API Latency"));
    assert!(markdown.contains("P99 < 200ms"));
    assert!(markdown.contains("#oncall"));
}

#[test]
fn test_runbook_generator_creates_markdown() {
    use canon_engine::observability::{RunbookStub, generators};
    let stubs = vec![RunbookStub {
        alert_trigger: "High Latency".to_string(),
        action_items: vec!["Check database".to_string()],
        escalation_path: "DBA Team".to_string(),
    }];
    let markdown = generators::generate_runbook_stubs(&stubs);
    assert!(markdown.contains("# Runbook Playbooks"));
    assert!(markdown.contains("High Latency"));
    assert!(markdown.contains("Check database"));
    assert!(markdown.contains("DBA Team"));
}

#[test]
fn test_evaluate_architecture_interactive_fallback() {
    use canon_engine::observability::evaluator;

    // Simulate interactive mode which should still return a valid plan
    let plan = evaluator::evaluate_architecture("vague architecture", true)
        .expect("Should succeed with interactive fallback");

    assert_eq!(plan.boundaries.len(), 1);
    assert_eq!(plan.boundaries[0].boundary_name, "Interactive Boundary");
}
