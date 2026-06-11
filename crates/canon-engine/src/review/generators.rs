use super::evaluator::Decision;
use super::findings::{CanonicalCommentSet, GithubComment, MissingTest, ReviewPacket};

/// Generates `review-summary.md` — the primary reviewer-facing entry point.
pub fn generate_review_summary(
    canonical: &CanonicalCommentSet,
    missing_tests: &[MissingTest],
    decision: &Decision,
    packet: &ReviewPacket,
) -> String {
    let mut out = String::new();
    append_title(&mut out);
    append_summary(&mut out, packet);
    append_review_type(&mut out, canonical);
    append_decision(&mut out, decision);
    append_executive_summary(&mut out, canonical, missing_tests, decision, packet);
    append_must_fix(&mut out, canonical);
    append_accepted_risks(&mut out, canonical);
    append_missing_tests_section(&mut out, missing_tests);
    append_github_ready(&mut out, canonical);
    append_general_findings(&mut out, packet);
    append_governance_notes(&mut out, packet);
    append_severity(&mut out, canonical, missing_tests, packet);
    append_final_disposition(&mut out, decision);
    append_status_line(&mut out, decision);
    out
}

// ── Section helpers (alphabetical order) ────────────────────────────────

fn append_title(out: &mut String) {
    out.push_str("# PR Review\n\n");
}

fn append_summary(out: &mut String, packet: &ReviewPacket) {
    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "Review of {base_ref}..{head_ref} across {surface_count} changed surface(s).\n\n",
        base_ref = packet.base_ref,
        head_ref = packet.head_ref,
        surface_count = packet.changed_surfaces.len(),
    ));
}

fn append_review_type(out: &mut String, canonical: &CanonicalCommentSet) {
    out.push_str("## Review Type\n\n");
    let (review_type, readiness) = if canonical.reviewer_status == "actionable_review_executed" {
        ("actionable_pr_review", "ready_for_disposition")
    } else if canonical.reviewer_status == "governance_only"
        || canonical.reviewer_status == "actionable_review_not_configured"
    {
        ("governance_only_review", "not_ready_for_pr_approval")
    } else {
        ("partial_review", "not_ready_for_pr_approval")
    };
    out.push_str(&format!("- **Review type**: {review_type}\n"));
    out.push_str(&format!("- **Actionable review status**: {}\n", canonical.reviewer_status));
    out.push_str(&format!("- **Approval readiness**: {readiness}\n"));
    if review_type == "governance_only_review" {
        out.push_str("\n> ⚠️ This packet contains **governance findings only**. ");
        out.push_str("No actionable code review was configured or executed. ");
        out.push_str("Do NOT treat this packet as PR approval evidence.\n");
    }
    out.push('\n');
}

fn append_decision(out: &mut String, decision: &Decision) {
    out.push_str("## Decision\n\n");
    match decision {
        Decision::Approve => out.push_str("**Approve**\n\n"),
        Decision::Comment => out.push_str("**Comment**\n\n"),
        Decision::RequestChanges => out.push_str("**Request changes**\n\n"),
    }
}

fn append_executive_summary(
    out: &mut String,
    canonical: &CanonicalCommentSet,
    missing_tests: &[MissingTest],
    decision: &Decision,
    packet: &ReviewPacket,
) {
    out.push_str("## Executive Summary\n\n");
    let blocking = canonical.blocking_count();
    let non_blocking = canonical.non_blocking_count();
    let test_count = missing_tests.len();
    let governance_count = packet.findings.len();

    match decision {
        Decision::Approve => out.push_str("No blocking findings remain. "),
        Decision::RequestChanges => {
            out.push_str("Blocking findings require resolution before this PR can be approved. ")
        }
        Decision::Comment => out.push_str("Non-blocking review notes are attached. "),
    }
    out.push_str(&format!(
        "This review found {blocking} blocking comment(s), {non_blocking} non-blocking comment(s), \
         {test_count} missing-test finding(s), and {governance_count} governance observation(s).\n\n"
    ));
}

fn append_must_fix(out: &mut String, canonical: &CanonicalCommentSet) {
    out.push_str("## Must-Fix Findings\n\n");
    let blocking: Vec<_> = canonical.comments.iter().filter(|c| c.blocking).collect();
    if blocking.is_empty() {
        out.push_str("- No must-fix findings remain.\n\n");
    } else {
        for c in &blocking {
            out.push_str(&format!(
                "- **[{id}]** {loc}: {body}\n",
                id = c.id,
                loc = comment_location(c),
                body = c.body,
            ));
        }
        out.push('\n');
    }
}

