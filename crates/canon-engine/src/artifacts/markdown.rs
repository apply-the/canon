use crate::domain::run::{
    ClarificationAnswerKind, ClarificationRefinementContext, ClarificationRefinementStatus,
    ClarificationResolutionState,
};

struct AuthoredSectionSpec<'a> {
    canonical_heading: &'a str,
    aliases: &'a [&'a str],
}

/// Marker heading used when a governed body section has not been authored yet.
pub const MISSING_AUTHORED_BODY_MARKER: &str = "## Missing Authored Body";
/// Marker heading used when a governed decision section has not been authored yet.
pub const MISSING_AUTHORED_DECISION_MARKER: &str = "## Missing Authored Decision";

/// Renders a generic titled Markdown document with a summary section.
pub fn render_markdown(title: &str, summary: &str) -> String {
    format!("# {title}\n\n## Summary\n\n{summary}\n")
}

/// Renders a run-local working brief by preserving the authored body and
/// appending the standard refinement appendix for targeted modes.
pub fn render_refinement_working_brief(
    authoritative_body: &str,
    refinement: &ClarificationRefinementContext,
) -> String {
    let mut lines = Vec::new();
    let trimmed_body = authoritative_body.trim_end();
    if !trimmed_body.is_empty() {
        lines.push(trimmed_body.to_string());
    }

    lines.push(String::new());
    lines.push("## Clarification Provenance".to_string());
    lines.push(String::new());
    lines.push("### Applied Answers".to_string());
    lines.push(String::new());
    append_record_lines(
        &mut lines,
        refinement,
        ClarificationAnswerKind::Explicit,
        "No applied answers recorded.",
    );

    lines.push(String::new());
    lines.push("### Applied Defaults".to_string());
    lines.push(String::new());
    append_record_lines(
        &mut lines,
        refinement,
        ClarificationAnswerKind::Defaulted,
        "No applied defaults recorded.",
    );

    lines.push(String::new());
    lines.push("## Source Snapshots".to_string());
    lines.push(String::new());
    append_source_snapshot_lines(&mut lines, refinement);

    lines.push(String::new());
    lines.push("## Unresolved Questions".to_string());
    lines.push(String::new());
    append_unresolved_question_lines(&mut lines, refinement);

    lines.push(String::new());
    lines.push("## Readiness Delta".to_string());
    lines.push(String::new());
    append_readiness_delta_lines(&mut lines, refinement);

    lines.push(String::new());
    lines.push("## Continuation State".to_string());
    lines.push(String::new());
    append_continuation_state_lines(&mut lines, refinement);

    format!("{}\n", lines.join("\n"))
}

fn append_record_lines(
    lines: &mut Vec<String>,
    refinement: &ClarificationRefinementContext,
    answer_kind: ClarificationAnswerKind,
    empty_message: &str,
) {
    let mut found = false;
    for record in &refinement.records {
        if record.answer_kind != answer_kind {
            continue;
        }
        found = true;
        lines.push(format!("- {}: {} -> {}", record.id, record.prompt, record.answer));
    }

    if !found {
        lines.push(format!("- {empty_message}"));
    }
}

fn append_source_snapshot_lines(
    lines: &mut Vec<String>,
    refinement: &ClarificationRefinementContext,
) {
    let mut snapshots = Vec::new();
    for snapshot in
        refinement.authoritative_input_refs.iter().chain(refinement.supporting_input_refs.iter())
    {
        if !snapshots.contains(snapshot) {
            snapshots.push(snapshot.clone());
        }
    }

    if snapshots.is_empty() {
        lines.push("- No source snapshots recorded.".to_string());
        return;
    }

    for snapshot in snapshots {
        lines.push(format!("- {snapshot}"));
    }
}

fn append_unresolved_question_lines(
    lines: &mut Vec<String>,
    refinement: &ClarificationRefinementContext,
) {
    let mut found = false;
    for record in &refinement.records {
        if !matches!(record.resolution_state, ClarificationResolutionState::Deferred) {
            continue;
        }
        found = true;
        lines.push(format!("- {}", record.prompt));
    }

    if !found {
        lines.push("- No unresolved questions remain.".to_string());
    }
}

fn append_readiness_delta_lines(
    lines: &mut Vec<String>,
    refinement: &ClarificationRefinementContext,
) {
    let unresolved =
        refinement.readiness_delta.iter().filter(|item| !item.resolved).collect::<Vec<_>>();

    if unresolved.is_empty() {
        lines.push("- No readiness delta remains.".to_string());
        return;
    }

    for item in unresolved {
        lines.push(format!("- {}", item.summary));
    }
}

fn append_continuation_state_lines(
    lines: &mut Vec<String>,
    refinement: &ClarificationRefinementContext,
) {
    let lifecycle_line = match refinement.status {
        ClarificationRefinementStatus::Active => {
            "- Same run identity retained during draft refinement."
        }
        ClarificationRefinementStatus::Ready => {
            "- The run remains the same work item and is ready for explicit continuation into governed execution."
        }
        ClarificationRefinementStatus::Superseded => {
            "- This refinement record has been superseded by a successor run."
        }
    };
    lines.push(lifecycle_line.to_string());

    if refinement.explicit_continuation_required {
        lines.push(
            "- Explicit continuation is still required before Canon mutates an existing run."
                .to_string(),
        );
    }

    if refinement.suggested_candidate.is_some() {
        lines.push(
            "- Candidate detection is advisory; continuation requires explicit intent.".to_string(),
        );
    }
}

// ── Submodules ────────────────────────────────────────────────────────────────

mod analysis;
mod architecture;
mod authoring;
mod delivery;
mod domain;
mod governance;
mod shared;

// ── Public API re-exports ─────────────────────────────────────────────────────

