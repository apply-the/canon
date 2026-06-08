use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

/// Classifies the structural concern behind a [`ReviewFinding`].
///
/// Each variant maps to a distinct review lens applied during diff inspection:
/// boundary ownership, code duplication, contract compatibility, test coverage,
/// and decision traceability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FindingCategory {
    /// A public or boundary-marked surface changed and needs explicit reviewer disposition.
    BoundaryCheck,
    /// Duplicate behavior or canonical ownership conflicts were detected.
    DuplicationCheck,
    /// A contract or API surface drifted without an explicit acceptance record.
    ContractDrift,
    /// Source files changed without companion test-surface updates.
    MissingTests,
    /// Structural changes imply decisions that are not yet explicitly accepted.
    DecisionImpact,
}

impl FindingCategory {
    /// Returns the kebab-case string representation of the category.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Indicates the urgency of a [`ReviewFinding`].
///
/// `MustFix` findings block readiness until the reviewer records an explicit
/// disposition. `Note` findings are informational and do not gate the review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FindingSeverity {
    /// Informational — does not block reviewer disposition.
    Note,
    /// Requires explicit reviewer disposition before the review can pass.
    MustFix,
}

impl FindingSeverity {
    /// Returns the kebab-case string representation of the severity.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// Scope at which a [`ReviewFinding`] applies within the reviewed diff.
///
/// Scope is derived deterministically from the finding's `changed_surfaces`:
/// - [`Pr`](ConventionalCommentScope::Pr): no surfaces, applies at the PR level.
/// - [`Surface`](ConventionalCommentScope::Surface): all surfaces belong to one functional group.
/// - [`File`](ConventionalCommentScope::File): surfaces span multiple functional groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ConventionalCommentScope {
    /// The comment applies to the entire pull request.
    Pr,
    /// The comment applies to a specific file.
    File,
    /// The comment applies to a specific surface within a file.
    Surface,
}

impl ConventionalCommentScope {
    /// Returns the kebab-case string representation of the scope.
    pub fn as_str(self) -> &'static str {
        self.into()
    }

    /// Returns all variants in declaration order.
    pub fn all() -> &'static [Self] {
        &[Self::Pr, Self::File, Self::Surface]
    }
}

/// Optional inline anchor for a [`ReviewFinding`].
///
/// Anchors are emitted only when the persisted diff evidence resolves to one
/// changed surface and one contiguous added-line interval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnchor {
    /// Repo-relative changed surface that owns the anchor.
    pub surface: String,
    /// Inclusive 1-based starting line in the target surface.
    pub line_start: usize,
    /// Inclusive 1-based ending line when the anchor spans multiple lines.
    pub line_end: Option<usize>,
}

impl ReviewAnchor {
    fn from_interval(surface: String, interval: PatchInterval) -> Self {
        let line_end =
            if interval.line_start == interval.line_end { None } else { Some(interval.line_end) };

        Self { surface, line_start: interval.line_start, line_end }
    }
}

/// A single bounded finding produced during diff inspection.
///
/// Findings are collected into a [`ReviewPacket`] and rendered into the
/// pr-review artifact bundle. Each finding carries enough context to map
/// directly to a Conventional Comment entry without additional inference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFinding {
    /// The structural lens that produced this finding.
    pub category: FindingCategory,
    /// Whether this finding must be explicitly resolved before the review can pass.
    pub severity: FindingSeverity,
    /// Short human-readable label used as the Conventional Comment subject.
    pub title: String,
    /// Expanded rationale surfaced in the `Why:` line of the comment entry.
    pub details: String,
    /// Scope of this finding, derived deterministically from `changed_surfaces`.
    pub scope: ConventionalCommentScope,
    /// Optional inline anchor when the diff evidence supports one precise interval.
    pub anchor: Option<ReviewAnchor>,
    /// The diff surfaces this finding is anchored to.
    pub changed_surfaces: Vec<String>,
}

impl ReviewFinding {
    /// Maps `(severity, category)` to the Conventional Comments kind string.
    ///
    /// The mapping is intentionally static: callers must not derive the kind
    /// independently to avoid drift between the domain model and the renderer.
    pub fn conventional_comment_kind(&self) -> &'static str {
        match (self.severity, self.category) {
            (FindingSeverity::MustFix, FindingCategory::BoundaryCheck) => "issue",
            (FindingSeverity::MustFix, FindingCategory::ContractDrift) => "issue",
            (FindingSeverity::MustFix, FindingCategory::MissingTests) => "todo",
            (FindingSeverity::MustFix, FindingCategory::DecisionImpact) => "question",
            (FindingSeverity::Note, FindingCategory::DuplicationCheck) => "praise",
            (FindingSeverity::Note, _) => "thought",
            (FindingSeverity::MustFix, _) => "issue",
        }
    }
}

