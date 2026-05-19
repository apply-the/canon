use super::*;

fn well_formed_requirements_brief() -> RequirementsBrief {
    RequirementsBrief {
        problem: "Reduce auth latency.".to_string(),
        outcome: "P99 auth latency under 50 ms.".to_string(),
        constraints: vec!["No breaking API changes.".to_string()],
        tradeoffs: vec!["Cache consistency vs latency.".to_string()],
        out_of_scope: vec!["UI changes.".to_string()],
        open_questions: vec!["Which cache backend?".to_string()],
        source_refs: vec!["canon-input/requirements.md".to_string()],
    }
}

fn empty_requirements_brief() -> RequirementsBrief {
    RequirementsBrief {
        problem: "NOT CAPTURED - missing.".to_string(),
        outcome: "NOT CAPTURED - missing.".to_string(),
        constraints: vec!["NOT CAPTURED - missing.".to_string()],
        tradeoffs: vec!["NOT CAPTURED - missing.".to_string()],
        out_of_scope: vec!["NOT CAPTURED - missing.".to_string()],
        open_questions: Vec::new(),
        source_refs: Vec::new(),
    }
}

#[test]
fn requirements_missing_context_returns_empty_for_complete_brief() {
    let brief = well_formed_requirements_brief();
    assert!(requirements_missing_context(&brief).is_empty());
}

#[test]
fn requirements_missing_context_lists_all_gaps() {
    let brief = empty_requirements_brief();
    let missing = requirements_missing_context(&brief);
    assert!(missing.len() >= 3, "should detect multiple missing items: {missing:?}");
    assert!(missing.iter().any(|m| m.contains("Problem")));
    assert!(missing.iter().any(|m| m.contains("Outcome")));
}

#[test]
fn prioritized_requirements_clarification_questions_capped_at_five() {
    let brief = empty_requirements_brief();
    let questions = prioritized_requirements_clarification_questions(&brief, "");
    assert!(questions.len() <= 5);
}

#[test]
fn question_prompt_adds_question_mark_when_missing() {
    assert_eq!(question_prompt("What is the problem"), "What is the problem?");
    assert_eq!(question_prompt("What is the problem?"), "What is the problem?");
}

#[test]
fn push_clarification_question_deduplicates_by_prompt() {
    let mut questions = Vec::new();
    push_clarification_question(&mut questions, "id-1", "What is the problem?", "r", "e");
    push_clarification_question(&mut questions, "id-2", "What is the problem?", "r2", "e2");
    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].status, "required");
}

#[test]
fn requirements_brief_summary_includes_problem_and_outcome() {
    let brief = well_formed_requirements_brief();
    let summary = brief.summary();
    assert!(summary.contains("Problem framing:"));
    assert!(summary.contains("Desired outcome:"));
    assert!(summary.contains("Source inputs:"));
}

#[test]
fn discovery_missing_context_returns_empty_for_complete_brief() {
    let brief = DiscoveryBrief {
        context_summary: "context".to_string(),
        problem: "Real problem.".to_string(),
        constraints: "Must not break auth.".to_string(),
        repo_focus: "crates/canon-engine".to_string(),
        unknowns: "Unknown concurrency model.".to_string(),
        next_phase: "Move to architecture.".to_string(),
    };
    assert!(discovery_missing_context(&brief).is_empty());
}

#[test]
fn discovery_missing_context_detects_not_captured_fields() {
    let brief = DiscoveryBrief {
        context_summary: "context".to_string(),
        problem: "NOT CAPTURED - missing".to_string(),
        constraints: "NOT CAPTURED - missing".to_string(),
        repo_focus: "focused".to_string(),
        unknowns: "some unknowns".to_string(),
        next_phase: "requirements".to_string(),
    };
    let missing = discovery_missing_context(&brief);
    assert_eq!(missing.len(), 2);
}

#[test]
fn requirements_reasoning_signals_produces_three_items() {
    let brief = well_formed_requirements_brief();
    let signals = requirements_reasoning_signals(&["canon-input/reqs.md".to_string()], &brief);
    assert_eq!(signals.len(), 3);
    assert!(signals[0].contains("1 authored input surface"));
}

#[test]
fn count_captured_list_items_counts_non_not_captured() {
    let items = vec![
        "real item".to_string(),
        "NOT CAPTURED - missing".to_string(),
        "another real".to_string(),
    ];
    assert_eq!(count_captured_list_items(&items), 2);
}

