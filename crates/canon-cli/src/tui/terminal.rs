//! Terminal lifecycle and preflight helpers for guided init.

use std::collections::VecDeque;
use std::env;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(not(test))]
use crossterm::cursor;
#[cfg(not(test))]
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[cfg(not(test))]
use crossterm::execute;
#[cfg(not(test))]
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

#[cfg(not(test))]
use std::io::IsTerminal;

use crate::error::{CliError, CliResult};

use super::init::{GuidedInitEvent, GuidedInitSession};
use super::render;

const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";
const TEST_CAPTURE_ENV: &str = "CANON_TUI_TEST_CAPTURE_PATH";
const BOOL_TRUE_VALUE: &str = "1";
const BOOL_FALSE_VALUE: &str = "0";
const BOOL_TRUE_TEXT: &str = "true";
const BOOL_FALSE_TEXT: &str = "false";
const SIZE_SEPARATOR: char = 'x';
const EVENT_SEPARATOR: char = ',';
const EVENT_UP: &str = "up";
const EVENT_DOWN: &str = "down";
const EVENT_ENTER: &str = "enter";
const EVENT_ESCAPE: &str = "esc";
const EVENT_CTRL_C: &str = "ctrl-c";
const FRAME_SEPARATOR: &str = "[[frame]]";
const TERMINAL_RESTORED_MARKER: &str = "terminal_restored=true";
const SCRIPT_EXHAUSTED_MESSAGE: &str =
    "guided init test event script ended before the flow completed";
const INTERRUPTED_MESSAGE: &str =
    "guided init interrupted before initialization; no .canon changes were made";
const LAYOUT_TOO_SMALL_MESSAGE: &str = "current terminal layout is too small for guided init; resize the terminal or use --non-interactive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TerminalBlockReason {
    NonInteractiveTerminal,
    LayoutTooSmall,
}

