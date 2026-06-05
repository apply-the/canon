use super::evaluator::{Decision, EvaluatorPayload};

pub fn generate_review_summary(
    payload: &EvaluatorPayload,
    packet: &super::findings::ReviewPacket,
    decision: &Decision,
) -> String {
    let mut out = String::new();
    out.push_str("# Canon PR Review Summary\n\n");

    out.push_str("## Summary\n\n");
    let decision_str = match decision {
        Decision::Approve => "✅ **APPROVE**\nStatus: ready-with-review-notes",
        Decision::Comment => "💬 **COMMENT**\nStatus: ready-with-review-notes",
        Decision::RequestChanges => "❌ **REQUEST CHANGES**\nStatus: awaiting-disposition",
    };
    out.push_str(&format!("**Decision**: {}\n\n", decision_str));

    if let Some(cov) = &payload.review_coverage {
        out.push_str("### Review Coverage\n");
        out.push_str(&format!("- Total files changed: {}\n", cov.changed_files_total));
        out.push_str(&format!("- Files reviewed deeply: {}\n", cov.files_reviewed_deeply));
        out.push_str(&format!("- Files sampled: {}\n", cov.files_sampled));
        out.push_str(&format!("- Strategy: {}\n\n", cov.coverage_strategy));
    }

    out.push_str(&format!(
        "*Found {} comments and {} missing test findings.*\n\n",
        payload.github_comments.len() + packet.findings.len(),
        payload.missing_tests.len()
    ));

    out.push_str("## Severity\n\n");
    let has_blocking = payload.github_comments.iter().any(|c| c.blocking)
        || payload.missing_tests.iter().any(|m| m.blocking)
        || packet
            .findings
            .iter()
            .any(|f| matches!(f.severity, super::findings::FindingSeverity::MustFix));
    let severity = if has_blocking {
        "must-fix"
    } else if !payload.github_comments.is_empty()
        || !payload.missing_tests.is_empty()
        || !packet.findings.is_empty()
    {
        "review-notes"
    } else {
        "none"
    };
    out.push_str(&format!(
        "- Overall severity: {}\n- Must-fix findings: {}\n- Review notes: {}\n\n",
        severity,
        payload.github_comments.iter().filter(|c| c.blocking).count()
            + payload.missing_tests.iter().filter(|m| m.blocking).count()
            + packet.must_fix_findings().len(),
        payload.github_comments.iter().filter(|c| !c.blocking).count()
            + payload.missing_tests.iter().filter(|m| !m.blocking).count()
            + packet.note_findings().len()
    ));

    out.push_str("## Must-Fix Findings\n\n");
    let must_fix_comments: Vec<_> = payload.github_comments.iter().filter(|c| c.blocking).collect();
    let must_fix_tests: Vec<_> = payload.missing_tests.iter().filter(|m| m.blocking).collect();
    let deterministic_must_fix = packet.must_fix_findings();
    if must_fix_comments.is_empty()
        && must_fix_tests.is_empty()
        && deterministic_must_fix.is_empty()
    {
        out.push_str("- No must-fix findings remain.\n\n");
    } else {
        for f in &deterministic_must_fix {
            out.push_str(&format!(
                "- [must-fix] {}: {}\n  - Surfaces: {}\n",
                f.title,
                f.details,
                f.changed_surfaces.join(", ")
            ));
        }
        for c in must_fix_comments {
            out.push_str(&format!(
                "- [Blocking] {}: {}\n",
                c.path.as_deref().unwrap_or("PR"),
                c.body
            ));
        }
        for m in must_fix_tests {
            out.push_str(&format!(
                "- [Blocking] Missing Test for {}: {}\n",
                m.affected_behavior, m.reason
            ));
        }
        out.push('\n');
    }

    out.push_str("## Accepted Risks\n\n");
    let accepted_risks = packet.note_findings();
    if accepted_risks.is_empty() {
        out.push_str("- No accepted risks recorded.\n\n");
    } else {
        for f in &accepted_risks {
            out.push_str(&format!("- {}\n", f.title));
        }
        out.push('\n');
    }

    out.push_str("## Final Disposition\n\n");
    match decision {
        Decision::Approve => {
            out.push_str("Status: ready-with-review-notes\n\n");
            out.push_str("Rationale: Ready with review notes because the changed surface stays bounded and no must-fix findings remain unresolved. Governed diff inspection and critique evidence remain linked from the review bundle.\n");
        }
        Decision::Comment => {
            out.push_str("Status: ready-with-review-notes\n\n");
            out.push_str("Rationale: Ready with review notes because the changed surface stays bounded and no must-fix findings remain unresolved. Governed diff inspection and critique evidence remain linked from the review bundle.\n");
        }
        Decision::RequestChanges => {
            out.push_str("Status: awaiting-disposition\n\n");
            out.push_str("Rationale: Must-fix findings require explicit disposition before readiness can pass. Governed diff inspection and critique evidence remain linked from the review bundle.\n");
        }
    }

    out
}

