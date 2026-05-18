//! Brief parsing and clarity helpers for inspectable authored modes.
//!
//! Owns mode-specific brief parsers plus the shared clarity-inspection
//! functions (missing context, clarification questions, reasoning signals).

use self::authored_family::{
    AuthoredClarityFamily, authored_boundary_fallback, authored_boundary_markers,
    authored_boundary_prompt, authored_clarity_family, authored_decision_fallback,
    authored_decision_markers, authored_gap_markers, authored_missing_boundary_message,
    authored_missing_primary_subject_message, authored_missing_support_message,
    authored_option_markers, authored_preserved_fallback, authored_preserved_markers,
    authored_primary_fallback, authored_primary_markers, authored_support_fallback,
    authored_support_markers, authored_support_prompt, authored_target_prompt,
    authored_tradeoff_markers,
};
pub(crate) use self::mode_briefs::{
    DiscoveryBrief, RequirementsBrief, SupplyChainAnalysisBrief, discovery_missing_context,
    discovery_reasoning_signals, discovery_summary, prioritized_discovery_clarification_questions,
    prioritized_requirements_clarification_questions,
    prioritized_supply_chain_analysis_clarification_questions, requirements_missing_context,
    requirements_reasoning_signals, supply_chain_analysis_missing_context,
    supply_chain_analysis_reasoning_signals,
};
use super::context_parse::{
    condense_context_block, count_markdown_entries, extract_context_list, extract_context_marker,
    first_meaningful_line, infer_discovery_next_phase, render_repo_surface_block,
    split_context_items, truncate_context_excerpt,
};
use super::{ClarificationQuestionSummary, OutputQualitySummary};
use crate::domain::mode::Mode;

mod authored_family;
mod mode_briefs;

// ── Simple list utilities (used by RequirementsBrief and clarity functions) ───

pub(crate) fn default_list(values: Vec<String>, fallback: &str) -> Vec<String> {
    if values.is_empty() { vec![fallback.to_string()] } else { values }
}

pub(crate) fn list_contains_missing_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("NOT CAPTURED"))
}

pub(crate) fn list_contains_missing_decision_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("MISSING AUTHORED DECISION"))
}

pub(crate) fn count_captured_list_items(values: &[String]) -> usize {
    values.iter().filter(|value| !value.contains("NOT CAPTURED")).count()
}

fn has_authored_value(value: &str) -> bool {
    !value.contains("NOT CAPTURED") && !value.contains("MISSING AUTHORED")
}

fn required_context_marker(
    source: &str,
    normalized: &str,
    markers: &[&str],
    fallback: &str,
    max_chars: usize,
) -> String {
    extract_context_marker(source, normalized, markers)
        .map(|value| condense_context_block(&value, max_chars))
        .unwrap_or_else(|| fallback.to_string())
}

// ── Shared authored-mode clarity ────────────────────────────────────────────

fn is_authored_gap_question(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.contains('?') {
        return true;
    }

    let normalized = trimmed.to_lowercase();
    [
        "what ", "which ", "how ", "who ", "when ", "where ", "should ", "can ", "is ", "are ",
        "does ", "do ",
    ]
    .iter()
    .any(|prefix| normalized.starts_with(prefix))
}

#[derive(Debug, Clone)]
pub(crate) struct AuthoredModeBrief {
    pub(crate) mode: Mode,
    pub(crate) family: AuthoredClarityFamily,
    pub(crate) primary_subject: String,
    pub(crate) boundary: String,
    pub(crate) support_evidence: String,
    pub(crate) decision_state: String,
    pub(crate) preserved_boundary: String,
    pub(crate) options: Vec<String>,
    pub(crate) tradeoffs: Vec<String>,
    pub(crate) questions_or_gaps: Vec<String>,
    pub(crate) source_refs: Vec<String>,
}

