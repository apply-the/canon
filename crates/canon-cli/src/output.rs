use canon_engine::{RunSummary, StatusSummary};
use serde::Serialize;
use serde_json::Value;

use crate::app::OutputFormat;
use crate::error::CliResult;

pub fn print_value<T: Serialize>(value: &T, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Text => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(value)?);
        }
        OutputFormat::Markdown => {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
    }

    Ok(())
}

pub fn print_run_summary(summary: &RunSummary, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Markdown => {
            println!("{}", render_run_summary_markdown(summary));
            Ok(())
        }
        other => print_value(summary, other),
    }
}

pub fn print_status_summary(summary: &StatusSummary, format: OutputFormat) -> CliResult<()> {
    match format {
        OutputFormat::Markdown => {
            println!("{}", render_status_summary_markdown(summary));
            Ok(())
        }
        other => print_value(summary, other),
    }
}

pub fn print_inspect<T: Serialize>(
    value: &T,
    target_name: &str,
    run_id: Option<&str>,
    format: OutputFormat,
) -> CliResult<()> {
    match format {
        OutputFormat::Text if target_name == "risk-zone" => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_risk_zone_text(&json));
            Ok(())
        }
        OutputFormat::Markdown => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_markdown_from_json(&json, target_name, run_id));
            Ok(())
        }
        other => print_value(value, other),
    }
}

fn render_markdown_from_json(value: &Value, target_name: &str, run_id: Option<&str>) -> String {
    let entries = value.get("entries").and_then(Value::as_array).cloned().unwrap_or_default();
    let system_context = value.get("system_context").and_then(Value::as_str);

    match target_name {
        "artifacts" => render_artifacts_markdown(&entries, run_id, system_context),
        "clarity" => render_clarity_markdown(&entries),
        "evidence" => render_evidence_markdown(&entries, run_id, system_context),
        "invocations" => render_invocations_markdown(&entries, run_id, system_context),
        "risk-zone" => render_risk_zone_markdown(&entries),
        _ => render_list_markdown(target_name, &entries),
    }
}

fn render_risk_zone_text(value: &Value) -> String {
    let entries = value.get("entries").and_then(Value::as_array).cloned().unwrap_or_default();
    let Some(entry) = entries.first().and_then(Value::as_object) else {
        return "TARGET=risk-zone".to_string();
    };

    let mut lines = vec!["TARGET=risk-zone".to_string()];
    render_kv_field(&mut lines, "MODE", entry.get("mode"));
    render_kv_field(&mut lines, "INFERRED_RISK", entry.get("risk"));
    render_kv_field(&mut lines, "INFERRED_ZONE", entry.get("zone"));
    render_kv_field(&mut lines, "RISK_WAS_SUPPLIED", entry.get("risk_was_supplied"));
    render_kv_field(&mut lines, "ZONE_WAS_SUPPLIED", entry.get("zone_was_supplied"));
    render_kv_field(&mut lines, "INFERENCE_CONFIDENCE", entry.get("confidence"));
    render_kv_field(&mut lines, "NEEDS_CONFIRMATION", entry.get("requires_confirmation"));
    render_kv_field(&mut lines, "INFERENCE_HEADLINE", entry.get("headline"));
    render_kv_field(&mut lines, "INFERENCE_RATIONALE", entry.get("rationale"));
    render_kv_field(&mut lines, "RISK_RATIONALE", entry.get("risk_rationale"));
    render_kv_field(&mut lines, "ZONE_RATIONALE", entry.get("zone_rationale"));

    for (index, signal) in string_list(entry.get("signals")).into_iter().enumerate() {
        lines.push(format!("SIGNAL_{}={signal}", index + 1));
    }
    for (index, signal) in string_list(entry.get("risk_signals")).into_iter().enumerate() {
        lines.push(format!("RISK_SIGNAL_{}={signal}", index + 1));
    }
    for (index, signal) in string_list(entry.get("zone_signals")).into_iter().enumerate() {
        lines.push(format!("ZONE_SIGNAL_{}={signal}", index + 1));
    }

    lines.join("\n")
}

fn render_run_summary_markdown(summary: &RunSummary) -> String {
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

    lines.join("\n")
}