#[test]
fn list_contains_missing_markers_detects_not_captured() {
    assert!(list_contains_missing_markers(&["NOT CAPTURED - x".to_string()]));
    assert!(!list_contains_missing_markers(&["valid item".to_string()]));
}

#[test]
fn default_list_returns_fallback_for_empty_vec() {
    let result = default_list(Vec::new(), "fallback value");
    assert_eq!(result, vec!["fallback value".to_string()]);
}

#[test]
fn default_list_returns_original_when_non_empty() {
    let result = default_list(vec!["item".to_string()], "fallback");
    assert_eq!(result, vec!["item".to_string()]);
}

#[test]
fn authored_mode_reasoning_signals_detect_materially_closed_decisions() {
    let brief = AuthoredModeBrief {
        mode: Mode::Architecture,
        family: AuthoredClarityFamily::Planning,
        primary_subject: "Split artifact rendering from summary posture.".to_string(),
        boundary: "Keep existing .canon schema unchanged.".to_string(),
        support_evidence: "Shared posture avoids drift across modes.".to_string(),
        decision_state: "Use shared posture helpers in the runtime layer.".to_string(),
        preserved_boundary: "NOT APPLICABLE".to_string(),
        options: vec!["Shared helper".to_string()],
        tradeoffs: vec!["Less per-mode wording freedom.".to_string()],
        questions_or_gaps: Vec::new(),
        source_refs: vec!["canon-input/architecture.md".to_string()],
    };

    let signals = authored_mode_reasoning_signals(&brief.source_refs, &brief);
    assert!(signals.iter().any(|signal| signal.contains("materially closes the decision")));
}

#[test]
fn clarity_output_quality_distinguishes_structural_and_publishable_posture() {
    let weak = clarity_output_quality(
        false,
        &["Planning support is missing.".to_string()],
        &[],
        &["Detected 1 authored input surface(s): canon-input/change.md.".to_string()],
    );
    assert_eq!(weak.posture, "structurally-complete");
    assert!(!weak.downgrade_reasons.is_empty());

    let publishable = clarity_output_quality(
        true,
        &[],
        &[],
        &["Detected 1 authored input surface(s): architecture.md.".to_string()],
    );
    assert_eq!(publishable.posture, "publishable");
    assert!(publishable.materially_closed);
    assert!(!publishable.evidence_signals.is_empty());
}

#[test]
fn authored_mode_missing_context_flags_execution_preservation_gaps() {
    let brief = AuthoredModeBrief {
        mode: Mode::Implementation,
        family: AuthoredClarityFamily::Execution,
        primary_subject: "Implement shared clarity helpers.".to_string(),
        boundary: "crates/canon-engine/src/orchestrator/service".to_string(),
        support_evidence: "inspect_clarity targeted tests".to_string(),
        decision_state: "Use shared authored-mode parsing.".to_string(),
        preserved_boundary: authored_preserved_fallback().to_string(),
        options: Vec::new(),
        tradeoffs: Vec::new(),
        questions_or_gaps: Vec::new(),
        source_refs: vec!["canon-input/implementation.md".to_string()],
    };

    let missing = authored_mode_missing_context(&brief);
    assert!(missing.iter().any(|item| item.contains("Preserved behavior")));
}

#[test]
fn architecture_reroute_guidance_prefers_discovery_for_unbounded_briefs() {
    let brief = AuthoredModeBrief {
        mode: Mode::Architecture,
        family: AuthoredClarityFamily::Planning,
        primary_subject: authored_primary_fallback(AuthoredClarityFamily::Planning).to_string(),
        boundary: authored_boundary_fallback(AuthoredClarityFamily::Planning).to_string(),
        support_evidence: authored_support_fallback(AuthoredClarityFamily::Planning).to_string(),
        decision_state: authored_decision_fallback(AuthoredClarityFamily::Planning).to_string(),
        preserved_boundary: "NOT APPLICABLE".to_string(),
        options: Vec::new(),
        tradeoffs: Vec::new(),
        questions_or_gaps: Vec::new(),
        source_refs: vec!["architecture.md".to_string()],
    };

    let guidance = architecture_reroute_guidance(&brief).expect("reroute guidance");
    assert!(guidance.contains("discovery"));
}

