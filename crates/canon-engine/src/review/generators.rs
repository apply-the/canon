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
pub fn generate_conventional_comments(canonical: &CanonicalCommentSet) -> String {
    let mut out = String::new();

    out.push_str("# Conventional Comments\n\n");
    out.push_str(
        "> Copy-ready review comments. Every comment ID matches `github-comments.json`.\n\n",
    );

    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "{} actionable comment(s): {} blocking, {} non-blocking.\n\n",
        canonical.comments.len(),
        canonical.blocking_count(),
        canonical.non_blocking_count(),
    ));

    // ── Blocking Comments ────────────────────────────────────────────────
    let blocking: Vec<_> = canonical.comments.iter().filter(|c| c.blocking).collect();
    out.push_str("## Blocking Comments\n\n");
    if !blocking.is_empty() {
        for c in &blocking {
            append_comment_markdown(&mut out, c);
        }
    } else {
        out.push_str("- No blocking comments.\n\n");
    }

    // ── Non-Blocking Comments ────────────────────────────────────────────
    let non_blocking: Vec<_> = canonical.comments.iter().filter(|c| !c.blocking).collect();
    out.push_str("## Non-Blocking Comments\n\n");
    if !non_blocking.is_empty() {
        for c in &non_blocking {
            append_comment_markdown(&mut out, c);
        }
    } else {
        out.push_str("- No non-blocking comments.\n\n");
    }

    out
}

/// Renders a single canonical comment into conventional-comments.md format.
fn append_comment_markdown(out: &mut String, c: &GithubComment) {
    let loc = match (&c.path, c.line) {
        (Some(_p), Some(l)) => format!("Target: line {l}\n"),
        (Some(p), None) => match &c.hunk_header {
            Some(h) => format!("Target: `{p}` at `{h}`\n"),
            None => format!("Target: `{p}`\n"),
        },
        _ => String::new(),
    };

    out.push_str(&format!(
        "### {id} — `{path}`\n\n",
        id = c.id,
        path = c.path.as_deref().unwrap_or("PR")
    ));
    out.push_str(&loc);
    out.push_str(&format!(
        "\n{kind}({severity}): {body}\n\n",
        kind = c.kind,
        severity = c.severity,
        body = c.body
    ));
    out.push_str("Why it matters:\n");
    out.push_str(&format!("{}\n\n", c.why_it_matters));
    out.push_str("Suggested remediation:\n");
    out.push_str(&format!("{}\n\n", c.suggested_remediation));
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

/// Generates `review-findings.json` from findings entries.
pub fn generate_review_findings_json(findings: &[super::findings::ReviewFindingEntry]) -> String {
    serde_json::to_string_pretty(&findings).unwrap_or_default()
}
