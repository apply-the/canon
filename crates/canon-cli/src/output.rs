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

pub fn print_inspect<T: Serialize>(
    value: &T,
    target_name: &str,
    run_id: Option<&str>,
    format: OutputFormat,
) -> CliResult<()> {
    match format {
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

    match target_name {
        "artifacts" => render_artifacts_markdown(&entries, run_id),
        "evidence" => render_evidence_markdown(&entries, run_id),
        "invocations" => render_invocations_markdown(&entries, run_id),
        _ => render_list_markdown(target_name, &entries),
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

fn render_artifacts_markdown(entries: &[Value], run_id: Option<&str>) -> String {
    let mut lines = vec!["# artifacts".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
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

fn render_evidence_markdown(entries: &[Value], run_id: Option<&str>) -> String {
    let mut lines = vec!["# evidence".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
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

fn render_invocations_markdown(entries: &[Value], run_id: Option<&str>) -> String {
    let mut lines = vec!["# invocations".to_string()];

    if let Some(run_id) = run_id {
        lines.push(String::new());
        lines.push(format!("Run ID: {run_id}"));
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
    use serde_json::json;

    use super::render_markdown_from_json;

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
                "artifact_provenance_links": ["artifacts/run-123/pr-review/review-summary.md"],
                "generation_paths": ["generation:req-1"],
                "validation_paths": ["validation:req-2"],
                "denied_invocations": ["req-3"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "evidence", Some("run-123"));

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
                "approval_state": "NotRequired",
                "latest_outcome": "Succeeded",
                "linked_artifacts": ["artifacts/run-123/brownfield-change/system-slice.md"]
            }]
        });

        let markdown = render_markdown_from_json(&value, "invocations", Some("run-123"));

        assert!(markdown.contains("# invocations"));
        assert!(markdown.contains("## req-7"));
        assert!(markdown.contains("Adapter: Shell"));
        assert!(markdown.contains("Capability: ValidateWithTool"));
        assert!(markdown.contains("Artifacts:"));
        assert!(markdown.contains("- .canon/artifacts/run-123/brownfield-change/system-slice.md"));
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
}