pub use analysis::{
    render_security_assessment_artifact, render_supply_chain_analysis_artifact,
    render_system_assessment_artifact,
};
pub use architecture::{
    C4_MISSING_AUTHORED_BODY_MARKER, architecture_artifact_enabled, architecture_view_authored,
    render_architecture_artifact,
};
pub use authoring::{
    render_brainstorming_artifact, render_discovery_artifact, render_requirements_artifact,
    render_requirements_artifact_from_evidence, render_system_shaping_artifact,
};
pub use delivery::{
    render_backlog_artifact, render_change_artifact, render_debugging_artifact,
    render_implementation_artifact, render_incident_artifact, render_migration_artifact,
    render_refactor_artifact,
};
pub use domain::{render_domain_language_artifact, render_domain_model_artifact};
pub use governance::{
    render_pr_review_artifact, render_review_artifact, render_verification_artifact,
};

// ── Private helpers re-exported for test visibility ──────────────────────────

#[cfg(test)]
use architecture::{
    extract_paragraph_nodes, first_paragraph, mermaid_label, render_linear_mermaid,
};
#[cfg(test)]
use shared::{extract_authored_h2_section, extract_marker, render_missing_authored_body_block};

#[cfg(test)]
mod tests {
    use super::{
        MISSING_AUTHORED_BODY_MARKER, architecture_artifact_enabled, extract_authored_h2_section,
        extract_marker, extract_paragraph_nodes, first_paragraph, mermaid_label,
        render_architecture_artifact, render_backlog_artifact, render_change_artifact,
        render_discovery_artifact, render_incident_artifact, render_linear_mermaid,
        render_migration_artifact, render_missing_authored_body_block, render_pr_review_artifact,
        render_refinement_working_brief, render_requirements_artifact,
        render_requirements_artifact_from_evidence, render_review_artifact,
        render_system_shaping_artifact, render_verification_artifact,
    };
    use crate::domain::run::{
        BacklogExecutionHandoff, BacklogGranularity, BacklogHandoffAvailability,
        BacklogPlanningContext, ClarificationAnswerKind, ClarificationRecord,
        ClarificationRefinementContext, ClarificationRefinementStatus,
        ClarificationResolutionState, ClosureAssessment, ClosureFinding, ClosureFindingSeverity,
        ContinuationCandidateSummary, ReadinessDeltaItem, ReadinessDeltaSourceKind,
        RefinementWorkflowFamily, RunState,
    };
    use crate::review::findings::{
        ConventionalCommentScope, FindingCategory, FindingSeverity, ReviewAnchor, ReviewFinding,
        ReviewPacket,
    };
    use crate::review::summary::{ReviewDisposition, ReviewSummary};
    use time::OffsetDateTime;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct SampleRefinementAppendix {
        mode_slug: &'static str,
        authoritative_inputs: Vec<String>,
        supporting_inputs: Vec<String>,
        applied_answers: Vec<(String, String)>,
        applied_defaults: Vec<(String, String)>,
        unresolved_questions: Vec<String>,
        readiness_delta: Vec<String>,
    }

    fn sample_refinement_appendix(mode_slug: &'static str) -> SampleRefinementAppendix {
        SampleRefinementAppendix {
            mode_slug,
            authoritative_inputs: vec![format!("canon-input/{mode_slug}/brief.md")],
            supporting_inputs: vec![format!("canon-input/{mode_slug}/context-links.md")],
            applied_answers: vec![(
                "cq-001".to_string(),
                "Which actor owns the problem? -> platform operators".to_string(),
            )],
            applied_defaults: vec![(
                "cq-002".to_string(),
                "Validation owner defaulted to repository maintainer review".to_string(),
            )],
            unresolved_questions: vec![
                "Which downstream team owns the rollout sign-off?".to_string(),
            ],
            readiness_delta: vec!["Independent validation owner is not yet named.".to_string()],
        }
    }

    fn sample_refinement_context(mode_slug: &'static str) -> ClarificationRefinementContext {
        ClarificationRefinementContext {
            workflow_family: RefinementWorkflowFamily::Planning,
            current_mode: match mode_slug {
                "requirements" => crate::domain::mode::Mode::Requirements,
                "discovery" => crate::domain::mode::Mode::Discovery,
                "system-shaping" => crate::domain::mode::Mode::SystemShaping,
                "architecture" => crate::domain::mode::Mode::Architecture,
                "change" => crate::domain::mode::Mode::Change,
                other => panic!("unsupported refinement mode slug: {other}"),
            },
            working_brief_path: format!(
                ".canon/runs/R-20260529-ab12cd34/artifacts/{mode_slug}/working-brief.md"
            ),
            template_ref: format!("defaults/templates/canon-input/{mode_slug}.md"),
            status: ClarificationRefinementStatus::Active,
            explicit_continuation_required: true,
            authoritative_input_refs: vec![format!("canon-input/{mode_slug}/brief.md")],
            supporting_input_refs: vec![format!("canon-input/{mode_slug}/context-links.md")],
            suggested_candidate: Some(ContinuationCandidateSummary {
                run_id: "R-20260529-prev0001".to_string(),
                mode: match mode_slug {
                    "requirements" => crate::domain::mode::Mode::Requirements,
                    "discovery" => crate::domain::mode::Mode::Discovery,
                    "system-shaping" => crate::domain::mode::Mode::SystemShaping,
                    "architecture" => crate::domain::mode::Mode::Architecture,
                    "change" => crate::domain::mode::Mode::Change,
                    other => panic!("unsupported refinement mode slug: {other}"),
                },
                state: RunState::Draft,
                match_reason: "same authoritative input fingerprint".to_string(),
                advisory: true,
            }),
            records: vec![
                ClarificationRecord {
                    id: "cq-001".to_string(),
                    prompt: "Which actor owns the problem?".to_string(),
                    answer: "platform operators".to_string(),
                    answer_kind: ClarificationAnswerKind::Explicit,
                    affected_sections: vec!["Actors".to_string(), "Problem Statement".to_string()],
                    resolution_state: ClarificationResolutionState::Resolved,
                    recorded_at: OffsetDateTime::UNIX_EPOCH,
                },
                ClarificationRecord {
                    id: "cq-004".to_string(),
                    prompt: "Who validates the release walkthrough?".to_string(),
                    answer: "Validation owner defaulted to repository maintainer review"
                        .to_string(),
                    answer_kind: ClarificationAnswerKind::Defaulted,
                    affected_sections: vec!["Validation Strategy".to_string()],
                    resolution_state: ClarificationResolutionState::Resolved,
                    recorded_at: OffsetDateTime::UNIX_EPOCH,
                },
                ClarificationRecord {
                    id: "cq-005".to_string(),
                    prompt: "Which downstream team owns rollout sign-off?".to_string(),
                    answer: "Still awaiting owner confirmation".to_string(),
                    answer_kind: ClarificationAnswerKind::Deferred,
                    affected_sections: vec!["Rollout Plan".to_string()],
                    resolution_state: ClarificationResolutionState::Deferred,
                    recorded_at: OffsetDateTime::UNIX_EPOCH,
                },
            ],
            readiness_delta: vec![ReadinessDeltaItem {
                id: "rd-001".to_string(),
                section: "Validation Strategy".to_string(),
                summary: "Independent validation owner is not yet named.".to_string(),
                blocking: true,
                source_kind: ReadinessDeltaSourceKind::MissingContext,
                default_available: false,
                resolved: false,
            }],
        }
    }