fn append_accepted_risks(out: &mut String, canonical: &CanonicalCommentSet) {
    out.push_str("## Accepted Risks\n\n");
    let non_blocking: Vec<_> = canonical.comments.iter().filter(|c| !c.blocking).collect();
    if non_blocking.is_empty() {
        out.push_str("- No non-blocking findings.\n\n");
    } else {
        for c in &non_blocking {
            out.push_str(&format!(
                "- **[{id}]** {loc}: {body}\n",
                id = c.id,
                loc = comment_location(c),
                body = c.body,
            ));
        }
        out.push('\n');
    }
}

fn append_missing_tests_section(out: &mut String, missing_tests: &[MissingTest]) {
    out.push_str("## Missing Tests\n\n");
    if missing_tests.is_empty() {
        out.push_str("- No missing tests identified.\n\n");
    } else {
        for mt in missing_tests {
            let prefix = if mt.blocking { "**[BLOCKING]**" } else { "[Non-blocking]" };
            out.push_str(&format!(
                "- [{id}] {prefix} {behavior}\n  - Risk: {risk}\n  - Suggested: {shape}\n",
                id = mt.id,
                behavior = mt.affected_behavior,
                risk = mt.risk,
                shape = mt.suggested_shape,
            ));
        }
        out.push('\n');
    }
}

fn append_github_ready(out: &mut String, canonical: &CanonicalCommentSet) {
    out.push_str("## GitHub-Ready Comments\n\n");
    if canonical.comments.is_empty() {
        out.push_str("- No GitHub-ready comments.\n");
    } else {
        for c in &canonical.comments {
            out.push_str(&format!("- **{id}** — {loc}\n", id = c.id, loc = comment_location(c)));
        }
    }
    out.push('\n');
}

fn append_general_findings(out: &mut String, packet: &ReviewPacket) {
    out.push_str("## General Findings\n\n");
    let gov: Vec<_> = packet.findings.iter().filter(|f| f.changed_surfaces.is_empty()).collect();
    if gov.is_empty() {
        out.push_str("- No general findings.\n\n");
    } else {
        for f in &gov {
            out.push_str(&format!("- {}: {}\n", f.title, f.details));
        }
        out.push('\n');
    }
}

fn append_governance_notes(out: &mut String, packet: &ReviewPacket) {
    out.push_str("## Governance Notes\n\n");
    out.push_str(&format!("- Base ref: `{}`, Head ref: `{}`\n", packet.base_ref, packet.head_ref,));
    out.push_str(&format!("- Changed surfaces: {}\n", packet.changed_surfaces.len()));
    if !packet.surprising_surface_area.is_empty() {
        out.push_str(&format!(
            "- Surprising surface area: {}\n",
            packet.surprising_surface_area.join(", ")
        ));
    }
    out.push_str(&format!("- Governance findings: {}\n", packet.findings.len()));
    out.push_str("- Governance artifacts are emitted as secondary outputs.\n");
}

fn append_severity(
    out: &mut String,
    canonical: &CanonicalCommentSet,
    missing_tests: &[MissingTest],
    packet: &ReviewPacket,
) {
    let must_fix_count = canonical.blocking_count()
        + missing_tests.iter().filter(|m| m.blocking).count()
        + packet.must_fix_findings().len();
    let review_note_count = canonical.non_blocking_count()
        + missing_tests.iter().filter(|m| !m.blocking).count()
        + packet.note_findings().len();
    let has_blocking = must_fix_count > 0;

    out.push_str("\n## Severity\n\n");
    out.push_str(&format!(
        "- Overall severity: {}\n- Must-fix findings: {must_fix_count}\n- Review notes: {review_note_count}\n\n",
        severity_label(has_blocking, must_fix_count, review_note_count),
    ));
}

fn severity_label(
    has_blocking: bool,
    must_fix_count: usize,
    review_note_count: usize,
) -> &'static str {
    if has_blocking {
        "must-fix"
    } else if must_fix_count > 0 || review_note_count > 0 {
        "review-notes"
    } else {
        "none"
    }
}