impl AuthoredModeBrief {
    pub(crate) fn from_context(
        mode: Mode,
        context_summary: String,
        source_refs: &[String],
    ) -> Self {
        let family = authored_clarity_family(mode);
        let normalized = context_summary.to_lowercase();

        Self {
            mode,
            family,
            primary_subject: required_context_marker(
                &context_summary,
                &normalized,
                authored_primary_markers(family),
                authored_primary_fallback(family),
                320,
            ),
            boundary: required_context_marker(
                &context_summary,
                &normalized,
                authored_boundary_markers(family),
                authored_boundary_fallback(family),
                320,
            ),
            support_evidence: required_context_marker(
                &context_summary,
                &normalized,
                authored_support_markers(family),
                authored_support_fallback(family),
                320,
            ),
            decision_state: required_context_marker(
                &context_summary,
                &normalized,
                authored_decision_markers(family),
                authored_decision_fallback(family),
                260,
            ),
            preserved_boundary: if matches!(family, AuthoredClarityFamily::Execution) {
                required_context_marker(
                    &context_summary,
                    &normalized,
                    authored_preserved_markers(),
                    authored_preserved_fallback(),
                    260,
                )
            } else {
                "NOT APPLICABLE".to_string()
            },
            options: extract_context_list(
                &context_summary,
                &normalized,
                authored_option_markers(family),
            ),
            tradeoffs: extract_context_list(
                &context_summary,
                &normalized,
                authored_tradeoff_markers(family),
            ),
            questions_or_gaps: extract_context_list(
                &context_summary,
                &normalized,
                authored_gap_markers(family),
            ),
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        }
    }

    pub(crate) fn materially_closed(&self) -> bool {
        has_authored_value(&self.decision_state) && count_captured_list_items(&self.options) <= 1
    }

    pub(crate) fn weak_reasoning(&self) -> bool {
        if !has_authored_value(&self.boundary) || !has_authored_value(&self.support_evidence) {
            return true;
        }

        match self.family {
            AuthoredClarityFamily::Planning => {
                count_captured_list_items(&self.options) == 0
                    && !has_authored_value(&self.decision_state)
            }
            AuthoredClarityFamily::Execution => !has_authored_value(&self.preserved_boundary),
            AuthoredClarityFamily::Assessment => {
                matches!(self.mode, Mode::Review | Mode::Verification)
                    && !has_authored_value(&self.decision_state)
            }
        }
    }

    pub(crate) fn summary(&self) -> String {
        let mut lines = match self.family {
            AuthoredClarityFamily::Planning => vec![
                format!("Planning focus: {}", truncate_context_excerpt(&self.primary_subject, 180)),
                format!("Boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                if has_authored_value(&self.decision_state) {
                    format!(
                        "Decision posture: {}",
                        truncate_context_excerpt(&self.decision_state, 180)
                    )
                } else {
                    format!(
                        "Decision posture: {} option signal(s), {} tradeoff signal(s)",
                        count_captured_list_items(&self.options),
                        count_captured_list_items(&self.tradeoffs)
                    )
                },
            ],
            AuthoredClarityFamily::Execution => vec![
                format!(
                    "Execution focus: {}",
                    truncate_context_excerpt(&self.primary_subject, 180)
                ),
                format!("Mutation boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                format!(
                    "Preservation posture: {}",
                    truncate_context_excerpt(&self.preserved_boundary, 180)
                ),
            ],
            AuthoredClarityFamily::Assessment => vec![
                format!(
                    "Assessment target: {}",
                    truncate_context_excerpt(&self.primary_subject, 180)
                ),
                format!("Scope boundary: {}", truncate_context_excerpt(&self.boundary, 180)),
                if has_authored_value(&self.decision_state) {
                    format!(
                        "Review posture: {}",
                        truncate_context_excerpt(&self.decision_state, 180)
                    )
                } else {
                    format!(
                        "Evidence basis: {}",
                        truncate_context_excerpt(&self.support_evidence, 180)
                    )
                },
            ],
        };

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.join("\n")
    }
}

pub(crate) fn authored_mode_missing_context(brief: &AuthoredModeBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if !has_authored_value(&brief.primary_subject) {
        missing.push(authored_missing_primary_subject_message(brief.family));
    }

    if !has_authored_value(&brief.boundary) {
        missing.push(authored_missing_boundary_message(brief.family));
    }

    if !has_authored_value(&brief.support_evidence) {
        missing.push(authored_missing_support_message(brief.family));
    }

    append_family_specific_missing_context(brief, &mut missing);

    missing
}

