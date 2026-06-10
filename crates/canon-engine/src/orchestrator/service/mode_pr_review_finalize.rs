//! Orchestrator for the `pr-review finalize` phase.
//!
//! Renders all reviewer-facing artifacts from the canonical comment set,
//! derives the final recommendation, writes the artifact manifest, and
//! transitions the run state to `finalized`.

use std::fs;
use std::path::Path;

use crate::review::onion::RunState;
use crate::review::{
    findings::{CanonicalCommentSet, GithubComment, ReviewPacket},
    render::{self, Recommendation},
};

use super::EngineService;

/// The primary artifact slug for pr-review output.
const PR_REVIEW_PACKET_SLUG: &str = "pr-review";

impl EngineService {
    /// Runs the finalize phase: renders artifacts and transitions to finalized.
    pub fn run_pr_review_finalize(&self, run_id: &str) -> Result<(), String> {
        let run_dir = self.canon_runtime_dir().join("runs").join(run_id).join("pr-review");

        let current_state = read_run_state(&run_dir)?;
        if current_state != RunState::ReviewerOutputAccepted {
            return Err(format!(
                "Cannot finalize: run state is {:?}, expected {:?}",
                current_state,
                RunState::ReviewerOutputAccepted
            ));
        }

        // ── Early signal status check (T034, T035, FR-006, FR-014) ──
        verify_early_signal_finalize_gate(&run_dir)?;

        let canonical = build_canonical_comment_set(&run_dir)?;
        let packet = build_review_packet(&run_dir)?;
        let changed_files = read_changed_files(&run_dir)?;
        let files_inspected = read_files_inspected(&run_dir)?;

        let recommendation = render::derive_recommendation(
            &canonical, &packet, true,  // actionable_review_executed
            false, // actionable_review_failed
        )?;

        // ── Coverage-aware analysis ─────────────────────────────────────
        let early_signal_skipped = run_dir.join("early-signal").join("skip-metadata.json").exists();
        let classifications = crate::review::classifier::classify_files(&changed_files);
        let deep_reviewed_count = count_deep_reviewed_files(&run_dir, &changed_files);
        let layer_completions = read_layer_completions(&run_dir);
        let coverage = crate::review::coverage::analyze_coverage(
            &classifications,
            deep_reviewed_count,
            early_signal_skipped,
            &layer_completions,
        );

        let summary = render::render_review_summary(
            &canonical,
            recommendation,
            &packet,
            &changed_files,
            &files_inspected,
            &coverage,
        );
        let report = render::render_review_report(
            &canonical,
            recommendation,
            &packet,
            &changed_files,
            &coverage,
        );

        // ── Coverage accounting (T041-T044, FR-008, FR-014) ──
        write_coverage_accounting(&run_dir)?;

        // Write artifacts
        write_artifact(&run_dir, "01-review-summary.md", &summary)?;
        write_artifact(&run_dir, "06-review-report.md", &report)?;
        write_manifest(&run_dir, recommendation)?;
        write_packet_metadata(&run_dir)?;

        // Write review-findings.json
        write_review_findings_json(&run_dir, &canonical, &packet)?;

        // Write missing-tests.md
        write_missing_tests(&run_dir, &canonical)?;

        write_run_state_finalize(&run_dir, RunState::Finalized)?;

        Ok(())
    }
}

/// Reads the changed files list.
fn read_changed_files(run_dir: &Path) -> Result<Vec<String>, String> {
    let path = run_dir.join("changed-files.tsv");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("read changed-files.tsv: {e}"))?;
    Ok(content
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| l.split('\t').next().unwrap_or(l).trim().to_string())
        .collect())
}

/// Reads files deeply inspected (all changed files as a simple approximation).
fn read_files_inspected(run_dir: &Path) -> Result<Vec<String>, String> {
    read_changed_files(run_dir)
}