fn append_final_disposition(out: &mut String, decision: &Decision) {
    out.push_str("## Final Disposition\n\n");
    let (status, rationale) = disposition_text(decision);
    out.push_str(&format!("Status: {status}\n\nRationale: {rationale}\n"));
}

fn disposition_text(decision: &Decision) -> (&'static str, &'static str) {
    match decision {
        Decision::Approve => ("ready", "Ready because no must-fix findings remain unresolved."),
        Decision::Comment => (
            "ready-with-review-notes",
            "Ready with review notes because the changed surface stays bounded and no must-fix findings remain unresolved.",
        ),
        Decision::RequestChanges => (
            "awaiting-disposition",
            "Must-fix findings require explicit disposition before readiness can pass.",
        ),
    }
}

fn append_status_line(out: &mut String, decision: &Decision) {
    let (status, _) = disposition_text(decision);
    out.push_str(&format!("\nStatus: {status}\n"));
}

/// Returns a human-readable location string for a GithubComment.
fn comment_location(c: &GithubComment) -> String {
    match (&c.path, c.line) {
        (Some(p), Some(l)) => format!("`{p}` line {l}"),
        (Some(p), None) => format!("`{p}` (hunk)"),
        _ => "PR-level".to_string(),
    }
}

/// Generates `conventional-comments.md` — human-readable, copy-ready rendering
/// of the canonical comment set.
///
/// Every comment ID matches `github-comments.json`.
/// Generates `conventional-comments.md` — template-compliant rendering.
///
/// File comments grouped by severity, sorted lexicographically by path.
/// Global comments at the end. Every ID matches `github-comments.json`.
pub fn generate_conventional_comments(canonical: &CanonicalCommentSet) -> String {
    let mut out = String::new();
    out.push_str("# Conventional Comments\n\n");
    out.push_str(
        "> Human-readable, copy-ready rendering of the canonical actionable comment set.\n",
    );
    out.push_str("> Every comment ID matches `github-comments.json`.\n\n");

    render_comment_summary(&mut out, canonical);
    render_file_comments_grouped(&mut out, canonical);
    render_global_comments(&mut out, canonical);
    render_empty_comment_set(&mut out, canonical);

    out
}

/// Renders the summary section with severity counts.
fn render_comment_summary(out: &mut String, canonical: &CanonicalCommentSet) {
    let total = canonical.comments.len();
    out.push_str("## Summary\n\n");
    out.push_str(&format!("{total} actionable comment(s):\n\n"));
    let severities = ["blocking", "major", "minor", "question", "nitpick"];
    let has_any = severities.iter().any(|sev| canonical.count_by_severity(sev) > 0);
    if has_any {
        out.push_str("| Severity | Count |\n|---|---|\n");
        for sev in &severities {
            let c = canonical.count_by_severity(sev);
            if c > 0 {
                out.push_str(&format!("| {sev} | {c} |\n"));
            }
        }
    }
    out.push('\n');
}

/// Renders file comments grouped by severity, sorted by path.
fn render_file_comments_grouped(out: &mut String, canonical: &CanonicalCommentSet) {
    let file_comments = canonical.file_comments_sorted();
    if file_comments.is_empty() {
        return;
    }
    let sev_headers = [
        ("blocking", "## Blocking File Comments"),
        ("major", "## Major File Comments"),
        ("minor", "## Minor File Comments"),
        ("question", "## Questions"),
        ("nitpick", "## Nitpicks"),
    ];
    for (sev_label, header) in &sev_headers {
        let group: Vec<_> = file_comments.iter().filter(|c| c.severity == *sev_label).collect();
        if group.is_empty() {
            continue;
        }
        out.push_str(&format!("\n{header}\n\n"));
        let mut current_path: Option<&str> = None;
        for c in &group {
            let path = c.path.as_deref().unwrap_or("PR");
            if current_path != Some(path) {
                current_path = Some(path);
                out.push_str(&format!("### `{path}`\n\n"));
            }
            append_comment_entry(out, c);
        }
    }
}

/// Renders global (no-path) comments after file comments.
fn render_global_comments(out: &mut String, canonical: &CanonicalCommentSet) {
    let global = canonical.global_comments();
    if global.is_empty() {
        return;
    }
    out.push_str("\n## Global Comments\n\n");
    for c in &global {
        append_comment_entry(out, c);
    }
}