pub(crate) fn prioritized_authored_mode_clarification_questions(
    brief: &AuthoredModeBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if !has_authored_value(&brief.primary_subject) {
        push_clarification_question(
            &mut questions,
            "clarify-authored-target",
            authored_target_prompt(brief.family),
            "Without a bounded target, Canon cannot tell whether the packet is actually reasoning about the intended surface.",
            authored_primary_fallback(brief.family),
        );
    }

    if !has_authored_value(&brief.boundary) {
        push_clarification_question(
            &mut questions,
            "clarify-authored-boundary",
            authored_boundary_prompt(brief.family),
            "Boundaries keep the packet honest about what Canon is allowed to interpret or recommend.",
            authored_boundary_fallback(brief.family),
        );
    }

    if !has_authored_value(&brief.support_evidence) {
        push_clarification_question(
            &mut questions,
            "clarify-authored-support",
            authored_support_prompt(brief.family),
            "Without explicit support, the packet risks sounding more grounded than it actually is.",
            authored_support_fallback(brief.family),
        );
    }

    push_family_specific_authored_questions(brief, &mut questions);

    push_authored_gap_questions(brief, &mut questions);

    questions.truncate(5);
    questions
}

fn append_family_specific_missing_context(brief: &AuthoredModeBrief, missing: &mut Vec<String>) {
    match brief.family {
        AuthoredClarityFamily::Planning => {
            if count_captured_list_items(&brief.options) == 0
                && !has_authored_value(&brief.decision_state)
            {
                missing.push(
                    "Decision posture is shallow; the packet names neither viable options nor a materially closed direction.".to_string(),
                );
            } else if count_captured_list_items(&brief.tradeoffs) == 0 && !brief.materially_closed()
            {
                missing.push(
                    "Tradeoff evidence is missing; planning output would otherwise overstate certainty.".to_string(),
                );
            }
        }
        AuthoredClarityFamily::Execution => {
            if !has_authored_value(&brief.preserved_boundary) {
                missing.push(
                    "Preserved behavior or compatibility boundary is missing; execution output would weaken change-preservation guarantees.".to_string(),
                );
            }
        }
        AuthoredClarityFamily::Assessment => {
            if matches!(brief.mode, Mode::Review | Mode::Verification)
                && !has_authored_value(&brief.decision_state)
            {
                missing.push(
                    "Disposition or verdict is missing; review-family output would otherwise imply a conclusion that is not yet authored.".to_string(),
                );
            }
        }
    }
}

fn push_family_specific_authored_questions(
    brief: &AuthoredModeBrief,
    questions: &mut Vec<ClarificationQuestionSummary>,
) {
    match brief.family {
        AuthoredClarityFamily::Planning => {
            if count_captured_list_items(&brief.options) == 0
                && !has_authored_value(&brief.decision_state)
            {
                push_planning_decision_posture_question(brief, questions);
            }

            if matches!(brief.mode, Mode::Architecture)
                && count_captured_list_items(&brief.tradeoffs) == 0
                && !brief.materially_closed()
            {
                push_architecture_clarification_question(
                    questions,
                    "clarify-authored-tradeoffs",
                    "Which tradeoffs would actually change the architectural choice?",
                    "Architecture clarification should stay limited to tradeoffs that could change the structural decision or next-mode routing.",
                    "No authored `## Pros`, `## Cons`, `## Tradeoffs`, or equivalent tradeoff section was detected in the supplied architecture brief.",
                );
            }
        }
        AuthoredClarityFamily::Execution => {
            if !has_authored_value(&brief.preserved_boundary) {
                push_clarification_question(
                    questions,
                    "clarify-authored-preservation",
                    "Which behavior, compatibility, or invariant guarantees must remain intact?",
                    "Execution packets need an explicit preservation boundary before they can claim grounded reasoning.",
                    authored_preserved_fallback(),
                );
            }
        }
        AuthoredClarityFamily::Assessment => {
            if matches!(brief.mode, Mode::Review | Mode::Verification)
                && !has_authored_value(&brief.decision_state)
            {
                push_clarification_question(
                    questions,
                    "clarify-authored-disposition",
                    "What disposition or verdict is actually justified by the authored evidence?",
                    "Review-family packets should not imply approval, contradiction, or rejection without an authored conclusion.",
                    authored_decision_fallback(brief.family),
                );
            }
        }
    }
}

