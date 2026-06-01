//! Guided init session state and event handling.

use canon_engine::AiTool;

const NONE_CHOICE_INDEX: usize = 0;
const CODEX_CHOICE_INDEX: usize = 1;
const COPILOT_CHOICE_INDEX: usize = 2;
const CLAUDE_CHOICE_INDEX: usize = 3;
const CURSOR_CHOICE_INDEX: usize = 4;
const ANTIGRAVITY_CHOICE_INDEX: usize = 5;
const FIRST_CHOICE_INDEX: usize = 0;
const ORDERED_CHOICES: [AssistantChoice; 6] = [
    AssistantChoice::None,
    AssistantChoice::Codex,
    AssistantChoice::Copilot,
    AssistantChoice::Claude,
    AssistantChoice::Cursor,
    AssistantChoice::Antigravity,
];
const LAST_CHOICE_INDEX: usize = ORDERED_CHOICES.len() - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssistantChoice {
    None,
    Codex,
    Copilot,
    Claude,
    Cursor,
    Antigravity,
}

impl AssistantChoice {
    pub(crate) fn from_ai_tool(ai_tool: Option<AiTool>) -> Self {
        match ai_tool {
            Some(AiTool::Codex) => Self::Codex,
            Some(AiTool::Copilot) => Self::Copilot,
            Some(AiTool::Claude) => Self::Claude,
            Some(AiTool::Cursor) => Self::Cursor,
            Some(AiTool::Antigravity) => Self::Antigravity,
            None => Self::None,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::None => "No assistant",
            Self::Codex => "Codex",
            Self::Copilot => "Copilot",
            Self::Claude => "Claude",
            Self::Cursor => "Cursor",
            Self::Antigravity => "Antigravity",
        }
    }

    pub(crate) fn to_ai_tool(self) -> Option<AiTool> {
        match self {
            Self::None => None,
            Self::Codex => Some(AiTool::Codex),
            Self::Copilot => Some(AiTool::Copilot),
            Self::Claude => Some(AiTool::Claude),
            Self::Cursor => Some(AiTool::Cursor),
            Self::Antigravity => Some(AiTool::Antigravity),
        }
    }

    fn index(self) -> usize {
        match self {
            Self::None => NONE_CHOICE_INDEX,
            Self::Codex => CODEX_CHOICE_INDEX,
            Self::Copilot => COPILOT_CHOICE_INDEX,
            Self::Claude => CLAUDE_CHOICE_INDEX,
            Self::Cursor => CURSOR_CHOICE_INDEX,
            Self::Antigravity => ANTIGRAVITY_CHOICE_INDEX,
        }
    }

    fn from_index(index: usize) -> Self {
        ORDERED_CHOICES.get(index).copied().unwrap_or(AssistantChoice::None)
    }
}

