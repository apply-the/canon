use canon_engine::review::evaluator::{Decision, derive_decision, evaluate_diff};
use canon_engine::review::findings::{
    CanonicalCommentSet, ConventionalCommentScope, FindingCategory, FindingSeverity, GithubComment,
    MissingTest, ReviewFinding, ReviewPacket,
};
use canon_engine::review::generators::{
    generate_conventional_comments, generate_github_comments_json, generate_missing_tests,
    generate_review_findings_json, generate_review_report, generate_review_summary,
};

const MOCK_PATCH: &str = "\
--- a/src/main.rs
+++ b/src/main.rs
@@ -10,3 +10,4 @@
line 10
line 11
line 12
+line 13
";

// ── Fixture helpers ─────────────────────────────────────────────────────

/// Builds a `GithubComment` with the most common defaults.
fn comment(path: &str, line: Option<u32>, blocking: bool, body: &str) -> GithubComment {
    GithubComment {
        id: String::new(),
        path: Some(path.to_string()),
        line,
        side: line.map(|_| "RIGHT".to_string()),
        hunk_header: None,
        area: "src".to_string(),
        kind: "issue".to_string(),
        blocking,
        severity: if blocking { "blocking" } else { "note" }.to_string(),
        category: if blocking { "bug" } else { "style" }.to_string(),
        body: body.to_string(),
        why_it_matters: "Reason.".to_string(),
        suggested_remediation: "Remediation.".to_string(),
        suggested_change: None,
    }
}

/// Hunk-level comment (line downgraded to hunk).
fn hunk_comment(path: &str, hunk: &str, blocking: bool, body: &str) -> GithubComment {
    GithubComment {
        id: String::new(),
        path: Some(path.to_string()),
        line: None,
        side: None,
        hunk_header: Some(hunk.to_string()),
        area: "src".to_string(),
        kind: "issue".to_string(),
        blocking,
        severity: if blocking { "blocking" } else { "note" }.to_string(),
        category: "bug".to_string(),
        body: body.to_string(),
        why_it_matters: "Reason.".to_string(),
        suggested_remediation: "Remediation.".to_string(),
        suggested_change: None,
    }
}

/// Builds a `MissingTest` fixture.
fn missing_test(id: &str, behavior: &str, blocking: bool) -> MissingTest {
    MissingTest {
        id: id.to_string(),
        affected_behavior: behavior.to_string(),
        reason: "missing".to_string(),
        risk: "high".to_string(),
        suggested_shape: "Add test".to_string(),
        blocking,
    }
}

/// Returns an empty `ReviewPacket` for use as dummy input.
fn empty_packet() -> ReviewPacket {
    ReviewPacket {
        base_ref: "base".to_string(),
        head_ref: "head".to_string(),
        changed_surfaces: Vec::new(),
        inferred_intent: String::new(),
        findings: Vec::new(),
        surprising_surface_area: Vec::new(),
    }
}

/// Returns a `ReviewPacket` with the given changed surfaces and findings.
fn packet_with(surfaces: Vec<String>, findings: Vec<ReviewFinding>) -> ReviewPacket {
    ReviewPacket {
        base_ref: "base".to_string(),
        head_ref: "head".to_string(),
        changed_surfaces: surfaces,
        inferred_intent: String::new(),
        findings,
        surprising_surface_area: Vec::new(),
    }
}

/// Builds a JSON evaluator payload string from comments and missing tests.
fn payload_json(comments: &[GithubComment], tests: &[MissingTest]) -> String {
    build_payload(comments, tests, None)
}

/// Builds a payload with coverage info for large diff tests.
fn payload_with_coverage(comments: &[GithubComment], tests: &[MissingTest]) -> String {
    let coverage = r#""review_coverage":{"changed_files_total":21,"files_reviewed_deeply":5,"files_sampled":16,"files_not_reviewed_deeply":0,"coverage_strategy":"Sampled","unreviewed_risk":"Low"}"#;
    build_payload(comments, tests, Some(coverage))
}

fn build_payload(comments: &[GithubComment], tests: &[MissingTest], extra: Option<&str>) -> String {
    let comments_json = comments.iter().map(serialize_comment).collect::<Vec<_>>().join(",");
    let tests_json = tests.iter().map(serialize_missing_test).collect::<Vec<_>>().join(",");
    let extra_block = extra.map(|e| format!(",{e}")).unwrap_or_default();
    format!(
        r#"{{"github_comments":[{comments_json}],"missing_tests":[{tests_json}]{extra_block}}}"#
    )
}