/// The complete review context derived from a bounded diff.
///
/// A `ReviewPacket` is the central input to all pr-review artifact renderers.
/// It is built from the diff between two refs and carries the full set of
/// [`ReviewFinding`]s produced by deterministic diff inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPacket {
    /// The base ref the diff is compared against (e.g. `origin/main`).
    pub base_ref: String,
    /// The head ref being reviewed (e.g. `HEAD` or `WORKTREE`).
    pub head_ref: String,
    /// All changed surfaces detected in the diff between `base_ref` and `head_ref`.
    pub changed_surfaces: Vec<String>,
    /// A one-line summary of the inferred review intent, used as artifact preamble.
    pub inferred_intent: String,
    /// Surfaces that are boundary- or contract-marked and therefore warrant extra scrutiny.
    pub surprising_surface_area: Vec<String>,
    /// All findings produced by diff inspection, in insertion order.
    pub findings: Vec<ReviewFinding>,
}

impl ReviewPacket {
    /// Builds a [`ReviewPacket`] from a raw diff between two refs.
    ///
    /// Runs deterministic diff inspection over `changed_surfaces` and `patch`
    /// to produce the initial finding set. No critique evidence is applied;
    /// use [`from_evidence`](Self::from_evidence) to include governed critique.
    pub fn from_diff(
        base_ref: &str,
        head_ref: &str,
        changed_surfaces: Vec<String>,
        patch: &str,
    ) -> Self {
        let surprising_surface_area = changed_surfaces
            .iter()
            .filter(|surface| is_contract_surface(surface) || is_boundary_surface(surface))
            .cloned()
            .collect::<Vec<_>>();
        let source_changed = changed_surfaces.iter().any(|surface| is_source_surface(surface));
        let tests_changed = changed_surfaces.iter().any(|surface| is_test_surface(surface));

        let inferred_intent = if changed_surfaces.is_empty() {
            "No changed surfaces were detected between the supplied refs.".to_string()
        } else {
            format!(
                "Review the bounded change across {} changed surface(s) and {} diff line(s) between {base_ref} and {head_ref}.",
                changed_surfaces.len(),
                patch.lines().count()
            )
        };

        let mut findings = Vec::new();

        if changed_surfaces.iter().any(|surface| is_boundary_surface(surface)) {
            let boundary_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_boundary_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&boundary_surfaces);
            let anchor = derive_anchor(&boundary_surfaces, patch);
            findings.push(ReviewFinding {
                category: FindingCategory::BoundaryCheck,
                severity: FindingSeverity::MustFix,
                title: "Boundary-marked surfaces changed".to_string(),
                details: "Public or boundary-marked files changed and require explicit reviewer disposition.".to_string(),
                scope,
                anchor,
                changed_surfaces: boundary_surfaces,
            });
        }

