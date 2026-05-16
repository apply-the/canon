//! Markdown renderers for `run` and `status` command output.
//!
//! These renderers produce human-readable Markdown from structured
//! [`canon_engine::RunSummary`] and [`canon_engine::StatusSummary`] values.

use canon_engine::{RunSummary, StatusSummary};

use super::primitives::humanize_path;

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

    render_mode_result(&mut lines, summary.mode_result.as_ref());
    render_runtime_blockers(&mut lines, &summary.blocked_gates);
    render_recommended_next_step(&mut lines, summary.recommended_next_action.as_ref());
    render_possible_actions(&mut lines, &summary.possible_actions);

    lines.join("\n")
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
    if let Some(execution_posture) = &mode_result.execution_posture {
        lines.push(String::new());
        lines.push(format!("Execution Posture: {execution_posture}"));
    }
    lines.push(String::new());
    lines.push(format!("Primary Artifact: {}", humanize_path(&mode_result.primary_artifact_path)));
    lines.push(format!(
        "Primary Artifact Action: {} ({})",
        mode_result.primary_artifact_action.label,
        humanize_path(&mode_result.primary_artifact_action.target)
    ));
    lines.push(String::new());
    lines.push("Excerpt:".to_string());
    lines.push(mode_result.result_excerpt.clone());

    if !mode_result.action_chips.is_empty() {
        lines.push(String::new());
        lines.push("Action Chips:".to_string());
        for chip in &mode_result.action_chips {
            let recommended = if chip.recommended { " (recommended)" } else { "" };
            lines.push(format!("- {}{}", chip.text_fallback, recommended));
        }
    }
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