/// Renders the empty-comment-set explanation when no comments exist.
fn render_empty_comment_set(out: &mut String, canonical: &CanonicalCommentSet) {
    if !canonical.comments.is_empty() {
        return;
    }
    let reason = empty_comment_reason(canonical);
    let is_governance_only =
        reason == "governance_only" || reason == "actionable_review_not_configured";

    out.push_str("\n## Empty Comment Set\n\n");
    if is_governance_only {
        out.push_str(
            "No actionable code review was configured or executed. \
             This packet contains governance findings only. \
             Do NOT treat this packet as PR approval evidence.\n\n",
        );
    } else {
        out.push_str("No actionable comments were emitted.\n\n");
    }
    out.push_str(&format!("Reason: {reason}\n\n"));
    out.push_str(&format!("Actionable review status: {}\n", canonical.reviewer_status));
}

fn append_comment_entry(out: &mut String, c: &GithubComment) {
    let target = match (&c.path, c.line, &c.hunk_header) {
        (_, Some(l), _) => format!("line {l}"),
        (_, _, Some(h)) => format!("hunk `{h}`"),
        _ => "whole PR".to_string(),
    };
    let prefix = if c.blocking { "blocking" } else { "non-blocking" };
    out.push_str(&format!("#### {}\n\n", c.id));
    out.push_str(&format!("Severity: {}\nTarget: {target}\n\n", c.severity));
    out.push_str(&format!("{}({prefix}): {}\n\n", c.kind, c.body));
    out.push_str("Why it matters:\n");
    out.push_str(&format!("{}\n\n", c.why_it_matters));
    out.push_str("Suggested remediation:\n");
    out.push_str(&format!("{}\n\n", c.suggested_remediation));
}

fn empty_comment_reason(canonical: &CanonicalCommentSet) -> &str {
    match canonical.reviewer_status.as_str() {
        "actionable_review_executed" => "valid_empty_actionable_review",
        "actionable_review_failed" => "actionable_review_failed",
        "actionable_review_not_configured" => "actionable_review_not_configured",
        "governance_only" => "governance_only",
        _ => "governance_only",
    }
}

/// Generates `missing-tests.md` with spec-format entries.
pub fn generate_missing_tests(missing_tests: &[MissingTest], packet: &ReviewPacket) -> String {
    let mut out = String::new();
    out.push_str("# Missing Tests\n\n");

    out.push_str("## Summary\n\n");
    if missing_tests.is_empty() {
        // Check if we can determine missing tests from governance
        let source_changed = packet.changed_surfaces.iter().any(|s| s.starts_with("src/"));
        let tests_changed = packet.changed_surfaces.iter().any(|s| s.starts_with("tests/"));
        if source_changed && !tests_changed && !packet.changed_surfaces.is_empty() {
            out.push_str("Source files changed without companion test updates. ");
            out.push_str("Manual verification of test coverage is recommended.\n\n");
        } else if packet.changed_surfaces.is_empty() {
            out.push_str("No changed surfaces to evaluate for test coverage.\n\n");
        } else {
            out.push_str("No missing test findings were identified. ");
            out.push_str("Changed surfaces include source and test updates.\n\n");
        }
    } else {
        out.push_str(&format!("{} missing test scenario(s) identified.\n\n", missing_tests.len()));
    }

    out.push_str("## Missing Tests\n\n");
    if missing_tests.is_empty() {
        out.push_str("No missing tests identified for the changed behavior.\n\n");
    } else {
        for mt in missing_tests {
            out.push_str(&format!("### {id}\n\n", id = mt.id));
            out.push_str(&format!("**Affected behavior**: {}\n\n", mt.affected_behavior));
            out.push_str(&format!("**Why it matters**: {}\n\n", mt.reason));
            out.push_str(&format!("**Risk**: {}\n\n", mt.risk));
            out.push_str(&format!("**Suggested test shape**: {}\n\n", mt.suggested_shape));
            out.push_str(&format!("**Blocking**: {}\n\n", if mt.blocking { "Yes" } else { "No" }));
        }
    }

    out
}

/// Generates `github-comments.json` from the canonical comment set.
///
/// This is equivalent to serializing the canonical set directly, but
/// provided as an explicit function for testability.
pub fn generate_github_comments_json(canonical: &CanonicalCommentSet) -> String {
    serde_json::to_string_pretty(&canonical.comments).unwrap_or_default()
}

