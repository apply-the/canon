//! Stdout rendering for early signal pass events.
//!
//! When `--output json` is selected during `canon pr-review prepare`, early
//! signal lifecycle events are emitted as one JSON object per line on stdout.
//! When `--output text` (default), a human-readable markdown summary is
//! emitted instead.

use crate::app::OutputFormat;
use canon_engine::domain::review::EarlySignalEvent;

/// Render early signal events to the given output format.
///
/// When `format` is `Json`, emits one JSON object per line on stdout.
/// When `format` is `Text`, emits a markdown summary.
#[allow(dead_code)]
pub fn render_early_signal_events(events: &[EarlySignalEvent], format: OutputFormat) {
    match format {
        OutputFormat::Json => {
            for event in events {
                if let Ok(line) = serde_json::to_string(event) {
                    println!("{}", line);
                }
            }
        }
        _ => {
            let total = events.len();
            println!("# Early Signal Pass Summary");
            println!();
            println!("**Total events**: {total}");
            println!();
            for event in events {
                let kind = format!("{:?}", event.event).to_lowercase();
                println!("- `{kind}` at `{}`", event.timestamp);
            }
        }
    }
}
