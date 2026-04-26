//! Brief parsing for requirements and discovery modes.
//!
//! Owns `RequirementsBrief`, `DiscoveryBrief`, and all clarity-inspection
//! functions (missing context, clarification questions, reasoning signals).

use super::ClarificationQuestionSummary;
use super::context_parse::{
    condense_context_block, count_markdown_entries, extract_context_list, extract_context_marker,
    first_meaningful_line, infer_discovery_next_phase, render_repo_surface_block,
    split_context_items, truncate_context_excerpt,
};

// ── Simple list utilities (used by RequirementsBrief and clarity functions) ───

pub(crate) fn default_list(values: Vec<String>, fallback: &str) -> Vec<String> {
    if values.is_empty() { vec![fallback.to_string()] } else { values }
}

pub(crate) fn list_contains_missing_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("NOT CAPTURED"))
}

pub(crate) fn count_captured_list_items(values: &[String]) -> usize {
    values.iter().filter(|value| !value.contains("NOT CAPTURED")).count()
}

// ── RequirementsBrief ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct RequirementsBrief {
    pub(crate) problem: String,
    pub(crate) outcome: String,
    pub(crate) constraints: Vec<String>,
    pub(crate) tradeoffs: Vec<String>,
    pub(crate) out_of_scope: Vec<String>,
    pub(crate) open_questions: Vec<String>,
    pub(crate) source_refs: Vec<String>,
}

impl RequirementsBrief {
    pub(crate) fn from_context(context_summary: String, source_refs: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "intent", "goal"])
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["abstract"]))
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["subject"]))
                .map(|value| condense_context_block(&value, 420))
                .unwrap_or_else(|| {
                    "NOT CAPTURED - Provide a `## Problem` section in the requirements input."
                        .to_string()
                });
        let outcome = extract_context_marker(
            &context_summary,
            &normalized,
            &["outcome", "desired outcome", "success signal", "objective"],
        )
        .map(|value| condense_context_block(&value, 320))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Outcome` section in the requirements input.".to_string()
        });

        let constraints = extract_context_list(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "non-negotiables"],
        );
        let tradeoffs =
            extract_context_list(&context_summary, &normalized, &["tradeoffs", "tradeoff"]);
        let out_of_scope = extract_context_list(
            &context_summary,
            &normalized,
            &["out of scope", "out-of-scope", "scope cuts", "excluded"],
        );
        let open_questions = extract_context_list(
            &context_summary,
            &normalized,
            &["open questions", "questions", "unknowns", "risks"],
        );

        let mut brief = Self {
            problem,
            outcome,
            constraints: default_list(
                constraints,
                "NOT CAPTURED - Provide a `## Constraints` section in the requirements input.",
            ),
            tradeoffs: default_list(
                tradeoffs,
                "NOT CAPTURED - Provide a `## Tradeoffs` section in the requirements input.",
            ),
            out_of_scope: default_list(
                out_of_scope,
                "NOT CAPTURED - Provide a `## Out of Scope` or `## Scope Cuts` section in the requirements input.",
            ),
            open_questions,
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        };

        if brief.open_questions.is_empty() {
            brief.open_questions.extend(
                prioritized_requirements_clarification_questions(&brief, &context_summary)
                    .into_iter()
                    .take(4)
                    .map(|question| question.prompt),
            );
        }

        if brief.open_questions.is_empty() {
            brief
                .open_questions
                .push("Which downstream mode should consume this packet first?".to_string());
        }

        brief
    }

    pub(crate) fn summary(&self) -> String {
        let mut lines = vec![
            format!("Problem framing: {}", truncate_context_excerpt(&self.problem, 180)),
            format!("Desired outcome: {}", truncate_context_excerpt(&self.outcome, 180)),
        ];

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.join("\n")
    }
}

// ── DiscoveryBrief ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub(crate) struct DiscoveryBrief {
    pub(crate) context_summary: String,
    pub(crate) problem: String,
    pub(crate) constraints: String,
    pub(crate) repo_focus: String,
    pub(crate) unknowns: String,
    pub(crate) next_phase: String,
}

