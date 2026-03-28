use serde::Serialize;
use serde_json::Value;

use crate::app::OutputFormat;

pub fn print_value<T: Serialize>(
    value: &T,
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
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
    format: OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Markdown => {
            let json = serde_json::to_value(value)?;
            println!("{}", render_markdown_from_json(&json));
            Ok(())
        }
        other => print_value(value, other),
    }
}

fn render_markdown_from_json(value: &Value) -> String {
    let target = value.get("target").and_then(Value::as_str).unwrap_or("inspect");
    let mut lines = vec![format!("# {target}")];

    match value.get("entries") {
        Some(Value::Array(entries)) if !entries.is_empty() => {
            lines.push(String::new());
            for entry in entries {
                match entry {
                    Value::String(item) => lines.push(format!("- {item}")),
                    other => lines.push(format!(
                        "- `{}`",
                        serde_json::to_string(other).unwrap_or_else(|_| "{}".to_string())
                    )),
                }
            }
        }
        _ => {
            lines.push(String::new());
            lines.push("- No entries recorded.".to_string());
        }
    }

    lines.join("\n")
}