    fn sample_backlog_planning_context() -> BacklogPlanningContext {
        BacklogPlanningContext {
            mode: "backlog".to_string(),
            delivery_intent: "Prepare a bounded delivery backlog for runtime honesty work."
                .to_string(),
            desired_granularity: BacklogGranularity::EpicPlusSlice,
            planning_horizon: Some("feature 033".to_string()),
            source_refs: vec!["specs/033-reasoning-evidence-clarity/plan.md".to_string()],
            priority_inputs: vec!["Remove fake reasoning from fallback artifacts".to_string()],
            constraints: vec!["Stay above task level".to_string()],
            out_of_scope: vec!["New runtime modes".to_string()],
            closure_assessment: ClosureAssessment::sufficient(),
            slice_ids: vec!["SLICE-RUNTIME-001".to_string()],
            handoff_availability: BacklogHandoffAvailability::Available,
            handoff_findings: vec!["selected slice has explicit implementation refs".to_string()],
            execution_handoff: Some(BacklogExecutionHandoff {
                selected_slice_id: "SLICE-RUNTIME-001".to_string(),
                selection_rationale: "Stabilize the first bounded runtime honesty slice."
                    .to_string(),
                implementation_artifact_refs: vec!["src/runtime/honesty.rs".to_string()],
                dependency_prerequisites: vec![
                    "reasoning posture contract remains stable".to_string(),
                ],
                independent_verification_anchors: vec![
                    "contract test proves fallback artifacts stay evidence-backed".to_string(),
                ],
                blocked_assumptions: vec!["No new runtime mode is required".to_string()],
            }),
        }
    }

    #[test]
    fn extract_marker_prefers_markdown_section_over_inline_mentions() {
        let source = "# Change Brief\n\n## Change Surface\n- bounded module\n- stable interface\n\nMutation posture: propose bounded legacy transformation within declared change surface: workspace, adjacent modules";
        let normalized = source.to_lowercase();

        let marker = extract_marker(source, &normalized, "change surface").expect("change surface");

        assert_eq!(marker, "- bounded module\n- stable interface");
    }

    #[test]
    fn render_change_surface_preserves_markdown_bullets() {
        let source = "# Change Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Change Surface\n- Public API entrypoints\n- Debug logging only\n\n## Owner\nLead Eng\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n";

        let rendered = render_change_artifact("change-surface.md", source, "");

        assert!(
            rendered
                .contains("## Change Surface\n\n- Public API entrypoints\n- Debug logging only")
        );
        assert!(rendered.contains("- Owner / risk / zone: `Lead Eng` / `low-impact` / `green`"));
    }

    #[test]
    fn render_change_validation_strategy_preserves_markdown_bullets() {
        let source = "# Change Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Validation Strategy\n- Unit tests\n- Log assertion checks\n";

        let rendered = render_change_artifact("validation-strategy.md", source, "");

        assert!(
            rendered.contains("## Validation Strategy\n\n- Unit tests\n- Log assertion checks")
        );
    }

    #[test]
    fn sample_refinement_appendix_builder_returns_mode_scoped_paths() {
        let appendix = sample_refinement_appendix("requirements");

        assert_eq!(appendix.mode_slug, "requirements");
        assert_eq!(
            appendix.authoritative_inputs,
            vec!["canon-input/requirements/brief.md".to_string()]
        );
        assert_eq!(
            appendix.supporting_inputs,
            vec!["canon-input/requirements/context-links.md".to_string()]
        );
        assert_eq!(appendix.applied_answers.len(), 1);
        assert_eq!(appendix.applied_defaults.len(), 1);
        assert_eq!(appendix.unresolved_questions.len(), 1);
        assert_eq!(appendix.readiness_delta.len(), 1);
    }

