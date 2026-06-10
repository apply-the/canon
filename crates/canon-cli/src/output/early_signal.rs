//! Stdout rendering for early signal pass events.
//!
//! When `--output json` is selected during `canon pr-review prepare`, early
//! signal lifecycle events are emitted as one JSON object per line on stdout.
//! When `--output text` (default), a human-readable markdown summary is
//! emitted instead.

use std::io::Write;

use crate::app::OutputFormat;
use canon_engine::domain::review::EarlySignalEvent;

/// Render early signal events to the given output format.
///
/// When `format` is `Json`, emits one JSON object per line.
/// When `format` is `Text`, emits a markdown summary.
#[allow(dead_code)]
pub fn render_early_signal_events(events: &[EarlySignalEvent], format: OutputFormat) {
    render_early_signal_events_to(events, format, &mut std::io::stdout());
}

/// Render early signal events to a generic writer (testable).
pub(crate) fn render_early_signal_events_to(
    events: &[EarlySignalEvent],
    format: OutputFormat,
    writer: &mut dyn Write,
) {
    match format {
        OutputFormat::Json => {
            for event in events {
                if let Ok(line) = serde_json::to_string(event) {
                    let _ = writeln!(writer, "{}", line);
                }
            }
        }
        _ => {
            let total = events.len();
            let _ = writeln!(writer, "# Early Signal Pass Summary\n\n**Total events**: {total}\n");
            for event in events {
                let kind = format!("{:?}", event.event).to_lowercase();
                let _ = writeln!(writer, "- `{kind}` at `{}`", event.timestamp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use canon_engine::domain::review::{EarlySignalEvent, EarlySignalEventKind};
    use time::OffsetDateTime;

    fn sample_event(kind: EarlySignalEventKind, run_id: &str) -> EarlySignalEvent {
        EarlySignalEvent {
            event: kind,
            run_id: run_id.to_string(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            total_files: None,
            file_classified: None,
            finding: None,
            completed: None,
            skipped: None,
            failed: None,
            duration_ms: None,
            rule_duration_ms: None,
            skipped_rules: None,
            stack_trace: None,
            host_info: None,
        }
    }

    #[test]
    fn render_json_emits_one_line_per_event() {
        let events = vec![
            sample_event(EarlySignalEventKind::Started, "prr-1"),
            sample_event(EarlySignalEventKind::Completed, "prr-1"),
        ];
        let mut buf = Vec::new();
        render_early_signal_events_to(&events, OutputFormat::Json, &mut buf);
        let output = String::from_utf8(buf).unwrap();
        let lines: Vec<_> = output.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("\"event\":\"started\""));
        assert!(lines[1].contains("\"event\":\"completed\""));
    }

    #[test]
    fn render_text_emits_summary() {
        let events = vec![sample_event(EarlySignalEventKind::Started, "prr-1")];
        let mut buf = Vec::new();
        render_early_signal_events_to(&events, OutputFormat::Text, &mut buf);
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Early Signal Pass Summary"));
        assert!(output.contains("**Total events**: 1"));
        assert!(output.contains("started"));
    }

    #[test]
    fn render_text_handles_empty_events() {
        let events: Vec<EarlySignalEvent> = vec![];
        let mut buf = Vec::new();
        render_early_signal_events_to(&events, OutputFormat::Text, &mut buf);
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Total events**: 0"));
    }
}