fn push_planning_decision_posture_question(
    brief: &AuthoredModeBrief,
    questions: &mut Vec<ClarificationQuestionSummary>,
) {
    if matches!(brief.mode, Mode::Architecture) {
        push_architecture_clarification_question(
            questions,
            "clarify-authored-decision-posture",
            "Which options were considered, or is the decision already materially closed?",
            "Architecture packets should either preserve viable alternatives or say directly that the decision is already closed.",
            authored_decision_fallback(brief.family),
        );
    } else {
        push_clarification_question(
            questions,
            "clarify-authored-decision-posture",
            "Which options were considered, or is the decision already materially closed?",
            "Planning packets should either preserve viable alternatives or say directly that the decision is already closed.",
            authored_decision_fallback(brief.family),
        );
    }
}

fn push_authored_gap_questions(
    brief: &AuthoredModeBrief,
    questions: &mut Vec<ClarificationQuestionSummary>,
) {
    for (index, gap) in brief.questions_or_gaps.iter().enumerate() {
        if !is_authored_gap_question(gap) {
            continue;
        }

        let prompt = question_prompt(gap);
        let id = format!("authored-gap-question-{}", index + 1);
        if matches!(brief.mode, Mode::Architecture) {
            push_architecture_clarification_question(
                questions,
                &id,
                &prompt,
                "This unresolved architecture question is already authored in the packet and should stay visible before a governed run starts.",
                "Captured from the authored gaps, open questions, or evidence-gap section.",
            );
        } else {
            push_clarification_question(
                questions,
                &id,
                &prompt,
                "This unresolved question is already authored in the packet and should stay visible before a governed run starts.",
                "Captured from the authored gaps, open questions, or evidence-gap section.",
            );
        }
    }
}

pub(crate) fn authored_mode_reasoning_signals(
    source_inputs: &[String],
    brief: &AuthoredModeBrief,
) -> Vec<String> {
    let mut signals = Vec::new();

    signals.push(format!(
        "Detected {} authored input surface(s): {}.",
        source_inputs.len(),
        if source_inputs.is_empty() {
            "no-authored-source-inputs-recorded".to_string()
        } else {
            source_inputs.join(", ")
        }
    ));
    signals.push(match brief.family {
        AuthoredClarityFamily::Planning => format!(
            "Captured {} option signal(s), {} tradeoff signal(s), and {} gap or question signal(s).",
            count_captured_list_items(&brief.options),
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.questions_or_gaps)
        ),
        AuthoredClarityFamily::Execution => format!(
            "Captured preservation posture `{}`, {} tradeoff signal(s), and {} gap or risk signal(s).",
            if has_authored_value(&brief.preserved_boundary) {
                truncate_context_excerpt(&brief.preserved_boundary, 72)
            } else {
                "NOT CAPTURED".to_string()
            },
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.questions_or_gaps)
        ),
        AuthoredClarityFamily::Assessment => format!(
            "Captured {} authored gap signal(s) with review posture `{}`.",
            count_captured_list_items(&brief.questions_or_gaps),
            if has_authored_value(&brief.decision_state) {
                truncate_context_excerpt(&brief.decision_state, 72)
            } else {
                "NOT CAPTURED".to_string()
            }
        ),
    });

    if brief.materially_closed() {
        signals.push(format!(
            "The authored packet already materially closes the decision around `{}`; Canon should preserve that closure instead of inventing balanced alternatives.",
            truncate_context_excerpt(&brief.decision_state, 96)
        ));
    } else if brief.weak_reasoning() {
        signals.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "Headings are present but the planning packet still lacks enough boundary, evidence, or decision support to justify strong reasoning.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "The execution packet still lacks enough preservation or safety-net support to read as grounded reasoning.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "The assessment packet still lacks enough evidence or conclusion support to read as review-ready reasoning.".to_string()
            }
        });
    } else {
        signals.push(match brief.family {
            AuthoredClarityFamily::Planning => {
                "The planning packet contains bounded intent, explicit support, and enough decision posture to reason without inventing extra balance.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "The execution packet contains bounded scope, preservation posture, and supporting evidence for grounded reasoning.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "The assessment packet contains a bounded target, evidence basis, and review posture strong enough for grounded reasoning.".to_string()
            }
        });
    }

    signals
}