pub(crate) fn ordered_choices() -> &'static [AssistantChoice] {
    &ORDERED_CHOICES
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GuidedInitStage {
    Selecting,
    Confirming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GuidedInitEvent {
    Up,
    Down,
    Enter,
    Esc,
    CtrlC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GuidedInitAction {
    Continue,
    Initialize(Option<AiTool>),
    Interrupted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GuidedInitSession {
    highlighted_choice: AssistantChoice,
    confirmed_choice: Option<AssistantChoice>,
    stage: GuidedInitStage,
}

impl GuidedInitSession {
    pub(crate) fn new(requested_ai: Option<AiTool>) -> Self {
        Self {
            highlighted_choice: AssistantChoice::from_ai_tool(requested_ai),
            confirmed_choice: None,
            stage: GuidedInitStage::Selecting,
        }
    }

    pub(crate) fn highlighted_choice(self) -> AssistantChoice {
        self.highlighted_choice
    }

    pub(crate) fn confirmed_choice(self) -> Option<AssistantChoice> {
        self.confirmed_choice
    }

    pub(crate) fn stage(self) -> GuidedInitStage {
        self.stage
    }

    pub(crate) fn apply_event(&mut self, event: GuidedInitEvent) -> GuidedInitAction {
        match event {
            GuidedInitEvent::Up => {
                self.move_highlight(-1);
                GuidedInitAction::Continue
            }
            GuidedInitEvent::Down => {
                self.move_highlight(1);
                GuidedInitAction::Continue
            }
            GuidedInitEvent::Enter => self.confirm_or_initialize(),
            GuidedInitEvent::Esc => GuidedInitAction::Continue,
            GuidedInitEvent::CtrlC => GuidedInitAction::Interrupted,
        }
    }

    fn confirm_or_initialize(&mut self) -> GuidedInitAction {
        match self.stage {
            GuidedInitStage::Selecting => {
                self.confirmed_choice = Some(self.highlighted_choice);
                self.stage = GuidedInitStage::Confirming;
                GuidedInitAction::Continue
            }
            GuidedInitStage::Confirming => {
                GuidedInitAction::Initialize(self.selected_choice().to_ai_tool())
            }
        }
    }

    fn move_highlight(&mut self, delta: isize) {
        let current_index = self.highlighted_choice.index();
        let next_index = clamp_choice_index(current_index, delta);
        self.highlighted_choice = AssistantChoice::from_index(next_index);

        if self.stage == GuidedInitStage::Confirming {
            self.confirmed_choice = None;
            self.stage = GuidedInitStage::Selecting;
        }
    }

    fn selected_choice(self) -> AssistantChoice {
        match self.confirmed_choice {
            Some(choice) => choice,
            None => self.highlighted_choice,
        }
    }
}

fn clamp_choice_index(current_index: usize, delta: isize) -> usize {
    let next_index = current_index.saturating_add_signed(delta);
    next_index.clamp(FIRST_CHOICE_INDEX, LAST_CHOICE_INDEX)
}

#[cfg(test)]
mod tests {
    use canon_engine::AiTool;

    use super::{
        AssistantChoice, GuidedInitAction, GuidedInitEvent, GuidedInitSession, GuidedInitStage,
    };

    #[test]
    fn preselects_requested_assistant() {
        let session = GuidedInitSession::new(Some(AiTool::Copilot));

        assert_eq!(session.highlighted_choice(), AssistantChoice::Copilot);
        assert_eq!(session.stage(), GuidedInitStage::Selecting);
        assert_eq!(session.confirmed_choice(), None);
    }

    #[test]
    fn navigation_moves_highlight_and_leaves_confirmation_mode() {
        let mut session = GuidedInitSession::new(Some(AiTool::Copilot));

        assert_eq!(session.apply_event(GuidedInitEvent::Enter), GuidedInitAction::Continue);
        assert_eq!(session.stage(), GuidedInitStage::Confirming);
        assert_eq!(session.confirmed_choice(), Some(AssistantChoice::Copilot));

        assert_eq!(session.apply_event(GuidedInitEvent::Down), GuidedInitAction::Continue);
        assert_eq!(session.highlighted_choice(), AssistantChoice::Claude);
        assert_eq!(session.stage(), GuidedInitStage::Selecting);
        assert_eq!(session.confirmed_choice(), None);
    }

    #[test]
    fn esc_is_ignored_in_selection_and_confirmation() {
        let mut session = GuidedInitSession::new(None);

        assert_eq!(session.apply_event(GuidedInitEvent::Esc), GuidedInitAction::Continue);
        assert_eq!(session.highlighted_choice(), AssistantChoice::None);
        assert_eq!(session.stage(), GuidedInitStage::Selecting);

        assert_eq!(session.apply_event(GuidedInitEvent::Enter), GuidedInitAction::Continue);
        assert_eq!(session.apply_event(GuidedInitEvent::Esc), GuidedInitAction::Continue);
        assert_eq!(session.highlighted_choice(), AssistantChoice::None);
        assert_eq!(session.stage(), GuidedInitStage::Confirming);
        assert_eq!(session.confirmed_choice(), Some(AssistantChoice::None));
    }

    #[test]
    fn enter_confirms_choice_then_starts_initialization() {
        let mut session = GuidedInitSession::new(Some(AiTool::Claude));

        assert_eq!(session.apply_event(GuidedInitEvent::Enter), GuidedInitAction::Continue);
        assert_eq!(session.stage(), GuidedInitStage::Confirming);
        assert_eq!(session.confirmed_choice(), Some(AssistantChoice::Claude));
        assert_eq!(
            session.apply_event(GuidedInitEvent::Enter),
            GuidedInitAction::Initialize(Some(AiTool::Claude))
        );
    }

    #[test]
    fn no_assistant_path_initializes_without_ai_tool() {
        let mut session = GuidedInitSession::new(None);

        assert_eq!(session.apply_event(GuidedInitEvent::Enter), GuidedInitAction::Continue);
        assert_eq!(session.apply_event(GuidedInitEvent::Enter), GuidedInitAction::Initialize(None));
    }

    #[test]
    fn ctrl_c_interrupts_before_initialization() {
        let mut session = GuidedInitSession::new(Some(AiTool::Codex));

        assert_eq!(session.apply_event(GuidedInitEvent::CtrlC), GuidedInitAction::Interrupted);
        assert_eq!(session.highlighted_choice(), AssistantChoice::Codex);
        assert_eq!(session.stage(), GuidedInitStage::Selecting);
    }

    #[test]
    fn up_key_clamps_at_first_choice_and_labels_remain_human_readable() {
        let mut session = GuidedInitSession::new(None);

        assert_eq!(session.highlighted_choice().label(), "No assistant");
        assert_eq!(session.apply_event(GuidedInitEvent::Up), GuidedInitAction::Continue);
        assert_eq!(session.highlighted_choice(), AssistantChoice::None);
        assert_eq!(AssistantChoice::Claude.label(), "Claude");
        assert_eq!(AssistantChoice::Cursor.label(), "Cursor");
        assert_eq!(AssistantChoice::Antigravity.label(), "Antigravity");
    }

    #[test]
    fn preselects_cursor_and_antigravity() {
        let cursor = GuidedInitSession::new(Some(AiTool::Cursor));
        let antigravity = GuidedInitSession::new(Some(AiTool::Antigravity));

        assert_eq!(cursor.highlighted_choice(), AssistantChoice::Cursor);
        assert_eq!(antigravity.highlighted_choice(), AssistantChoice::Antigravity);
    }
}
