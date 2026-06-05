use canon_engine::review::evaluator::{Decision, derive_decision, evaluate_diff};

const MOCK_PATCH: &str = "\
--- a/src/main.rs
+++ b/src/main.rs
@@ -10,3 +10,4 @@
 line 10
 line 11
 line 12
+line 13
";

#[test]
fn test_integration_extract_and_map_comments() {
    let payload = r#"{
        "github_comments": [
            {
                "id": "c1",
                "path": "src/main.rs",
                "line": 13,
                "side": "RIGHT",
                "area": "src",
                "type": "issue",
                "blocking": false,
                "severity": "note",
                "category": "style",
                "body": "Fix this.",
                "why_it_matters": "...",
                "suggested_remediation": "...",
                "suggested_change": null
            },
            {
                "id": "c2",
                "path": "src/main.rs",
                "line": 99,
                "side": "RIGHT",
                "area": "src",
                "type": "issue",
                "blocking": false,
                "severity": "note",
                "category": "style",
                "body": "Hallucinated line.",
                "why_it_matters": "...",
                "suggested_remediation": "...",
                "suggested_change": null
            }
        ],
        "missing_tests": []
    }"#;

    let eval = evaluate_diff(MOCK_PATCH, 1, 10, payload).expect("should parse");

    // c1 is valid, should map exactly
    assert_eq!(eval.github_comments[0].line, Some(13));

    // c2 is invalid, should be downgraded to hunk
    assert_eq!(eval.github_comments[1].line, None);
    assert_eq!(eval.github_comments[1].hunk_header, Some("@@ -10,3 +10,4 @@".to_string()));
}

#[test]
fn test_decision_approve_never_returned_if_blocking() {
    let payload = r#"{
        "github_comments": [
            {
                "id": "c1",
                "path": "src/main.rs",
                "line": 13,
                "side": "RIGHT",
                "area": "src",
                "type": "issue",
                "blocking": true,
                "severity": "must-fix",
                "category": "bug",
                "body": "Fix this.",
                "why_it_matters": "...",
                "suggested_remediation": "...",
                "suggested_change": null
            }
        ],
        "missing_tests": []
    }"#;

    let eval = evaluate_diff(MOCK_PATCH, 1, 10, payload).expect("should parse");
    let dummy_packet = canon_engine::review::findings::ReviewPacket {
        base_ref: "base".to_string(),
        head_ref: "head".to_string(),
        changed_surfaces: Vec::new(),
        inferred_intent: String::new(),
        findings: Vec::new(),
        surprising_surface_area: Vec::new(),
    };
    let decision = derive_decision(&eval, &dummy_packet);
    assert_eq!(decision, Decision::RequestChanges);
}

#[test]
fn test_missing_test_finding_without_behavior_rejected() {
    let payload = r#"{
        "github_comments": [],
        "missing_tests": [
            {
                "id": "m1",
                "affected_behavior": "",
                "reason": "Missing unit test",
                "risk": "Low",
                "suggested_shape": "Add test",
                "blocking": false
            }
        ]
    }"#;

    let eval_result = evaluate_diff(MOCK_PATCH, 1, 10, payload);
    assert!(eval_result.is_err());
    assert!(eval_result.unwrap_err().contains("affected_behavior must be explicit"));
}