/// Generates `review-findings.json` from a [`ReviewFindingsDocument`].
pub fn generate_review_findings_json(doc: &super::findings::ReviewFindingsDocument) -> String {
    serde_json::to_string_pretty(&doc).unwrap_or_default()
}

/// Generates `review-report.md` — template-compliant severity-oriented report.
pub fn generate_review_report(
    canonical: &CanonicalCommentSet,
    decision: &Decision,
    packet: &ReviewPacket,
    changed_files: &[String],
    files_inspected: &[String],
    files_skipped: &[String],
) -> String {
    let mut out = String::new();
    out.push_str("# PR Review Report\n\n");

    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "Severity-oriented review report. {} actionable comment(s), {} files changed.\n\n",
        canonical.comments.len(),
        changed_files.len(),
    ));

    let rec = match decision {
        Decision::Approve => "Approve",
        Decision::Comment => "Comment",
        Decision::RequestChanges => "Request changes",
    };
    out.push_str("## Recommendation\n\n");
    out.push_str(&format!("**{rec}**\n\n"));

    out.push_str("## Recommendation Rationale\n\n");
    out.push_str(&format!("{}\n\n", rationale_for(decision, canonical)));

    // Severity Summary
    out.push_str("## Severity Summary\n\n");
    out.push_str("| Severity | Count |\n|---|---|\n");
    for sev in &["blocking", "major", "minor", "question", "nitpick"] {
        out.push_str(&format!("| {sev} | {} |\n", canonical.count_by_severity(sev)));
    }

    // Issues
    for (sev, header) in &[
        ("blocking", "## Blocking Issues"),
        ("major", "## Major Issues"),
        ("minor", "## Minor Issues"),
        ("question", "## Questions"),
        ("nitpick", "## Nitpicks"),
    ] {
        out.push_str(&format!("\n{header}\n\n"));
        let items: Vec<_> = canonical.comments.iter().filter(|c| c.severity == *sev).collect();
        if items.is_empty() {
            out.push_str("- None.\n");
        } else {
            for c in &items {
                let loc = comment_location(c);
                out.push_str(&format!("- **{id}** — {loc}: {body}\n", id = c.id, body = c.body));
            }
        }
    }

    // Coverage
    out.push_str("\n## Review Coverage\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Actionable review status | {} |\n", canonical.reviewer_status));
    out.push_str(&format!("| Files changed | {} |\n", changed_files.len()));
    out.push_str(&format!("| Files inspected deeply | {} |\n", files_inspected.len()));
    out.push_str(&format!("| Files skipped | {} |\n", files_skipped.len()));

    // Governance
    out.push_str("\n## Governance Observations\n\n");
    if packet.findings.is_empty() {
        out.push_str("- No governance observations.\n");
    } else {
        for f in &packet.findings {
            out.push_str(&format!("- {}: {}\n", f.title, f.details));
        }
    }

    // Decision Rules
    out.push_str("\n## Decision Rules Applied\n\n");
    out.push_str(&format!(
        "- Reviewer status: {}\n- Blocking comments: {}\n- Governance must-fix: {}\n",
        canonical.reviewer_status,
        canonical.blocking_count(),
        packet.must_fix_findings().len(),
    ));

    // Final
    out.push_str("\n## Final Recommendation\n\n");
    out.push_str(&format!("**{rec}**\n\n"));
    out.push_str(&format!("{}\n", rationale_for(decision, canonical)));

    out
}

fn rationale_for(decision: &Decision, canonical: &CanonicalCommentSet) -> String {
    match decision {
        Decision::Approve => {
            "No blocking findings, sufficient coverage, no unresolved gates.".to_string()
        }
        Decision::Comment => {
            let reviewer_note = if canonical.reviewer_status == "governance_only" {
                "Governance-only inspection performed; no actionable code review was executed."
            } else if canonical.reviewer_status == "actionable_review_not_configured" {
                "Actionable reviewer not configured; governance-only inspection performed."
            } else {
                "Review completed with notes."
            };
            format!("{} non-blocking finding(s). {}", canonical.non_blocking_count(), reviewer_note,)
        }
        Decision::RequestChanges => format!(
            "{} blocking comment(s) require resolution before approval.",
            canonical.blocking_count(),
        ),
    }
}
