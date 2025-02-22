use super::{BinActions, CurrentScreen};
use crate::tui::expire_at_input_widget::ExpireAtWidget;
use crate::tui::input_widget::InputWidget;
use crate::tui::multiline_input_widget::MultilineInput;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use serde_json::json;

pub struct NewScreen {
    title_input: InputWidget<'static>,
    content_input: MultilineInput<'static>,
    expire_at: ExpireAtWidget<'static>,
    focused_element: NewElements,
}

enum NewElements {
    Title,
    Content,
    Expire,
}

impl NewScreen {
    pub fn new() -> Self {
        Self {
            title_input: InputWidget::new(
                String::new(),
                Style::default(),
                Style::default().fg(Color::Black).bg(Color::White),
            ),
            content_input: MultilineInput::new(String::new()),
            expire_at: ExpireAtWidget::new(
                Style::default(),
                Style::default().fg(Color::Black).bg(Color::White),
            ),
            focused_element: NewElements::Title,
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        self.new_bin_widget(chunks[0], f);
        self.help_widget(chunks[1], f);
    }

    // Keyboard event handler

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<BinActions> {
        match e.code {
            KeyCode::Esc => Some(BinActions::ChangeScreen(CurrentScreen::Main, None)),
            KeyCode::Char('s') if e.modifiers == KeyModifiers::CONTROL => {
                let title = self.title_input.content();
                let content = self.content_input.content();
                let expire_at = self.expire_at.time();

                let event = Some(json!({
                    "action": "new",
                    "data": {
                        "title": title,
                        "content": content,
                        "expire": {"time": expire_at.time, "unit": expire_at.unit.to_string()}
                    }
                }));

                Some(BinActions::SendEvent(event))
            }
            KeyCode::Tab => {
                match self.focused_element {
                    NewElements::Title => {
                        self.focused_element = NewElements::Content;
                    }
                    NewElements::Content => {
                        self.focused_element = NewElements::Expire;
                    }
                    NewElements::Expire => {
                        self.focused_element = NewElements::Title;
                    }
                }
                None
            }
            _ => {
                match self.focused_element {
                    NewElements::Title => self.title_input.handle_key(e),
                    NewElements::Content => self.content_input.handle_key(e),
                    NewElements::Expire => self.expire_at.handle_key(e),
                }
                None
            }
        }
    }

    fn new_bin_widget(&mut self, area: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .constraints([
                Constraint::Length(4),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let title_style = if matches!(self.focused_element, NewElements::Title) {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::Gray)
        };

        let title_block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(title_style)
            .style(Style::new().green())
            .title("Title");
        let title_widget = self.title_input.clone().block(title_block);
        f.render_widget(&title_widget, chunks[0]);

        let content_style = if matches!(self.focused_element, NewElements::Content) {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::Gray)
        };
        let content_block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(content_style)
            .style(Style::new().green())
            .title("Content");
        let content_widget = self.content_input.clone().block(content_block);
        f.render_widget(&content_widget, chunks[1]);

        let expire_at_style = if matches!(self.focused_element, NewElements::Expire) {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::Gray)
        };
        let expire_at_block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(expire_at_style)
            .style(Style::new().green())
            .title("Expire in");
        let expire_at_widget = self.expire_at.clone().block(expire_at_block);
        f.render_widget(&expire_at_widget, chunks[2]);
    }

    fn help_widget(&self, area: Rect, f: &mut Frame) {
        let help_message = "(Esc) to cancel / (Ctrl-S) to save";

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }
}