#[test]
fn supply_chain_analysis_brief_from_context_preserves_authored_sections() {
    let source_refs = vec!["canon-input/supply-chain-analysis.md".to_string()];
    let brief = SupplyChainAnalysisBrief::from_context(
        "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests under crates/ and GitHub Actions workflows.\n\n## Licensing Posture\noss-permissive\n\n## Distribution Model\nexternal distribution\n\n## Ecosystems In Scope\n- cargo\n- github actions\n\n## Out Of Scope Components\n- vendored ui assets\n\n## Scanner Decisions\n- prefer OSS scanners first\n"
            .to_string(),
        &source_refs,
    );

    assert_eq!(brief.declared_scope, "Cargo manifests under crates/ and GitHub Actions workflows.");
    assert_eq!(brief.licensing_posture, "oss-permissive");
    assert_eq!(brief.distribution_model, "external distribution");
    assert_eq!(brief.ecosystems_in_scope.len(), 2);
    assert_eq!(brief.out_of_scope_components, vec!["vendored ui assets".to_string()]);
    assert_eq!(brief.scanner_decisions, vec!["prefer OSS scanners first".to_string()]);

    let summary = brief.summary();
    assert!(summary.contains("Declared scope:"));
    assert!(summary.contains("Source inputs: canon-input/supply-chain-analysis.md"));
    assert!(summary.contains("Ecosystems in scope: 2"));
}

#[test]
fn supply_chain_analysis_questions_cover_unresolved_decisions() {
    let brief = SupplyChainAnalysisBrief::from_context(
        "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests under crates/.\n"
            .to_string(),
        &[],
    );

    let missing = supply_chain_analysis_missing_context(&brief);
    assert_eq!(missing.len(), 5);
    assert!(missing.iter().any(|item| item.contains("Licensing posture is unresolved")));
    assert!(missing.iter().any(|item| item.contains("Distribution model is unresolved")));
    assert!(missing.iter().any(|item| item.contains("Ecosystem scope is unresolved")));
    assert!(missing.iter().any(|item| item.contains("Scanner policy is unresolved")));

    let questions = prioritized_supply_chain_analysis_clarification_questions(&brief);
    assert_eq!(questions.len(), 5);
    assert!(questions.iter().any(|question| {
        question.prompt.contains("What licensing posture governs this repository surface")
    }));
    assert!(
        questions
            .iter()
            .any(|question| { question.prompt.contains("Are non-OSS scanner proposals allowed") })
    );
}

#[test]
fn supply_chain_analysis_reasoning_signals_surface_incomplete_policy_markers() {
    let source_refs = vec!["supply-chain-analysis.md".to_string()];
    let brief = SupplyChainAnalysisBrief::from_context(
        "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests only.\n".to_string(),
        &source_refs,
    );

    let signals = supply_chain_analysis_reasoning_signals(&source_refs, &brief);
    assert!(
        signals
            .iter()
            .any(|signal| signal.contains("Source inputs inspected: supply-chain-analysis.md"))
    );
    assert!(signals.iter().any(|signal| signal.contains("Captured ecosystems in scope: 1")));
    assert!(signals.iter().any(|signal| {
        signal.contains("Licensing posture still requires explicit maintainer confirmation")
    }));
    assert!(signals.iter().any(|signal| {
        signal.contains("Distribution model still requires explicit maintainer confirmation")
    }));
    assert!(signals.iter().any(|signal| signal.contains("Scanner policy remains incomplete")));
}

#[test]
fn requirements_brief_from_context_backfills_open_questions_when_none_are_authored() {
    let source_refs = vec!["idea.md".to_string()];
    let brief = RequirementsBrief::from_context(
        "# Requirements Brief\n\n## Problem\nReduce auth latency.\n\n## Outcome\nP99 auth latency under 50 ms.\n".to_string(),
        &source_refs,
    );

    assert_eq!(brief.source_refs, source_refs);
    assert_eq!(brief.problem, "Reduce auth latency.");
    assert_eq!(brief.outcome, "P99 auth latency under 50 ms.");
    assert!(brief.constraints[0].contains("Provide a `## Constraints` section"));
    assert!(brief.tradeoffs[0].contains("Provide a `## Tradeoffs` section"));
    assert!(!brief.open_questions.is_empty());
    assert!(brief.open_questions.len() <= 4);
}

#[test]
fn discovery_brief_from_context_uses_repo_surface_fallbacks_and_prompt_helpers() {
    let repo_surfaces = vec!["crates/canon-engine/src/orchestrator/service".to_string()];
    let brief = DiscoveryBrief::from_context(
        "Need to bound orchestrator coverage before broad runtime work.\n\n## Constraints\nStay inside direct runtime tests first.\n\n## Unknowns\nWhich slices still need direct coverage?\n".to_string(),
        &repo_surfaces,
    );

    assert_eq!(brief.problem, "Need to bound orchestrator coverage before broad runtime work.");
    assert!(brief.repo_focus.contains("crates/canon-engine/src/orchestrator/service"));
    assert!(
        brief.next_phase.contains("Provide a `## Next Phase` section")
            || !brief.next_phase.is_empty()
    );

    let generation = brief.generation_prompt(&repo_surfaces);
    assert!(generation.contains("## Repo Surface"));
    assert!(generation.contains("Stay inside direct runtime tests first."));

    let critique = brief.critique_prompt("Generated discovery summary.", &repo_surfaces);
    assert!(critique.contains("## Generated Framing"));
    assert!(critique.contains("Check whether the generated framing stays anchored"));
}

