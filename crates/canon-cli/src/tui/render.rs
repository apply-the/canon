//! Rendering helpers for the guided init terminal UI.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::init::{AssistantChoice, GuidedInitSession, GuidedInitStage, ordered_choices};

const BRAND_RUNTIME_TITLE: &str = "AI-Assisted Engineering Governance Runtime";
const ASCII_LOGO_LINES: [&str; 6] = [
    "██████╗ █████╗ ███╗   ██╗ ██████╗ ███╗   ██╗",
    "██╔════╝██╔══██╗████╗  ██║██╔═══██╗████╗  ██║",
    "██║     ███████║██╔██╗ ██║██║   ██║██╔██╗ ██║",
    "██║     ██╔══██║██║╚██╗██║██║   ██║██║╚██╗██║",
    "╚██████╗██║  ██║██║ ╚████║╚██████╔╝██║ ╚████║",
    " ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝",
];
const BODY_TITLE: &str = "Assistant Selection";
const FOOTER_TITLE: &str = "Keyboard";
const COMPACT_RUNTIME_TITLE: &str = "Canon Guided Setup";
const SELECTING_PROMPT: &str = "Select an assistant for this workspace.";
const CONFIRMING_PROMPT: &str = "Confirm the assistant choice before initialization.";
const SELECTING_INSTRUCTIONS: &str = "Arrows move, Enter confirms, Ctrl+C exits";
const CONFIRMING_INSTRUCTIONS: &str = "Enter starts, Arrows change, Ctrl+C exits";
const HIGHLIGHT_PREFIX: &str = "> ";
const IDLE_PREFIX: &str = "  ";
const FULL_MIN_TERMINAL_WIDTH: u16 = 50;
const FULL_MIN_TERMINAL_HEIGHT: u16 = 18;
const COMPACT_MIN_TERMINAL_WIDTH: u16 = 42;
const COMPACT_MIN_TERMINAL_HEIGHT: u16 = 11;
const FULL_HEADER_HEIGHT: u16 = 10;
const FULL_FOOTER_HEIGHT: u16 = 4;
const COMPACT_HEADER_HEIGHT: u16 = 2;
const COMPACT_BODY_MIN_HEIGHT: u16 = 8;
const COMPACT_FOOTER_HEIGHT: u16 = 1;
const BRAND_PRIMARY: Color = Color::Rgb(141, 61, 255);
const BRAND_ACCENT: Color = Color::Rgb(182, 76, 255);
const TEXT_LIGHT: Color = Color::Rgb(248, 245, 255);
const LOGO_GRADIENT: [Color; 6] = [
    Color::Rgb(102, 36, 200),
    Color::Rgb(126, 34, 206),
    Color::Rgb(141, 61, 255),
    Color::Rgb(182, 76, 255),
    Color::Rgb(208, 92, 255),
    Color::Rgb(242, 109, 255),
];
const LOGO_FINAL_COLOR: Color = Color::Rgb(242, 109, 255);

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuidedInitView {
    prompt: &'static str,
    status: String,
    instructions: &'static str,
    options: Vec<GuidedOptionView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuidedOptionView {
    label: &'static str,
    highlighted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutMode {
    Full,
    Compact,
}

pub(crate) fn required_terminal_size() -> (u16, u16) {
    (COMPACT_MIN_TERMINAL_WIDTH, COMPACT_MIN_TERMINAL_HEIGHT)
}

pub(crate) fn snapshot(session: &GuidedInitSession) -> String {
    let view = build_view(session);
    let mut lines = Vec::new();
    lines.extend(ASCII_LOGO_LINES.iter().map(|line| line.to_string()));
    lines.push(runtime_title_text().to_string());
    lines.push(runtime_version_text());
    lines.push(String::new());
    lines.push(view.prompt.to_string());

    lines.extend(view.options.into_iter().map(snapshot_option_line));
    lines.push(String::new());
    lines.push(view.status);
    lines.push(view.instructions.to_string());
    lines.join("\n")
}

pub(crate) fn draw(frame: &mut Frame, session: &GuidedInitSession) {
    let view = build_view(session);

    frame.render_widget(Clear, frame.area());

    match layout_mode((frame.area().width, frame.area().height)) {
        LayoutMode::Full => draw_full_layout(frame, &view),
        LayoutMode::Compact => draw_compact_layout(frame, &view),
    }
}

fn layout_mode(size: (u16, u16)) -> LayoutMode {
    if size.0 >= FULL_MIN_TERMINAL_WIDTH && size.1 >= FULL_MIN_TERMINAL_HEIGHT {
        LayoutMode::Full
    } else {
        LayoutMode::Compact
    }
}

fn draw_full_layout(frame: &mut Frame, view: &GuidedInitView) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(FULL_HEADER_HEIGHT),
            Constraint::Min(6),
            Constraint::Length(FULL_FOOTER_HEIGHT),
        ])
        .split(frame.area());

    frame.render_widget(full_header_widget(), areas[0]);
    frame.render_widget(body_widget(view), areas[1]);
    frame.render_widget(footer_widget(view.instructions), areas[2]);
}

