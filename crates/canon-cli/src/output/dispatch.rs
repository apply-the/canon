//! Top-level Markdown dispatch for `inspect` command output.
//!
//! [`render_markdown_from_json`] routes an inspect payload to the correct
//! domain-specific renderer based on the inspect target name.

use serde_json::Value;

use super::inspect::{
    render_artifacts_markdown, render_clarity_markdown, render_evidence_markdown,
    render_invocations_markdown, render_list_markdown, render_refinement_markdown,
    render_risk_zone_markdown,
};

/// Routes a JSON inspect payload to the appropriate Markdown renderer.
///
/// The `target_name` string matches the inspect subcommand (e.g. `"artifacts"`,
/// `"evidence"`, `"risk-zone"`).  Unknown targets fall back to a generic bullet
/// list via [`render_list_markdown`].
pub(super) fn render_markdown_from_json(
    value: &Value,
    target_name: &str,
    run_id: Option<&str>,
) -> String {
    let entries = value.get("entries").and_then(Value::as_array).cloned().unwrap_or_default();
    let system_context = value.get("system_context").and_then(Value::as_str);

    match target_name {
        "artifacts" => render_artifacts_markdown(&entries, run_id, system_context),
        "clarity" => render_clarity_markdown(&entries),
        "evidence" => render_evidence_markdown(&entries, run_id, system_context),
        "invocations" => render_invocations_markdown(&entries, run_id, system_context),
        "refinement" => render_refinement_markdown(&entries, run_id, system_context),
        "risk-zone" => render_risk_zone_markdown(&entries),
        _ => render_list_markdown(target_name, &entries),
    }
}
