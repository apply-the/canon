//! Terminal UI support for guided `canon init`.

pub mod init;
pub mod render;
pub mod terminal;

use canon_engine::AiTool;

use crate::error::{CliError, CliResult};

use self::init::{GuidedInitAction, GuidedInitSession};

pub(crate) fn detect_guided_terminal_readiness() -> CliResult<terminal::TerminalReadiness> {
    terminal::detect_terminal_readiness(render::required_terminal_size())
}

pub(crate) fn run_guided_init(requested_ai: Option<AiTool>) -> CliResult<Option<AiTool>> {
    let mut session = GuidedInitSession::new(requested_ai);
    let mut terminal = terminal::GuidedTerminal::enter()?;

    loop {
        terminal.draw(&session)?;

        match session.apply_event(terminal.next_event()?) {
            GuidedInitAction::Continue => {}
            GuidedInitAction::Initialize(ai_tool) => return Ok(ai_tool),
            GuidedInitAction::Interrupted => {
                return Err(CliError::InvalidInput(terminal::interrupted_message().to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::path::Path;
    use std::sync::MutexGuard;

    use canon_engine::AiTool;
    use tempfile::tempdir;

    use super::{detect_guided_terminal_readiness, run_guided_init};
    use crate::tui::terminal::guided_init_test_env_lock;

    const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
    const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
    const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";
    const TEST_CAPTURE_ENV: &str = "CANON_TUI_TEST_CAPTURE_PATH";

    struct TestEnv {
        _guard: MutexGuard<'static, ()>,
        saved: Vec<(&'static str, Option<OsString>)>,
    }

    impl TestEnv {
        fn set(
            interactive: Option<&str>,
            size: Option<&str>,
            events: Option<&str>,
            capture_path: Option<&Path>,
        ) -> Self {
            let guard = guided_init_test_env_lock().lock().expect("env lock");
            let names = [TEST_INTERACTIVE_ENV, TEST_SIZE_ENV, TEST_EVENTS_ENV, TEST_CAPTURE_ENV];
            let saved = names.into_iter().map(|name| (name, env::var_os(name))).collect();

            for name in names {
                unsafe { env::remove_var(name) };
            }
            if let Some(value) = interactive {
                unsafe { env::set_var(TEST_INTERACTIVE_ENV, value) };
            }
            if let Some(value) = size {
                unsafe { env::set_var(TEST_SIZE_ENV, value) };
            }
            if let Some(value) = events {
                unsafe { env::set_var(TEST_EVENTS_ENV, value) };
            }
            if let Some(value) = capture_path {
                unsafe { env::set_var(TEST_CAPTURE_ENV, value) };
            }

            Self { _guard: guard, saved }
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            for (name, value) in self.saved.drain(..) {
                match value {
                    Some(original) => unsafe { env::set_var(name, original) },
                    None => unsafe { env::remove_var(name) },
                }
            }
        }
    }

    #[test]
    fn detect_guided_terminal_readiness_honors_scripted_overrides() {
        let _env = TestEnv::set(Some("1"), Some("120x40"), None, None);

        let readiness = detect_guided_terminal_readiness().expect("scripted readiness");

        assert!(readiness.interactive());
        assert_eq!(readiness.block_reason(), None);
        assert!(readiness.scripted());
    }

    #[test]
    fn run_guided_init_returns_selected_assistant_from_scripted_flow() {
        let workspace = tempdir().expect("tempdir");
        let capture_path = workspace.path().join("guided-init.log");
        let _env = TestEnv::set(
            Some("1"),
            Some("120x40"),
            Some("enter,enter"),
            Some(capture_path.as_path()),
        );

        let selected = run_guided_init(Some(AiTool::Copilot)).expect("guided init succeeds");

        assert_eq!(selected, Some(AiTool::Copilot));
        let capture = std::fs::read_to_string(capture_path).expect("capture log");
        assert!(capture.contains("Copilot"));
    }

    #[test]
    fn run_guided_init_reports_interrupted_flow() {
        let _env = TestEnv::set(Some("1"), Some("120x40"), Some("ctrl-c"), None);

        let error = run_guided_init(None).expect_err("ctrl-c should interrupt guided init");

        assert_eq!(
            error.to_string(),
            "guided init interrupted before initialization; no .canon changes were made"
        );
    }
}