#[test]
fn authored_mode_brief_for_review_uses_assessment_family_and_detects_weak_reasoning() {
    let brief = AuthoredModeBrief::from_context(
        Mode::Review,
        "# Review Brief\n\n## Review Target\nAuth session rollback readiness.\n\n## Assessment Scope\nAuth session boundary only.\n\n## Evidence Basis\nContract tests and rollback rehearsal notes.\n\n## Evidence Gaps\n- Should the review reject the packet until rollback rehearsal is refreshed?\n"
            .to_string(),
        &["review.md".to_string()],
    );

    assert_eq!(brief.family, AuthoredClarityFamily::Assessment);
    assert!(brief.summary().contains("Assessment target: Auth session rollback readiness."));
    assert!(brief.summary().contains("Scope boundary: Auth session boundary only."));
    assert!(brief.weak_reasoning());
    assert!(!brief.materially_closed());
}

#[test]
fn prioritized_authored_mode_clarification_questions_for_review_add_disposition_and_gap_items() {
    let brief = AuthoredModeBrief {
        mode: Mode::Review,
        family: AuthoredClarityFamily::Assessment,
        primary_subject: "Auth session rollback readiness.".to_string(),
        boundary: "Auth session boundary only.".to_string(),
        support_evidence: "Contract tests and rollback rehearsal notes.".to_string(),
        decision_state: authored_decision_fallback(AuthoredClarityFamily::Assessment).to_string(),
        preserved_boundary: "NOT APPLICABLE".to_string(),
        options: Vec::new(),
        tradeoffs: Vec::new(),
        questions_or_gaps: vec![
            "Should the review reject the packet until rollback rehearsal is refreshed?"
                .to_string(),
        ],
        source_refs: vec!["review.md".to_string()],
    };

    let questions = prioritized_authored_mode_clarification_questions(&brief);
    assert!(questions.iter().any(|question| question.id == "clarify-authored-disposition"));
    assert!(questions.iter().any(|question| {
        question
            .prompt
            .contains("Should the review reject the packet until rollback rehearsal is refreshed")
    }));
}

#[test]
fn authored_mode_recommended_focus_handles_materially_closed_and_question_only_packets() {
    let materially_closed = AuthoredModeBrief {
        mode: Mode::Architecture,
        family: AuthoredClarityFamily::Planning,
        primary_subject: "Split artifact rendering from runtime posture.".to_string(),
        boundary: "Keep the runtime schema unchanged.".to_string(),
        support_evidence: "Existing packets already share the same runtime contract.".to_string(),
        decision_state: "Use shared posture helpers in the runtime layer.".to_string(),
        preserved_boundary: "NOT APPLICABLE".to_string(),
        options: vec!["Use shared posture helpers.".to_string()],
        tradeoffs: vec!["Less per-mode wording freedom.".to_string()],
        questions_or_gaps: Vec::new(),
        source_refs: vec!["architecture.md".to_string()],
    };
    assert!(
        authored_mode_recommended_focus(&materially_closed, &[], &[])
            .contains("materially closes the decision")
    );

    let question_only = AuthoredModeBrief {
        mode: Mode::Change,
        family: AuthoredClarityFamily::Planning,
        primary_subject: "Bound the auth repository slice.".to_string(),
        boundary: "Auth service and repository only.".to_string(),
        support_evidence: "Contract checks already protect audit ordering.".to_string(),
        decision_state: authored_decision_fallback(AuthoredClarityFamily::Planning).to_string(),
        preserved_boundary: "NOT APPLICABLE".to_string(),
        options: vec!["Keep repository helper local to auth.".to_string()],
        tradeoffs: vec!["Some duplication remains inside auth.".to_string()],
        questions_or_gaps: vec!["Should cleanup rollout stay in the same slice?".to_string()],
        source_refs: vec!["change.md".to_string()],
    };
    let questions = prioritized_authored_mode_clarification_questions(&question_only);
    assert!(authored_mode_missing_context(&question_only).is_empty());
    assert!(
        authored_mode_recommended_focus(&question_only, &[], &questions).contains(
            "Review the remaining authored planning questions before starting change mode"
        )
    );
}

