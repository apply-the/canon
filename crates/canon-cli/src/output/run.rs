//! Markdown renderers for `run` and `status` command output.
//!
//! These renderers produce human-readable Markdown from structured
//! [`canon_engine::RunSummary`] and [`canon_engine::StatusSummary`] values.

use canon_engine::{RunSummary, StatusSummary};

use super::primitives::humanize_path;

/// Renders a [`StatusSummary`] as human-readable text.
pub(super) fn render_status_summary_text(summary: &StatusSummary) -> String {
    let mut lines = vec![format!("run id: {}", summary.run), format!("state: {}", summary.state)];
    if let Some(system_context) = &summary.system_context {
        lines.push(format!("system context: {system_context}"));
    }

    render_refinement_state_text(&mut lines, summary.refinement_state.as_ref());

    lines.join("\n")
}

/// Renders a [`RunSummary`] as a Markdown document.
pub(super) fn render_run_summary_markdown(summary: &RunSummary) -> String {
    let mut lines = vec!["# run".to_string(), String::new()];
    lines.push(format!("Run ID: {}", summary.run_id));
    if let Some(uuid) = &summary.uuid {
        lines.push(format!("UUID: {uuid}"));
    }
    lines.push(format!("Mode: {}", summary.mode));
    lines.push(format!("State: {}", summary.state));
    lines.push(format!("Risk: {}", summary.risk));
    lines.push(format!("Zone: {}", summary.zone));
    if let Some(system_context) = &summary.system_context {
        lines.push(format!("System Context: {system_context}"));
    }

    render_mode_result(&mut lines, summary.mode_result.as_ref());
    render_runtime_blockers(&mut lines, &summary.blocked_gates);
    render_recommended_next_step(&mut lines, summary.recommended_next_action.as_ref());
    render_possible_actions(&mut lines, &summary.possible_actions);

    lines.join("\n")
}

/// Renders a [`StatusSummary`] as a Markdown document.
pub(super) fn render_status_summary_markdown(summary: &StatusSummary) -> String {
    let mut lines = vec!["# status".to_string(), String::new()];
    lines.push(format!("Run ID: {}", summary.run));
    lines.push(format!("State: {}", summary.state));
    if let Some(system_context) = &summary.system_context {
        lines.push(format!("System Context: {system_context}"));
    }

    render_refinement_state(&mut lines, summary.refinement_state.as_ref());
    render_mode_result(&mut lines, summary.mode_result.as_ref());
    render_runtime_blockers(&mut lines, &summary.blocked_gates);
    render_recommended_next_step(&mut lines, summary.recommended_next_action.as_ref());
    render_possible_actions(&mut lines, &summary.possible_actions);

    lines.join("\n")
}

fn render_refinement_state(
    lines: &mut Vec<String>,
    refinement: Option<&canon_engine::orchestrator::service::RefinementStateSummary>,
) {
    let Some(refinement) = refinement else {
        return;
    };

    lines.push(String::new());
    lines.push("## Refinement State".to_string());
    lines.push(String::new());
    lines.push(format!("Current Mode: {}", refinement.current_mode));
    lines.push(format!("Status: {}", refinement.status));
    lines.push(format!("Working Brief: {}", humanize_path(&refinement.working_brief_path)));
    lines.push(format!("Unresolved Clarification Records: {}", refinement.unresolved_records));

    if !refinement.readiness_delta.is_empty() {
        lines.push("Readiness Delta:".to_string());
        for item in &refinement.readiness_delta {
            lines.push(format!("- {item}"));
        }
    }

    if refinement.explicit_continuation_required || refinement.suggested_candidate.is_some() {
        lines.push(String::new());
        lines.push("## Continuation Guidance".to_string());
        lines.push(String::new());
        if refinement.explicit_continuation_required {
            lines.push(
                "Explicit continuation is still required before Canon mutates an existing run."
                    .to_string(),
            );
        }
        if let Some(candidate) = &refinement.suggested_candidate {
            lines.push(format!(
                "Suggested Continuation: {} ({}, {})",
                candidate.run_id, candidate.mode, candidate.state
            ));
            lines.push(format!("Match Reason: {}", candidate.match_reason));
            if candidate.advisory {
                lines.push(
                    "Candidate detection is advisory; continuation requires explicit intent."
                        .to_string(),
                );
            }
        }
    }
}