        if changed_surfaces.iter().any(|surface| is_contract_surface(surface)) {
            let contract_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_contract_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&contract_surfaces);
            let anchor = derive_anchor(&contract_surfaces, patch);
            findings.push(ReviewFinding {
                category: FindingCategory::ContractDrift,
                severity: FindingSeverity::MustFix,
                title: "Contract-facing files changed".to_string(),
                details: "Contract or API surfaces drifted and need explicit acceptance before readiness can pass.".to_string(),
                scope,
                anchor,
                changed_surfaces: contract_surfaces,
            });
        }

        if source_changed && !tests_changed {
            let source_surfaces: Vec<String> = changed_surfaces
                .iter()
                .filter(|surface| is_source_surface(surface))
                .cloned()
                .collect();
            let scope = derive_scope(&source_surfaces);
            let anchor = derive_anchor(&source_surfaces, patch);
            findings.push(ReviewFinding {
                category: FindingCategory::MissingTests,
                severity: FindingSeverity::MustFix,
                title: "Source changes lack companion verification updates".to_string(),
                details: "Changed source files do not have adjacent test-surface updates in the reviewed diff.".to_string(),
                scope,
                anchor,
                changed_surfaces: source_surfaces,
            });
        }

        if !surprising_surface_area.is_empty() {
            let scope = derive_scope(&surprising_surface_area);
            let anchor = derive_anchor(&surprising_surface_area, patch);
            findings.push(ReviewFinding {
                category: FindingCategory::DecisionImpact,
                severity: FindingSeverity::MustFix,
                title: "High-impact surfaces imply hidden decisions".to_string(),
                details: "Boundary or contract changes imply architectural consequences that need an explicit reviewer disposition.".to_string(),
                scope,
                anchor,
                changed_surfaces: surprising_surface_area.clone(),
            });
        }

        if findings.is_empty() {
            let scope = derive_scope(&changed_surfaces);
            let anchor = derive_anchor(&changed_surfaces, patch);
            findings.push(ReviewFinding {
                category: FindingCategory::DuplicationCheck,
                severity: FindingSeverity::Note,
                title: "No material duplication concerns inferred".to_string(),
                details:
                    "The diff remains bounded and changed tests moved with the source surface."
                        .to_string(),
                scope,
                anchor,
                changed_surfaces: changed_surfaces.clone(),
            });
        }

        Self {
            base_ref: base_ref.to_string(),
            head_ref: head_ref.to_string(),
            changed_surfaces,
            inferred_intent,
            surprising_surface_area,
            findings,
        }
    }

    /// Builds a [`ReviewPacket`] and appends governed critique evidence.
    ///
    /// Delegates to [`from_diff`](Self::from_diff) for the initial finding set,
    /// then appends `critique_summary` to `inferred_intent` when non-empty.
    pub fn from_evidence(
        base_ref: &str,
        head_ref: &str,
        changed_surfaces: Vec<String>,
        patch: &str,
        critique_summary: &str,
    ) -> Self {
        let mut packet = Self::from_diff(base_ref, head_ref, changed_surfaces, patch);

        if !critique_summary.trim().is_empty() {
            packet.inferred_intent = format!(
                "{}\n\nGoverned critique evidence: {}",
                packet.inferred_intent, critique_summary
            );
        }

        packet
    }

    /// Returns all findings whose [`FindingCategory`] matches `category`.
    pub fn findings_for(&self, category: FindingCategory) -> Vec<&ReviewFinding> {
        self.findings.iter().filter(|finding| finding.category == category).collect()
    }

    /// Returns all findings with [`FindingSeverity::MustFix`] severity.
    pub fn must_fix_findings(&self) -> Vec<&ReviewFinding> {
        self.findings
            .iter()
            .filter(|finding| matches!(finding.severity, FindingSeverity::MustFix))
            .collect()
    }

    /// Returns all findings with [`FindingSeverity::Note`] severity.
    pub fn note_findings(&self) -> Vec<&ReviewFinding> {
        self.findings
            .iter()
            .filter(|finding| matches!(finding.severity, FindingSeverity::Note))
            .collect()
    }
}

fn is_source_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.starts_with("src/") || normalized.contains("/src/")
}

fn is_test_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.starts_with("tests/")
        || normalized.contains("/tests/")
        || normalized.ends_with("_test.rs")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with(".snap")
}

fn is_contract_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.contains("contract") || normalized.contains("api") || normalized.contains("schema")
}

fn is_boundary_surface(surface: &str) -> bool {
    let normalized = surface.to_ascii_lowercase();
    normalized.contains("boundary")
        || normalized.contains("public")
        || normalized.contains("interface")
}