#[test]
fn authored_clarity_family_maps_modes_and_labels_across_profiles() {
    assert_eq!(AuthoredClarityFamily::Planning.label(), "planning");
    assert_eq!(AuthoredClarityFamily::Execution.label(), "execution");
    assert_eq!(AuthoredClarityFamily::Assessment.label(), "assessment");

    assert_eq!(authored_clarity_family(Mode::Requirements), AuthoredClarityFamily::Planning);
    assert_eq!(authored_clarity_family(Mode::Implementation), AuthoredClarityFamily::Execution);
    assert_eq!(authored_clarity_family(Mode::Migration), AuthoredClarityFamily::Execution);
    assert_eq!(authored_clarity_family(Mode::Verification), AuthoredClarityFamily::Assessment);
    assert_eq!(
        authored_clarity_family(Mode::SecurityAssessment),
        AuthoredClarityFamily::Assessment
    );
    assert_eq!(authored_clarity_family(Mode::SystemAssessment), AuthoredClarityFamily::Assessment);
    assert_eq!(authored_clarity_family(Mode::DomainLanguage), AuthoredClarityFamily::Assessment);
    assert_eq!(authored_clarity_family(Mode::DomainModel), AuthoredClarityFamily::Assessment);
}

#[test]
fn authored_family_profiles_expose_execution_and_assessment_tables() {
    let execution = AuthoredClarityFamily::Execution;
    assert!(authored_primary_markers(execution).contains(&"task mapping"));
    assert!(authored_boundary_markers(execution).contains(&"mutation bounds"));
    assert!(authored_support_markers(execution).contains(&"verification checks"));
    assert!(authored_decision_markers(execution).contains(&"migration decisions"));
    assert!(authored_tradeoff_markers(execution).contains(&"temporary incompatibilities"));
    assert!(authored_option_markers(execution).contains(&"options matrix"));
    assert!(authored_gap_markers(execution).contains(&"feature audit"));
    assert!(authored_primary_fallback(execution).contains("Task Mapping"));
    assert!(authored_boundary_fallback(execution).contains("Mutation Bounds"));
    assert!(authored_support_fallback(execution).contains("Verification Checks"));
    assert!(authored_decision_fallback(execution).contains("Migration Decisions"));
    assert!(
        authored_missing_primary_subject_message(execution).contains("Execution target is missing")
    );
    assert!(authored_missing_boundary_message(execution).contains("Mutation boundary is missing"));
    assert!(authored_missing_support_message(execution).contains("Execution evidence is missing"));
    assert!(
        authored_target_prompt(execution)
            .contains("implementation, refactor, or migration surface")
    );
    assert!(
        authored_boundary_prompt(execution)
            .contains("paths, mutation bounds, or transition boundaries")
    );
    assert!(
        authored_support_prompt(execution).contains("safety-net, validation, or rollback evidence")
    );

    let assessment = AuthoredClarityFamily::Assessment;
    assert!(authored_primary_markers(assessment).contains(&"review target"));
    assert!(authored_boundary_markers(assessment).contains(&"assessment scope"));
    assert!(authored_support_markers(assessment).contains(&"evidence basis"));
    assert!(authored_decision_markers(assessment).contains(&"final disposition"));
    assert!(authored_tradeoff_markers(assessment).contains(&"accepted risks"));
    assert!(authored_option_markers(assessment).is_empty());
    assert!(authored_gap_markers(assessment).contains(&"evidence gaps"));
    assert!(authored_primary_fallback(assessment).contains("Review Target"));
    assert!(authored_boundary_fallback(assessment).contains("Assessment Scope"));
    assert!(authored_support_fallback(assessment).contains("Evidence Basis"));
    assert!(authored_decision_fallback(assessment).contains("Disposition"));
    assert!(
        authored_missing_primary_subject_message(assessment)
            .contains("Assessment target is missing")
    );
    assert!(
        authored_missing_boundary_message(assessment).contains("Assessment boundary is missing")
    );
    assert!(authored_missing_support_message(assessment).contains("Evidence basis is missing"));
    assert!(
        authored_target_prompt(assessment)
            .contains("review, verification, incident, or assessment target")
    );
    assert!(authored_boundary_prompt(assessment).contains("evidence surfaces are in scope"));
    assert!(authored_support_prompt(assessment).contains("evidence basis supports this packet"));
}
