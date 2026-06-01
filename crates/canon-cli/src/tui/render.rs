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
const SELECTING_PROMPT: &str = "Select an assistant for this workspace.";
const CONFIRMING_PROMPT: &str = "Confirm the assistant choice before initialization.";
const SELECTING_INSTRUCTIONS: &str = "Up/Down choose, Enter confirms, Ctrl+C exits";
const CONFIRMING_INSTRUCTIONS: &str = "Enter initializes, Up/Down changes selection, Ctrl+C exits";
const HIGHLIGHT_PREFIX: &str = "> ";
const IDLE_PREFIX: &str = "  ";
const MIN_TERMINAL_WIDTH: u16 = 50;
const MIN_TERMINAL_HEIGHT: u16 = 18;
const HEADER_HEIGHT: u16 = 10;
const FOOTER_HEIGHT: u16 = 4;
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

pub(crate) fn required_terminal_size() -> (u16, u16) {
    (MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT)
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
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Min(6),
            Constraint::Length(FOOTER_HEIGHT),
        ])
        .split(frame.area());

    frame.render_widget(Clear, frame.area());
    frame.render_widget(header_widget(), areas[0]);
    frame.render_widget(body_widget(&view), areas[1]);
    frame.render_widget(footer_widget(view.instructions), areas[2]);
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

fn header_widget() -> Paragraph<'static> {
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

fn footer_widget(instructions: &'static str) -> Paragraph<'static> {
    Paragraph::new(Line::from(Span::styled(instructions, Style::default().fg(TEXT_LIGHT))))
        .alignment(Alignment::Center)
        .block(themed_block(FOOTER_TITLE))
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
        assert_eq!(required_terminal_size(), (50, 18));
    }

    #[test]
    fn draw_renders_frame_for_selected_session() {
        let session = GuidedInitSession::new(Some(AiTool::Copilot));
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).expect("test terminal");

        terminal.draw(|frame| draw(frame, &session)).expect("render guided init frame");
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