/// Counts how many changed files were deeply reviewed by the application-source
/// layer (layer 2). A file is considered deeply reviewed if it appears in
/// `layers/02-application-source/output.md` with non-placeholder content.
fn count_deep_reviewed_files(run_dir: &Path, _changed_files: &[String]) -> usize {
    let layer_output = run_dir.join("layers").join("02-application-source").join("output.md");
    if !layer_output.exists() {
        return 0;
    }
    // Conservative heuristic: if the output.md exists and is non-empty and
    // non-placeholder, count all changed files as deeply reviewed. In the
    // future, per-file review tracking will refine this.
    if let Ok(content) = fs::read_to_string(&layer_output) {
        let trimmed = content.trim();
        if trimmed.len() > 30
            && !trimmed.starts_with("# Application-Source Review\n\n*No output yet.*")
        {
            // Count all changed files as deeply reviewed for now.
            // This is a coarse approximation; per-file tracking will narrow this.
            return _changed_files.len();
        }
    }
    0
}

/// Reads layer completion statuses from the layers directory.
/// Returns (layer_slug, LayerStatus) for each layer.
fn read_layer_completions(run_dir: &Path) -> Vec<(String, crate::review::onion::LayerStatus)> {
    use crate::review::onion::LayerStatus;

    let layer_slugs = &[
        "early-signal",
        "application-source",
        "high-risk-surfaces",
        "related-context",
        "logical-stress",
        "tests",
        "coverage-accounting",
    ];

    layer_slugs
        .iter()
        .enumerate()
        .map(|(idx, slug)| {
            let ordinal = idx + 1;
            let layer_dir = run_dir.join("layers").join(format!("{:02}-{}", ordinal, slug));
            if !layer_dir.exists() {
                return (slug.to_string(), LayerStatus::Failed);
            }
            // Check for output.md content
            if let Ok(content) = fs::read_to_string(layer_dir.join("output.md")) {
                let trimmed = content.trim();
                if trimmed.len() > 30 {
                    return (slug.to_string(), LayerStatus::Completed);
                }
            }
            // Check for deferral
            if layer_dir.join("deferral.toml").exists() {
                return (slug.to_string(), LayerStatus::SkippedWithReason);
            }
            (slug.to_string(), LayerStatus::Failed)
        })
        .collect()
}

/// Reads the current run state from run-state.json.
fn read_run_state(run_dir: &Path) -> Result<RunState, String> {
    let path = run_dir.join("run-state.json");
    if !path.exists() {
        return Ok(RunState::Prepared);
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("read run-state.json: {e}"))?;
    let val: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("parse run-state.json: {e}"))?;
    let state_str = val.get("state").and_then(|s| s.as_str()).unwrap_or("prepared");
    let state = match state_str {
        "prepared" => RunState::Prepared,
        "awaiting_diff_review" => RunState::AwaitingDiffReview,
        "diff_review_recorded" => RunState::DiffReviewRecorded,
        "awaiting_whole_file_review" => RunState::AwaitingWholeFileReview,
        "whole_file_review_recorded" => RunState::WholeFileReviewRecorded,
        "awaiting_related_context_review" => RunState::AwaitingRelatedContextReview,
        "related_context_review_recorded" => RunState::RelatedContextReviewRecorded,
        "awaiting_stress_review" => RunState::AwaitingStressReview,
        "stress_review_recorded" => RunState::StressReviewRecorded,
        "awaiting_test_review" => RunState::AwaitingTestReview,
        "test_review_recorded" => RunState::TestReviewRecorded,
        "reviewer_output_accepted" => RunState::ReviewerOutputAccepted,
        "reviewer_output_rejected" => RunState::ReviewerOutputRejected,
        "finalized" => RunState::Finalized,
        _ => RunState::Prepared,
    };
    Ok(state)
}