fn render_status_summary_markdown(summary: &StatusSummary) -> String {
    let mut lines = vec!["# status".to_string(), String::new()];
    lines.push(format!("Run ID: {}", summary.run));
    lines.push(format!("State: {}", summary.state));
    if let Some(system_context) = &summary.system_context {
        lines.push(format!("System Context: {system_context}"));
    }

    render_mode_result(&mut lines, summary.mode_result.as_ref());
    render_runtime_blockers(&mut lines, &summary.blocked_gates);
    render_recommended_next_step(&mut lines, summary.recommended_next_action.as_ref());

    lines.join("\n")
}

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

fn render_list_markdown(title: &str, entries: &[Value]) -> String {
    let mut lines = vec![format!("# {title}"), String::new()];

    if entries.is_empty() {
        lines.push("- No entries recorded.".to_string());
        return lines.join("\n");
    }

    for entry in entries {
        match entry {
            Value::String(item) => lines.push(format!("- {item}")),
            other => lines.push(format!(
                "- {}",
                serde_json::to_string(other).unwrap_or_else(|_| "{}".to_string())
            )),
        }
    }

    lines.join("\n")
}

fn render_artifacts_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# artifacts".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
    }
    if let Some(system_context) = system_context {
        lines.push(String::new());
        lines.push(format!("System Context: {system_context}"));
    }

    lines.push(String::new());
    lines.push("## Readable Artifacts".to_string());

    if entries.is_empty() {
        lines.push(String::new());
        lines.push("- No artifacts recorded.".to_string());
        return lines.join("\n");
    }

    lines.push(String::new());
    for entry in entries {
        if let Some(path) = entry.as_str() {
            lines.push(format!("- {}", humanize_path(path)));
        }
    }

    lines.join("\n")
}

fn render_evidence_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# evidence".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
    }
    if let Some(system_context) = system_context {
        lines.push(String::new());
        lines.push(format!("System Context: {system_context}"));
    }

    if entries.is_empty() {
        lines.push(String::new());
        lines.push("- No evidence recorded.".to_string());
        return lines.join("\n");
    }

    let Some(entry) = entries.first().and_then(Value::as_object) else {
        lines.push(String::new());
        lines.push("- No evidence recorded.".to_string());
        return lines.join("\n");
    };

    let artifact_links = string_list(entry.get("artifact_provenance_links"));
    let generation_paths = string_list(entry.get("generation_paths"));
    let validation_paths = string_list(entry.get("validation_paths"));
    let denied_invocations = string_list(entry.get("denied_invocations"));
    let upstream_source_refs = string_list(entry.get("upstream_source_refs"));
    let carried_forward_items = string_list(entry.get("carried_forward_items"));

    render_scalar_field(&mut lines, "Execution Posture", entry.get("execution_posture"));
    render_scalar_field(&mut lines, "Feature Slice", entry.get("upstream_feature_slice"));
    render_scalar_field(&mut lines, "Primary Upstream Mode", entry.get("primary_upstream_mode"));
    render_scalar_field(
        &mut lines,
        "Excluded Upstream Scope",
        entry.get("excluded_upstream_scope"),
    );

    if !upstream_source_refs.is_empty() {
        lines.push(String::new());
        lines.push("## Upstream Sources".to_string());
        lines.push(String::new());
        for source in upstream_source_refs {
            lines.push(format!("- {source}"));
        }
    }

    if !carried_forward_items.is_empty() {
        lines.push(String::new());
        lines.push("## Carried-Forward Context".to_string());
        lines.push(String::new());
        for item in carried_forward_items {
            lines.push(format!("- {item}"));
        }
    }

    if !artifact_links.is_empty() {
        lines.push(String::new());
        lines.push("## Readable Artifacts".to_string());
        lines.push(String::new());
        for path in artifact_links {
            lines.push(format!("- {}", humanize_path(&path)));
        }
    }

    if !generation_paths.is_empty() {
        lines.push(String::new());
        lines.push("## Generation Paths".to_string());
        lines.push(String::new());
        for path in generation_paths {
            lines.push(format!("- {path}"));
        }
    }

    if !validation_paths.is_empty() {
        lines.push(String::new());
        lines.push("## Validation Paths".to_string());
        lines.push(String::new());
        for path in validation_paths {
            lines.push(format!("- {path}"));
        }
    }

    if !denied_invocations.is_empty() {
        lines.push(String::new());
        lines.push("## Denied Invocations".to_string());
        lines.push(String::new());
        for request_id in denied_invocations {
            lines.push(format!("- {request_id}"));
        }
    }

    lines.join("\n")
}