fn draw_compact_layout(frame: &mut Frame, view: &GuidedInitView) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(COMPACT_HEADER_HEIGHT),
            Constraint::Min(COMPACT_BODY_MIN_HEIGHT),
            Constraint::Length(COMPACT_FOOTER_HEIGHT),
        ])
        .split(frame.area());

    frame.render_widget(compact_header_widget(), areas[0]);
    frame.render_widget(compact_body_widget(view), areas[1]);
    frame.render_widget(compact_footer_widget(view.instructions), areas[2]);
}

fn build_view(session: &GuidedInitSession) -> GuidedInitView {
    let choice = selected_choice(session);
    let prompt = match session.stage() {
        GuidedInitStage::Selecting => SELECTING_PROMPT,
        GuidedInitStage::Confirming => CONFIRMING_PROMPT,
    };
    let status = match session.stage() {
        GuidedInitStage::Selecting => format!("Selected: {}", choice.label()),
        GuidedInitStage::Confirming => confirm_status(choice),
    };
    let instructions = match session.stage() {
        GuidedInitStage::Selecting => SELECTING_INSTRUCTIONS,
        GuidedInitStage::Confirming => CONFIRMING_INSTRUCTIONS,
    };
    let options = option_views(session);

    GuidedInitView { prompt, status, instructions, options }
}

fn option_views(session: &GuidedInitSession) -> Vec<GuidedOptionView> {
    let selected = session.highlighted_choice();
    ordered_choices()
        .iter()
        .copied()
        .map(|choice| GuidedOptionView { label: choice.label(), highlighted: choice == selected })
        .collect()
}

fn selected_choice(session: &GuidedInitSession) -> AssistantChoice {
    match session.confirmed_choice() {
        Some(choice) => choice,
        None => session.highlighted_choice(),
    }
}

fn confirm_status(choice: AssistantChoice) -> String {
    match choice {
        AssistantChoice::None => "Initialize without an assistant.".to_string(),
        other => format!("Initialize with {}.", other.label()),
    }
}

fn snapshot_option_line(option: GuidedOptionView) -> String {
    let prefix = if option.highlighted { HIGHLIGHT_PREFIX } else { IDLE_PREFIX };
    format!("{prefix}{}", option.label)
}

fn full_header_widget() -> Paragraph<'static> {
    let mut lines = Vec::new();
    lines.extend(ASCII_LOGO_LINES.iter().enumerate().map(|(index, line)| {
        let color = LOGO_GRADIENT[index];
        Line::from(Span::styled(*line, Style::default().fg(color).add_modifier(Modifier::BOLD)))
    }));
    lines.push(Line::default());
    lines.push(Line::from(Span::styled(
        runtime_title_text(),
        Style::default().fg(LOGO_FINAL_COLOR).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        runtime_version_text(),
        Style::default().fg(LOGO_FINAL_COLOR).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::default());

    Paragraph::new(lines).alignment(Alignment::Center)
}

fn compact_header_widget() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(Span::styled(
            COMPACT_RUNTIME_TITLE,
            Style::default().fg(LOGO_FINAL_COLOR).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(runtime_version_text(), Style::default().fg(TEXT_LIGHT))),
    ])
    .alignment(Alignment::Center)
}

fn runtime_title_text() -> &'static str {
    BRAND_RUNTIME_TITLE
}

fn runtime_version_text() -> String {
    format!("Version {version}", version = env!("CARGO_PKG_VERSION"))
}