impl TerminalBlockReason {
    pub(crate) fn message(self) -> &'static str {
        match self {
            Self::NonInteractiveTerminal => "",
            Self::LayoutTooSmall => LAYOUT_TOO_SMALL_MESSAGE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TerminalReadiness {
    interactive: bool,
    size: Option<(u16, u16)>,
    block_reason: Option<TerminalBlockReason>,
    scripted: bool,
}

impl TerminalReadiness {
    pub(crate) fn supported(size: (u16, u16), scripted: bool) -> Self {
        Self { interactive: true, size: Some(size), block_reason: None, scripted }
    }

    pub(crate) fn blocked_non_interactive(scripted: bool) -> Self {
        Self {
            interactive: false,
            size: None,
            block_reason: Some(TerminalBlockReason::NonInteractiveTerminal),
            scripted,
        }
    }

    pub(crate) fn blocked_layout_too_small(size: (u16, u16), scripted: bool) -> Self {
        Self {
            interactive: true,
            size: Some(size),
            block_reason: Some(TerminalBlockReason::LayoutTooSmall),
            scripted,
        }
    }

    pub(crate) fn interactive(self) -> bool {
        self.interactive
    }

    pub(crate) fn block_reason(self) -> Option<TerminalBlockReason> {
        self.block_reason
    }

    #[cfg(test)]
    pub(crate) fn scripted(self) -> bool {
        self.scripted
    }

    #[cfg(test)]
    pub(crate) fn size(self) -> Option<(u16, u16)> {
        self.size
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuidedTerminalConfig {
    scripted_mode: bool,
    scripted_events: VecDeque<GuidedInitEvent>,
    capture_path: Option<PathBuf>,
}

pub(crate) struct GuidedTerminal {
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
    scripted_events: VecDeque<GuidedInitEvent>,
    capture_path: Option<PathBuf>,
    restored: bool,
}

pub(crate) fn detect_terminal_readiness(required_size: (u16, u16)) -> CliResult<TerminalReadiness> {
    let overrides = TerminalOverrideConfig::from_env()?;
    let interactive = match overrides.interactive {
        Some(value) => value,
        None => current_terminal_interactive(),
    };

    if !interactive {
        return Ok(TerminalReadiness::blocked_non_interactive(overrides.scripted_mode));
    }

    let size = match overrides.size {
        Some(size) => size,
        None => current_terminal_size()?,
    };

    if layout_fits(size, required_size) {
        Ok(TerminalReadiness::supported(size, overrides.scripted_mode))
    } else {
        Ok(TerminalReadiness::blocked_layout_too_small(size, overrides.scripted_mode))
    }
}

pub(crate) fn interrupted_message() -> &'static str {
    INTERRUPTED_MESSAGE
}

impl GuidedTerminal {
    pub(crate) fn enter() -> CliResult<Self> {
        let config = GuidedTerminalConfig::from_env()?;
        if config.scripted_mode {
            return Ok(Self {
                terminal: None,
                scripted_events: config.scripted_events,
                capture_path: config.capture_path,
                restored: false,
            });
        }
        let terminal = enter_live_terminal()?;

        Ok(Self {
            terminal: Some(terminal),
            scripted_events: config.scripted_events,
            capture_path: config.capture_path,
            restored: false,
        })
    }

    pub(crate) fn draw(&mut self, session: &GuidedInitSession) -> CliResult<()> {
        if let Some(path) = &self.capture_path {
            append_capture(path, &format!("{FRAME_SEPARATOR}\n{}\n", render::snapshot(session)))?;
        }

        if let Some(terminal) = &mut self.terminal {
            terminal.draw(|frame| render::draw(frame, session))?;
        }

        Ok(())
    }

    pub(crate) fn next_event(&mut self) -> CliResult<GuidedInitEvent> {
        if let Some(event) = self.scripted_events.pop_front() {
            return Ok(event);
        }

        if self.terminal.is_none() {
            return Err(CliError::InvalidInput(SCRIPT_EXHAUSTED_MESSAGE.to_string()));
        }

        next_live_event()
    }

    #[cfg(test)]
    fn scripted_for_tests(capture_path: Option<PathBuf>) -> Self {
        Self { terminal: None, scripted_events: VecDeque::new(), capture_path, restored: false }
    }

    fn restore(&mut self) -> io::Result<()> {
        if self.restored {
            return Ok(());
        }
        self.restored = true;

        let mut first_error = None;

        if let Some(terminal) = &mut self.terminal
            && let Err(error) = restore_live_terminal(terminal)
        {
            first_error.get_or_insert(error);
        }

        if let Some(path) = &self.capture_path
            && let Err(error) = append_capture(path, &format!("{TERMINAL_RESTORED_MARKER}\n"))
        {
            first_error.get_or_insert(error);
        }

        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

impl Drop for GuidedTerminal {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TerminalOverrideConfig {
    interactive: Option<bool>,
    size: Option<(u16, u16)>,
    scripted_mode: bool,
}

impl TerminalOverrideConfig {
    fn from_env() -> CliResult<Self> {
        let interactive = parse_bool_override(TEST_INTERACTIVE_ENV)?;
        let size = parse_size_override(env::var_os(TEST_SIZE_ENV).as_deref())?;
        let scripted_mode = interactive.is_some()
            || size.is_some()
            || env::var_os(TEST_EVENTS_ENV).is_some()
            || env::var_os(TEST_CAPTURE_ENV).is_some();

        Ok(Self { interactive, size, scripted_mode })
    }
}

impl GuidedTerminalConfig {
    fn from_env() -> CliResult<Self> {
        let overrides = TerminalOverrideConfig::from_env()?;
        let scripted_events = parse_scripted_events(env::var_os(TEST_EVENTS_ENV).as_deref())?;
        let capture_path = env::var_os(TEST_CAPTURE_ENV).map(PathBuf::from);

        Ok(Self { scripted_mode: overrides.scripted_mode, scripted_events, capture_path })
    }
}

fn layout_fits(actual_size: (u16, u16), required_size: (u16, u16)) -> bool {
    actual_size.0 >= required_size.0 && actual_size.1 >= required_size.1
}

#[cfg(not(test))]
fn current_terminal_interactive() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

#[cfg(test)]
fn current_terminal_interactive() -> bool {
    true
}

#[cfg(not(test))]
fn current_terminal_size() -> io::Result<(u16, u16)> {
    crossterm::terminal::size()
}

#[cfg(test)]
fn current_terminal_size() -> io::Result<(u16, u16)> {
    Ok((120, 40))
}

#[cfg(not(test))]
fn enter_live_terminal() -> CliResult<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    if let Err(error) = execute!(&mut stdout, EnterAlternateScreen, cursor::Hide) {
        let _ = disable_raw_mode();
        return Err(error.into());
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(terminal) => terminal,
        Err(error) => {
            let mut cleanup = io::stdout();
            let _ = execute!(&mut cleanup, LeaveAlternateScreen, cursor::Show);
            let _ = disable_raw_mode();
            return Err(io::Error::other(error.to_string()).into());
        }
    };

    if let Err(error) = terminal.clear() {
        let _ = terminal.show_cursor();
        let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show);
        let _ = disable_raw_mode();
        return Err(error.into());
    }

    Ok(terminal)
}

#[cfg(test)]
fn enter_live_terminal() -> CliResult<Terminal<CrosstermBackend<io::Stdout>>> {
    let (width, height) = current_terminal_size()?;

    Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        ratatui::TerminalOptions {
            viewport: ratatui::Viewport::Fixed(ratatui::layout::Rect::new(0, 0, width, height)),
        },
    )
    .map_err(|error| io::Error::other(error.to_string()).into())
}

fn next_live_event() -> CliResult<GuidedInitEvent> {
    loop {
        if let Some(event) = map_live_event(read_live_event()?) {
            return Ok(event);
        }
    }
}

#[cfg(not(test))]
fn read_live_event() -> io::Result<Event> {
    event::read()
}

#[cfg(test)]
fn read_live_event() -> io::Result<Event> {
    Ok(Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)))
}

#[cfg(not(test))]
fn restore_live_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut first_error = None;