fn render_invocations_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# invocations".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
    }
    if let Some(system_context) = system_context {
        lines.push(String::new());
        lines.push(format!("System Context: {system_context}"));
    }

    if entries.is_empty() {
        lines.push(String::new());
        lines.push("- No invocations recorded.".to_string());
        return lines.join("\n");
    }

    for entry in entries {
        let Some(entry) = entry.as_object() else {
            continue;
        };
        let request_id =
            entry.get("request_id").and_then(Value::as_str).unwrap_or("unknown-request");
        lines.push(String::new());
        lines.push(format!("## {request_id}"));
        lines.push(String::new());
        render_scalar_field(&mut lines, "Adapter", entry.get("adapter"));
        render_scalar_field(&mut lines, "Capability", entry.get("capability"));
        render_scalar_field(&mut lines, "Orientation", entry.get("orientation"));
        render_scalar_field(&mut lines, "Policy Decision", entry.get("policy_decision"));
        render_scalar_field(&mut lines, "Recommendation Only", entry.get("recommendation_only"));
        render_scalar_field(&mut lines, "Approval State", entry.get("approval_state"));
        render_scalar_field(&mut lines, "Latest Outcome", entry.get("latest_outcome"));

        let linked_artifacts = string_list(entry.get("linked_artifacts"));
        if !linked_artifacts.is_empty() {
            lines.push(String::new());
            lines.push("Artifacts:".to_string());
            for path in linked_artifacts {
                lines.push(format!("- {}", humanize_path(&path)));
            }
        }
    }

    lines.join("\n")
}

fn render_risk_zone_markdown(entries: &[Value]) -> String {
    let mut lines = vec!["# risk-zone".to_string()];

    let Some(entry) = entries.first().and_then(Value::as_object) else {
        lines.push(String::new());
        lines.push("- No classification suggestion recorded.".to_string());
        return lines.join("\n");
    };

    lines.push(String::new());
    render_scalar_field(&mut lines, "Mode", entry.get("mode"));
    lines.push(String::new());
    lines.push("## Suggested Classification".to_string());
    lines.push(String::new());
    lines.push(format!(
        "Risk: {}{}",
        scalar_value(entry.get("risk")).unwrap_or_else(|| "unknown".to_string()),
        supplied_suffix(entry.get("risk_was_supplied"))
    ));
    lines.push(format!(
        "Zone: {}{}",
        scalar_value(entry.get("zone")).unwrap_or_else(|| "unknown".to_string()),
        supplied_suffix(entry.get("zone_was_supplied"))
    ));
    render_scalar_field(&mut lines, "Confidence", entry.get("confidence"));
    lines.push(format!("Needs Confirmation: {}", yes_no(entry.get("requires_confirmation"))));

    lines.push(String::new());
    lines.push("## Why".to_string());
    lines.push(String::new());
    if let Some(headline) = scalar_value(entry.get("headline")) {
        lines.push(headline);
    }
    if let Some(rationale) = scalar_value(entry.get("rationale")) {
        lines.push(String::new());
        lines.push(rationale);
    }

    let signals = string_list(entry.get("signals"));
    if !signals.is_empty() {
        lines.push(String::new());
        lines.push("## Signals".to_string());
        lines.push(String::new());
        for signal in signals {
            lines.push(format!("- {signal}"));
        }
    }

    lines.join("\n")
}

