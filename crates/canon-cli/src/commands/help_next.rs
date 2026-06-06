use canon_engine::EngineService;
use serde::Serialize;

use crate::app::OutputFormat;
use crate::error::CliResult;
use crate::output;

/// Run the help-next diagnostic and return the recommendation.
pub fn execute(service: &EngineService, format: OutputFormat) -> CliResult<i32> {
    let rec = service.help_next()?;
    match format {
        OutputFormat::Text | OutputFormat::Markdown => {
            println!("{}", render_human(&rec));
        }
        other => output::print_value(&render_json_value(&rec)?, other)?,
    }
    Ok(0)
}

fn render_human(rec: &canon_engine::domain::help_next::CanonHelpNextRecommendation) -> String {
    let mut out = String::new();
    out.push_str(&format!("State: {}\n", rec.state.label()));

    if let Some(ref primary) = rec.primary_issue {
        out.push_str("Blockers found: yes\n---\n");
        out.push_str(&format!("{}\n", primary.message));
    } else {
        out.push_str("No blockers found.\n");
    }

    out.push_str(&format!("Next action: {}\n", rec.recommended_action));
    if let Some(ref cmd) = rec.recommended_command {
        out.push_str(&format!("Command: {cmd}\n"));
    }
    out.push_str(&format!("Why: {}\n", rec.reason));
    if let Some(ref link) = rec.docs_link {
        out.push_str(&format!("Docs: {link}\n"));
    }

    if rec.additional_count > 0 {
        out.push_str(&format!(
            "{} additional issue{} detected.\n",
            rec.additional_count,
            if rec.additional_count == 1 { "" } else { "s" }
        ));
    }
    out
}

#[derive(Serialize)]
struct JsonCanonOutput {
    state: String,
    blockers_found: bool,
    recommended_action: String,
    recommended_command: Option<String>,
    reason: String,
    docs_link: Option<String>,
    additional_count: u64,
}

fn render_json_value(
    rec: &canon_engine::domain::help_next::CanonHelpNextRecommendation,
) -> CliResult<serde_json::Value> {
    let json_output = JsonCanonOutput {
        state: rec.state.label().to_string(),
        blockers_found: rec.blockers_found,
        recommended_action: rec.recommended_action.clone(),
        recommended_command: rec.recommended_command.clone(),
        reason: rec.reason.clone(),
        docs_link: rec.docs_link.clone(),
        additional_count: rec.additional_count,
    };
    Ok(serde_json::to_value(json_output)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_output_renders_state_and_command() {
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::ready(None);
        let out = render_human(&rec);
        assert!(out.contains("State: ready"));
        assert!(out.contains("canon publish"));
    }

    #[test]
    fn human_output_with_blockers_shows_primary_issue() {
        let diag = canon_engine::domain::help_next::CanonHelpNextDiagnostic {
            key: "block".into(),
            severity: canon_engine::domain::help_next::CanonDiagnosticSeverity::Blocking,
            message: "blocked: missing doc".into(),
            source: None,
            command: Some("canon run".into()),
            docs_key: "fallback".into(),
        };
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::from_diagnostics(
            canon_engine::domain::help_next::CanonHelpNextState::IncompleteDocuments,
            vec![diag],
            None,
        );
        let out = render_human(&rec);
        assert!(out.contains("Blockers found: yes"));
        assert!(out.contains("blocked: missing doc"));
    }

    #[test]
    fn human_output_shows_additional_count() {
        let d1 = canon_engine::domain::help_next::CanonHelpNextDiagnostic {
            key: "b".into(),
            severity: canon_engine::domain::help_next::CanonDiagnosticSeverity::Blocking,
            message: "blocked".into(),
            source: None,
            command: None,
            docs_key: "fallback".into(),
        };
        let d2 = canon_engine::domain::help_next::CanonHelpNextDiagnostic {
            key: "w".into(),
            severity: canon_engine::domain::help_next::CanonDiagnosticSeverity::Warning,
            message: "warn".into(),
            source: None,
            command: None,
            docs_key: "fallback".into(),
        };
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::from_diagnostics(
            canon_engine::domain::help_next::CanonHelpNextState::IncompleteDocuments,
            vec![d1, d2],
            None,
        );
        let out = render_human(&rec);
        assert!(out.contains("1 additional issue"));
    }

    #[test]
    fn human_output_with_docs_link() {
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::ready(Some(
            "wiki/canon-help".into(),
        ));
        let out = render_human(&rec);
        assert!(out.contains("Docs: wiki/canon-help"));
    }

    #[test]
    fn json_output_is_valid_json() {
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::ready(None);
        let val = render_json_value(&rec).unwrap();
        assert_eq!(val["state"], "ready");
        assert_eq!(val["blockers_found"], false);
        assert_eq!(val["recommended_command"], "canon publish");
    }

    #[test]
    fn json_output_with_blockers() {
        let diag = canon_engine::domain::help_next::CanonHelpNextDiagnostic {
            key: "block".into(),
            severity: canon_engine::domain::help_next::CanonDiagnosticSeverity::Blocking,
            message: "blocked".into(),
            source: None,
            command: Some("canon run".into()),
            docs_key: "fallback".into(),
        };
        let rec = canon_engine::domain::help_next::CanonHelpNextRecommendation::from_diagnostics(
            canon_engine::domain::help_next::CanonHelpNextState::NotInitialized,
            vec![diag],
            None,
        );
        let val = render_json_value(&rec).unwrap();
        assert_eq!(val["state"], "not-initialized");
        assert_eq!(val["blockers_found"], true);
        assert_eq!(val["additional_count"], 0);
    }
}