impl DiscoveryBrief {
    pub(crate) fn from_context(context_summary: String, repo_surfaces: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "problem domain"])
                .unwrap_or_else(|| {
                    let line = first_meaningful_line(&context_summary);
                    if line.contains("Bound the problem to") {
                        "NOT CAPTURED - Provide a `## Problem` section in the discovery brief."
                            .to_string()
                    } else {
                        line
                    }
                });
        let constraints = extract_context_marker(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "boundary"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Constraints` section in the discovery brief.".to_string()
        });
        let repo_focus = extract_context_marker(
            &context_summary,
            &normalized,
            &["repo focus", "repository focus", "current state", "system slice"],
        )
        .unwrap_or_else(|| {
            if repo_surfaces.is_empty() {
                "NOT CAPTURED - Provide a `## Repo Focus` section in the discovery brief."
                    .to_string()
            } else {
                format!(
                    "Ground discovery in these repository surfaces: {}",
                    repo_surfaces.join(", ")
                )
            }
        });
        let unknowns = extract_context_marker(
            &context_summary,
            &normalized,
            &["unknowns", "open questions", "risks"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide an `## Unknowns` section in the discovery brief.".to_string()
        });
        let next_phase = extract_context_marker(
            &context_summary,
            &normalized,
            &["next phase", "handoff", "translation trigger"],
        )
        .unwrap_or_else(|| {
            let inferred = infer_discovery_next_phase(&context_summary);
            if inferred.contains("Translate this packet") {
                "NOT CAPTURED - Provide a `## Next Phase` section in the discovery brief."
                    .to_string()
            } else {
                inferred
            }
        });

        Self { context_summary, problem, constraints, repo_focus, unknowns, next_phase }
    }

    pub(crate) fn generation_prompt(&self, repo_surfaces: &[String]) -> String {
        format!(
            "# Discovery Brief\n\n## Problem\n{}\n\n## Constraints\n{}\n\n## Repo Focus\n{}\n\n## Repo Surface\n{}\n\n## Unknowns\n{}\n\n## Next Phase\n{}",
            self.problem,
            self.constraints,
            self.repo_focus,
            render_repo_surface_block(repo_surfaces),
            self.unknowns,
            self.next_phase,
        )
    }

    pub(crate) fn critique_prompt(
        &self,
        generation_summary: &str,
        repo_surfaces: &[String],
    ) -> String {
        format!(
            "# Discovery Critique Target\n\n## Context Summary\n{}\n\n## Repo Surface\n{}\n\n## Generated Framing\n{}\n\n## Challenge\nCheck whether the generated framing stays anchored to the repository surfaces, preserves the stated constraints, and points to a concrete next governed mode.",
            self.context_summary,
            render_repo_surface_block(repo_surfaces),
            generation_summary,
        )
    }
}

// ── Clarification question helpers ────────────────────────────────────────────

pub(crate) fn push_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    if questions.iter().any(|question| question.prompt.eq_ignore_ascii_case(prompt)) {
        return;
    }

    questions.push(ClarificationQuestionSummary {
        id: id.to_string(),
        prompt: prompt.to_string(),
        rationale: rationale.to_string(),
        evidence: evidence.to_string(),
    });
}

pub(crate) fn is_default_requirements_open_question(question: &str) -> bool {
    question.eq_ignore_ascii_case("Which downstream mode should consume this packet first?")
}

pub(crate) fn question_prompt(question: &str) -> String {
    let trimmed = question.trim().trim_end_matches('.');
    if trimmed.ends_with('?') { trimmed.to_string() } else { format!("{trimmed}?") }
}

// ── Requirements clarity ──────────────────────────────────────────────────────

pub(crate) fn requirements_missing_context(brief: &RequirementsBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing explicit authored intent or operator goal.".to_string(),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        missing.push(
            "Outcome framing is missing an explicit success signal or bounded result.".to_string(),
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        missing.push(
            "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        missing.push(
            "Tradeoffs are incomplete; option evaluation would drift toward generic guidance."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        missing.push(
            "Scope cuts are incomplete; the packet does not yet name explicit exclusions."
                .to_string(),
        );
    }

    missing
}

pub(crate) fn prioritized_requirements_clarification_questions(
    brief: &RequirementsBrief,
    context_summary: &str,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();
    let first_line = first_meaningful_line(context_summary);

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-problem",
            "What bounded operator or engineering problem should this requirements packet frame?",
            "Without an explicit problem statement, later modes will optimize for the wrong boundary.",
            &format!("Current intake starts with: {first_line}"),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-outcome",
            "What explicit outcome or success signal should this packet preserve?",
            "A requirements packet needs a bounded success condition before tradeoffs or exclusions make sense.",
            "No authored `## Outcome` or equivalent success section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        push_clarification_question(
            &mut questions,
            "clarify-constraints",
            "Which constraints are non-negotiable for this work?",
            "Constraints determine whether downstream shaping stays repo-specific instead of becoming generic planning advice.",
            "No authored `## Constraints`, `## Constraint`, or `## Non-Negotiables` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        push_clarification_question(
            &mut questions,
            "clarify-tradeoffs",
            "Which tradeoffs are acceptable, and which ones are explicitly rejected?",
            "Tradeoffs anchor option evaluation and keep the packet honest about what the team is willing to sacrifice.",
            "No authored `## Tradeoffs` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        push_clarification_question(
            &mut questions,
            "clarify-scope-cuts",
            "What is explicitly out of scope or deferred for this packet?",
            "Scope cuts keep the packet bounded and prevent later modes from inventing extra work.",
            "No authored `## Out of Scope`, `## Scope Cuts`, or equivalent exclusions section was detected in the supplied inputs.",
        );
    }

    for (index, question) in brief.open_questions.iter().enumerate() {
        if question.contains("NOT CAPTURED") || is_default_requirements_open_question(question) {
            continue;
        }

        let prompt = question_prompt(question);
        push_clarification_question(
            &mut questions,
            &format!("authored-open-question-{}", index + 1),
            &prompt,
            "This question is already explicit in the supplied brief and should be resolved before the packet is treated as stable downstream input.",
            "Captured from the authored open-questions or unknowns section.",
        );
    }

    questions.truncate(5);
    questions
}

