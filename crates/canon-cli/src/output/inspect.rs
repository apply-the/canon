//! Markdown and text renderers for `inspect` command output.
//!
//! Each public function in this module corresponds to one inspect target
//! (e.g. `artifacts`, `evidence`, `risk-zone`).  The text-mode renderer for
//! `risk-zone` ([`render_risk_zone_text`]) is also kept here because it
//! operates on the same inspect payload shape.

use serde_json::{Map, Value};

use super::primitives::{
    humanize_path, render_kv_field, render_scalar_field, scalar_value, string_list,
    supplied_suffix, yes_no,
};

/// Renders the `risk-zone` inspect payload as machine-parsable KEY=VALUE text.
///
/// Each line is a `KEY=VALUE` pair suitable for shell `source` or `grep`
/// extraction.  Signal lists are expanded with a numeric suffix
/// (`SIGNAL_1`, `RISK_SIGNAL_1`, etc.).
pub(super) fn render_risk_zone_text(value: &Value) -> String {
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

/// Renders an `entries` list as a generic Markdown bullet list.
///
/// Used as the fallback branch in [`super::dispatch::render_markdown_from_json`]
/// for any target name that has no dedicated renderer.
pub(super) fn render_list_markdown(title: &str, entries: &[Value]) -> String {
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

/// Renders the `artifacts` inspect payload as a Markdown artifact listing.
pub(super) fn render_artifacts_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# artifacts".to_string()];

    append_inspect_metadata(&mut lines, run_id, system_context);

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

/// Renders the `evidence` inspect payload as a structured Markdown document.
pub(super) fn render_evidence_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# evidence".to_string()];

    append_inspect_metadata(&mut lines, run_id, system_context);

    if entries.is_empty() {
        append_placeholder(&mut lines, "- No evidence recorded.");
        return lines.join("\n");
    }

    let Some(entry) = entries.first().and_then(Value::as_object) else {
        append_placeholder(&mut lines, "- No evidence recorded.");
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

    append_heading_string_list(&mut lines, "## Upstream Sources", &upstream_source_refs);
    append_heading_string_list(&mut lines, "## Carried-Forward Context", &carried_forward_items);
    append_heading_path_list(&mut lines, "## Readable Artifacts", &artifact_links);
    append_heading_string_list(&mut lines, "## Generation Paths", &generation_paths);
    append_heading_string_list(&mut lines, "## Validation Paths", &validation_paths);
    append_heading_string_list(&mut lines, "## Denied Invocations", &denied_invocations);

    lines.join("\n")
}

/// Renders the `invocations` inspect payload as a per-request Markdown document.
pub(super) fn render_invocations_markdown(
    entries: &[Value],
    run_id: Option<&str>,
    system_context: Option<&str>,
) -> String {
    let mut lines = vec!["# invocations".to_string()];

    append_inspect_metadata(&mut lines, run_id, system_context);

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

/// Renders the `risk-zone` inspect payload as a structured Markdown document.
pub(super) fn render_risk_zone_markdown(entries: &[Value]) -> String {
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

/// Renders the `clarity` inspect payload as a full Markdown inspection report.
pub(super) fn render_clarity_markdown(entries: &[Value]) -> String {
    let mut lines = vec!["# clarity".to_string()];

    let Some(entry) = entries.first().and_then(Value::as_object) else {
        append_placeholder(&mut lines, "- No clarity inspection recorded.");
        return lines.join("\n");
    };

    lines.push(String::new());
    render_scalar_field(&mut lines, "Mode", entry.get("mode"));
    lines.push(format!("Requires Clarification: {}", yes_no(entry.get("requires_clarification"))));

    append_optional_text_section(
        &mut lines,
        "## Document Summary",
        scalar_value(entry.get("summary")),
    );
    append_heading_path_list(
        &mut lines,
        "## Source Inputs",
        &string_list(entry.get("source_inputs")),
    );
    append_authoring_lifecycle_section(&mut lines, entry.get("authoring_lifecycle"));
    append_heading_string_list(
        &mut lines,
        "## Reasoning Signals",
        &string_list(entry.get("reasoning_signals")),
    );
    append_output_quality_section(&mut lines, entry.get("output_quality"));
    append_heading_string_list(
        &mut lines,
        "## Missing Context",
        &string_list(entry.get("missing_context")),
    );
    append_clarification_questions_section(&mut lines, entry.get("clarification_questions"));
    append_optional_text_section(
        &mut lines,
        "## Recommended Focus",
        scalar_value(entry.get("recommended_focus")),
    );

    lines.join("\n")
}

fn append_inspect_metadata(
    lines: &mut Vec<String>,
    run_id: Option<&str>,
    system_context: Option<&str>,
) {
    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
    }
    if let Some(system_context) = system_context {
        lines.push(String::new());
        lines.push(format!("System Context: {system_context}"));
    }
}

fn append_placeholder(lines: &mut Vec<String>, message: &str) {
    lines.push(String::new());
    lines.push(message.to_string());
}

fn push_heading(lines: &mut Vec<String>, heading: &str) {
    lines.push(String::new());
    lines.push(heading.to_string());
    lines.push(String::new());
}

fn append_heading_string_list(lines: &mut Vec<String>, heading: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    push_heading(lines, heading);
    for item in items {
        lines.push(format!("- {item}"));
    }
}

fn append_heading_path_list(lines: &mut Vec<String>, heading: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    push_heading(lines, heading);
    for item in items {
        lines.push(format!("- {}", humanize_path(item)));
    }
}

fn append_labeled_string_list(lines: &mut Vec<String>, label: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push(label.to_string());
    for item in items {
        lines.push(format!("- {item}"));
    }
}

fn append_labeled_path_list(lines: &mut Vec<String>, label: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push(label.to_string());
    for item in items {
        lines.push(format!("- {}", humanize_path(item)));
    }
}

fn append_optional_text_section(lines: &mut Vec<String>, heading: &str, text: Option<String>) {
    let Some(text) = text else {
        return;
    };

    push_heading(lines, heading);
    lines.push(text);
}

fn append_optional_detail_line(lines: &mut Vec<String>, label: &str, value: Option<&Value>) {
    if let Some(value) = scalar_value(value) {
        lines.push(format!("{label}: {value}"));
    }
}

fn append_authoring_lifecycle_section(lines: &mut Vec<String>, value: Option<&Value>) {
    let Some(authoring_lifecycle) = value.and_then(Value::as_object) else {
        return;
    };

    push_heading(lines, "## Authoring Lifecycle");
    append_optional_detail_line(lines, "Packet Shape", authoring_lifecycle.get("packet_shape"));
    append_optional_detail_line(
        lines,
        "Authority Status",
        authoring_lifecycle.get("authority_status"),
    );
    append_labeled_path_list(
        lines,
        "Authoritative Inputs:",
        &string_list(authoring_lifecycle.get("authoritative_inputs")),
    );
    append_labeled_path_list(
        lines,
        "Supporting Inputs:",
        &string_list(authoring_lifecycle.get("supporting_inputs")),
    );
    append_labeled_string_list(
        lines,
        "Readiness Delta:",
        &string_list(authoring_lifecycle.get("readiness_delta")),
    );

    if let Some(next_authoring_step) = scalar_value(authoring_lifecycle.get("next_authoring_step"))
    {
        lines.push(String::new());
        lines.push("Next Authoring Step:".to_string());
        lines.push(next_authoring_step);
    }
}

fn append_output_quality_section(lines: &mut Vec<String>, value: Option<&Value>) {
    let Some(output_quality) = value.and_then(Value::as_object) else {
        return;
    };

    push_heading(lines, "## Output Quality");
    append_optional_detail_line(lines, "Posture", output_quality.get("posture"));
    append_optional_detail_line(
        lines,
        "Materially Closed",
        output_quality.get("materially_closed"),
    );
    append_labeled_string_list(
        lines,
        "Evidence Signals:",
        &string_list(output_quality.get("evidence_signals")),
    );
    append_labeled_string_list(
        lines,
        "Downgrade Reasons:",
        &string_list(output_quality.get("downgrade_reasons")),
    );
}

fn append_clarification_questions_section(lines: &mut Vec<String>, value: Option<&Value>) {
    let questions = value.and_then(Value::as_array).cloned().unwrap_or_default();
    if questions.is_empty() {
        return;
    }

    push_heading(lines, "## Clarification Questions");
    let last_index = questions.len().saturating_sub(1);
    for (index, question) in questions.iter().enumerate() {
        let Some(question) = question.as_object() else {
            continue;
        };

        append_clarification_question(lines, question, index, index < last_index);
    }
}

fn append_clarification_question(
    lines: &mut Vec<String>,
    question: &Map<String, Value>,
    index: usize,
    has_following_question: bool,
) {
    let prompt = scalar_value(question.get("prompt"))
        .unwrap_or_else(|| "Missing clarification prompt".to_string());
    lines.push(format!("{}. {prompt}", index + 1));
    append_optional_detail_line(lines, "Why", question.get("rationale"));
    append_optional_detail_line(lines, "Evidence", question.get("evidence"));
    append_optional_detail_line(lines, "Affects", question.get("affects"));
    append_optional_detail_line(lines, "Default if skipped", question.get("default_if_skipped"));
    append_optional_detail_line(lines, "Status", question.get("status"));

    if has_following_question {
        lines.push(String::new());
    }
}