    #[test]
    fn render_refinement_working_brief_appends_required_refinement_sections() {
        let refinement = sample_refinement_context("requirements");
        let rendered = render_refinement_working_brief(
            "# Requirements Brief\n\n## Problem\nClarify same-work continuation.\n",
            &refinement,
        );

        assert!(rendered.contains("## Clarification Provenance"));
        assert!(rendered.contains("### Applied Answers"));
        assert!(rendered.contains("- cq-001: Which actor owns the problem? -> platform operators"));
        assert!(rendered.contains("### Applied Defaults"));
        assert!(rendered.contains("Validation owner defaulted to repository maintainer review"));
        assert!(rendered.contains("## Source Snapshots"));
        assert!(rendered.contains("canon-input/requirements/brief.md"));
        assert!(rendered.contains("canon-input/requirements/context-links.md"));
        assert!(rendered.contains("## Unresolved Questions"));
        assert!(rendered.contains("Which downstream team owns rollout sign-off?"));
        assert!(rendered.contains("## Readiness Delta"));
        assert!(rendered.contains("Independent validation owner is not yet named."));
        assert!(rendered.contains("## Continuation State"));
        assert!(
            rendered.contains(
                "Candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn render_requirements_artifacts_cover_named_templates_and_fallback() {
        let summary = "Bound the requirements work before planning";
        let authored = "# Requirements Brief\n\n## Constraints\n\n- Keep the implementation local-first and auditable.\n\n## Non-Negotiables\n\n- Preserve explicit human ownership.\n\n## Tradeoffs\n\n- Favoring governability reduces raw generation speed.\n\n## Consequences\n\n- The product will feel opinionated by design.\n";

        let constraints = render_requirements_artifact("constraints.md", summary);
        let fallback = render_requirements_artifact("custom-note.md", summary);
        let evidence = render_requirements_artifact_from_evidence(
            "tradeoffs.md",
            summary,
            authored,
            "generated framing",
            "critique note",
            "denied mutation request remained visible",
        );
        let missing = render_requirements_artifact_from_evidence(
            "problem-statement.md",
            summary,
            "# Requirements Brief\n\n## Problem\n\nBound the requirements work before planning.\n",
            "generated framing",
            "critique note",
            "denied mutation request remained visible",
        );

        assert!(constraints.contains("## Non-Negotiables"));
        assert!(constraints.contains("Risk and zone classification happen before generation."));
        assert!(fallback.starts_with(
            "# custom-note.md\n\n## Summary\n\nBound the requirements work before planning"
        ));
        assert!(
            evidence
                .contains("## Tradeoffs\n\n- Favoring governability reduces raw generation speed.")
        );
        assert!(
            evidence.contains("## Consequences\n\n- The product will feel opinionated by design.")
        );
        assert!(missing.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(missing.contains("`## Outcome`"));
    }

    #[test]
    fn authored_h2_extraction_requires_exact_heading_level_and_documented_aliases() {
        let near_miss = "# Requirements Brief\n\n### Problem\n\nThis should not count.\n";
        let alias = "# Requirements Brief\n\n## Out of Scope\n\n- No GUI\n";

        assert!(extract_authored_h2_section(near_miss, "Problem", &[]).is_none());
        assert_eq!(
            extract_authored_h2_section(alias, "Scope Cuts", &["Out of Scope"]),
            Some("- No GUI".to_string())
        );
    }

    #[test]
    fn missing_authored_body_block_names_the_canonical_heading() {
        let rendered = render_missing_authored_body_block("Outcome");

        assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(rendered.contains("NOT CAPTURED - No `## Outcome` section was authored"));
    }

    #[test]
    fn render_change_artifact_reports_missing_context_and_default_metadata() {
        let source = "# Change Brief\n\n## System Slice\nSession repository\n\n## Intended Change\nStabilize resumable execution\n";

        let invariants = render_change_artifact("legacy-invariants.md", source, "");
        let decision = render_change_artifact("decision-record.md", source, "");

        assert!(invariants.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(invariants.contains("`## Legacy Invariants`"));
        assert!(decision.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(decision.contains("`## Decision Record`"));
        assert!(decision.contains("- Owner / risk / zone: `bounded-system-maintainer` / `unspecified-risk` / `unspecified-zone`"));
    }

    #[test]
    fn render_pr_review_artifacts_handle_empty_and_populated_findings() {
        let review_notes_packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );
        let review_notes_summary = ReviewSummary::from_packet(&review_notes_packet, false);
        let boundary = render_pr_review_artifact(
            "boundary-check.md",
            &review_notes_packet,
            &review_notes_summary,
        );

        assert!(boundary.contains("- No boundary findings detected."));
        assert!(boundary.contains("Status: no-structural-impact-detected"));

        let must_fix_packet = ReviewPacket {
            base_ref: "origin/main".to_string(),
            head_ref: "feature".to_string(),
            changed_surfaces: vec!["contracts/public-api.json".to_string()],
            inferred_intent: "Review contract drift on a public API change.".to_string(),
            surprising_surface_area: vec!["contracts/public-api.json".to_string()],
            findings: vec![
                ReviewFinding {
                    category: FindingCategory::ContractDrift,
                    severity: FindingSeverity::MustFix,
                    title: "Contract-facing files changed".to_string(),
                    details: "Compatibility drift needs explicit reviewer acceptance.".to_string(),
                    scope: ConventionalCommentScope::Surface,
                    anchor: None,
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
                ReviewFinding {
                    category: FindingCategory::DecisionImpact,
                    severity: FindingSeverity::Note,
                    title: "Decision note".to_string(),
                    details: "A broader acceptance note should be recorded.".to_string(),
                    scope: ConventionalCommentScope::Surface,
                    anchor: None,
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
            ],
        };
        let must_fix_summary = ReviewSummary {
            disposition: ReviewDisposition::AcceptedWithApproval,
            rationale: "Explicit reviewer approval accepted the remaining must-fix findings with named ownership.".to_string(),
            must_fix_findings: vec!["Contract-facing files changed".to_string()],
            accepted_risks: vec!["Decision note".to_string()],
        };

        let contract =
            render_pr_review_artifact("contract-drift.md", &must_fix_packet, &must_fix_summary);
        let conventional_comments = render_pr_review_artifact(
            "conventional-comments.md",
            &must_fix_packet,
            &must_fix_summary,
        );
        let summary =
            render_pr_review_artifact("review-summary.md", &must_fix_packet, &must_fix_summary);

        assert!(contract.contains("Status: explicit-contract-drift"));
        assert!(contract.contains(
            "Compatibility risk remains explicit until reviewer disposition is recorded."
        ));
        assert!(conventional_comments.contains("issue(scope:"));
        assert!(conventional_comments.contains("thought(scope:"));
        assert!(conventional_comments.contains("contracts/public-api.json"));
        assert!(summary.contains("Overall severity: must-fix"));
        assert!(summary.contains("Status: accepted-with-approval"));
        assert!(summary.contains("- Decision note"));
    }

    #[test]
    fn render_conventional_comments_includes_scope_annotation() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["contracts/public-api.json".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let summary = crate::review::summary::ReviewSummary::from_packet(&packet, false);
        let output = render_pr_review_artifact("conventional-comments.md", &packet, &summary);
        // Each entry should carry a scope annotation in the kind label.
        assert!(output.contains("scope:"), "scope annotation missing from output:\n{output}");
    }

    #[test]
    fn render_conventional_comments_pr_scope_when_no_surfaces() {
        let packet = ReviewPacket {
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
            changed_surfaces: vec![],
            inferred_intent: "No surfaces detected.".to_string(),
            surprising_surface_area: vec![],
            findings: vec![ReviewFinding {
                category: FindingCategory::DuplicationCheck,
                severity: FindingSeverity::Note,
                title: "PR-level note".to_string(),
                details: "No surfaces, applies at PR level.".to_string(),
                scope: ConventionalCommentScope::Pr,
                anchor: None,
                changed_surfaces: vec![],
            }],
        };
        let summary = crate::review::summary::ReviewSummary::from_packet(&packet, false);
        let output = render_pr_review_artifact("conventional-comments.md", &packet, &summary);
        assert!(output.contains("scope:pr"), "expected scope:pr in output:\n{output}");
        // No scope surfaces line for Pr scope.
        assert!(
            !output.contains("Scope surfaces:"),
            "unexpected Scope surfaces line for Pr scope:\n{output}"
        );
    }

    #[test]
    fn render_conventional_comments_surface_scope_lists_surfaces() {
        use crate::review::findings::ConventionalCommentScope;
        let packet = ReviewPacket {
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
            changed_surfaces: vec!["contracts/api.json".to_string()],
            inferred_intent: "Contract change.".to_string(),
            surprising_surface_area: vec![],
            findings: vec![ReviewFinding {
                category: FindingCategory::ContractDrift,
                severity: FindingSeverity::MustFix,
                title: "Contract drift".to_string(),
                details: "API surface drifted.".to_string(),
                scope: ConventionalCommentScope::Surface,
                anchor: None,
                changed_surfaces: vec!["contracts/api.json".to_string()],
            }],
        };
        let summary = crate::review::summary::ReviewSummary::from_packet(&packet, false);
        let output = render_pr_review_artifact("conventional-comments.md", &packet, &summary);
        assert!(output.contains("scope:surface"), "expected scope:surface in output:\n{output}");
        assert!(
            output.contains("Scope surfaces: contracts/api.json"),
            "expected Scope surfaces in output:\n{output}"
        );
    }

    #[test]
    fn render_conventional_comments_includes_host_agnostic_anchor_text() {
        let packet = ReviewPacket {
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
            changed_surfaces: vec!["contracts/api.json".to_string()],
            inferred_intent: "Contract change.".to_string(),
            surprising_surface_area: vec![],
            findings: vec![ReviewFinding {
                category: FindingCategory::ContractDrift,
                severity: FindingSeverity::MustFix,
                title: "Contract drift".to_string(),
                details: "API surface drifted.".to_string(),
                scope: ConventionalCommentScope::Surface,
                anchor: Some(ReviewAnchor {
                    surface: "contracts/api.json".to_string(),
                    line_start: 9,
                    line_end: Some(12),
                }),
                changed_surfaces: vec!["contracts/api.json".to_string()],
            }],
        };
        let summary = crate::review::summary::ReviewSummary::from_packet(&packet, false);
        let output = render_pr_review_artifact("conventional-comments.md", &packet, &summary);
        assert!(output.contains("scope:surface"), "expected scope in output:\n{output}");
        assert!(
            output.contains("Anchor: contracts/api.json:9-12"),
            "expected host-agnostic anchor text in output:\n{output}"
        );
    }

    #[test]
    fn conventional_comments_evidence_posture_mentions_scope_model() {
        let packet = ReviewPacket::from_diff(
            "main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-a\n+b\n",
        );
        let summary = crate::review::summary::ReviewSummary::from_packet(&packet, false);
        let output = render_pr_review_artifact("conventional-comments.md", &packet, &summary);
        assert!(output.contains("scope"), "evidence posture should mention scope:\n{output}");
        assert!(
            output.contains("Inline anchors appear only when persisted diff evidence resolves"),
            "updated posture text should explain anchor evidence bounds:\n{output}"
        );
    }

    #[test]
    fn render_review_artifacts_preserve_authored_status_sections() {
        let disposition = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\n## Review Target\n\n- bounded service boundary package.\n\n## Evidence Basis\n\n- owned interfaces, current tests, and rollback notes.\n\n## Final Disposition\n\nStatus: ready-with-review-notes\n\nRationale: the review packet is bounded enough for downstream inspection.\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.",
            "",
            "",
            "",
        );

        assert!(disposition.contains("## Final Disposition\n\nStatus: ready-with-review-notes"));
        assert!(disposition.contains(
            "## Accepted Risks\n\n- residual review notes remain bounded to this package."
        ));
    }

    #[test]
    fn render_review_artifacts_emit_missing_marker_for_absent_final_disposition() {
        let disposition = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.",
            "",
            "",
            "",
        );

        assert!(disposition.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(disposition.contains("`## Final Disposition`"));
    }

    #[test]
    fn render_verification_artifacts_preserve_authored_status_sections() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Contract Assumptions\n\n- rollback metadata remains explicit\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported\n\nRationale: the current evidence covers the authored claim set.",
            "",
            "",
            "",
        );

        assert!(report.contains("## Overall Verdict\n\nStatus: supported"));
        assert!(report.contains("## Verified Claims\n\n- rollback remains bounded and auditable"));
    }

    #[test]
    fn render_verification_artifacts_emit_missing_marker_for_absent_overall_verdict() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- none recorded",
            "",
            "",
            "",
        );

        assert!(report.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(report.contains("`## Overall Verdict`"));
    }

    #[test]
    fn render_verification_artifacts_preserve_multiline_sections() {
        let review = render_verification_artifact(
            "adversarial-review.md",
            "# Verification Brief\n\n## Challenge Findings\n\n- First challenge finding\n- Second challenge finding\n\n## Contradictions\n\n- First contradiction\n- Second contradiction",
            "",
            "",
            "",
        );

        assert!(
            review.contains("## Contradictions\n\n- First contradiction\n- Second contradiction")
        );
        assert!(review.contains(
            "## Challenge Findings\n\n- First challenge finding\n- Second challenge finding"
        ));
        assert!(!review.contains("- First contradiction - Second contradiction"));
    }

    #[test]
    fn render_incident_and_migration_artifacts_preserve_all_named_sections() {
        let incident_source = "# Incident Brief\n\n## Incident Scope\n\npayments-api and checkout flow only.\n\n## Trigger And Current State\n\nelevated 5xx responses after the deploy.\n\n## Operational Constraints\n\n- no autonomous remediation\n\n## Known Facts\n\n- rollback remains available\n\n## Working Hypotheses\n\n- retry amplification is exhausting the service\n\n## Evidence Gaps\n\n- saturation evidence is incomplete\n\n## Impacted Surfaces\n\n- payments-api\n\n## Propagation Paths\n\n- checkout request path\n\n## Confidence And Unknowns\n\n- medium confidence\n\n## Immediate Actions\n\n- disable retries\n\n## Ordered Sequence\n\n1. capture blast radius\n2. disable retries\n\n## Stop Conditions\n\n- error rate stabilizes\n\n## Decision Points\n\n- decide whether rollback is still required\n\n## Approved Actions\n\n- disable retries in the bounded surface\n\n## Deferred Actions\n\n- schema changes stay out of scope\n\n## Verification Checks\n\n- confirm 5xx rate drops\n\n## Release Readiness\n\n- remain recommendation-only until owner approval\n\n## Follow-Up Work\n\n- add a saturation dashboard\n";

        let hypothesis = render_incident_artifact("hypothesis-log.md", incident_source);
        let blast_radius = render_incident_artifact("blast-radius-map.md", incident_source);
        let containment = render_incident_artifact("containment-plan.md", incident_source);
        let decision = render_incident_artifact("incident-decision-record.md", incident_source);
        let follow_up = render_incident_artifact("follow-up-verification.md", incident_source);
        let incident_fallback = render_incident_artifact("custom-incident.md", incident_source);

        assert!(hypothesis.contains("## Known Facts\n\n- rollback remains available"));
        assert!(blast_radius.contains("## Propagation Paths\n\n- checkout request path"));
        assert!(containment.contains("## Stop Conditions\n\n- error rate stabilizes"));
        assert!(
            decision.contains("## Decision Points\n\n- decide whether rollback is still required")
        );
        assert!(
            follow_up.contains(
                "## Release Readiness\n\n- remain recommendation-only until owner approval"
            )
        );
        assert!(
            incident_fallback.starts_with("# custom-incident.md\n\n## Summary\n\n# Incident Brief")
        );

        let migration_source = "# Migration Brief\n\n## Current State\n\nauth-v1 serves login and token refresh traffic.\n\n## Target State\n\nauth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\nlogin and token refresh only.\n\n## Guaranteed Compatibility\n\n- existing tokens continue to validate\n\n## Temporary Incompatibilities\n\n- admin reporting stays on v1\n\n## Coexistence Rules\n\n- dual-write session metadata\n\n## Ordered Steps\n\n1. enable shadow reads\n2. start dual-write\n\n## Parallelizable Work\n\n- docs and dashboards can update in parallel\n\n## Cutover Criteria\n\n- token validation remains stable\n\n## Rollback Triggers\n\n- elevated login errors\n\n## Fallback Paths\n\n- route bounded traffic back to auth-v1\n\n## Re-Entry Criteria\n\n- regressions are resolved and revalidated\n\n## Verification Checks\n\n- login and token validation pass against auth-v2\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n\n## Release Readiness\n\n- keep recommendation-only posture until owner accepts the packet\n\n## Migration Decisions\n\n- retain dual-write during cutover\n\n## Deferred Decisions\n\n- move admin reporting later\n\n## Approval Notes\n\n- migration lead sign-off is required\n";

        let compatibility = render_migration_artifact("compatibility-matrix.md", migration_source);
        let sequencing = render_migration_artifact("sequencing-plan.md", migration_source);
        let fallback = render_migration_artifact("fallback-plan.md", migration_source);
        let verification =
            render_migration_artifact("migration-verification-report.md", migration_source);
        let decision = render_migration_artifact("decision-record.md", migration_source);
        let migration_fallback = render_migration_artifact("custom-migration.md", migration_source);

        assert!(compatibility.contains("## Coexistence Rules\n\n- dual-write session metadata"));
        assert!(sequencing.contains("## Cutover Criteria\n\n- token validation remains stable"));
        assert!(fallback.contains("## Fallback Paths\n\n- route bounded traffic back to auth-v1"));
        assert!(
            verification.contains(
                "## Residual Risks\n\n- admin reporting remains temporarily inconsistent"
            )
        );
        assert!(decision.contains("## Approval Notes\n\n- migration lead sign-off is required"));
        assert!(
            migration_fallback
                .starts_with("# custom-migration.md\n\n## Summary\n\n# Migration Brief")
        );
    }

    #[test]
    fn render_review_and_verification_auxiliary_artifacts_support_aliases_and_fallbacks() {
        let review_source = "# Review Brief\n\n## Review Target\n\n- bounded service boundary package\n\n## Evidence Basis\n\n- owned interfaces and rollback notes\n\n## Boundary Concern\n\n- a shared DTO crosses the intended boundary\n\n## Ownership Notes\n\n- reviewer owns the final decision\n\n## Open Concern\n\n- a production trace sample is still missing\n\n## Collection Priorities\n\n- capture one bounded trace\n\n## Pending Decision\n\n- decide whether the shared DTO remains acceptable\n\n## Reversibility Concerns\n\n- rollback semantics would be harder to preserve after wider adoption\n";

        let boundary = render_review_artifact("boundary-assessment.md", review_source, "", "", "");
        let missing = render_review_artifact("missing-evidence.md", review_source, "", "", "");
        let decision = render_review_artifact("decision-impact.md", review_source, "", "", "");
        let review_fallback = render_review_artifact("custom-review.md", review_source, "", "", "");

        assert!(
            boundary
                .contains("## Boundary Findings\n\n- a shared DTO crosses the intended boundary")
        );
        assert!(
            missing.contains("## Missing Evidence\n\n- a production trace sample is still missing")
        );
        assert!(
            decision.contains(
                "## Decision Impact\n\n- decide whether the shared DTO remains acceptable"
            )
        );
        assert!(review_fallback.starts_with("# custom-review.md\n\n## Summary\n\n# Review Brief"));

        let verification_source = "# Verification Brief\n\n## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Contract Surface\n\n- rollback metadata remains explicit\n\n## Verification Outcome\n\n- contract assumptions hold for the bounded target\n\n## Challenge Focus\n\n- stress the rollback metadata path\n\n## Contradictions\n\n- none recorded\n\n## Open Findings\n\n- add one more rollback stress probe\n\n## Required Follow-up\n\n- implement the additional probe before release readiness passes\n";

        let contract =
            render_verification_artifact("contract-matrix.md", verification_source, "", "", "");
        let adversarial =
            render_verification_artifact("adversarial-review.md", verification_source, "", "", "");
        let unresolved =
            render_verification_artifact("unresolved-findings.md", verification_source, "", "", "");
        let verification_fallback =
            render_verification_artifact("custom-verification.md", verification_source, "", "", "");

        assert!(
            contract.contains("## Contract Assumptions\n\n- rollback metadata remains explicit")
        );
        assert!(
            adversarial.contains("## Challenge Findings\n\n- stress the rollback metadata path")
        );
        assert!(unresolved.contains(
            "## Required Follow-Up\n\n- implement the additional probe before release readiness passes"
        ));
        assert!(
            verification_fallback
                .starts_with("# custom-verification.md\n\n## Summary\n\n# Verification Brief")
        );
    }

    #[test]
    fn render_analysis_mode_artifacts_include_required_sections() {
        let discovery = render_discovery_artifact(
            "context-boundary.md",
            "# Discovery Brief\n\n## Problem Domain\n\nExplore a bounded notification routing problem.\n\n## Repo Surface\n\n- src/router.rs\n- tests/router_contract.rs\n\n## Immediate Tensions\n\n- Retry ownership is still unclear.\n\n## Downstream Handoff\n\nTranslate this discovery packet into architecture mode with named boundaries.\n\n## Unknowns\n\n- Which caller owns retry policy?\n\n## Assumptions\n\n- Routing ownership should remain explicit.\n\n## Validation Targets\n\n- Check src/router.rs and tests/router_contract.rs.\n\n## Confidence Levels\n\n- Medium until retry ownership is explicit.\n\n## In-Scope Context\n\n- Notification routing only.\n\n## Out-of-Scope Context\n\n- No implementation edits.\n\n## Translation Trigger\n\nTranslate this discovery packet into architecture mode with named boundaries.\n\n## Options\n\n1. Stay in discovery.\n\n## Constraints\n\n- Preserve current routing ownership boundaries.\n\n## Recommended Direction\n\nStay bounded to routing ownership.\n\n## Next-Phase Shape\n\nUse architecture mode to capture boundary choices.\n\n## Pressure Points\n\n- Retry ownership remains unresolved.\n\n## Blocking Decisions\n\n- Decide where retry ownership lives.\n\n## Open Questions\n\n- Which caller owns retry policy?\n\n## Recommended Owner\n\n- routing-architect\n",
        );
        let system_shaping = render_system_shaping_artifact(
            "system-shape.md",
            "Shape a new notification delivery capability.",
            "Separate ingest, routing, and delivery responsibilities.",
            "Keep delivery phase boundaries explicit and reversible.",
        );
        let architecture = render_architecture_artifact(
            "tradeoff-matrix.md",
            "Evaluate architectural boundaries for routing state.",
            "Compare centralized and partitioned routing designs.",
            "Partitioned routing better preserves ownership boundaries.",
        );

        assert!(discovery.contains("## In-Scope Context"));
        assert!(discovery.contains("## Repo Surface"));
        assert!(discovery.contains("## Translation Trigger"));
        assert!(system_shaping.contains("## System Shape"));
        assert!(system_shaping.contains("## Boundary Decisions"));
        assert!(architecture.contains("## Evaluation Criteria"));
        assert!(architecture.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(architecture.contains("`## Why Not The Others`"));
    }

    #[test]
    fn render_backlog_fallback_artifacts_keep_missing_authored_body_explicit() {
        let brief = "# Backlog Brief\n\n## Delivery Intent\n\nPrepare a bounded delivery backlog for runtime honesty work.\n\n## Desired Granularity\n\nepic-plus-slice\n\n## Planning Horizon\n\nfeature 033\n\n## Source References\n\n- specs/033-reasoning-evidence-clarity/plan.md\n\n## Constraints\n\n- Stay above task level\n";
        let planning_context = sample_backlog_planning_context();

        let epic_tree = render_backlog_artifact("epic-tree.md", brief, &planning_context);
        let slices = render_backlog_artifact("delivery-slices.md", brief, &planning_context);
        let sequencing = render_backlog_artifact("sequencing-plan.md", brief, &planning_context);
        let anchors = render_backlog_artifact("acceptance-anchors.md", brief, &planning_context);
        let planning_risks = render_backlog_artifact("planning-risks.md", brief, &planning_context);

        assert!(epic_tree.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(epic_tree.contains("did not synthesize placeholder epics"));
        assert!(!epic_tree.contains("Epic 1:"));
        assert!(slices.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(slices.contains("did not synthesize example slices"));
        assert!(!slices.contains("Slice 1:"));
        assert!(sequencing.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(sequencing.contains("did not invent sequencing steps"));
        assert!(!sequencing.contains("1. Establish the bounded foundation"));
        assert!(anchors.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(anchors.contains("did not fabricate acceptance anchors"));
        assert!(!anchors.contains("Anchor A:"));
        assert!(planning_risks.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(planning_risks.contains("instead of inventing risk bullets"));
        assert!(!planning_risks.contains("Sequencing uncertainty:"));
    }

    #[test]
    fn render_backlog_artifacts_preserve_authored_sections_and_context_fallbacks() {
        let brief = "# Backlog Brief\n\n## Delivery Intent\n\nPrepare a bounded delivery backlog for runtime honesty work.\n\n## Epic Tree\n\n- Epic 1: Bound the runtime honesty surface.\n\n## Capability To Epic Map\n\n- Runtime evidence hygiene -> Epic 1\n\n## Dependency Map\n\n- Epic 1 depends on the upstream evidence packet.\n\n## Delivery Slices\n\n- Slice A: Preserve existing packet boundaries.\n\n## Sequencing Plan\n\n1. Stabilize the upstream packet.\n\n## Acceptance Anchors\n\n- Anchor A: operators can inspect the bounded slice.\n\n## Planning Risks\n\n- Risk: upstream packet drift could invalidate the slice.\n";
        let mut planning_context = sample_backlog_planning_context();
        planning_context.source_refs = vec![
            "specs/033-reasoning-evidence-clarity/plan.md".to_string(),
            "tech-docs/guides/skill-runtime-contracts.md".to_string(),
        ];
        planning_context.constraints = vec![
            "Stay above task level".to_string(),
            "Do not infer implementation steps".to_string(),
        ];
        planning_context.out_of_scope =
            vec!["Tracker tickets".to_string(), "Implementation edits".to_string()];
        planning_context.closure_assessment.findings = vec![ClosureFinding {
            category: "missing-evidence".to_string(),
            severity: ClosureFindingSeverity::Blocking,
            affected_scope: "upstream backlog brief".to_string(),
            recommended_followup: "attach the missing closure notes".to_string(),
        }];

        let epic_tree = render_backlog_artifact("epic-tree.md", brief, &planning_context);
        let capability_map =
            render_backlog_artifact("capability-to-epic-map.md", brief, &planning_context);
        let dependency_map = render_backlog_artifact("dependency-map.md", brief, &planning_context);
        let slices = render_backlog_artifact("delivery-slices.md", brief, &planning_context);
        let sequencing = render_backlog_artifact("sequencing-plan.md", brief, &planning_context);
        let anchors = render_backlog_artifact("acceptance-anchors.md", brief, &planning_context);
        let planning_risks = render_backlog_artifact("planning-risks.md", brief, &planning_context);
        let fallback = render_backlog_artifact("custom-note.md", brief, &planning_context);

        assert!(epic_tree.contains("## Epic Tree\n\n- Epic 1: Bound the runtime honesty surface."));
        assert!(
            capability_map
                .contains("## Capability Mapping\n\n- Runtime evidence hygiene -> Epic 1")
        );
        assert!(capability_map.contains("tech-docs/guides/skill-runtime-contracts.md"));
        assert!(capability_map.contains(
            "- [blocking] missing-evidence on upstream backlog brief. Follow-up: attach the missing closure notes"
        ));
        assert!(
            dependency_map
                .contains("## Dependencies\n\n- Epic 1 depends on the upstream evidence packet.")
        );
        assert!(
            slices
                .contains("## Delivery Slices\n\n- Slice A: Preserve existing packet boundaries.")
        );
        assert!(sequencing.contains("## Sequencing\n\n1. Stabilize the upstream packet."));
        assert!(anchors.contains(
            "## Acceptance Anchors\n\n- Anchor A: operators can inspect the bounded slice."
        ));
        assert!(planning_risks.contains(
            "## Planning Risks\n\n- Risk: upstream packet drift could invalidate the slice."
        ));
        assert!(fallback.starts_with("# custom-note.md\n\n## Summary\n\n# Backlog Brief"));
    }

    #[test]
    fn render_linear_mermaid_emits_missing_node_for_empty_body() {
        let result = render_linear_mermaid("Container View", "");
        assert!(result.contains("No structured view content was authored."));
        assert!(result.starts_with("flowchart LR"));
    }

    #[test]
    fn extract_paragraph_nodes_splits_on_double_newline() {
        let body = "First node\n\nSecond node\n\nThird node";
        let nodes = extract_paragraph_nodes(body);
        assert_eq!(nodes, vec!["First node", "Second node", "Third node"]);
    }

    #[test]
    fn extract_paragraph_nodes_ignores_empty_paragraphs() {
        let body = "Node A\n\n\n\nNode B";
        let nodes = extract_paragraph_nodes(body);
        assert_eq!(nodes, vec!["Node A", "Node B"]);
    }

    #[test]
    fn first_paragraph_skips_bullet_lines() {
        let body = "- bullet item\n\nActual paragraph";
        assert_eq!(first_paragraph(body), Some("Actual paragraph"));
    }

    #[test]
    fn first_paragraph_returns_none_for_all_bullet_body() {
        let body = "- first\n\n- second";
        assert_eq!(first_paragraph(body), None);
    }

    #[test]
    fn mermaid_label_replaces_quotes_and_newlines() {
        assert_eq!(mermaid_label("say \"hello\""), "say 'hello'");
        assert_eq!(mermaid_label("line one\nline two"), "line one line two");
    }

    #[test]
    fn architecture_artifact_enabled_returns_false_for_unmentioned_optional_views() {
        // component-view and dynamic-view are optional; they are disabled when their
        // authored section is absent from the context summary.
        assert!(!architecture_artifact_enabled("component-view.md", ""));
        assert!(!architecture_artifact_enabled("component-view.mmd", ""));
        assert!(!architecture_artifact_enabled("dynamic-view.md", ""));
        assert!(!architecture_artifact_enabled("dynamic-view.mmd", ""));
    }

    #[test]
    fn architecture_artifact_enabled_returns_true_for_required_views() {
        // System-context and container views are always enabled.
        assert!(architecture_artifact_enabled("system-context.md", ""));
        assert!(architecture_artifact_enabled("container-view.md", ""));
        assert!(architecture_artifact_enabled("architecture-decisions.md", ""));
    }
}