fn serialize_comment(c: &GithubComment) -> String {
    format!(
        r#"{{"id":"{}","path":{},"line":{},"side":{},"hunk_header":{},"area":"{}","type":"{}","blocking":{},"severity":"{}","category":"{}","body":"{}","why_it_matters":"{}","suggested_remediation":"{}","suggested_change":{}}}"#,
        c.id,
        serde_json::to_string(&c.path).unwrap(),
        serde_json::to_string(&c.line).unwrap(),
        serde_json::to_string(&c.side).unwrap(),
        serde_json::to_string(&c.hunk_header).unwrap(),
        c.area,
        c.kind,
        c.blocking,
        c.severity,
        c.category,
        c.body,
        c.why_it_matters,
        c.suggested_remediation,
        serde_json::to_string(&c.suggested_change).unwrap(),
    )
}

fn serialize_missing_test(mt: &MissingTest) -> String {
    format!(
        r#"{{"id":"{}","affected_behavior":"{}","reason":"{}","risk":"{}","suggested_shape":"{}","blocking":{}}}"#,
        mt.id, mt.affected_behavior, mt.reason, mt.risk, mt.suggested_shape, mt.blocking
    )
}

// ── Evaluator tests ─────────────────────────────────────────────────────

#[test]
fn test_integration_extract_and_map_comments() {
    let c1 = comment("src/main.rs", Some(13), false, "Fix this.");
    let c2 = comment("src/main.rs", Some(99), false, "Hallucinated line.");
    let payload = payload_json(&[c1, c2], &[]);
    let eval = evaluate_diff(MOCK_PATCH, 1, 10, &payload).expect("should parse");
    assert_eq!(eval.github_comments[0].line, Some(13));
    assert_eq!(eval.github_comments[1].line, None);
    assert_eq!(eval.github_comments[1].hunk_header, Some("@@ -10,3 +10,4 @@".to_string()));
}

#[test]
fn test_decision_approve_never_returned_if_blocking() {
    let c = comment("src/main.rs", Some(13), true, "Fix this.");
    let payload = payload_json(&[c], &[]);
    let eval = evaluate_diff(MOCK_PATCH, 1, 10, &payload).expect("should parse");
    let decision = derive_decision(&eval, &empty_packet());
    assert_eq!(decision, Decision::RequestChanges);
}

#[test]
fn test_missing_test_finding_without_behavior_rejected() {
    let mt = missing_test("m1", "", false);
    let payload = payload_json(&[], &[mt]);
    let eval_result = evaluate_diff(MOCK_PATCH, 1, 10, &payload);
    assert!(eval_result.is_err());
    assert!(eval_result.unwrap_err().contains("affected_behavior must be explicit"));
}

#[test]
fn test_large_diff_requires_review_coverage() {
    let empty = payload_json(&[], &[]);
    let eval_result = evaluate_diff(MOCK_PATCH, 21, 501, &empty);
    assert!(eval_result.is_err());
    assert!(eval_result.unwrap_err().contains("require a review_coverage block"));

    let valid = payload_with_coverage(&[], &[]);
    let eval_result2 = evaluate_diff(MOCK_PATCH, 21, 501, &valid);
    assert!(eval_result2.is_ok());
}