fn render_clarity_markdown(entries: &[Value]) -> String {
    let mut lines = vec!["# clarity".to_string()];

    let Some(entry) = entries.first().and_then(Value::as_object) else {
        lines.push(String::new());
        lines.push("- No clarity inspection recorded.".to_string());
        return lines.join("\n");
    };

    lines.push(String::new());
    render_scalar_field(&mut lines, "Mode", entry.get("mode"));
    lines.push(format!("Requires Clarification: {}", yes_no(entry.get("requires_clarification"))));

    if let Some(summary) = scalar_value(entry.get("summary")) {
        lines.push(String::new());
        lines.push("## Document Summary".to_string());
        lines.push(String::new());
        lines.push(summary);
    }

    let source_inputs = string_list(entry.get("source_inputs"));
    if !source_inputs.is_empty() {
        lines.push(String::new());
        lines.push("## Source Inputs".to_string());
        lines.push(String::new());
        for input in source_inputs {
            lines.push(format!("- {}", humanize_path(&input)));
        }
    }

    let reasoning_signals = string_list(entry.get("reasoning_signals"));
    if !reasoning_signals.is_empty() {
        lines.push(String::new());
        lines.push("## Reasoning Signals".to_string());
        lines.push(String::new());
        for signal in reasoning_signals {
            lines.push(format!("- {signal}"));
        }
    }

    let missing_context = string_list(entry.get("missing_context"));
    if !missing_context.is_empty() {
        lines.push(String::new());
        lines.push("## Missing Context".to_string());
        lines.push(String::new());
        for gap in missing_context {
            lines.push(format!("- {gap}"));
        }
    }

    let clarification_questions =
        entry.get("clarification_questions").and_then(Value::as_array).cloned().unwrap_or_default();
    if !clarification_questions.is_empty() {
        lines.push(String::new());
        lines.push("## Clarification Questions".to_string());
        lines.push(String::new());

        for (index, question) in clarification_questions.iter().enumerate() {
            let Some(question) = question.as_object() else {
                continue;
            };

            let prompt = scalar_value(question.get("prompt"))
                .unwrap_or_else(|| "Missing clarification prompt".to_string());
            lines.push(format!("{}. {prompt}", index + 1));
            if let Some(rationale) = scalar_value(question.get("rationale")) {
                lines.push(format!("Why: {rationale}"));
            }
            if let Some(evidence) = scalar_value(question.get("evidence")) {
                lines.push(format!("Evidence: {evidence}"));
            }
            if index + 1 < clarification_questions.len() {
                lines.push(String::new());
            }
        }
    }

    if let Some(recommended_focus) = scalar_value(entry.get("recommended_focus")) {
        lines.push(String::new());
        lines.push("## Recommended Focus".to_string());
        lines.push(String::new());
        lines.push(recommended_focus);
    }

    lines.join("\n")
}

fn render_scalar_field(lines: &mut Vec<String>, label: &str, value: Option<&Value>) {
    let Some(value) = value else {
        return;
    };

    match value {
        Value::Null => {}
        Value::String(text) if !text.is_empty() => lines.push(format!("{label}: {text}")),
        other => lines.push(format!(
            "{label}: {}",
            serde_json::to_string(other).unwrap_or_else(|_| "{}".to_string())
        )),
    }
}

fn render_kv_field(lines: &mut Vec<String>, label: &str, value: Option<&Value>) {
    let Some(value) = scalar_value(value) else {
        return;
    };

    lines.push(format!("{label}={value}"));
}

fn scalar_value(value: Option<&Value>) -> Option<String> {
    let value = value?;
    match value {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Number(number) => Some(number.to_string()),
        other => serde_json::to_string(other).ok(),
    }
}

fn supplied_suffix(value: Option<&Value>) -> &'static str {
    if value.and_then(Value::as_bool).unwrap_or(false) { " (provided)" } else { " (inferred)" }
}

fn yes_no(value: Option<&Value>) -> &'static str {
    if value.and_then(Value::as_bool).unwrap_or(false) { "yes" } else { "no" }
}