    if let Err(error) = terminal.show_cursor() {
        first_error.get_or_insert(error);
    }
    if let Err(error) = execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show) {
        first_error.get_or_insert(error);
    }
    if let Err(error) = disable_raw_mode() {
        first_error.get_or_insert(error);
    }

    match first_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

#[cfg(test)]
fn restore_live_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    terminal.show_cursor()
}

fn map_live_event(event: Event) -> Option<GuidedInitEvent> {
    match event {
        Event::Key(key_event) => map_key_event(key_event),
        _ => None,
    }
}

fn map_key_event(key_event: KeyEvent) -> Option<GuidedInitEvent> {
    if key_event.kind != KeyEventKind::Press {
        return None;
    }

    match key_event.code {
        KeyCode::Up => Some(GuidedInitEvent::Up),
        KeyCode::Down => Some(GuidedInitEvent::Down),
        KeyCode::Enter => Some(GuidedInitEvent::Enter),
        KeyCode::Esc => Some(GuidedInitEvent::Esc),
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(GuidedInitEvent::CtrlC)
        }
        _ => None,
    }
}

fn append_capture(path: &PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(content.as_bytes())
}

fn parse_bool_override(env_name: &str) -> CliResult<Option<bool>> {
    match env::var(env_name) {
        Ok(value) => parse_bool_value(env_name, &value).map(Some),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => {
            Err(CliError::InvalidInput(format!("{env_name} must contain a UTF-8 boolean override")))
        }
    }
}

fn parse_bool_value(env_name: &str, value: &str) -> CliResult<bool> {
    match value {
        BOOL_TRUE_VALUE | BOOL_TRUE_TEXT => Ok(true),
        BOOL_FALSE_VALUE | BOOL_FALSE_TEXT => Ok(false),
        _ => Err(CliError::InvalidInput(format!(
            "{env_name} must be one of {BOOL_TRUE_VALUE}, {BOOL_FALSE_VALUE}, {BOOL_TRUE_TEXT}, or {BOOL_FALSE_TEXT}"
        ))),
    }
}

fn parse_size_override(value: Option<&OsStr>) -> CliResult<Option<(u16, u16)>> {
    match value {
        None => Ok(None),
        Some(raw) => {
            let text = raw.to_str().ok_or_else(|| {
                CliError::InvalidInput(
                    "CANON_TUI_TEST_SIZE must contain a UTF-8 size override".to_string(),
                )
            })?;
            let (width, height) = text.split_once(SIZE_SEPARATOR).ok_or_else(|| {
                CliError::InvalidInput(
                    "CANON_TUI_TEST_SIZE must use the format <width>x<height>".to_string(),
                )
            })?;

            Ok(Some((
                parse_dimension(width, TEST_SIZE_ENV)?,
                parse_dimension(height, TEST_SIZE_ENV)?,
            )))
        }
    }
}