/// Builds a `CanonicalCommentSet` from the reviewer output.
fn build_canonical_comment_set(run_dir: &Path) -> Result<CanonicalCommentSet, String> {
    let path = run_dir.join("canonical-review-output.json");
    if !path.exists() {
        return Ok(CanonicalCommentSet {
            comments: Vec::new(),
            reviewer_status: "actionable_review_not_configured".to_string(),
        });
    }
    // Build from the accepted reviewer output by reading the original reviewer-output.md
    let reviewer_path = run_dir.join("reviewer-output.md");
    if reviewer_path.exists() {
        let raw = fs::read_to_string(&reviewer_path)
            .map_err(|e| format!("read reviewer-output.md: {e}"))?;
        // Try to extract findings from the reviewer output
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw)
            && let Some(findings) = parsed.get("findings").and_then(|f| f.as_array())
        {
            let comments: Vec<GithubComment> = findings
                .iter()
                .enumerate()
                .map(|(i, f)| GithubComment {
                    id: format!("C{:03}", i + 1),
                    path: f.get("path").and_then(|p| p.as_str()).map(String::from),
                    line: f.get("line").and_then(|l| l.as_u64()).map(|l| l as u32),
                    side: f.get("side").and_then(|s| s.as_str()).map(String::from),
                    hunk_header: f.get("hunk_header").and_then(|h| h.as_str()).map(String::from),
                    area: String::new(),
                    kind: f
                        .get("kind")
                        .and_then(|k| k.as_str())
                        .unwrap_or("observation")
                        .to_string(),
                    blocking: f
                        .get("severity")
                        .and_then(|s| s.as_str())
                        .is_some_and(|s| s == "blocking"),
                    severity: f
                        .get("severity")
                        .and_then(|s| s.as_str())
                        .unwrap_or("minor")
                        .to_string(),
                    category: f.get("category").and_then(|c| c.as_str()).unwrap_or("").to_string(),
                    body: f
                        .get("summary")
                        .or_else(|| f.get("body"))
                        .and_then(|b| b.as_str())
                        .unwrap_or("No details provided")
                        .to_string(),
                    why_it_matters: f
                        .get("why_it_matters")
                        .and_then(|w| w.as_str())
                        .unwrap_or("")
                        .to_string(),
                    suggested_remediation: f
                        .get("suggested_remediation")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string(),
                    suggested_change: f
                        .get("suggested_change")
                        .and_then(|s| s.as_str())
                        .map(String::from),
                })
                .collect();
            return Ok(CanonicalCommentSet {
                comments,
                reviewer_status: "actionable_review_executed".to_string(),
            });
        }
    }
    Ok(CanonicalCommentSet {
        comments: Vec::new(),
        reviewer_status: "actionable_review_executed".to_string(),
    })
}

/// Builds a minimal `ReviewPacket` with empty governance findings.
fn build_review_packet(_run_dir: &Path) -> Result<ReviewPacket, String> {
    Ok(ReviewPacket {
        base_ref: "main".to_string(),
        head_ref: "HEAD".to_string(),
        findings: Vec::new(),
        changed_surfaces: Vec::new(),
        inferred_intent: String::new(),
        surprising_surface_area: Vec::new(),
    })
}

/// Writes an artifact file under the run directory.
fn write_artifact(run_dir: &Path, filename: &str, content: &str) -> Result<(), String> {
    fs::write(run_dir.join(filename), content).map_err(|e| format!("write {filename}: {e}"))
}

