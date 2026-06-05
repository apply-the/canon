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

#[test]
fn test_large_diff_requires_review_coverage() {
    let payload = r#"{
        "github_comments": [],
        "missing_tests": []
    }"#;
    let eval_result = evaluate_diff(MOCK_PATCH, 21, 501, payload);
    assert!(eval_result.is_err());
    assert!(eval_result.unwrap_err().contains("require a review_coverage block"));

    let valid_payload = r#"{
        "github_comments": [],
        "missing_tests": [],
        "review_coverage": {
            "changed_files_total": 21,
            "files_reviewed_deeply": 5,
            "files_sampled": 16,
            "files_not_reviewed_deeply": 0,
            "coverage_strategy": "Sampled",
            "unreviewed_risk": "Low"
        }
    }"#;
    let eval_result2 = evaluate_diff(MOCK_PATCH, 21, 501, valid_payload);
    assert!(eval_result2.is_ok());
}

#[test]
fn test_derive_decision_comment_and_approve() {
    use canon_engine::review::findings::{
        ConventionalCommentScope, FindingCategory, FindingSeverity, ReviewFinding, ReviewPacket,
    };

    let empty_payload = r#"{
        "github_comments": [],
        "missing_tests": []
    }"#;
    let eval = evaluate_diff(MOCK_PATCH, 1, 10, empty_payload).unwrap();
    let mut packet = ReviewPacket {
        base_ref: "base".to_string(),
        head_ref: "head".to_string(),
        changed_surfaces: Vec::new(),
        inferred_intent: String::new(),
        findings: Vec::new(),
        surprising_surface_area: Vec::new(),
    };

    assert_eq!(derive_decision(&eval, &packet), Decision::Approve);

    packet.findings.push(ReviewFinding {
        category: FindingCategory::DuplicationCheck,
        severity: FindingSeverity::Note,
        title: "Note".to_string(),
        details: "Detail".to_string(),
        scope: ConventionalCommentScope::Pr,
        anchor: None,
        changed_surfaces: vec![],
    });

    assert_eq!(derive_decision(&eval, &packet), Decision::Comment);
}

#[test]
fn test_generators_produce_expected_markdown() {
    use canon_engine::review::findings::{
        ConventionalCommentScope, FindingCategory, FindingSeverity, ReviewFinding, ReviewPacket,
    };
    use canon_engine::review::generators::{
        generate_conventional_comments, generate_missing_tests, generate_review_summary,
    };

    let payload_str = r#"{
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
                "suggested_change": "let a = 1;"
            }
        ],
        "missing_tests": [
            {
                "id": "m1",
                "affected_behavior": "login",
                "reason": "missing",
                "risk": "high",
                "suggested_shape": "test_login",
                "blocking": true
            }
        ],
        "review_coverage": {
            "changed_files_total": 21,
            "files_reviewed_deeply": 5,
            "files_sampled": 16,
            "files_not_reviewed_deeply": 0,
            "coverage_strategy": "Sampled",
            "unreviewed_risk": "Low"
        }
    }"#;
    let eval = evaluate_diff(MOCK_PATCH, 21, 501, payload_str).unwrap();
    let packet = ReviewPacket {
        base_ref: "base".to_string(),
        head_ref: "head".to_string(),
        changed_surfaces: vec!["src/main.rs".to_string()],
        inferred_intent: String::new(),
        findings: vec![
            ReviewFinding {
                category: FindingCategory::DuplicationCheck,
                severity: FindingSeverity::Note,
                title: "Note".to_string(),
                details: "Detail".to_string(),
                scope: ConventionalCommentScope::File,
                anchor: None,
                changed_surfaces: vec!["src/main.rs".to_string()],
            },
            ReviewFinding {
                category: FindingCategory::MissingTests,
                severity: FindingSeverity::MustFix,
                title: "Deterministic Missing Test".to_string(),
                details: "Det detail".to_string(),
                scope: ConventionalCommentScope::Pr,
                anchor: None,
                changed_surfaces: vec![],
            },
        ],
        surprising_surface_area: Vec::new(),
    };

    let summary = generate_review_summary(&eval, &packet, &Decision::RequestChanges);
    assert!(summary.contains("REQUEST CHANGES"));
    assert!(summary.contains("Review Coverage"));
    assert!(summary.contains("Deterministic Missing Test"));
    assert!(summary.contains("Fix this"));

    let conventional = generate_conventional_comments(&eval, &packet);
    assert!(conventional.contains("src/main.rs:13 - issue"));
    assert!(conventional.contains("```rust"));

    let missing = generate_missing_tests(&eval, &packet);
    assert!(missing.contains("login"));

    // Test empty branch
    let empty_payload =
        evaluate_diff(MOCK_PATCH, 1, 10, r#"{"github_comments":[],"missing_tests":[]}"#).unwrap();
    let empty_packet = ReviewPacket {
        base_ref: "b".to_string(),
        head_ref: "h".to_string(),
        changed_surfaces: vec![],
        inferred_intent: "".to_string(),
        findings: vec![],
        surprising_surface_area: vec![],
    };
    let s2 = generate_review_summary(&empty_payload, &empty_packet, &Decision::Approve);
    assert!(s2.contains("No must-fix findings remain"));
    assert!(s2.contains("No accepted risks recorded"));

    let c2 = generate_conventional_comments(&empty_payload, &empty_packet);
    assert!(c2.contains("No conventional comments were generated"));

    let m2 = generate_missing_tests(&empty_payload, &empty_packet);
    assert!(m2.contains("No missing invariant checks inferred"));
}

#[test]
fn test_diff_parser_zero_line_count() {
    use canon_engine::review::diff::validate_and_map_line;
    let patch = "@@ -0,0 +1,0 @@\n";
    // This just exercises the line_count == 0 branch in diff.rs
    let _ = validate_and_map_line("src/main.rs", 1, patch);
}