fn render_refinement_state_text(
    lines: &mut Vec<String>,
    refinement: Option<&canon_engine::orchestrator::service::RefinementStateSummary>,
) {
    let Some(refinement) = refinement else {
        return;
    };

    lines.push(String::new());
    lines.push("refinement state:".to_string());
    lines.push(format!("current mode: {}", refinement.current_mode));
    lines.push(format!("status: {}", refinement.status));
    lines.push(format!("working brief: {}", humanize_path(&refinement.working_brief_path)));
    if !refinement.authoritative_input_refs.is_empty() {
        lines.push("authoritative inputs:".to_string());
        for input in &refinement.authoritative_input_refs {
            lines.push(format!("- {}", humanize_path(input)));
        }
    }
    if !refinement.supporting_input_refs.is_empty() {
        lines.push("supporting inputs:".to_string());
        for input in &refinement.supporting_input_refs {
            lines.push(format!("- {}", humanize_path(input)));
        }
    }
    lines.push(format!(
        "clarification records: {} total, {} unresolved",
        refinement.records_total, refinement.unresolved_records
    ));

    if !refinement.readiness_delta.is_empty() {
        lines.push("readiness delta:".to_string());
        for item in &refinement.readiness_delta {
            lines.push(format!("- {item}"));
        }
    }

    if refinement.explicit_continuation_required || refinement.suggested_candidate.is_some() {
        lines.push(String::new());
        lines.push("continuation guidance:".to_string());
        if refinement.explicit_continuation_required {
            lines.push(
                "explicit continuation is still required before Canon mutates an existing run."
                    .to_string(),
            );
        }
        if let Some(candidate) = &refinement.suggested_candidate {
            lines.push(format!(
                "suggested continuation: {} ({}, {})",
                candidate.run_id, candidate.mode, candidate.state
            ));
            lines.push(format!("match reason: {}", candidate.match_reason));
            if candidate.advisory {
                lines.push(
                    "candidate detection is advisory; continuation requires explicit intent."
                        .to_string(),
                );
            }
        }
    }
}

/// Appends the mode result block (headline, excerpt, action chips) to `lines`.
fn render_mode_result(
    lines: &mut Vec<String>,
    mode_result: Option<&canon_engine::ModeResultSummary>,
) {
    let Some(mode_result) = mode_result else {
        return;
    };

    lines.push(String::new());
    lines.push("## Result".to_string());
    lines.push(String::new());
    lines.push(mode_result.headline.clone());
    lines.push(String::new());
    lines.push(mode_result.artifact_packet_summary.clone());
    render_execution_posture(lines, mode_result.execution_posture.as_deref());
    render_primary_artifact(lines, mode_result);
    render_result_excerpt(lines, &mode_result.result_excerpt);
    render_action_chips(lines, &mode_result.action_chips);
}

fn render_execution_posture(lines: &mut Vec<String>, execution_posture: Option<&str>) {
    let Some(execution_posture) = execution_posture else {
        return;
    };

    lines.push(String::new());
    lines.push(format!("Execution Posture: {execution_posture}"));
}

fn render_primary_artifact(lines: &mut Vec<String>, mode_result: &canon_engine::ModeResultSummary) {
    lines.push(String::new());
    lines.push(format!("Primary Artifact: {}", humanize_path(&mode_result.primary_artifact_path)));
    lines.push(format!(
        "Primary Artifact Action: {} ({})",
        mode_result.primary_artifact_action.label,
        humanize_path(&mode_result.primary_artifact_action.target)
    ));
}

fn render_result_excerpt(lines: &mut Vec<String>, result_excerpt: &str) {
    lines.push(String::new());
    lines.push("Excerpt:".to_string());
    lines.push(result_excerpt.to_string());
}

fn render_action_chips(lines: &mut Vec<String>, action_chips: &[canon_engine::ActionChip]) {
    if action_chips.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push("Action Chips:".to_string());
    for chip in action_chips {
        lines.push(format!("- {}{}", chip.text_fallback, recommended_suffix(chip.recommended)));
    }
}

fn recommended_suffix(recommended: bool) -> &'static str {
    if recommended { " (recommended)" } else { "" }
}