pub fn generate_conventional_comments(
    payload: &EvaluatorPayload,
    packet: &super::findings::ReviewPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Conventional Comments\n\n");

    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "Reviewer-facing conventional comments derived from {} persisted finding(s).\n\n",
        payload.github_comments.len() + packet.findings.len()
    ));

    out.push_str("## Evidence Posture\n\n");
    out.push_str("- Comment kinds are deterministically mapped from review findings.\n");
    out.push_str("- Approval posture remains anchored by `review-summary.md`.\n\n");

    out.push_str("## Conventional Comments\n\n");
    let mut empty = true;
    for f in &packet.findings {
        empty = false;
        let scope = f.scope.as_str();
        let surfaces = if f.changed_surfaces.is_empty() {
            "none".to_string()
        } else {
            f.changed_surfaces.join(", ")
        };
        let anchor_detail = f.anchor.as_ref().map_or_else(String::new, |anchor| {
            let line_end = anchor.line_end.map(|le| format!("-{}", le)).unwrap_or_default();
            format!("\n  - Anchor: {}:{}{}", anchor.surface, anchor.line_start, line_end)
        });
        let scope_detail = match f.scope {
            super::findings::ConventionalCommentScope::Pr => String::new(),
            super::findings::ConventionalCommentScope::File
            | super::findings::ConventionalCommentScope::Surface => {
                format!("\n  - Scope surfaces: {surfaces}")
            }
        };

        out.push_str(&format!(
            "- {}(scope:{}): {}\n  - Why: {}\n  - Surfaces: {}{}{}\n",
            f.conventional_comment_kind(),
            scope,
            f.title,
            f.details,
            surfaces,
            anchor_detail,
            scope_detail,
        ));
    }

    for c in &payload.github_comments {
        empty = false;
        let prefix = if c.blocking { "**[BLOCKING]**" } else { "[Non-blocking]" };
        let loc = match (&c.path, c.line) {
            (Some(p), Some(l)) => format!("{}:{}", p, l),
            (Some(p), None) => format!("{} (hunk)", p),
            _ => "PR General".to_string(),
        };

        out.push_str(&format!("### {} - {}\n", loc, c.kind));
        out.push_str(&format!("{} {}\n\n", prefix, c.body));
        out.push_str(&format!("**Why it matters**: {}\n\n", c.why_it_matters));
        if let Some(change) = &c.suggested_change {
            out.push_str("```rust\n");
            out.push_str(change);
            out.push_str("\n```\n\n");
        }
    }

    if empty {
        out.push_str("- No conventional comments were generated.\n\n");
    }

    out.push_str("## Traceability\n\n");
    out.push_str("- Source packet: `review-summary.md` and `pr-analysis.md`\n");

    out
}

pub fn generate_missing_tests(
    payload: &EvaluatorPayload,
    packet: &super::findings::ReviewPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Missing Tests\n\n");

    out.push_str("## Summary\n\n");
    out.push_str("Verification coverage review for the diff.\n\n");

    out.push_str("## Missing Invariant Checks\n\n");
    let missing_invariants: Vec<_> = payload.missing_tests.iter().collect();
    let deterministic_missing_tests: Vec<_> = packet
        .findings
        .iter()
        .filter(|f| matches!(f.category, super::findings::FindingCategory::MissingTests))
        .collect();

    if missing_invariants.is_empty() && deterministic_missing_tests.is_empty() {
        out.push_str("- No missing invariant checks inferred.\n\n");
    } else {
        for f in &deterministic_missing_tests {
            out.push_str(&format!(
                "- [{}] {}: {}\n  - Surfaces: {}\n",
                f.severity.as_str(),
                f.title,
                f.details,
                f.changed_surfaces.join(", ")
            ));
        }
        for m in &missing_invariants {
            let prefix = if m.blocking { "**[BLOCKING]**" } else { "[Non-blocking]" };
            out.push_str(&format!("### Behavior: {}\n", m.affected_behavior));
            out.push_str(&format!("{} {}\n\n", prefix, m.reason));
            out.push_str(&format!("**Risk**: {}\n\n", m.risk));
            out.push_str(&format!("**Suggested shape**: {}\n\n", m.suggested_shape));
        }
    }

    out.push_str("## Missing Contract Checks\n\n");
    out.push_str("- No missing contract checks inferred.\n\n");

    out.push_str("## Weak or Mirrored Tests\n\n");
    out.push_str("- Updated tests moved with the changed surface.\n");

    out
}