fn parse_dimension(value: &str, env_name: &str) -> CliResult<u16> {
    value
        .parse::<u16>()
        .map_err(|_| CliError::InvalidInput(format!("{env_name} must use numeric dimensions")))
}

fn parse_scripted_events(value: Option<&OsStr>) -> CliResult<VecDeque<GuidedInitEvent>> {
    let Some(raw) = value else {
        return Ok(VecDeque::new());
    };
    let text = raw.to_str().ok_or_else(|| {
        CliError::InvalidInput("CANON_TUI_TEST_EVENTS must contain UTF-8 tokens".to_string())
    })?;

    text.split(EVENT_SEPARATOR)
        .filter(|token| !token.trim().is_empty())
        .map(parse_scripted_event)
        .collect()
}

fn parse_scripted_event(token: &str) -> CliResult<GuidedInitEvent> {
    match token.trim() {
        EVENT_UP => Ok(GuidedInitEvent::Up),
        EVENT_DOWN => Ok(GuidedInitEvent::Down),
        EVENT_ENTER => Ok(GuidedInitEvent::Enter),
        EVENT_ESCAPE => Ok(GuidedInitEvent::Esc),
        EVENT_CTRL_C => Ok(GuidedInitEvent::CtrlC),
        other => Err(CliError::InvalidInput(format!(
            "unsupported guided init test event token: {other}"
        ))),
    }
}