/// Appends the blockers section to `lines` when any gates are blocked.
fn render_runtime_blockers(
    lines: &mut Vec<String>,
    blocked_gates: &[canon_engine::GateInspectSummary],
) {
    if blocked_gates.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push("## Blockers".to_string());
    lines.push(String::new());
    for gate in blocked_gates {
        lines.push(format!("- {}: {}", gate.gate, gate.blockers.join(" | ")));
    }
}

/// Appends the recommended next step section when an action is present.
fn render_recommended_next_step(
    lines: &mut Vec<String>,
    action: Option<&canon_engine::RecommendedActionSummary>,
) {
    let Some(action) = action else {
        return;
    };

    lines.push(String::new());
    lines.push("## Recommended Next Step".to_string());
    lines.push(String::new());
    lines.push(format!("Action: {}", action.action));
    lines.push(format!("Why: {}", action.rationale));
    if let Some(target) = &action.target {
        lines.push(format!("Target: {target}"));
    }
}

/// Appends the possible actions section when actions are present.
fn render_possible_actions(
    lines: &mut Vec<String>,
    actions: &[canon_engine::PossibleActionSummary],
) {
    if actions.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push("## Possible Actions".to_string());
    lines.push(String::new());
    for action in actions {
        lines.push(format!("- {}", action.text));
    }
}

#[cfg(test)]
mod tests {
    use canon_engine::StatusSummary;
    use canon_engine::orchestrator::service::{RefinementCandidateSummary, RefinementStateSummary};

    use super::{render_status_summary_markdown, render_status_summary_text};

    fn summary_with_refinement(advisory: bool) -> StatusSummary {
        StatusSummary {
            run: "R-20260529-status".to_string(),
            owner: "owner".to_string(),
            state: "Draft".to_string(),
            system_context: Some("existing".to_string()),
            invocations_total: 0,
            pending_invocation_approvals: 0,
            validation_independence_satisfied: true,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: Vec::new(),
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            possible_actions: Vec::new(),
            refinement_state: Some(RefinementStateSummary {
                workflow_family: "planning".to_string(),
                current_mode: "requirements".to_string(),
                working_brief_path: "artifacts/R-20260529-status/requirements/working-brief.md"
                    .to_string(),
                template_ref: "docs/templates/canon-input/requirements.md".to_string(),
                status: "active".to_string(),
                explicit_continuation_required: true,
                authoritative_input_refs: vec!["canon-input/requirements/brief.md".to_string()],
                supporting_input_refs: vec!["canon-input/requirements/context.md".to_string()],
                records_total: 2,
                unresolved_records: 1,
                readiness_delta: vec!["Independent validation owner is not yet named.".to_string()],
                readiness_items: Vec::new(),
                suggested_candidate: Some(RefinementCandidateSummary {
                    run_id: "R-20260529-prev".to_string(),
                    mode: "requirements".to_string(),
                    state: "Draft".to_string(),
                    match_reason: "shared authoritative fingerprints".to_string(),
                    advisory,
                }),
            }),
            mode_result: None,
            recommended_next_action: None,
        }
    }

    #[test]
    fn status_markdown_renders_refinement_lists_and_advisory_message() {
        let rendered = render_status_summary_markdown(&summary_with_refinement(true));

        assert!(rendered.contains("## Refinement State"));
        assert!(rendered.contains("Readiness Delta:"));
        assert!(rendered.contains("## Continuation Guidance"));
        assert!(rendered.contains("Suggested Continuation: R-20260529-prev (requirements, Draft)"));
        assert!(
            rendered.contains(
                "Candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn status_text_renders_authoritative_and_supporting_inputs() {
        let rendered = render_status_summary_text(&summary_with_refinement(true));

        assert!(rendered.contains("refinement state:"));
        assert!(rendered.contains("authoritative inputs:"));
        assert!(rendered.contains("supporting inputs:"));
        assert!(rendered.contains("readiness delta:"));
        assert!(
            rendered.contains(
                "candidate detection is advisory; continuation requires explicit intent."
            )
        );
    }

    #[test]
    fn status_text_omits_advisory_line_when_candidate_is_not_advisory() {
        let rendered = render_status_summary_text(&summary_with_refinement(false));

        assert!(!rendered.contains("candidate detection is advisory"));
    }
}