/// Writes the review-findings.json artifact.
fn write_review_findings_json(
    run_dir: &Path,
    canonical: &CanonicalCommentSet,
    packet: &ReviewPacket,
) -> Result<(), String> {
    let governance_findings: Vec<serde_json::Value> = packet
        .findings
        .iter()
        .map(|f| {
            serde_json::json!({
                "category": f.category.as_str(),
                "severity": f.severity.as_str(),
                "title": f.title,
                "details": f.details,
                "scope": f.scope.as_str(),
            })
        })
        .collect();

    let review_findings: Vec<serde_json::Value> = canonical
        .comments
        .iter()
        .map(|c| {
            serde_json::json!({
                "id": c.id,
                "severity": c.severity,
                "path": c.path,
                "line": c.line,
                "blocking": c.blocking,
                "kind": c.kind,
                "summary": c.body,
                "why_it_matters": c.why_it_matters,
                "suggested_remediation": c.suggested_remediation,
            })
        })
        .collect();

    let output = serde_json::json!({
        "schema_version": "1.0",
        "review_status": canonical.reviewer_status,
        "actionable_findings": review_findings,
        "governance_findings": governance_findings,
    });

    let content = serde_json::to_string_pretty(&output)
        .map_err(|e| format!("serialize review-findings.json: {e}"))?;
    fs::write(run_dir.join("review-findings.json"), &content)
        .map_err(|e| format!("write review-findings.json: {e}"))
}

/// Writes the missing-tests.md artifact with concrete test gap analysis.
fn write_missing_tests(run_dir: &Path, canonical: &CanonicalCommentSet) -> Result<(), String> {
    let mut out = String::new();
    out.push_str("# Missing Tests Analysis\n\n");
    out.push_str("## Overview\n\n");

    let blocking = canonical.blocking_count();
    let non_blocking = canonical.non_blocking_count();

    out.push_str("This report identifies test gaps inferred from the review findings.\n\n");
    out.push_str("| Metric | Value |\n|---|---|\n");
    out.push_str(&format!("| Actionable findings | {len} |\n", len = canonical.comments.len()));
    out.push_str(&format!("| Blocking | {blocking} |\n"));
    out.push_str(&format!("| Non-blocking | {non_blocking} |\n\n"));

    let test_related: Vec<_> = canonical
        .comments
        .iter()
        .filter(|c| {
            c.path
                .as_deref()
                .map(|p| p.contains("test") || p.contains("spec") || p.contains("__test"))
                .unwrap_or(false)
        })
        .collect();

    if test_related.is_empty() {
        out.push_str("## Test Gaps\n\n");
        out.push_str("No test-specific issues identified.\n\n");
    } else {
        out.push_str("## Identified Test Gaps\n\n");
        for c in &test_related {
            out.push_str(&format!(
                "- **{id}** ({severity}): {body}\n",
                id = c.id,
                severity = c.severity,
                body = c.body,
            ));
        }
        out.push('\n');
    }

    out.push_str("## Recommendations\n\n");
    if blocking > 0 {
        out.push_str("- Address blocking findings before merging.\n");
    }
    if canonical.comments.is_empty() {
        out.push_str("- No additional test coverage gaps identified.\n");
    } else {
        out.push_str("- Consider adding regression tests for surfaced issues.\n");
    }

    fs::write(run_dir.join("missing-tests.md"), &out)
        .map_err(|e| format!("write missing-tests.md: {e}"))
}

/// Writes the artifact manifest.
fn write_manifest(run_dir: &Path, recommendation: Recommendation) -> Result<(), String> {
    let manifest = format!(
        r#"# PR Review Manifest

## Artifacts

| File | Purpose |
|------|---------|
| 01-review-summary.md | Primary reviewer-facing summary with recommendation and severity breakdown |
| 06-review-report.md | Severity-oriented report with actionable findings |
| review-findings.json | Machine-readable findings with governance and actionable sections |
| missing-tests.md | Concrete test gap analysis |

## Recommendation

**{recommendation}**

## Decision Rules

- Blocking actionable findings → Request changes
- Non-blocking or partial coverage → Comment
- No findings, sufficient coverage → Approve
"#,
        recommendation = recommendation.as_str(),
    );
    fs::write(run_dir.join("manifest.toml"), &manifest)
        .map_err(|e| format!("write manifest.toml: {e}"))
}

