//! Low-level rendering helpers shared across all output submodules.
//!
//! Each function is small and pure: it either extracts a scalar from a
//! `serde_json::Value` or formats a string fragment.  No I/O takes place here.

use serde_json::Value;

/// Appends `"{label}: {value}"` to `lines` when `value` is a non-null,
/// non-empty JSON value.  Null and empty strings are silently skipped.
pub(super) fn render_scalar_field(lines: &mut Vec<String>, label: &str, value: Option<&Value>) {
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

/// Appends `"{label}={value}"` (shell key-value style) to `lines`.
///
/// Skips absent and non-scalar values.
pub(super) fn render_kv_field(lines: &mut Vec<String>, label: &str, value: Option<&Value>) {
    let Some(value) = scalar_value(value) else {
        return;
    };

    lines.push(format!("{label}={value}"));
}

/// Extracts a scalar string representation from a JSON `Value`, or `None`
/// when the value is absent or `null`.
pub(super) fn scalar_value(value: Option<&Value>) -> Option<String> {
    let value = value?;
    match value {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Number(number) => Some(number.to_string()),
        other => serde_json::to_string(other).ok(),
    }
}

/// Returns `" (provided)"` or `" (inferred)"` based on the boolean flag.
pub(super) fn supplied_suffix(value: Option<&Value>) -> &'static str {
    if value.and_then(Value::as_bool).unwrap_or(false) { " (provided)" } else { " (inferred)" }
}

/// Returns `"yes"` or `"no"` based on the boolean flag.
pub(super) fn yes_no(value: Option<&Value>) -> &'static str {
    if value.and_then(Value::as_bool).unwrap_or(false) { "yes" } else { "no" }
}

/// Extracts a JSON array of strings into a `Vec<String>`, filtering out
/// non-string elements.  Returns an empty vec when the value is absent or
/// not an array.
pub(super) fn string_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items.iter().filter_map(Value::as_str).map(ToString::to_string).collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Ensures `.canon/` is the leading prefix for stored artifact paths.
///
/// Paths already starting with `.canon/` are returned unchanged. Paths
/// starting with `artifacts/` get the `.canon/` prefix prepended. All other
/// paths (e.g. workspace-relative source files) are returned as-is.
pub(super) fn humanize_path(path: &str) -> String {
    if path.starts_with(".canon/") {
        path.to_string()
    } else if path.starts_with("artifacts/") {
        format!(".canon/{path}")
    } else {
        path.to_string()
    }
}