pub(crate) fn clarity_output_quality(
    materially_closed: bool,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
    reasoning_signals: &[String],
) -> OutputQualitySummary {
    let posture = if !missing_context.is_empty() {
        "structurally-complete"
    } else if clarification_questions.is_empty() {
        "publishable"
    } else {
        "materially-useful"
    };

    let mut evidence_signals = reasoning_signals.iter().take(2).cloned().collect::<Vec<_>>();
    if materially_closed {
        evidence_signals.push(
            "The authored packet is materially closed around one viable direction and keeps that closure explicit.".to_string(),
        );
    } else if missing_context.is_empty() {
        evidence_signals.push(
            "No explicit missing-context markers remain in the inspected packet.".to_string(),
        );
    }
    if evidence_signals.is_empty() {
        evidence_signals.push(
            "The inspect surface computed a bounded output-quality posture from the authored packet."
                .to_string(),
        );
    }

    let mut downgrade_reasons = missing_context.to_vec();
    if !clarification_questions.is_empty() {
        downgrade_reasons.push(format!(
            "{} clarification question(s) remain before Canon should treat this packet as unambiguously publishable.",
            clarification_questions.len()
        ));
    }

    OutputQualitySummary {
        posture: posture.to_string(),
        materially_closed,
        evidence_signals,
        downgrade_reasons,
    }
}

pub(crate) fn authored_mode_output_quality(
    brief: &AuthoredModeBrief,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
    reasoning_signals: &[String],
) -> OutputQualitySummary {
    clarity_output_quality(
        brief.materially_closed(),
        missing_context,
        clarification_questions,
        reasoning_signals,
    )
}

pub(crate) fn authored_mode_recommended_focus(
    brief: &AuthoredModeBrief,
    missing_context: &[String],
    clarification_questions: &[ClarificationQuestionSummary],
) -> String {
    if !missing_context.is_empty() {
        if matches!(brief.mode, Mode::Architecture)
            && let Some(reroute) = architecture_reroute_guidance(brief)
        {
            return reroute;
        }

        return match brief.family {
            AuthoredClarityFamily::Planning => {
                "Resolve the missing planning boundaries and evidence before treating this packet as reasoning-ready downstream input.".to_string()
            }
            AuthoredClarityFamily::Execution => {
                "Resolve the missing execution boundary, preservation, and safety-net support before using this packet to justify mutation.".to_string()
            }
            AuthoredClarityFamily::Assessment => {
                "Resolve the missing evidence or scope boundaries before treating this packet as reviewable assessment output.".to_string()
            }
        };
    }

    if brief.materially_closed() {
        return "The authored packet already materially closes the decision; preserve that closure and its supporting evidence instead of manufacturing extra balance.".to_string();
    }

    if !clarification_questions.is_empty() {
        if matches!(brief.mode, Mode::Architecture) {
            return "Answer the remaining decision-changing architecture questions, or let their documented defaults carry forward into `readiness-assessment.md`, before starting architecture mode.".to_string();
        }

        return format!(
            "Review the remaining authored {} questions before starting {} mode.",
            brief.family.label(),
            brief.mode.as_str()
        );
    }

    format!(
        "No critical clarification questions detected; the authored packet is bounded enough for {} mode.",
        brief.mode.as_str()
    )
}

// ── Clarification question helpers ────────────────────────────────────────────