pub(crate) fn requirements_reasoning_signals(
    source_inputs: &[String],
    brief: &RequirementsBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Captured {} constraint point(s), {} tradeoff point(s), {} scope cut(s), and {} open question(s).",
            count_captured_list_items(&brief.constraints),
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.out_of_scope),
            count_captured_list_items(&brief.open_questions)
        ),
        if brief.problem.contains("NOT CAPTURED") || brief.outcome.contains("NOT CAPTURED") {
            "The problem/outcome pair is still incomplete, so downstream design would rely on interpretation instead of authored intent.".to_string()
        } else {
            "The problem/outcome pair is explicit enough to bound a requirements packet before downstream mode selection.".to_string()
        },
    ]
}

// ── Discovery clarity ─────────────────────────────────────────────────────────

pub(crate) fn discovery_summary(brief: &DiscoveryBrief) -> String {
    format!(
        "Problem framing: {}\nConstraints: {}\nRepo focus: {}\nNext phase: {}",
        truncate_context_excerpt(&brief.problem, 180),
        truncate_context_excerpt(&brief.constraints, 180),
        truncate_context_excerpt(&brief.repo_focus, 180),
        truncate_context_excerpt(&brief.next_phase, 180),
    )
}

pub(crate) fn discovery_missing_context(brief: &DiscoveryBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing; discovery still needs an explicit problem domain."
                .to_string(),
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        missing.push(
            "Constraints are missing; discovery does not yet name the boundary it must preserve."
                .to_string(),
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        missing.push(
            "Repository focus is missing; discovery is not yet anchored to concrete repo surfaces."
                .to_string(),
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        missing.push(
            "Unknowns are missing; discovery does not yet name the unresolved decision pressure points."
                .to_string(),
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        missing.push(
            "Next-phase handoff is missing; discovery does not yet say which downstream mode should consume the packet."
                .to_string(),
        );
    }

    missing
}

pub(crate) fn prioritized_discovery_clarification_questions(
    brief: &DiscoveryBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-problem",
            "What exact problem domain should discovery bound before handoff?",
            "Discovery needs a named problem domain so later packets do not drift across unrelated repository surfaces.",
            "No authored `## Problem` or `## Problem Domain` section was detected in the supplied discovery brief.",
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-constraints",
            "Which explicit constraints or boundary rules must discovery preserve?",
            "Constraints keep the discovery packet honest about what later modes are allowed to change or assume.",
            "No authored `## Constraints` or equivalent boundary section was detected in the supplied discovery brief.",
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-repo-focus",
            "Which repository surfaces, modules, or files should discovery stay anchored to?",
            "Repo focus determines whether discovery remains grounded in the actual workspace instead of generic planning language.",
            "No authored `## Repo Focus`, `## Repository Focus`, or `## System Slice` section was detected in the supplied discovery brief.",
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-unknowns",
            "Which unknowns or decision risks should discovery make explicit before handoff?",
            "Discovery becomes weaker when it names a boundary but not the unresolved questions that still matter.",
            "No authored `## Unknowns` or `## Open Questions` section was detected in the supplied discovery brief.",
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-next-phase",
            "Which downstream governed mode should consume this packet next, and under what trigger?",
            "A discovery packet needs a concrete translation path so the result remains actionable.",
            "No authored `## Next Phase`, `## Handoff`, or `## Translation Trigger` section was detected in the supplied discovery brief.",
        );
    }

    if !brief.unknowns.contains("NOT CAPTURED") {
        for (index, question) in split_context_items(&brief.unknowns).into_iter().enumerate() {
            if question.contains("NOT CAPTURED") {
                continue;
            }

            let prompt = question_prompt(&question);
            push_clarification_question(
                &mut questions,
                &format!("authored-discovery-question-{}", index + 1),
                &prompt,
                "This unknown is already explicit in the discovery brief and should stay visible before downstream translation.",
                "Captured from the authored unknowns or open-questions section.",
            );
        }
    }

    questions.truncate(5);
    questions
}

pub(crate) fn discovery_reasoning_signals(
    source_inputs: &[String],
    repo_surfaces: &[String],
    brief: &DiscoveryBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Mapped {} repository surface hint(s) for discovery anchoring.",
            repo_surfaces.len()
        ),
        format!(
            "Captured {} unknown or open-question item(s) and inferred next phase `{}`.",
            if brief.unknowns.contains("NOT CAPTURED") {
                0
            } else {
                count_markdown_entries(&brief.unknowns)
            },
            truncate_context_excerpt(&brief.next_phase, 96)
        ),
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
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
}