/// Derives a [`ConventionalCommentScope`] from a set of changed surfaces.
///
/// - Returns [`Pr`](ConventionalCommentScope::Pr) when no surfaces are present.
/// - Returns [`Surface`](ConventionalCommentScope::Surface) when every surface
///   belongs to the same functional group (test, source, contract, or boundary).
/// - Returns [`File`](ConventionalCommentScope::File) in all other cases.
fn derive_scope(changed_surfaces: &[String]) -> ConventionalCommentScope {
    if changed_surfaces.is_empty() {
        return ConventionalCommentScope::Pr;
    }
    let all_test = changed_surfaces.iter().all(|s| is_test_surface(s));
    let all_source = changed_surfaces.iter().all(|s| is_source_surface(s));
    let all_contract = changed_surfaces.iter().all(|s| is_contract_surface(s));
    let all_boundary = changed_surfaces.iter().all(|s| is_boundary_surface(s));
    if all_test || all_source || all_contract || all_boundary {
        ConventionalCommentScope::Surface
    } else {
        ConventionalCommentScope::File
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PatchInterval {
    line_start: usize,
    line_end: usize,
}

const PATCH_TARGET_PREFIX: &str = "+++ b/";
const PATCH_NULL_TARGET: &str = "/dev/null";
const PATCH_HUNK_PREFIX: &str = "@@";

fn derive_anchor(changed_surfaces: &[String], patch: &str) -> Option<ReviewAnchor> {
    let surface = single_anchor_surface(changed_surfaces)?;
    let intervals = patch_intervals_for_surface(patch, surface);
    let [interval] = intervals.as_slice() else {
        return None;
    };

    Some(ReviewAnchor::from_interval(surface.to_string(), *interval))
}

fn single_anchor_surface(changed_surfaces: &[String]) -> Option<&str> {
    let [surface] = changed_surfaces else {
        return None;
    };
    let surface = surface.trim();
    if surface.is_empty() { None } else { Some(surface) }
}

fn patch_intervals_for_surface(patch: &str, target_surface: &str) -> Vec<PatchInterval> {
    let mut current_surface: Option<&str> = None;
    let mut intervals = Vec::new();

    for line in patch.lines() {
        if let Some(surface) = line.strip_prefix(PATCH_TARGET_PREFIX) {
            current_surface = if surface == PATCH_NULL_TARGET { None } else { Some(surface) };
            continue;
        }

        if line.starts_with(PATCH_HUNK_PREFIX)
            && current_surface.is_some_and(|surface| surface == target_surface)
            && let Some(interval) = parse_patch_interval(line)
        {
            push_patch_interval(&mut intervals, interval);
        }
    }

    intervals
}

fn parse_patch_interval(hunk_header: &str) -> Option<PatchInterval> {
    let added_range = hunk_header.split_whitespace().find(|part| part.starts_with('+'))?;
    let added_range = added_range.strip_prefix('+')?;
    let (line_start, line_count) = parse_patch_range(added_range)?;
    if line_count == 0 {
        return None;
    }
    let line_end = line_start.checked_add(line_count.checked_sub(1)?)?;
    Some(PatchInterval { line_start, line_end })
}

fn parse_patch_range(range: &str) -> Option<(usize, usize)> {
    let (line_start, line_count) = match range.split_once(',') {
        Some((line_start, line_count)) => (line_start, line_count),
        None => (range, "1"),
    };

    let line_start = line_start.parse::<usize>().ok()?;
    let line_count = line_count.parse::<usize>().ok()?;
    Some((line_start, line_count))
}

fn push_patch_interval(intervals: &mut Vec<PatchInterval>, next: PatchInterval) {
    if let Some(last) = intervals.last_mut() {
        let extends_last =
            last.line_end.checked_add(1).is_some_and(|adjacent| next.line_start <= adjacent)
                || next.line_start <= last.line_end;
        if extends_last {
            if next.line_end > last.line_end {
                last.line_end = next.line_end;
            }
            return;
        }
    }

    intervals.push(next);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GithubComment {
    pub id: String,
    pub path: Option<String>,
    pub line: Option<u32>,
    pub side: Option<String>,
    pub hunk_header: Option<String>,
    pub area: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub blocking: bool,
    pub severity: String,
    pub category: String,
    pub body: String,
    pub why_it_matters: String,
    pub suggested_remediation: String,
    pub suggested_change: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingTest {
    pub id: String,
    pub affected_behavior: String,
    pub reason: String,
    pub risk: String,
    pub suggested_shape: String,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCoverage {
    pub changed_files_total: u32,
    pub files_reviewed_deeply: u32,
    pub files_sampled: u32,
    pub files_not_reviewed_deeply: u32,
    pub coverage_strategy: String,
    pub unreviewed_risk: String,
}

/// A normalized review finding emitted in `review-findings.json`.
///
/// Findings may be linked to a GitHub comment via `github_comment_id`.
/// Governance-only findings omit the comment link and are marked with
/// `kind = "governance"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFindingEntry {
    /// Stable finding ID (F001, F002, ...).
    pub id: String,
    /// Kind of finding: `code`, `governance`, `test`, or `coverage`.
    pub kind: String,
    /// Changed file path when applicable.
    pub path: Option<String>,
    /// Line number when applicable.
    pub line: Option<u32>,
    /// Diff hunk header when exact line cannot be determined.
    pub hunk_header: Option<String>,
    /// Severity: `blocking` or `non-blocking`.
    pub severity: String,
    /// Category label (e.g. `contract-compliance`, `bug`, `missing-test`).
    pub category: String,
    /// One-line summary of the finding.
    pub summary: String,
    /// What the spec or contract requires.
    pub expected_behavior: String,
    /// What was observed in the implementation.
    pub observed_behavior: String,
    /// Evidence references supporting this finding.
    pub evidence: Vec<String>,
    /// Recommended action to resolve the finding.
    pub recommended_action: String,
    /// Link to the corresponding GitHub comment ID (C001, ...), when applicable.
    pub github_comment_id: Option<String>,
}

/// A canonical set of actionable review comments shared between
/// `github-comments.json` and `conventional-comments.md`.
///
/// Every comment carries a stable `C`-prefixed ID that appears in both
/// renderings. Governance-only signals MUST NOT be included.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCommentSet {
    /// Actionable review comments with stable IDs.
    pub comments: Vec<GithubComment>,
    /// Status of the actionable reviewer adapter.
    pub reviewer_status: String,
}

impl CanonicalCommentSet {
    /// Builds a canonical comment set from evaluated GitHub comments.
    pub fn from_evaluated(comments: Vec<GithubComment>) -> Self {
        let actionable: Vec<GithubComment> = comments
            .into_iter()
            .filter(|c| c.path.is_some() || c.hunk_header.is_some())
            .enumerate()
            .map(|(i, mut c)| {
                c.id = format!("C{:03}", i + 1);
                c
            })
            .collect();
        Self {
            comments: actionable,
            reviewer_status: "actionable_review_not_configured".to_string(),
        }
    }

    /// Builds from reviewer adapter output, converting findings to comments.
    pub fn from_reviewer(
        findings: Vec<canon_adapters::reviewer::ReviewerFinding>,
        status: &str,
    ) -> Self {
        let comments: Vec<GithubComment> = findings
            .into_iter()
            .enumerate()
            .map(|(i, f)| {
                let severity_str = f.severity.as_str().to_string();
                GithubComment {
                    id: format!("C{:03}", i + 1),
                    path: f.path,
                    line: f.line,
                    side: f.side,
                    hunk_header: f.hunk_header,
                    area: String::new(),
                    kind: f.kind,
                    blocking: matches!(
                        f.severity,
                        canon_adapters::reviewer::ReviewerSeverity::Blocking
                    ),
                    severity: severity_str,
                    category: String::new(),
                    body: f.summary,
                    why_it_matters: f.why_it_matters,
                    suggested_remediation: f.suggested_remediation,
                    suggested_change: f.suggested_change,
                }
            })
            .collect();
        Self { comments, reviewer_status: status.to_string() }
    }

    /// Returns the number of blocking comments.
    pub fn blocking_count(&self) -> usize {
        self.comments.iter().filter(|c| c.blocking).count()
    }

    /// Returns the number of non-blocking comments.
    pub fn non_blocking_count(&self) -> usize {
        self.comments.iter().filter(|c| !c.blocking).count()
    }

    /// Count comments by severity string.
    pub fn count_by_severity(&self, severity: &str) -> usize {
        self.comments.iter().filter(|c| c.severity == severity).count()
    }

    /// Returns file-level comments sorted lexicographically by path, then severity, then line.
    pub fn file_comments_sorted(&self) -> Vec<&GithubComment> {
        let mut file_comments: Vec<&GithubComment> =
            self.comments.iter().filter(|c| c.path.is_some()).collect();
        file_comments.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then_with(|| severity_order(&a.severity).cmp(&severity_order(&b.severity)))
                .then_with(|| a.line.cmp(&b.line))
        });
        file_comments
    }

    /// Returns global comments (no path).
    pub fn global_comments(&self) -> Vec<&GithubComment> {
        self.comments.iter().filter(|c| c.path.is_none()).collect()
    }
}

fn severity_order(severity: &str) -> u8 {
    match severity {
        "blocking" => 0,
        "major" => 1,
        "minor" => 2,
        "question" => 3,
        "nitpick" => 4,
        _ => 5,
    }
}

/// Builds normalized review findings from canonical comments and governance context.
///
/// Each actionable GitHub comment produces a corresponding finding with
/// `kind = "code"`. Governance-only signals from the review packet are emitted
/// as separate findings with `kind = "governance"`.
pub fn build_review_findings(
    canonical: &CanonicalCommentSet,
    packet: &ReviewPacket,
) -> Vec<ReviewFindingEntry> {
    let mut findings = Vec::new();

    // Map canonical comments to code findings
    for comment in &canonical.comments {
        let expected =
            "Implementation must satisfy the spec contract for this behavior.".to_string();
        let observed = format!(
            "{} comment at {}: {}",
            comment.severity,
            comment.path.as_deref().unwrap_or(comment.hunk_header.as_deref().unwrap_or("PR")),
            comment.body
        );
        findings.push(ReviewFindingEntry {
            id: format!("F{:03}", findings.len() + 1),
            kind: "code".to_string(),
            path: comment.path.clone(),
            line: comment.line,
            hunk_header: comment.hunk_header.clone(),
            severity: if comment.blocking {
                "blocking".to_string()
            } else {
                "non-blocking".to_string()
            },
            category: comment.category.clone(),
            summary: comment.body.clone(),
            expected_behavior: expected,
            observed_behavior: observed,
            evidence: vec![format!("github-comment:{}", comment.id)],
            recommended_action: comment.suggested_remediation.clone(),
            github_comment_id: Some(comment.id.clone()),
        });
    }

    // Map governance packet findings to governance findings
    for finding in &packet.findings {
        let severity = if matches!(finding.severity, FindingSeverity::MustFix) {
            "blocking"
        } else {
            "non-blocking"
        };
        findings.push(ReviewFindingEntry {
            id: format!("F{:03}", findings.len() + 1),
            kind: "governance".to_string(),
            path: finding.changed_surfaces.first().cloned(),
            line: finding.anchor.as_ref().map(|a| a.line_start as u32),
            hunk_header: None,
            severity: severity.to_string(),
            category: finding.category.as_str().to_string(),
            summary: finding.title.clone(),
            expected_behavior: "Changed surfaces must have explicit reviewer disposition."
                .to_string(),
            observed_behavior: finding.details.clone(),
            evidence: finding.changed_surfaces.clone(),
            recommended_action: "Review the changed surfaces and record an explicit disposition."
                .to_string(),
            github_comment_id: None,
        });
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::{
        ConventionalCommentScope, FindingCategory, FindingSeverity, ReviewAnchor, ReviewFinding,
        ReviewPacket, derive_anchor,
    };

    #[test]
    fn from_diff_emits_expected_must_fix_findings_for_boundary_and_contract_changes() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec![
                "src/boundary/router.rs".to_string(),
                "contracts/schema.json".to_string(),
                "src/service.rs".to_string(),
            ],
            "@@ -1,2 +1,3 @@\n-old\n+new\n",
        );

        assert_eq!(packet.must_fix_findings().len(), 4);
        assert!(packet.note_findings().is_empty());
        assert_eq!(packet.findings_for(FindingCategory::BoundaryCheck).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::ContractDrift).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::MissingTests).len(), 1);
        assert_eq!(packet.findings_for(FindingCategory::DecisionImpact).len(), 1);
        assert!(packet.surprising_surface_area.contains(&"src/boundary/router.rs".to_string()));
        assert!(packet.surprising_surface_area.contains(&"contracts/schema.json".to_string()));
    }

    #[test]
    fn from_diff_falls_back_to_note_when_tests_move_with_source() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );

        assert!(packet.must_fix_findings().is_empty());
        let notes = packet.note_findings();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].category, FindingCategory::DuplicationCheck);
        assert_eq!(notes[0].severity, FindingSeverity::Note);
    }

    #[test]
    fn from_evidence_appends_critique_summary_to_intent() {
        let packet = ReviewPacket::from_evidence(
            "origin/main",
            "HEAD",
            vec!["tests/lib_test.rs".to_string()],
            "@@ -0,0 +1 @@\n+test\n",
            "Independent critique highlighted missing rollback notes.",
        );

        assert!(packet.inferred_intent.contains(
            "Governed critique evidence: Independent critique highlighted missing rollback notes."
        ));
    }

    #[test]
    fn pr_review_conventional_comment_mapping_matches_first_slice_table() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec![
                "src/boundary/router.rs".to_string(),
                "contracts/schema.json".to_string(),
                "src/service.rs".to_string(),
            ],
            "@@ -1,2 +1,3 @@\n-old\n+new\n",
        );

        let rendered_expectations = [
            (FindingCategory::BoundaryCheck, FindingSeverity::MustFix, "issue"),
            (FindingCategory::ContractDrift, FindingSeverity::MustFix, "issue"),
            (FindingCategory::MissingTests, FindingSeverity::MustFix, "todo"),
            (FindingCategory::DecisionImpact, FindingSeverity::MustFix, "question"),
        ];

        for (category, severity, expected_kind) in rendered_expectations {
            let finding = packet
                .findings
                .iter()
                .find(|finding| finding.category == category && finding.severity == severity)
                .expect("expected review finding");
            assert_eq!(
                finding.conventional_comment_kind(),
                expected_kind,
                "unexpected comment kind for {category:?}"
            );
        }

        let note_packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );
        let note = note_packet.note_findings().pop().expect("note finding");
        assert_eq!(note.conventional_comment_kind(), "praise");
    }

    #[test]
    fn conventional_comment_scope_serde_round_trip() {
        use super::ConventionalCommentScope;
        let cases = [
            (ConventionalCommentScope::Pr, "\"pr\""),
            (ConventionalCommentScope::File, "\"file\""),
            (ConventionalCommentScope::Surface, "\"surface\""),
        ];
        for (variant, expected_json) in cases {
            let serialized = serde_json::to_string(&variant).expect("serialized");
            assert_eq!(serialized, expected_json, "variant {variant:?}");
            let deserialized: ConventionalCommentScope =
                serde_json::from_str(&serialized).expect("deserialized");
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn conventional_comment_scope_as_str() {
        use super::ConventionalCommentScope;
        assert_eq!(ConventionalCommentScope::Pr.as_str(), "pr");
        assert_eq!(ConventionalCommentScope::File.as_str(), "file");
        assert_eq!(ConventionalCommentScope::Surface.as_str(), "surface");
    }

    #[test]
    fn scope_pr_when_no_surfaces() {
        // derive_scope returns Pr when no surfaces are present.
        use super::{ConventionalCommentScope, derive_scope};
        assert_eq!(derive_scope(&[]), ConventionalCommentScope::Pr);
        // An empty-surface diff still produces the DuplicationCheck note finding
        // (from the `if findings.is_empty()` fallback), with File scope because
        // changed_surfaces is empty and derive_scope yields Pr.
        let packet = ReviewPacket::from_diff("main", "HEAD", vec![], "");
        // The note finding uses derive_scope on the empty changed_surfaces clone,
        // yielding Pr scope.
        if let Some(note) = packet.findings.first() {
            assert_eq!(note.scope, ConventionalCommentScope::Pr);
        }
    }

    #[test]
    fn scope_surface_when_all_test_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["tests/foo_test.rs".to_string(), "tests/bar_test.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_surface_when_all_contract_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["contracts/api.json".to_string(), "contracts/schema.json".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_surface_when_all_source_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "src/main.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::Surface);
    }

    #[test]
    fn scope_file_when_mixed_surfaces() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::File);
    }

    #[test]
    fn scope_file_when_non_empty_but_no_single_group() {
        use super::{ConventionalCommentScope, derive_scope};
        let surfaces = vec!["src/lib.rs".to_string(), "contracts/api.json".to_string()];
        assert_eq!(derive_scope(&surfaces), ConventionalCommentScope::File);
    }

    #[test]
    fn from_diff_boundary_finding_has_surface_scope() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/public/api.rs".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let finding = packet
            .findings
            .iter()
            .find(|f| f.category == FindingCategory::BoundaryCheck)
            .expect("boundary finding");
        assert_eq!(finding.scope, ConventionalCommentScope::Surface);
    }

    #[test]
    fn from_diff_note_finding_scope_derived_from_surfaces() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let note = packet.note_findings().pop().expect("note finding");
        // Mixed src + tests → File scope
        assert_eq!(note.scope, ConventionalCommentScope::File);
    }

    #[test]
    fn finding_category_as_str_returns_kebab_case_for_all_variants() {
        assert_eq!(FindingCategory::BoundaryCheck.as_str(), "boundary-check");
        assert_eq!(FindingCategory::DuplicationCheck.as_str(), "duplication-check");
        assert_eq!(FindingCategory::ContractDrift.as_str(), "contract-drift");
        assert_eq!(FindingCategory::MissingTests.as_str(), "missing-tests");
        assert_eq!(FindingCategory::DecisionImpact.as_str(), "decision-impact");
    }

    #[test]
    fn conventional_comment_scope_all_returns_all_three_variants_in_order() {
        use super::ConventionalCommentScope;
        let all = ConventionalCommentScope::all();
        assert_eq!(
            all,
            &[
                ConventionalCommentScope::Pr,
                ConventionalCommentScope::File,
                ConventionalCommentScope::Surface,
            ]
        );
    }

    #[test]
    fn conventional_comment_kind_must_fix_duplication_check_falls_through_to_issue() {
        let finding = ReviewFinding {
            category: FindingCategory::DuplicationCheck,
            severity: FindingSeverity::MustFix,
            title: "dup".to_string(),
            details: "overlapping logic".to_string(),
            scope: ConventionalCommentScope::Pr,
            anchor: None,
            changed_surfaces: vec![],
        };
        assert_eq!(finding.conventional_comment_kind(), "issue");
    }

    #[test]
    fn review_anchor_serde_round_trip() {
        let anchor =
            ReviewAnchor { surface: "src/lib.rs".to_string(), line_start: 12, line_end: Some(14) };

        let serialized = serde_json::to_string(&anchor).expect("serialized anchor");
        let deserialized: ReviewAnchor =
            serde_json::from_str(&serialized).expect("deserialized anchor");
        assert_eq!(deserialized, anchor);
    }

    #[test]
    fn derive_anchor_returns_line_anchor_for_single_surface_single_interval() {
        let anchor = derive_anchor(
            &["src/reviewer.rs".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -1 +1 @@\n-old\n+new\n",
        )
        .expect("line anchor");

        assert_eq!(
            anchor,
            ReviewAnchor { surface: "src/reviewer.rs".to_string(), line_start: 1, line_end: None }
        );
    }

    #[test]
    fn derive_anchor_returns_span_anchor_for_single_surface_multi_line_interval() {
        let anchor = derive_anchor(
            &["src/reviewer.rs".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -2,0 +2,3 @@\n+one\n+two\n+three\n",
        )
        .expect("span anchor");

        assert_eq!(
            anchor,
            ReviewAnchor {
                surface: "src/reviewer.rs".to_string(),
                line_start: 2,
                line_end: Some(4),
            }
        );
    }

    #[test]
    fn derive_anchor_returns_none_for_multiple_surfaces() {
        assert!(derive_anchor(
            &["src/reviewer.rs".to_string(), "tests/reviewer.md".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -1 +1 @@\n-old\n+new\n"
        )
        .is_none());
    }

    #[test]
    fn derive_anchor_returns_none_for_disjoint_intervals() {
        assert!(derive_anchor(
            &["src/reviewer.rs".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -1 +1 @@\n-old\n+new\n@@ -8 +8 @@\n-older\n+newer\n"
        )
        .is_none());
    }

    #[test]
    fn derive_anchor_returns_none_when_patch_lacks_durable_interval() {
        assert!(derive_anchor(&["src/reviewer.rs".to_string()], "").is_none());
        assert!(derive_anchor(
            &["src/reviewer.rs".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -4,1 +4,0 @@\n-old\n"
        )
        .is_none());
    }

    #[test]
    fn from_diff_populates_anchor_for_single_surface_note_finding() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["tests/reviewer.md".to_string()],
            "diff --git a/tests/reviewer.md b/tests/reviewer.md\n--- a/tests/reviewer.md\n+++ b/tests/reviewer.md\n@@ -2,0 +2,2 @@\n+one\n+two\n",
        );

        let note = packet.note_findings().pop().expect("note finding");
        assert_eq!(note.scope, ConventionalCommentScope::Surface);
        assert_eq!(
            note.anchor,
            Some(ReviewAnchor {
                surface: "tests/reviewer.md".to_string(),
                line_start: 2,
                line_end: Some(3),
            })
        );
    }

    #[test]
    fn from_diff_leaves_anchor_empty_for_cross_surface_findings() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/reviewer.rs".to_string(), "tests/reviewer.md".to_string()],
            "diff --git a/src/reviewer.rs b/src/reviewer.rs\n--- a/src/reviewer.rs\n+++ b/src/reviewer.rs\n@@ -1 +1 @@\n-old\n+new\n\ndiff --git a/tests/reviewer.md b/tests/reviewer.md\n--- a/tests/reviewer.md\n+++ b/tests/reviewer.md\n@@ -1 +1 @@\n-old\n+new\n",
        );

        let note = packet.note_findings().pop().expect("note finding");
        assert_eq!(note.anchor, None);
    }
}