fn body_widget(view: &GuidedInitView) -> Paragraph<'static> {
    let mut lines = vec![Line::from(view.prompt.to_string())];
    lines.push(Line::default());
    for option in &view.options {
        lines.push(option_line(option));
    }
    lines.push(Line::default());
    lines.push(Line::from(Span::styled(
        view.status.clone(),
        Style::default().fg(BRAND_ACCENT).add_modifier(Modifier::BOLD),
    )));

    Paragraph::new(lines).block(themed_block(BODY_TITLE))
}

fn compact_body_widget(view: &GuidedInitView) -> Paragraph<'static> {
    let mut lines = vec![Line::from(Span::styled(
        view.prompt,
        Style::default().fg(TEXT_LIGHT).add_modifier(Modifier::BOLD),
    ))];
    for option in &view.options {
        lines.push(option_line(option));
    }
    lines.push(Line::from(Span::styled(
        view.status.clone(),
        Style::default().fg(BRAND_ACCENT).add_modifier(Modifier::BOLD),
    )));

    Paragraph::new(lines)
}

fn footer_widget(instructions: &'static str) -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(instructions, Style::default().fg(TEXT_LIGHT))))
        .alignment(Alignment::Center)
        .block(themed_block(FOOTER_TITLE))
}

fn compact_footer_widget(instructions: &'static str) -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(instructions, Style::default().fg(TEXT_LIGHT))))
        .alignment(Alignment::Center)
}

fn themed_block(title: &'static str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BRAND_PRIMARY))
        .title(Span::styled(title, Style::default().fg(BRAND_ACCENT).add_modifier(Modifier::BOLD)))
}

fn option_line(option: &GuidedOptionView) -> Line<'static> {
    let prefix = if option.highlighted { HIGHLIGHT_PREFIX } else { IDLE_PREFIX };
    let style = if option.highlighted {
        Style::default().fg(BRAND_ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT_LIGHT)
    };

    Line::from(vec![Span::styled(prefix, style), Span::styled(option.label, style)])
}

#[cfg(test)]
mod tests {
    use canon_engine::AiTool;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::{
        CONFIRMING_INSTRUCTIONS, SELECTING_INSTRUCTIONS, draw, required_terminal_size,
        runtime_title_text, runtime_version_text, snapshot,
    };
    use crate::tui::init::{GuidedInitEvent, GuidedInitSession};

    #[test]
    fn required_terminal_size_matches_layout_contract() {
        assert_eq!(required_terminal_size(), (42, 11));
    }

    #[test]
    fn draw_renders_frame_for_selected_session() {
        let session = GuidedInitSession::new(Some(AiTool::Copilot));
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal.draw(|frame| draw(frame, &session)).expect("render guided init frame");
    }

    #[test]
    fn draw_renders_compact_frame_for_selected_session() {
        let session = GuidedInitSession::new(Some(AiTool::Copilot));
        let backend = TestBackend::new(42, 11);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal.draw(|frame| draw(frame, &session)).expect("render compact guided init frame");
    }

    #[test]
    fn snapshot_includes_branding_and_selecting_instructions() {
        let session = GuidedInitSession::new(Some(AiTool::Copilot));
        let screen = snapshot(&session);

        assert!(screen.contains(runtime_title_text()));
        assert!(screen.contains(&runtime_version_text()));
        assert!(screen.contains("> Copilot"));
        assert!(screen.contains(SELECTING_INSTRUCTIONS));
    }

    #[test]
    fn snapshot_switches_to_confirmation_prompt_after_enter() {
        let mut session = GuidedInitSession::new(None);
        let _ = session.apply_event(GuidedInitEvent::Enter);

        let screen = snapshot(&session);

        assert!(screen.contains("Initialize without an assistant."));
        assert!(screen.contains(CONFIRMING_INSTRUCTIONS));
    }

    #[test]
    fn snapshot_confirmation_with_selected_assistant_uses_confirmed_status() {
        let mut session = GuidedInitSession::new(Some(AiTool::Claude));
        let _ = session.apply_event(GuidedInitEvent::Enter);

        let screen = snapshot(&session);

        assert!(screen.contains("Initialize with Claude."));
        assert!(screen.contains(CONFIRMING_INSTRUCTIONS));
    }
}