pub(crate) fn push_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    push_clarification_question_with_metadata(
        questions,
        id,
        prompt,
        rationale,
        evidence,
        ClarificationQuestionMetadata {
            affects: "packet readiness",
            default_if_skipped: "Keep the item visible as unresolved authored context before downstream reasoning.",
            status: "required",
        },
    );
}

fn push_architecture_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    let metadata = architecture_question_metadata(id);
    push_clarification_question_with_metadata(questions, id, prompt, rationale, evidence, metadata);
}

struct ClarificationQuestionMetadata<'a> {
    affects: &'a str,
    default_if_skipped: &'a str,
    status: &'a str,
}

fn push_clarification_question_with_metadata(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
    metadata: ClarificationQuestionMetadata<'_>,
) {
    if questions.iter().any(|question| question.prompt.eq_ignore_ascii_case(prompt)) {
        return;
    }

    questions.push(ClarificationQuestionSummary {
        id: id.to_string(),
        prompt: prompt.to_string(),
        rationale: rationale.to_string(),
        evidence: evidence.to_string(),
        affects: metadata.affects.to_string(),
        default_if_skipped: metadata.default_if_skipped.to_string(),
        status: metadata.status.to_string(),
    });
}

fn architecture_question_metadata(id: &str) -> ClarificationQuestionMetadata<'static> {
    match id {
        "clarify-authored-target" => ClarificationQuestionMetadata {
            affects: "recommended next mode",
            default_if_skipped: "Reroute to discovery until the problem and decision surface are bounded enough for architecture tradeoffs.",
            status: "required",
        },
        "clarify-authored-boundary" => ClarificationQuestionMetadata {
            affects: "architecture-decisions.md and context-map.md",
            default_if_skipped: "Reroute to requirements until the architecture boundary and scope limits are explicit.",
            status: "required",
        },
        "clarify-authored-support" => ClarificationQuestionMetadata {
            affects: "readiness-assessment.md",
            default_if_skipped: "Keep the packet conditional in readiness-assessment.md instead of treating the decision as fully grounded.",
            status: "required",
        },
        "clarify-authored-decision-posture" | "clarify-authored-tradeoffs" => {
            ClarificationQuestionMetadata {
                affects: "tradeoff-matrix.md",
                default_if_skipped: "Keep the structural options unresolved and record the missing tradeoff as a readiness blocker.",
                status: "required",
            }
        }
        _ if id.starts_with("authored-gap-question-") => ClarificationQuestionMetadata {
            affects: "readiness-assessment.md",
            default_if_skipped: "Carry the unanswered item forward into readiness-assessment.md as an unresolved question.",
            status: "required",
        },
        _ => ClarificationQuestionMetadata {
            affects: "architecture readiness",
            default_if_skipped: "Keep the gap visible as unresolved authored context before downstream reasoning.",
            status: "required",
        },
    }
}

fn architecture_reroute_guidance(brief: &AuthoredModeBrief) -> Option<String> {
    if !has_authored_value(&brief.primary_subject) {
        return Some(
            "Architecture mode is not ready yet; reroute to discovery until the problem and decision surface are bounded enough to compare structural options.".to_string(),
        );
    }

    if !has_authored_value(&brief.boundary) || !has_authored_value(&brief.support_evidence) {
        return Some(
            "Architecture mode is not ready yet; reroute to requirements until the scope, constraints, and supporting rationale are explicit.".to_string(),
        );
    }

    if count_captured_list_items(&brief.options) == 0
        && count_captured_list_items(&brief.tradeoffs) == 0
        && !has_authored_value(&brief.decision_state)
    {
        return Some(
            "Architecture mode is not ready yet; reroute to system-shaping until the candidate boundaries and structural options are explicit enough to compare.".to_string(),
        );
    }

    None
}

pub(crate) fn is_default_requirements_open_question(question: &str) -> bool {
    question.eq_ignore_ascii_case("Which downstream mode should consume this packet first?")
}

pub(crate) fn question_prompt(question: &str) -> String {
    let trimmed = question.trim().trim_end_matches('.');
    if trimmed.ends_with('?') { trimmed.to_string() } else { format!("{trimmed}?") }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests;