/// Writes packet-metadata.json.
fn write_packet_metadata(run_dir: &Path) -> Result<(), String> {
    let metadata = serde_json::json!({
        "packet_type": PR_REVIEW_PACKET_SLUG,
        "primary_artifact": "01-review-summary.md",
        "artifacts": [
            {"file": "01-review-summary.md", "purpose": "primary"},
            {"file": "06-review-report.md", "purpose": "severity_report"},
            {"file": "review-findings.json", "purpose": "machine_readable"},
            {"file": "missing-tests.md", "purpose": "test_analysis"},
            {"file": "manifest.toml", "purpose": "manifest"},
            {"file": "canonical-review-output.json", "purpose": "validated_output"},
        ],
    });
    let content = serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("serialize packet-metadata.json: {e}"))?;
    fs::write(run_dir.join("packet-metadata.json"), &content)
        .map_err(|e| format!("write packet-metadata.json: {e}"))
}

/// Updates run-state.json with the finalized state.
fn write_run_state_finalize(run_dir: &Path, state: RunState) -> Result<(), String> {
    let path = run_dir.join("run-state.json");

    let mut current: serde_json::Value = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| format!("read run-state.json: {e}"))?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::json!({})
    };

    if let Some(obj) = current.as_object_mut() {
        obj.insert("state".to_string(), serde_json::json!(state.as_str()));
        let now = time::OffsetDateTime::now_utc();
        if let Ok(ts) = serde_json::to_value(now) {
            obj.insert("updated_at".to_string(), ts);
        }
    }

    let content =
        serde_json::to_string_pretty(&current).map_err(|e| format!("serialize run-state: {e}"))?;
    fs::write(&path, &content).map_err(|e| format!("write run-state.json: {e}"))?;
    Ok(())
}

/// Produces a `coverage-accounting.md` artifact listing all 7 layers
/// with their status (reviewed/deferred/skipped) and explicit reasons.
///
/// If early signal was skipped, `overall_confidence` is capped at `medium`
/// or lower (FR-014). Per FR-008 and FR-009, every deferred layer must
/// carry a non-empty reason.
fn write_coverage_accounting(run_dir: &Path) -> Result<(), String> {
    let layers_dir = run_dir.join("layers");
    let es_dir = run_dir.join("early-signal");
    let skip_meta = es_dir.join("skip-metadata.json");

    let early_signal_skipped = skip_meta.exists();
    let mut overall_confidence = if early_signal_skipped { "medium" } else { "high" };

    let mut content = String::from("# Coverage Accounting\n\n");
    content.push_str("## Layer Disposition\n\n");
    content.push_str("| Layer | Status | Reason |\n");
    content.push_str("|---|---|---|\n");

    let mut deferred_count = 0u32;

    if layers_dir.exists() {
        for (idx, slug) in
            crate::orchestrator::service::mode_pr_review_accept::LAYER_SLUGS.iter().enumerate()
        {
            let ordinal = idx + 1;
            let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
            let display = if *slug == "early-signal" {
                "Early Signal Pass"
            } else if *slug == "application-source" {
                "Application-Source Review"
            } else if *slug == "high-risk-surfaces" {
                "High-Risk Surfaces Review"
            } else if *slug == "related-context" {
                "Related-Context Review"
            } else if *slug == "logical-stress" {
                "Logical Stress Review"
            } else if *slug == "tests" {
                "Tests Review"
            } else {
                "Coverage Accounting"
            };

            let (status, reason) = if ordinal == 1 && early_signal_skipped {
                let reason = fs::read_to_string(&skip_meta)
                    .ok()
                    .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                    .and_then(|v| v.get("skip_reason")?.as_str().map(String::from))
                    .unwrap_or_else(|| "unspecified".to_string());
                ("skipped", reason)
            } else if layer_dir.join("deferral.toml").exists() {
                deferred_count += 1;
                let reason = fs::read_to_string(layer_dir.join("deferral.toml"))
                    .unwrap_or_else(|_| "reason not recorded".to_string());
                ("deferred", reason.trim().to_string())
            } else if layer_dir.join("output.md").exists() {
                ("reviewed", "—".to_string())
            } else {
                deferred_count += 1;
                ("unreviewed", "no output or deferral".to_string())
            };

            content.push_str(&format!("| {ordinal}. {display} | {status} | {reason} |\n"));
        }
    }

    if deferred_count > 1 {
        overall_confidence = "low";
    } else if deferred_count == 1 && !early_signal_skipped {
        overall_confidence = "medium";
    }

    if early_signal_skipped {
        content.push_str("\n## Early Signal Status\n\n");
        content.push_str("⚠️ Early signal pass was **skipped**. ");
        content.push_str("This review does **not** imply full early-risk coverage. ");
        if let Ok(meta) = fs::read_to_string(&skip_meta)
            && let Ok(v) = serde_json::from_str::<serde_json::Value>(&meta)
            && let Some(reason) = v.get("skip_reason").and_then(|r| r.as_str())
        {
            content.push_str(&format!("Skip reason: _{reason}_."));
        }
        content.push('\n');
    }

    content.push_str(&format!("\n**Overall confidence**: `{overall_confidence}`\n"));

    fs::write(run_dir.join("coverage-accounting.md"), content).map_err(|e| e.to_string())
}