fn string_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items.iter().filter_map(Value::as_str).map(ToString::to_string).collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn humanize_path(path: &str) -> String {
    if path.starts_with(".canon/") {
        path.to_string()
    } else if path.starts_with("artifacts/") {
        format!(".canon/{path}")
    } else {
        path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use canon_engine::{
        GateInspectSummary, ModeResultSummary, RecommendedActionSummary, ResultActionSummary,
        RunSummary,
    };
    use serde_json::json;

    use super::{render_markdown_from_json, render_risk_zone_text, render_run_summary_markdown};

    #[test]
    fn clarity_markdown_surfaces_questions_and_signals() {
        let value = json!({
            "entries": [{
                "mode": "requirements",
                "summary": "Problem framing: Build a bounded USB flashing CLI.\nDesired outcome: Operators can flash firmware safely over USB with explicit logs.\nSource inputs: idea.md",
                "source_inputs": ["idea.md"],
                "requires_clarification": true,
                "missing_context": [
                    "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                ],
                "clarification_questions": [{
                    "id": "clarify-constraints",
                    "prompt": "Which constraints are non-negotiable for this work?",
                    "rationale": "Constraints determine whether downstream shaping stays repo-specific instead of becoming generic planning advice.",
                    "evidence": "No authored `## Constraints`, `## Constraint`, or `## Non-Negotiables` section was detected in the supplied inputs."
                }],
                "reasoning_signals": [
                    "Detected 1 authored input surface(s): idea.md."
                ],
                "recommended_focus": "Resolve the missing context items before starting a requirements run or handing the packet to downstream design work."
            }]
        });

        let markdown = render_markdown_from_json(&value, "clarity", None);

        assert!(markdown.contains("# clarity"));
        assert!(markdown.contains("Mode: requirements"));
        assert!(markdown.contains("Requires Clarification: yes"));
        assert!(markdown.contains("## Clarification Questions"));
        assert!(markdown.contains("1. Which constraints are non-negotiable for this work?"));
        assert!(markdown.contains("## Recommended Focus"));
    }

    #[test]
    fn artifacts_markdown_humanizes_artifact_paths() {
        let value = json!({
            "entries": [
                "artifacts/run-123/requirements/problem-statement.md",
                ".canon/artifacts/run-123/requirements/options.md"
            ]
        });

        let markdown = render_markdown_from_json(&value, "artifacts", Some("run-123"));

        assert!(markdown.contains("# artifacts"));
        assert!(markdown.contains("Run ID: run-123"));
        assert!(markdown.contains("- .canon/artifacts/run-123/requirements/problem-statement.md"));
        assert!(markdown.contains("- .canon/artifacts/run-123/requirements/options.md"));
    }

    #[test]
    fn evidence_markdown_renders_sections_for_available_lineage() {
        let value = json!({
            "entries": [{
                "execution_posture": "recommendation-only",
                "upstream_feature_slice": "auth session revocation",
                "primary_upstream_mode": "change",
                "upstream_source_refs": [
                    "docs/changes/R-20260422-AUTHREVOC/change-surface.md"
                ],
                "carried_forward_items": [
                    "Revocation output formatting stays stable."
                ],
                "excluded_upstream_scope": "login UI flow",
                "artifact_provenance_links": ["artifacts/run-123/pr-review/review-summary.md"],
                "generation_paths": ["generation:req-1"],
                "validation_paths": ["validation:req-2"],
                "denied_invocations": ["req-3"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "evidence", Some("run-123"));

        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains("Feature Slice: auth session revocation"));
        assert!(markdown.contains("Primary Upstream Mode: change"));
        assert!(markdown.contains("Excluded Upstream Scope: login UI flow"));
        assert!(markdown.contains("## Upstream Sources"));
        assert!(markdown.contains("- docs/changes/R-20260422-AUTHREVOC/change-surface.md"));
        assert!(markdown.contains("## Carried-Forward Context"));
        assert!(markdown.contains("- Revocation output formatting stays stable."));
        assert!(markdown.contains("## Readable Artifacts"));
        assert!(markdown.contains("- .canon/artifacts/run-123/pr-review/review-summary.md"));
        assert!(markdown.contains("## Generation Paths"));
        assert!(markdown.contains("- generation:req-1"));
        assert!(markdown.contains("## Validation Paths"));
        assert!(markdown.contains("- validation:req-2"));
        assert!(markdown.contains("## Denied Invocations"));
        assert!(markdown.contains("- req-3"));
    }

    #[test]
    fn invocations_markdown_renders_scalar_fields_and_linked_artifacts() {
        let value = json!({
            "entries": [{
                "request_id": "req-7",
                "adapter": "Shell",
                "capability": "ValidateWithTool",
                "orientation": "Validation",
                "policy_decision": "AllowConstrained",
                "recommendation_only": true,
                "approval_state": "NotRequired",
                "latest_outcome": "Succeeded",
                "linked_artifacts": ["artifacts/run-123/change/system-slice.md"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "invocations", Some("run-123"));

        assert!(markdown.contains("# invocations"));
        assert!(markdown.contains("## req-7"));
        assert!(markdown.contains("Adapter: Shell"));
        assert!(markdown.contains("Capability: ValidateWithTool"));
        assert!(markdown.contains("Recommendation Only: true"));
        assert!(markdown.contains("Artifacts:"));
        assert!(markdown.contains("- .canon/artifacts/run-123/change/system-slice.md"));
    }

    #[test]
    fn list_markdown_falls_back_for_unknown_targets() {
        let value = json!({
            "entries": ["one", {"two": 2}]
        });

        let markdown = render_markdown_from_json(&value, "methods", None);

        assert!(markdown.contains("# methods"));
        assert!(markdown.contains("- one"));
        assert!(markdown.contains("- {\"two\":2}"));
    }

    #[test]
    fn risk_zone_markdown_surfaces_provisional_classification() {
        let value = json!({
            "entries": [{
                "mode": "discovery",
                "risk": "bounded-impact",
                "zone": "yellow",
                "risk_was_supplied": false,
                "zone_was_supplied": true,
                "confidence": "moderate",
                "requires_confirmation": true,
                "headline": "Canon inferred the missing risk class as `bounded-impact` from the supplied intake.",
                "rationale": "Use the inferred pair as a provisional starting point.",
                "signals": ["Detected bounded-impact signal `boundary` in the intake."],
                "risk_signals": ["Detected bounded-impact signal `boundary` in the intake."],
                "zone_signals": ["User or caller already supplied the usage zone explicitly."]
            }]
        });

        let markdown = render_markdown_from_json(&value, "risk-zone", None);

        assert!(markdown.contains("# risk-zone"));
        assert!(markdown.contains("Risk: bounded-impact (inferred)"));
        assert!(markdown.contains("Zone: yellow (provided)"));
        assert!(markdown.contains("Needs Confirmation: yes"));
        assert!(markdown.contains("## Signals"));
    }

    #[test]
    fn risk_zone_text_is_machine_parsable() {
        let value = json!({
            "entries": [{
                "mode": "requirements",
                "risk": "low-impact",
                "zone": "green",
                "risk_was_supplied": false,
                "zone_was_supplied": false,
                "confidence": "low",
                "requires_confirmation": true,
                "headline": "Canon inferred `low-impact` risk and `green` zone from the supplied intake.",
                "rationale": "Use the inferred pair as a provisional starting point.",
                "risk_rationale": "The intake looks exploratory.",
                "zone_rationale": "The intake reads like isolated planning work.",
                "signals": ["Mode `requirements` stays read-only and exploratory at this stage."],
                "risk_signals": ["Mode `requirements` stays read-only and exploratory at this stage."],
                "zone_signals": ["Mode `requirements` can stay in green when the intake is still isolated to planning or analysis."]
            }]
        });

        let text = render_risk_zone_text(&value);

        assert!(text.contains("TARGET=risk-zone"));
        assert!(text.contains("INFERRED_RISK=low-impact"));
        assert!(text.contains("INFERRED_ZONE=green"));
        assert!(text.contains("NEEDS_CONFIRMATION=true"));
        assert!(text.contains(
            "RISK_SIGNAL_1=Mode `requirements` stays read-only and exploratory at this stage."
        ));
    }

    #[test]
    fn run_summary_markdown_surfaces_mode_result_without_mandatory_next_step() {
        let summary = RunSummary {
            run_id: "run-123".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "requirements".to_string(),
            risk: "bounded-impact".to_string(),
            zone: "yellow".to_string(),
            system_context: None,
            state: "Completed".to_string(),
            artifact_count: 6,
            invocations_total: 3,
            invocations_denied: 1,
            invocations_pending_approval: 0,
            blocking_classification: None,
            blocked_gates: vec![GateInspectSummary {
                gate: "release-readiness".to_string(),
                status: "Blocked".to_string(),
                blockers: vec!["missing approval".to_string()],
            }],
            approval_targets: Vec::new(),
            artifact_paths: vec![".canon/artifacts/run-123/requirements/problem-statement.md".to_string()],
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            mode_result: Some(ModeResultSummary {
                headline: "Requirements packet ready for downstream review.".to_string(),
                artifact_packet_summary: "Primary artifact is ready.".to_string(),
                execution_posture: Some("recommendation-only".to_string()),
                primary_artifact_title: "Problem Statement".to_string(),
                primary_artifact_path: ".canon/artifacts/run-123/requirements/problem-statement.md".to_string(),
                primary_artifact_action: ResultActionSummary {
                    id: "open-primary-artifact".to_string(),
                    label: "Open primary artifact".to_string(),
                    host_action: "open-file".to_string(),
                    target: ".canon/artifacts/run-123/requirements/problem-statement.md"
                        .to_string(),
                    text_fallback:
                        "Open the primary artifact at .canon/artifacts/run-123/requirements/problem-statement.md."
                            .to_string(),
                },
                result_excerpt: "Build a bounded USB flashing CLI.".to_string(),
                action_chips: Vec::new(),
            }),
            recommended_next_action: None,
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("## Result"));
        assert!(markdown.contains("Requirements packet ready for downstream review."));
        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains(
            "Primary Artifact: .canon/artifacts/run-123/requirements/problem-statement.md"
        ));
        assert!(markdown.contains("Primary Artifact Action: Open primary artifact (.canon/artifacts/run-123/requirements/problem-statement.md)"));
        assert!(!markdown.contains("## Recommended Next Step"));
        assert!(markdown.contains("## Blockers"));
    }

    #[test]
    fn run_summary_markdown_renders_operational_mode_action_chips() {
        let summary = RunSummary {
            run_id: "run-incident-123".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "incident".to_string(),
            risk: "systemic-impact".to_string(),
            zone: "red".to_string(),
            system_context: Some("existing".to_string()),
            state: "AwaitingApproval".to_string(),
            artifact_count: 6,
            invocations_total: 4,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: None,
            blocked_gates: Vec::new(),
            approval_targets: vec!["gate:risk".to_string()],
            artifact_paths: vec![".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string()],
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            mode_result: Some(ModeResultSummary {
                headline: "Incident packet ready for governed containment review.".to_string(),
                artifact_packet_summary:
                    "Primary artifact bounds the active incident surface and preserves containment posture."
                        .to_string(),
                execution_posture: Some("recommendation-only".to_string()),
                primary_artifact_title: "Incident Frame".to_string(),
                primary_artifact_path:
                    ".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string(),
                primary_artifact_action: ResultActionSummary {
                    id: "open-primary-artifact".to_string(),
                    label: "Open primary artifact".to_string(),
                    host_action: "open-file".to_string(),
                    target:
                        ".canon/artifacts/run-incident-123/incident/incident-frame.md".to_string(),
                    text_fallback:
                        "Open the primary artifact at .canon/artifacts/run-incident-123/incident/incident-frame.md."
                            .to_string(),
                },
                result_excerpt: "Containment stays bounded to payments-api and checkout flow."
                    .to_string(),
                action_chips: vec![canon_engine::ActionChip {
                    id: "inspect-evidence".to_string(),
                    label: "Inspect evidence".to_string(),
                    skill: "canon-inspect-evidence".to_string(),
                    intent: "Inspect".to_string(),
                    prefilled_args: std::collections::BTreeMap::new(),
                    required_user_inputs: Vec::new(),
                    visibility_condition:
                        "state is AwaitingApproval or Completed".to_string(),
                    recommended: true,
                    text_fallback:
                        "Inspect evidence for run run-incident-123: canon inspect evidence --run run-incident-123."
                            .to_string(),
                }],
            }),
            recommended_next_action: None,
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("Mode: incident"));
        assert!(markdown.contains("Execution Posture: recommendation-only"));
        assert!(markdown.contains("Action Chips:"));
        assert!(markdown.contains(
            "Inspect evidence for run run-incident-123: canon inspect evidence --run run-incident-123. (recommended)"
        ));
    }

    #[test]
    fn run_summary_markdown_keeps_mandatory_next_step_for_gated_runs() {
        let summary = RunSummary {
            run_id: "run-456".to_string(),
            uuid: None,
            owner: "Owner".to_string(),
            mode: "change".to_string(),
            risk: "systemic-impact".to_string(),
            zone: "yellow".to_string(),
            system_context: Some("existing".to_string()),
            state: "AwaitingApproval".to_string(),
            artifact_count: 0,
            invocations_total: 2,
            invocations_denied: 0,
            invocations_pending_approval: 1,
            blocking_classification: Some("approval-gated".to_string()),
            blocked_gates: Vec::new(),
            approval_targets: vec!["invocation:req-1".to_string()],
            artifact_paths: Vec::new(),
            closure_status: None,
            decomposition_scope: None,
            closure_findings: Vec::new(),
            closure_notes: None,
            mode_result: None,
            recommended_next_action: Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "Approval is required; inspect the evidence lineage before deciding."
                    .to_string(),
                target: None,
            }),
        };

        let markdown = render_run_summary_markdown(&summary);

        assert!(markdown.contains("## Recommended Next Step"));
        assert!(markdown.contains("Action: inspect-evidence"));
    }
}
