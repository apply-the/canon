use serde_json::Value;
use std::path::Path;

/// Required method IDs that every Canon assistant plugin must expose.
pub const REQUIRED_METHODS: &[&str] = &[
    "clarify-input",
    "start-governed-packet",
    "inspect-status",
    "inspect-evidence",
    "review-packet",
    "verify-claims",
    "publish-packet",
];

/// Required top-level metadata fields in a Canon plugin manifest.
pub const REQUIRED_METADATA_FIELDS: &[&str] = &[
    "name",
    "displayName",
    "version",
    "description",
    "author",
    "homepage",
    "repository",
    "license",
    "keywords",
    "capabilities",
];

/// Marketing or positioning terms that Canon plugin descriptions must not use.
pub const PROHIBITED_POSITIONING: &[&str] =
    &["agent framework", "orchestrator", "coding agent", "workspace mutation engine"];

/// Extracts the workspace package version string from a `Cargo.toml` file contents.
pub fn workspace_version_from_toml(cargo_toml: &str) -> Result<String, String> {
    let parsed: toml::Value =
        toml::from_str(cargo_toml).map_err(|error| format!("Cargo.toml parse error: {error}"))?;
    parsed["workspace"]["package"]["version"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "workspace package version must be a string".to_string())
}

/// Extracts a string array from a JSON value by field name.
pub fn string_array<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, String> {
    let entries = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{field} must be an array"))?;

    entries
        .iter()
        .map(|entry| entry.as_str().ok_or_else(|| format!("{field} entries must be strings")))
        .collect()
}

/// Extracts the `id` fields from the `capabilities` array in a plugin manifest.
pub fn capability_ids(value: &Value) -> Result<Vec<String>, String> {
    id_array(value, "capabilities", "capability")
}

/// Extracts the `id` fields from the `commands` array in a plugin manifest.
pub fn command_ids(value: &Value) -> Result<Vec<String>, String> {
    id_array(value, "commands", "command")
}

fn id_array(value: &Value, field: &str, label: &str) -> Result<Vec<String>, String> {
    let entries = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{field} must be an array"))?;

    entries
        .iter()
        .map(|entry| {
            entry
                .get("id")
                .and_then(Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| format!("{label} id must be a string"))
        })
        .collect()
}

/// Returns the first prohibited positioning term found anywhere in the JSON value, or `None`.
pub fn string_contains_any(value: &Value, prohibited: &[&str]) -> Option<String> {
    match value {
        Value::String(text) => prohibited
            .iter()
            .find(|term| text.to_ascii_lowercase().contains(&term.to_ascii_lowercase()))
            .map(|term| (*term).to_string()),
        Value::Array(entries) => {
            entries.iter().find_map(|entry| string_contains_any(entry, prohibited))
        }
        Value::Object(map) => map.values().find_map(|entry| string_contains_any(entry, prohibited)),
        _ => None,
    }
}

/// Validates a plugin manifest JSON value and returns a list of validation error strings.
///
/// Checks for required metadata fields, version alignment, required methods,
/// prohibited capability IDs, and prohibited positioning language.
pub fn manifest_errors(manifest: &Value, version: &str, root: &Path) -> Vec<String> {
    let mut errors = Vec::new();

    append_missing_metadata_errors(manifest, &mut errors);
    append_version_error(manifest, version, &mut errors);
    append_required_capability_errors(manifest, &mut errors);
    append_path_errors(manifest, root, &mut errors);
    append_positioning_errors(manifest, &mut errors);

    errors
}

fn append_missing_metadata_errors(manifest: &Value, errors: &mut Vec<String>) {
    for field in REQUIRED_METADATA_FIELDS {
        if manifest.get(field).is_none() {
            errors.push(format!("missing required field: {field}"));
        }
    }
}

fn append_version_error(manifest: &Value, version: &str, errors: &mut Vec<String>) {
    if manifest.get("version").and_then(Value::as_str) != Some(version) {
        errors.push("manifest version does not match workspace version".to_string());
    }
}

fn append_required_capability_errors(manifest: &Value, errors: &mut Vec<String>) {
    match capability_ids(manifest) {
        Ok(capabilities) => {
            for required in REQUIRED_METHODS {
                if !capabilities.iter().any(|capability| capability == required) {
                    errors.push(format!("missing required governed method: {required}"));
                }
            }
        }
        Err(error) => errors.push(error),
    }
}

fn append_path_errors(manifest: &Value, root: &Path, errors: &mut Vec<String>) {
    let Some(paths) = manifest.get("paths").and_then(Value::as_object) else {
        errors.push("missing paths object".to_string());
        return;
    };

    for value in paths.values() {
        let Some(path) = value.as_str() else {
            errors.push("path references must be strings".to_string());
            continue;
        };
        if !root.join(path).exists() {
            errors.push(format!("referenced path does not exist: {path}"));
        }
    }
}

fn append_positioning_errors(manifest: &Value, errors: &mut Vec<String>) {
    if let Some(term) = string_contains_any(manifest, PROHIBITED_POSITIONING) {
        errors.push(format!("prohibited positioning term found: {term}"));
    }
}