/// Verifies that the early signal pass was either completed or explicitly
/// skipped with a recorded reason before allowing finalization.
///
/// If early signal was skipped, the final report must not imply full
/// early-risk coverage (FR-014). Per FR-006, Canon must NOT allow
/// finalization after layer 1 alone unless all 7 layers are accounted for.
fn verify_early_signal_finalize_gate(run_dir: &Path) -> Result<(), String> {
    let es_dir = run_dir.join("early-signal");
    let skip_meta = es_dir.join("skip-metadata.json");

    if skip_meta.exists() {
        // Early signal was skipped — verify skip reason is recorded
        let content =
            fs::read_to_string(&skip_meta).map_err(|e| format!("read skip metadata: {e}"))?;
        let meta: serde_json::Value =
            serde_json::from_str(&content).map_err(|e| format!("parse skip metadata: {e}"))?;
        let reason = meta.get("skip_reason").and_then(|v| v.as_str()).unwrap_or("");
        if reason.trim().is_empty() || reason == "unspecified" {
            return Err(
                "Cannot finalize: early signal was skipped without a valid recorded reason."
                    .to_string(),
            );
        }
        // Skipped early signal reduces confidence — record this in the final report
        return Ok(());
    }

    // Early signal was not skipped — verify findings artifacts exist
    if es_dir.join("findings.json").exists() || es_dir.join("summary.md").exists() {
        return Ok(());
    }

    // Neither skipped nor executed — this is a gap
    Err("Cannot finalize: early signal pass has no findings or skip metadata. Run prepare first."
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestrator::service::mode_pr_review_accept::LAYER_SLUGS;
    use tempfile::TempDir;

    fn make_temp_run_dir() -> (TempDir, std::path::PathBuf) {
        let dir = TempDir::new().unwrap();
        let pr_review_dir = dir.path().join("pr-review");
        fs::create_dir_all(&pr_review_dir).unwrap();
        (dir, pr_review_dir)
    }

    #[test]
    fn test_read_run_state_parses_valid_states() {
        let (_guard, run_dir) = make_temp_run_dir();
        let state = serde_json::json!({"state": "reviewer_output_accepted"});
        fs::write(run_dir.join("run-state.json"), state.to_string()).unwrap();
        assert_eq!(read_run_state(&run_dir).unwrap(), RunState::ReviewerOutputAccepted);
    }

    #[test]
    fn test_read_run_state_defaults_to_prepared() {
        let dir = TempDir::new().unwrap();
        assert_eq!(read_run_state(dir.path()).unwrap(), RunState::Prepared);
    }

    #[test]
    fn test_write_missing_tests_produces_report() {
        let (_guard, run_dir) = make_temp_run_dir();
        let canonical = CanonicalCommentSet {
            comments: vec![GithubComment {
                id: "C001".to_string(),
                path: Some("src/test/foo.rs".to_string()),
                line: Some(10),
                side: None,
                hunk_header: None,
                area: String::new(),
                kind: "observation".to_string(),
                blocking: true,
                severity: "blocking".to_string(),
                category: String::new(),
                body: "Missing edge case test".to_string(),
                why_it_matters: String::new(),
                suggested_remediation: String::new(),
                suggested_change: None,
            }],
            reviewer_status: "actionable_review_executed".to_string(),
        };
        write_missing_tests(&run_dir, &canonical).unwrap();
        let content = fs::read_to_string(run_dir.join("missing-tests.md")).unwrap();
        assert!(content.contains("Missing Tests Analysis"));
        assert!(content.contains("Missing edge case test"));
    }

    #[test]
    fn test_write_review_findings_json_produces_valid_structure() {
        let (_guard, run_dir) = make_temp_run_dir();
        let canonical = CanonicalCommentSet {
            comments: Vec::new(),
            reviewer_status: "actionable_review_executed".to_string(),
        };
        let packet = ReviewPacket {
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
            findings: Vec::new(),
            changed_surfaces: Vec::new(),
            inferred_intent: String::new(),
            surprising_surface_area: Vec::new(),
        };
        write_review_findings_json(&run_dir, &canonical, &packet).unwrap();
        let content = fs::read_to_string(run_dir.join("review-findings.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["schema_version"], "1.0");
        assert!(parsed["actionable_findings"].as_array().unwrap().is_empty());
        assert!(parsed["governance_findings"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_write_manifest_includes_recommendation() {
        let (_guard, run_dir) = make_temp_run_dir();
        write_manifest(&run_dir, Recommendation::RequestChanges).unwrap();
        let content = fs::read_to_string(run_dir.join("manifest.toml")).unwrap();
        assert!(content.contains("Request changes"));
        assert!(content.contains("01-review-summary.md"));
    }

    #[test]
    fn test_build_review_packet_returns_empty_packet() {
        let packet = build_review_packet(std::path::Path::new(".")).unwrap();
        assert!(packet.findings.is_empty());
        assert_eq!(packet.base_ref, "main");
    }

    #[test]
    fn test_read_changed_files_tsv() {
        let (_guard, run_dir) = make_temp_run_dir();
        fs::write(run_dir.join("changed-files.tsv"), "src/a.rs\nsrc/b.rs\n").unwrap();
        let files = read_changed_files(&run_dir).unwrap();
        assert_eq!(files, vec!["src/a.rs", "src/b.rs"]);
    }

    #[test]
    fn verify_early_signal_gate_rejects_missing_findings_and_skip() {
        let dir = TempDir::new().unwrap();
        // Neither findings.json nor skip-metadata.json — should fail
        let result = verify_early_signal_finalize_gate(dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Run prepare first"));
    }

    #[test]
    fn verify_early_signal_gate_accepts_findings_json() {
        let dir = TempDir::new().unwrap();
        let es_dir = dir.path().join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        fs::write(es_dir.join("findings.json"), "[]").unwrap();
        assert!(verify_early_signal_finalize_gate(dir.path()).is_ok());
    }

    #[test]
    fn verify_early_signal_gate_accepts_skip_with_valid_reason() {
        let dir = TempDir::new().unwrap();
        let es_dir = dir.path().join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        let skip = serde_json::json!({
            "early_signal_status": "skipped_with_reason",
            "skip_reason": "debugging accept flow",
            "source": "operator",
            "confidence_impact": "medium",
        });
        fs::write(es_dir.join("skip-metadata.json"), skip.to_string()).unwrap();
        assert!(verify_early_signal_finalize_gate(dir.path()).is_ok());
    }

    #[test]
    fn write_coverage_accounting_produces_markdown() {
        let dir = TempDir::new().unwrap();
        let es_dir = dir.path().join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        // Mark early signal as executed
        fs::write(es_dir.join("findings.json"), "[]").unwrap();
        // Create layers with output
        let layers_dir = dir.path().join("layers");
        for (idx, slug) in LAYER_SLUGS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            fs::write(layer_dir.join("output.md"), format!("# {} review\n\nDone.\n", slug))
                .unwrap();
        }
        write_coverage_accounting(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("coverage-accounting.md")).unwrap();
        assert!(content.contains("Coverage Accounting"));
        assert!(content.contains("Early Signal Pass"));
        assert!(content.contains("Overall confidence"));
    }

    #[test]
    fn write_coverage_accounting_caps_confidence_on_skip() {
        let dir = TempDir::new().unwrap();
        let es_dir = dir.path().join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        let skip = serde_json::json!({
            "skip_reason": "not needed for this PR",
        });
        fs::write(es_dir.join("skip-metadata.json"), skip.to_string()).unwrap();
        let layers_dir = dir.path().join("layers");
        for (idx, slug) in LAYER_SLUGS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            fs::write(layer_dir.join("output.md"), format!("# {} review\n\nDone.\n", slug))
                .unwrap();
        }
        write_coverage_accounting(dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("coverage-accounting.md")).unwrap();
        // Skipped early signal caps confidence at medium
        assert!(content.contains("`medium`"));
        assert!(content.contains("does **not** imply full early-risk coverage"));
    }

    // ── run_pr_review_finalize integration test ──────────────────────────

    #[test]
    fn run_pr_review_finalize_succeeds_with_full_fixture() {
        let workspace = TempDir::new().unwrap();
        let run_id = "test-finalize-engine";
        let run_dir = workspace.path().join(".canon").join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).unwrap();

        // Run state: ReviewerOutputAccepted
        let run_state = serde_json::json!({"state": "reviewer_output_accepted"});
        fs::write(run_dir.join("run-state.json"), run_state.to_string()).unwrap();

        // Early signal findings (already executed)
        let es_dir = run_dir.join("early-signal");
        fs::create_dir_all(&es_dir).unwrap();
        fs::write(es_dir.join("findings.json"), "[]").unwrap();

        // Reviewer output (needed for canonical comment set)
        let reviewer_output = serde_json::json!({
            "schema_version": "1.0",
            "findings": [
                {
                    "id": "f1",
                    "severity": "minor",
                    "path": "src/a.rs",
                    "line": 10,
                    "comment_id": "c1",
                    "layer": "early-signal"
                }
            ],
            "recommendation": "Approve"
        });
        fs::write(run_dir.join("reviewer-output.md"), reviewer_output.to_string()).unwrap();

        // Canonical review output
        let canonical = serde_json::json!({
            "valid": true,
            "errors": [],
            "downgrades": []
        });
        fs::write(run_dir.join("canonical-review-output.json"), canonical.to_string()).unwrap();

        // Changed files
        fs::write(run_dir.join("changed-files.tsv"), "src/a.rs\n").unwrap();

        // Layer directories with output.md
        let layers_dir = run_dir.join("layers");
        for (idx, slug) in LAYER_SLUGS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            let output = format!(
                "# {slug} Review\n\nDetailed findings for layer {ordinal}.\n\nReview complete.\n"
            );
            fs::write(layer_dir.join("output.md"), output).unwrap();
        }

        let service = EngineService::new(workspace.path());
        let result = service.run_pr_review_finalize(run_id);
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());

        // Verify key artifacts were written
        assert!(run_dir.join("01-review-summary.md").exists(), "summary not written");
        assert!(run_dir.join("06-review-report.md").exists(), "report not written");
        assert!(run_dir.join("review-findings.json").exists(), "findings not written");
        assert!(run_dir.join("coverage-accounting.md").exists(), "coverage accounting not written");

        // Verify run state was updated to finalized
        let state_content = fs::read_to_string(run_dir.join("run-state.json")).unwrap();
        assert!(state_content.contains("finalized"), "state not finalized");
    }
}