#[cfg(test)]
pub(crate) fn guided_init_test_env_lock() -> &'static std::sync::Mutex<()> {
    use std::sync::{Mutex, OnceLock};

    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::ffi::OsStr;
    use std::ffi::OsString;
    use std::io;
    use std::os::unix::ffi::OsStringExt;
    use std::sync::MutexGuard;

    use tempfile::tempdir;

    use super::{
        GuidedInitEvent, GuidedTerminal, INTERRUPTED_MESSAGE, TERMINAL_RESTORED_MARKER,
        TerminalBlockReason, TerminalReadiness, detect_terminal_readiness,
        guided_init_test_env_lock, map_key_event, map_live_event, parse_bool_override,
        parse_bool_value, parse_scripted_events, parse_size_override,
    };
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    const TEST_INTERACTIVE_ENV: &str = "CANON_TUI_TEST_INTERACTIVE";
    const TEST_SIZE_ENV: &str = "CANON_TUI_TEST_SIZE";
    const TEST_EVENTS_ENV: &str = "CANON_TUI_TEST_EVENTS";
    const TEST_CAPTURE_ENV: &str = "CANON_TUI_TEST_CAPTURE_PATH";

    struct TestEnv {
        _guard: MutexGuard<'static, ()>,
        saved: Vec<(&'static str, Option<OsString>)>,
    }

    impl TestEnv {
        fn new() -> Self {
            let guard = guided_init_test_env_lock().lock().expect("env lock");
            let names = [TEST_INTERACTIVE_ENV, TEST_SIZE_ENV, TEST_EVENTS_ENV, TEST_CAPTURE_ENV];
            let saved = names.into_iter().map(|name| (name, env::var_os(name))).collect();

            for name in names {
                unsafe { env::remove_var(name) };
            }

            Self { _guard: guard, saved }
        }

        fn set(self, name: &'static str, value: impl AsRef<OsStr>) -> Self {
            unsafe { env::set_var(name, value.as_ref()) };
            self
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
    fn block_reason_messages_match_terminal_contract() {
        assert_eq!(TerminalBlockReason::NonInteractiveTerminal.message(), "");
        assert!(TerminalBlockReason::LayoutTooSmall.message().contains("--non-interactive"));
    }

    #[test]
    fn readiness_supported_records_terminal_size() {
        let readiness = TerminalReadiness::supported((120, 40), false);

        assert!(readiness.interactive());
        assert_eq!(readiness.size(), Some((120, 40)));
        assert_eq!(readiness.block_reason(), None);
        assert!(!readiness.scripted());
    }

    #[test]
    fn readiness_blocks_layouts_that_do_not_fit() {
        let readiness = TerminalReadiness::blocked_layout_too_small((40, 12), true);

        assert!(readiness.interactive());
        assert_eq!(readiness.size(), Some((40, 12)));
        assert_eq!(readiness.block_reason(), Some(TerminalBlockReason::LayoutTooSmall));
        assert!(readiness.scripted());
    }

    #[test]
    fn readiness_distinguishes_non_interactive_terminals() {
        let readiness = TerminalReadiness::blocked_non_interactive(false);

        assert!(!readiness.interactive());
        assert_eq!(readiness.size(), None);
        assert_eq!(readiness.block_reason(), Some(TerminalBlockReason::NonInteractiveTerminal));
        assert!(!readiness.scripted());
    }

    #[test]
    fn scripted_terminal_records_restore_marker_on_drop() {
        let workspace = tempdir().expect("tempdir");
        let capture_path = workspace.path().join("guided-terminal.log");

        {
            let _terminal = GuidedTerminal::scripted_for_tests(Some(capture_path.clone()));
        }

        let capture = std::fs::read_to_string(&capture_path).expect("capture log");
        assert!(capture.contains(TERMINAL_RESTORED_MARKER));
    }

    #[test]
    fn detect_terminal_readiness_uses_default_test_terminal_state() {
        let _env = TestEnv::new();

        let readiness = detect_terminal_readiness((72, 18)).expect("default readiness");

        assert!(readiness.interactive());
        assert_eq!(readiness.size(), Some((120, 40)));
        assert_eq!(readiness.block_reason(), None);
        assert!(!readiness.scripted());
    }

    #[test]
    fn detect_terminal_readiness_respects_non_interactive_override() {
        let _env = TestEnv::new().set(TEST_INTERACTIVE_ENV, "0");

        let readiness = detect_terminal_readiness((72, 18)).expect("blocked readiness");

        assert!(!readiness.interactive());
        assert_eq!(readiness.block_reason(), Some(TerminalBlockReason::NonInteractiveTerminal));
        assert!(readiness.scripted());
    }

    #[test]
    fn detect_terminal_readiness_respects_small_layout_override() {
        let _env = TestEnv::new().set(TEST_INTERACTIVE_ENV, "1").set(TEST_SIZE_ENV, "40x12");

        let readiness = detect_terminal_readiness((72, 18)).expect("layout readiness");

        assert!(readiness.interactive());
        assert_eq!(readiness.size(), Some((40, 12)));
        assert_eq!(readiness.block_reason(), Some(TerminalBlockReason::LayoutTooSmall));
        assert!(readiness.scripted());
    }

    #[test]
    fn guided_terminal_non_scripted_path_can_draw_read_and_restore() {
        let _env = TestEnv::new();
        let session = crate::tui::init::GuidedInitSession::new(None);
        let mut terminal = GuidedTerminal::enter().expect("live terminal fallback");

        terminal.draw(&session).expect("draw live frame");
        assert_eq!(terminal.next_event().expect("read test event"), GuidedInitEvent::Enter);
        terminal.restore().expect("restore once");
        terminal.restore().expect("restore idempotent");
    }

    #[test]
    fn scripted_terminal_reports_exhausted_event_scripts() {
        let mut terminal = GuidedTerminal::scripted_for_tests(None);

        let error = terminal.next_event().expect_err("script exhaustion should error");

        assert!(error.to_string().contains("event script ended before the flow completed"));
    }

    #[test]
    fn restore_surfaces_capture_errors() {
        let workspace = tempdir().expect("tempdir");
        let mut terminal = GuidedTerminal::scripted_for_tests(Some(workspace.path().to_path_buf()));

        let error = terminal.restore().expect_err("directory capture path should fail");

        assert_eq!(error.kind(), io::ErrorKind::IsADirectory);
    }

    #[test]
    fn parses_terminal_bool_overrides() {
        assert!(parse_bool_value("TEST", "1").expect("bool true"));
        assert!(!parse_bool_value("TEST", "false").expect("bool false"));
    }

    #[test]
    fn parse_bool_override_handles_missing_invalid_and_non_utf8_values() {
        {
            let _env = TestEnv::new();
            assert_eq!(parse_bool_override("MISSING_BOOL_OVERRIDE").expect("missing env"), None);
        }

        {
            let _env = TestEnv::new().set("INVALID_BOOL_OVERRIDE", "maybe");
            let error = parse_bool_override("INVALID_BOOL_OVERRIDE").expect_err("invalid bool");
            assert!(error.to_string().contains("INVALID_BOOL_OVERRIDE must be one of"));
        }

        {
            let _env = TestEnv::new()
                .set("NON_UTF8_BOOL_OVERRIDE", OsString::from_vec(vec![0x66, 0x6f, 0x80]));
            let error = parse_bool_override("NON_UTF8_BOOL_OVERRIDE").expect_err("non-utf8 bool");
            assert!(error.to_string().contains("UTF-8 boolean override"));
        }
    }

    #[test]
    fn parse_size_override_rejects_invalid_inputs() {
        assert_eq!(parse_size_override(None).expect("missing size"), None);

        let error = parse_size_override(Some(OsStr::new("120"))).expect_err("missing separator");
        assert!(error.to_string().contains("format <width>x<height>"));

        let error = parse_size_override(Some(OsStr::new("widex40"))).expect_err("bad width");
        assert!(error.to_string().contains("numeric dimensions"));

        let non_utf8 = OsString::from_vec(vec![0x66, 0x6f, 0x80]);
        let error = parse_size_override(Some(non_utf8.as_os_str())).expect_err("non-utf8 size");
        assert!(error.to_string().contains("UTF-8 size override"));
    }

    #[test]
    fn parses_size_override_and_scripted_events() {
        assert_eq!(
            parse_size_override(Some(OsStr::new("120x40"))).expect("size override"),
            Some((120, 40))
        );
        assert_eq!(
            parse_scripted_events(Some(OsStr::new("down,enter,ctrl-c")))
                .expect("scripted events")
                .len(),
            3
        );
    }

    #[test]
    fn parse_scripted_events_handles_missing_non_utf8_and_invalid_tokens() {
        assert!(parse_scripted_events(None).expect("missing events").is_empty());

        let filtered = parse_scripted_events(Some(OsStr::new(" down , , enter , ctrl-c ")))
            .expect("filtered scripted events");
        assert_eq!(filtered.len(), 3);

        let non_utf8 = OsString::from_vec(vec![0x65, 0x6e, 0x80]);
        let error = parse_scripted_events(Some(non_utf8.as_os_str())).expect_err("non-utf8 events");
        assert!(error.to_string().contains("UTF-8 tokens"));

        let error =
            parse_scripted_events(Some(OsStr::new("left"))).expect_err("invalid event token");
        assert!(error.to_string().contains("unsupported guided init test event token"));
    }

    #[test]
    fn event_mapping_covers_supported_and_ignored_inputs() {
        assert_eq!(
            map_live_event(Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE))),
            Some(GuidedInitEvent::Up)
        );
        assert_eq!(
            map_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
            Some(GuidedInitEvent::Down)
        );
        assert_eq!(
            map_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            Some(GuidedInitEvent::Enter)
        );
        assert_eq!(
            map_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
            Some(GuidedInitEvent::Esc)
        );
        assert_eq!(
            map_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(GuidedInitEvent::CtrlC)
        );
        assert_eq!(map_key_event(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE)), None);
        assert_eq!(
            map_key_event(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Release,
                state: crossterm::event::KeyEventState::NONE,
            }),
            None
        );
        assert_eq!(map_live_event(Event::Resize(80, 20)), None);
    }

    #[test]
    fn parse_bool_value_rejects_unknown_text() {
        let error = parse_bool_value("BOOL_ENV", "yes").expect_err("invalid bool text");

        assert!(error.to_string().contains("BOOL_ENV must be one of"));
    }

    #[test]
    fn interruption_message_remains_stable() {
        assert!(INTERRUPTED_MESSAGE.contains("no .canon changes were made"));
    }
}
