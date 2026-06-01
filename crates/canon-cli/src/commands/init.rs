use canon_engine::{AiTool, EngineService};

use crate::app::OutputFormat;
use crate::error::{CliError, CliResult};
use crate::output;
use crate::tui;
use crate::tui::terminal::TerminalReadiness;

const STRUCTURED_OUTPUT_REQUIRES_NON_INTERACTIVE_MESSAGE: &str =
    "structured output for `canon init` requires --non-interactive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitCommandMode {
    GuidedDefault,
    NonInteractive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitExecutionPath {
    Guided,
    NonInteractive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InitCommandRequest {
    ai_tool: Option<AiTool>,
    mode: InitCommandMode,
    format: OutputFormat,
}

impl InitCommandRequest {
    pub(crate) fn new(
        ai_tool: Option<AiTool>,
        non_interactive: bool,
        format: OutputFormat,
    ) -> Self {
        Self {
            ai_tool,
            mode: if non_interactive {
                InitCommandMode::NonInteractive
            } else {
                InitCommandMode::GuidedDefault
            },
            format,
        }
    }

    pub(crate) fn ai_tool(self) -> Option<AiTool> {
        self.ai_tool
    }

    pub(crate) fn format(self) -> OutputFormat {
        self.format
    }

    pub(crate) fn mode(self) -> InitCommandMode {
        self.mode
    }

    pub(crate) fn output_for(self, path: InitExecutionPath) -> OutputFormat {
        match path {
            InitExecutionPath::Guided => OutputFormat::Text,
            InitExecutionPath::NonInteractive => self.format(),
        }
    }

    pub(crate) fn resolve_guided_default_path(
        self,
        readiness: TerminalReadiness,
    ) -> CliResult<InitExecutionPath> {
        if !readiness.interactive() {
            return Ok(InitExecutionPath::NonInteractive);
        }

        match readiness.block_reason() {
            Some(reason) => Err(CliError::InvalidInput(reason.message().to_string())),
            None => Ok(InitExecutionPath::Guided),
        }
    }

    fn validate_output_mode(self) -> CliResult<()> {
        if self.mode != InitCommandMode::NonInteractive && self.format != OutputFormat::Text {
            return Err(CliError::InvalidInput(
                STRUCTURED_OUTPUT_REQUIRES_NON_INTERACTIVE_MESSAGE.to_string(),
            ));
        }

        Ok(())
    }
}

pub fn execute(
    service: &EngineService,
    ai_tool: Option<AiTool>,
    non_interactive: bool,
    format: OutputFormat,
) -> CliResult<i32> {
    let request = InitCommandRequest::new(ai_tool, non_interactive, format);
    request.validate_output_mode()?;

    let path = match request.mode() {
        InitCommandMode::NonInteractive => InitExecutionPath::NonInteractive,
        InitCommandMode::GuidedDefault => {
            request.resolve_guided_default_path(tui::detect_guided_terminal_readiness()?)?
        }
    };

    let selected_ai = match path {
        InitExecutionPath::Guided => tui::run_guided_init(request.ai_tool())?,
        InitExecutionPath::NonInteractive => request.ai_tool(),
    };

    let summary = service.init(selected_ai)?;
    output::print_value(&summary, request.output_for(path))?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsString;
    use std::sync::MutexGuard;

    use canon_engine::{AiTool, EngineService};
    use tempfile::tempdir;

    use super::{
        InitCommandMode, InitCommandRequest, InitExecutionPath,
        STRUCTURED_OUTPUT_REQUIRES_NON_INTERACTIVE_MESSAGE, execute,
    };
    use crate::app::OutputFormat;
    use crate::tui::terminal::{TerminalReadiness, guided_init_test_env_lock};

    const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
    const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
    const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";

    struct TestEnv {
        _guard: MutexGuard<'static, ()>,
        saved: Vec<(&'static str, Option<OsString>)>,
    }

    impl TestEnv {
        fn scripted(events: &str) -> Self {
            let guard = guided_init_test_env_lock().lock().expect("env lock");
            Self::scripted_with_guard(events, guard)
        }

        fn scripted_with_guard(events: &str, guard: MutexGuard<'static, ()>) -> Self {
            let names = [TEST_INTERACTIVE_ENV, TEST_SIZE_ENV, TEST_EVENTS_ENV];
            let saved = names.into_iter().map(|name| (name, env::var_os(name))).collect();

            for name in names {
                unsafe { env::remove_var(name) };
            }
            unsafe { env::set_var(TEST_INTERACTIVE_ENV, "1") };
            unsafe { env::set_var(TEST_SIZE_ENV, "120x40") };
            unsafe { env::set_var(TEST_EVENTS_ENV, events) };

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
    fn request_tracks_non_interactive_mode() {
        let request = InitCommandRequest::new(None, true, OutputFormat::Json);

        assert_eq!(request.mode(), InitCommandMode::NonInteractive);
        assert_eq!(request.ai_tool(), None);
        assert_eq!(request.format(), OutputFormat::Json);
    }

    #[test]
    fn request_uses_guided_default_mode_without_flag() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Text);

        assert_eq!(request.mode(), InitCommandMode::GuidedDefault);
        assert_eq!(request.format(), OutputFormat::Text);
    }

    #[test]
    fn guided_output_always_uses_text_format() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Json);

        assert_eq!(request.output_for(InitExecutionPath::Guided), OutputFormat::Text);
    }

    #[test]
    fn guided_default_falls_back_when_terminal_is_unavailable() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Text);

        let path = request
            .resolve_guided_default_path(TerminalReadiness::blocked_non_interactive(false))
            .expect("non-interactive fallback");

        assert_eq!(path, InitExecutionPath::NonInteractive);
    }

    #[test]
    fn guided_default_rejects_layouts_that_do_not_fit() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Text);

        let error = request
            .resolve_guided_default_path(TerminalReadiness::blocked_layout_too_small(
                (40, 12),
                true,
            ))
            .expect_err("layout should reject guided init");

        assert_eq!(
            error.to_string(),
            "current terminal layout is too small for guided init; resize the terminal or use --non-interactive"
        );
    }

    #[test]
    fn guided_default_uses_guided_path_when_terminal_is_supported() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Text);

        let path = request
            .resolve_guided_default_path(TerminalReadiness::supported((120, 40), false))
            .expect("guided path");

        assert_eq!(path, InitExecutionPath::Guided);
    }

    #[test]
    fn structured_output_requires_non_interactive_mode() {
        let request = InitCommandRequest::new(None, false, OutputFormat::Json);

        let error =
            request.validate_output_mode().expect_err("structured output should be rejected");

        assert_eq!(error.to_string(), STRUCTURED_OUTPUT_REQUIRES_NON_INTERACTIVE_MESSAGE);
    }

    #[test]
    fn execute_initializes_runtime_state_in_temp_workspace() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());

        let code = execute(&service, None, true, OutputFormat::Json).expect("init should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join(".canon").is_dir());
    }

    #[test]
    fn scripted_test_env_restores_previous_values_on_drop() {
        let guard = guided_init_test_env_lock().lock().expect("env lock");
        unsafe { env::set_var(TEST_EVENTS_ENV, "before") };

        {
            let _env = TestEnv::scripted_with_guard("enter,enter", guard);
            assert_eq!(env::var(TEST_EVENTS_ENV).expect("scripted env value"), "enter,enter");
        }

        assert_eq!(env::var(TEST_EVENTS_ENV).expect("restored env value"), "before");
        unsafe { env::remove_var(TEST_EVENTS_ENV) };
    }

    #[test]
    fn execute_guided_default_runs_scripted_guided_flow() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let _env = TestEnv::scripted("enter,enter");

        let code = execute(&service, Some(AiTool::Claude), false, OutputFormat::Text)
            .expect("guided init should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join(".canon").is_dir());
    }
}