#[test]
fn test_derive_decision_comment_and_approve() {
    let empty = payload_json(&[], &[]);
    let eval = evaluate_diff(MOCK_PATCH, 1, 10, &empty).unwrap();
    let mut packet = empty_packet();
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

// ── Generator coverage tests ────────────────────────────────────────────

#[test]
fn test_generators_produce_expected_markdown() {
    let c1 = comment("src/main.rs", Some(13), true, "Fix this.");
    let mt1 = missing_test("m1", "login", true);
    let payload = payload_with_coverage(&[c1], &[mt1]);
    let eval = evaluate_diff(MOCK_PATCH, 21, 501, &payload).unwrap();
    let canonical = CanonicalCommentSet::from_evaluated(eval.github_comments.clone());
    let review_findings =
        canon_engine::review::findings::build_review_findings(&canonical, &empty_packet());
    let packet = packet_with(vec!["src/main.rs".to_string()], vec![]);

    // RequestChanges decision
    let summary = generate_review_summary(
        &canonical,
        &eval.missing_tests,
        &Decision::RequestChanges,
        &packet,
    );
    assert!(summary.contains("Request changes"));
    assert!(summary.contains("C001"));
    assert!(summary.contains("Fix this"));
    assert!(summary.contains("Governance Notes"));
    assert!(summary.contains("Status: awaiting-disposition"));

    let conventional = generate_conventional_comments(&canonical);
    assert!(conventional.contains("C001"));
    assert!(conventional.contains("src/main.rs"));
    assert!(conventional.contains("Fix this"));

    let missing = generate_missing_tests(&eval.missing_tests, &packet);
    assert!(missing.contains("login"));

    let gh_json = generate_github_comments_json(&canonical);
    assert!(gh_json.contains("C001"));
    assert!(gh_json.contains("Fix this"));

    let rf_json = generate_review_findings_json(&review_findings);
    assert!(rf_json.contains("F001"));
    assert!(rf_json.contains("code"));

    // Empty canonical set + Approve
    let empty_canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let empty_p = empty_packet();
    let s2 = generate_review_summary(&empty_canonical, &[], &Decision::Approve, &empty_p);
    assert!(s2.contains("Approve"));
    assert!(s2.contains("No blocking findings"));
    assert!(s2.contains("Status: ready"));

    let c2 = generate_conventional_comments(&empty_canonical);
    assert!(c2.contains("## Empty Comment Set"));
}

#[test]
fn test_review_summary_with_comment_decision() {
    let c = comment("src/a.rs", Some(5), false, "Nitpick.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c]);
    let packet = packet_with(vec!["src/a.rs".to_string()], vec![]);
    let summary = generate_review_summary(&canonical, &[], &Decision::Comment, &packet);
    assert!(summary.contains("**Comment**"));
    assert!(summary.contains("Nitpick"));
    assert!(summary.contains("1 non-blocking comment"));
    assert!(summary.contains("Status: ready-with-review-notes"));
}

#[test]
fn test_review_summary_with_non_blocking_and_hunk_comments() {
    let c1 = comment("src/a.rs", Some(5), false, "Nitpick.");
    let c2 = hunk_comment("src/b.rs", "@@ -20,3 +20,4 @@", false, "Hunk issue.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c1, c2]);
    let packet = packet_with(vec!["src/a.rs".to_string()], vec![]);
    let summary = generate_review_summary(&canonical, &[], &Decision::Comment, &packet);

    // Non-blocking comments rendered
    assert!(summary.contains("Nitpick"));
    assert!(summary.contains("Hunk issue"));
    // Hunk-level: path shown, no line
    assert!(summary.contains("`src/b.rs` (hunk)"));
    assert!(summary.contains("Status: ready-with-review-notes"));
}

#[test]
fn test_review_summary_with_surprising_surface_area() {
    let canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let mut packet = empty_packet();
    packet.surprising_surface_area =
        vec!["contracts/api.json".to_string(), "src/boundary.rs".to_string()];
    let summary = generate_review_summary(&canonical, &[], &Decision::Approve, &packet);
    assert!(summary.contains("Surprising surface area"));
    assert!(summary.contains("contracts/api.json"));
    assert!(summary.contains("src/boundary.rs"));
}

#[test]
fn test_review_summary_with_general_findings() {
    let canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let mut packet = empty_packet();
    // General findings are those with empty changed_surfaces
    packet.findings = vec![ReviewFinding {
        category: FindingCategory::DecisionImpact,
        severity: FindingSeverity::MustFix,
        title: "Global concern".to_string(),
        details: "Affects entire codebase.".to_string(),
        scope: ConventionalCommentScope::Pr,
        anchor: None,
        changed_surfaces: vec![],
    }];
    let summary = generate_review_summary(&canonical, &[], &Decision::Approve, &packet);
    assert!(summary.contains("General Findings"));
    assert!(summary.contains("Global concern"));
}

#[test]
fn test_conventional_comments_renders_hunk_target() {
    let c = hunk_comment("src/b.rs", "@@ -20,3 +20,4 @@", true, "Hunk issue.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c]);
    let cc_md = generate_conventional_comments(&canonical);
    assert!(cc_md.contains("C001"));
    assert!(cc_md.contains("hunk `@@ -20,3 +20,4 @@`"));
    assert!(cc_md.contains("Hunk issue."));
}

#[test]
fn test_missing_tests_all_paths() {
    // Source+test → no warning
    let p1 = packet_with(vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()], vec![]);
    let out1 = generate_missing_tests(&[], &p1);
    assert!(out1.contains("Changed surfaces include source and test updates"));

    // Source only → warning
    let p2 = packet_with(vec!["src/lib.rs".to_string()], vec![]);
    let out2 = generate_missing_tests(&[], &p2);
    assert!(out2.contains("Source files changed without companion test updates"));

    // Empty surfaces
    let p3 = empty_packet();
    let out3 = generate_missing_tests(&[], &p3);
    assert!(out3.contains("No changed surfaces to evaluate"));

    // Non-empty missing tests
    let mt = missing_test("t1", "login flow", true);
    let out4 = generate_missing_tests(&[mt], &p1);
    assert!(out4.contains("login flow"));
    assert!(out4.contains("**Blocking**: Yes"));
}

#[test]
fn test_review_findings_json_with_governance_entries() {
    let c = comment("src/a.rs", Some(5), true, "Bug.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c]);
    let mut packet = empty_packet();
    packet.findings = vec![ReviewFinding {
        category: FindingCategory::BoundaryCheck,
        severity: FindingSeverity::MustFix,
        title: "Boundary change".to_string(),
        details: "Public API changed.".to_string(),
        scope: ConventionalCommentScope::Surface,
        anchor: None,
        changed_surfaces: vec!["src/public/api.rs".to_string()],
    }];
    let findings = canon_engine::review::findings::build_review_findings(&canonical, &packet);
    // Code finding + governance finding
    assert_eq!(findings.len(), 2);
    let code = findings.iter().find(|f| f.kind == "code").expect("code finding");
    assert_eq!(code.github_comment_id, Some("C001".to_string()));
    let gov = findings.iter().find(|f| f.kind == "governance").expect("governance finding");
    assert!(gov.summary.contains("Boundary change"));
    assert!(gov.github_comment_id.is_none());
}

// ── Contract-enforcement tests ──────────────────────────────────────────

#[test]
fn test_github_comments_and_conventional_comments_have_matching_ids() {
    let c = comment("src/a.rs", Some(10), true, "Fix this bug.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c]);
    let gh_json = generate_github_comments_json(&canonical);
    let cc_md = generate_conventional_comments(&canonical);
    assert!(gh_json.contains("C001"));
    assert!(cc_md.contains("C001"));
    assert!(gh_json.contains("Fix this bug."));
    assert!(cc_md.contains("Fix this bug."));
}

#[test]
fn test_canonical_set_filters_out_comments_without_path_or_hunk() {
    // Governance-only: no path, no hunk
    let gov = GithubComment {
        id: String::new(),
        path: None,
        line: None,
        side: None,
        hunk_header: None,
        area: "governance".to_string(),
        kind: "issue".to_string(),
        blocking: true,
        severity: "blocking".to_string(),
        category: "scope".to_string(),
        body: "Contract-facing files changed.".to_string(),
        why_it_matters: "Governance signal.".to_string(),
        suggested_remediation: "Review.".to_string(),
        suggested_change: None,
    };
    let actionable = comment("src/lib.rs", Some(42), true, "Null pointer.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![gov, actionable]);
    assert_eq!(canonical.comments.len(), 1);
    assert_eq!(canonical.comments[0].id, "C001");
}

#[test]
fn test_canonical_comment_set_assigns_sequential_ids() {
    let c1 = comment("src/a.rs", Some(1), false, "A.");
    let c2 = comment("src/b.rs", Some(1), false, "B.");
    let c3 = comment("src/c.rs", Some(1), false, "C.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c1, c2, c3]);
    assert_eq!(canonical.comments.len(), 3);
    assert_eq!(canonical.comments[0].id, "C001");
    assert_eq!(canonical.comments[1].id, "C002");
    assert_eq!(canonical.comments[2].id, "C003");
}

#[test]
fn test_blocking_count_and_non_blocking_count() {
    let b1 = comment("src/a.rs", Some(1), true, "Blocking 1.");
    let nb = comment("src/b.rs", Some(1), false, "Non-blocking.");
    let b2 = comment("src/c.rs", Some(1), true, "Blocking 2.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![b1, nb, b2]);
    assert_eq!(canonical.blocking_count(), 2);
    assert_eq!(canonical.non_blocking_count(), 1);
}

#[test]
fn test_generate_review_report_with_non_empty_canonical_set() {
    let b1 = comment("src/a.rs", Some(1), true, "Blocking 1.");
    let mj = comment("src/b.rs", Some(2), false, "Major issue.");
    let mn = comment("src/c.rs", Some(3), false, "Minor issue.");
    let mut canonical = CanonicalCommentSet::from_evaluated(vec![b1, mj, mn]);
    canonical.reviewer_status = "actionable_review_executed".to_string();
    let changed: Vec<String> =
        vec!["src/a.rs".to_string(), "src/b.rs".to_string(), "src/c.rs".to_string()];
    let inspected: Vec<String> = vec!["src/a.rs".to_string()];
    let skipped: Vec<String> = vec![];
    let packet = empty_packet();
    let report = generate_review_report(
        &canonical,
        &Decision::RequestChanges,
        &packet,
        &changed,
        &inspected,
        &skipped,
    );
    assert!(report.contains("# PR Review Report"));
    assert!(report.contains("## Summary"));
    assert!(report.contains("## Recommendation"));
    assert!(report.contains("**Request changes**"));
    assert!(report.contains("## Recommendation Rationale"));
    assert!(report.contains("## Severity Summary"));
    assert!(report.contains("## Blocking Issues"));
    assert!(report.contains("## Major Issues"));
    assert!(report.contains("## Minor Issues"));
    assert!(report.contains("## Questions"));
    assert!(report.contains("## Nitpicks"));
    assert!(report.contains("## Review Coverage"));
    assert!(report.contains("## Governance Observations"));
    assert!(report.contains("## Decision Rules Applied"));
    assert!(report.contains("## Final Recommendation"));
}

#[test]
fn test_generate_review_report_with_approve_decision() {
    let canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let packet = empty_packet();
    let empty: Vec<String> = vec![];
    let report =
        generate_review_report(&canonical, &Decision::Approve, &packet, &empty, &empty, &empty);
    assert!(report.contains("**Approve**"));
    assert!(report.contains("No blocking findings"));
}

#[test]
fn test_generate_review_report_with_comment_decision_and_not_configured() {
    let mut canonical = CanonicalCommentSet::from_evaluated(vec![]);
    canonical.reviewer_status = "actionable_review_not_configured".to_string();
    let changed: Vec<String> = vec!["src/a.rs".to_string()];
    let empty: Vec<String> = vec![];
    let skipped: Vec<String> = vec!["src/a.rs".to_string()];
    let packet = empty_packet();
    let report =
        generate_review_report(&canonical, &Decision::Comment, &packet, &changed, &empty, &skipped);
    assert!(report.contains("**Comment**"));
    assert!(report.contains("Actionable reviewer not configured"));
}

#[test]
fn test_generate_github_comments_json_with_non_empty_set() {
    let c = comment("src/a.rs", Some(5), true, "Bug.");
    let canonical = CanonicalCommentSet::from_evaluated(vec![c]);
    let json = generate_github_comments_json(&canonical);
    assert!(json.contains("C001"));
    assert!(json.contains("src/a.rs"));
}

#[test]
fn test_generate_conventional_comments_with_empty_set() {
    let canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let out = generate_conventional_comments(&canonical);
    assert!(out.contains("## Empty Comment Set"));
    assert!(out.contains("No actionable comments were emitted"));
}

#[test]
fn test_generate_missing_tests_with_non_empty_list() {
    let mt = missing_test("MT001", "login flow", false);
    let packet = empty_packet();
    let out = generate_missing_tests(&[mt], &packet);
    assert!(out.contains("login flow"));
    assert!(out.contains("**Blocking**: No"));
}

#[test]
fn test_generate_conventional_comments_sorts_file_comments_by_path_and_severity() {
    let mut c1 = comment("src/a.rs", Some(1), false, "Minor in a.");
    c1.severity = "minor".to_string();
    let mut c2 = comment("src/b.rs", Some(1), true, "Blocking in b.");
    c2.severity = "blocking".to_string();
    let canonical = CanonicalCommentSet::from_evaluated(vec![c1, c2]);
    let out = generate_conventional_comments(&canonical);
    assert!(out.contains("## Blocking File Comments"));
    assert!(out.contains("`src/b.rs`"));
    assert!(out.contains("Blocking in b."));
    assert!(out.contains("## Minor File Comments"));
    assert!(out.contains("`src/a.rs`"));
    assert!(out.contains("Minor in a."));
}

#[test]
fn test_generate_review_report_skipped_files_section() {
    let canonical = CanonicalCommentSet::from_evaluated(vec![]);
    let mut packet = empty_packet();
    packet.findings = vec![ReviewFinding {
        category: FindingCategory::BoundaryCheck,
        severity: FindingSeverity::MustFix,
        title: "Governance issue".to_string(),
        details: "Requires attention".to_string(),
        scope: ConventionalCommentScope::Pr,
        anchor: None,
        changed_surfaces: vec!["src/boundary.rs".to_string()],
    }];
    let empty: Vec<String> = vec![];
    let report =
        generate_review_report(&canonical, &Decision::Comment, &packet, &empty, &empty, &empty);
    assert!(report.contains("Governance issue"));
    assert!(report.contains("Requires attention"));
}
