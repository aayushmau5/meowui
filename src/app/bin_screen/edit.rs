use super::{Bin, BinActions, CurrentScreen, File};
use crate::tui::{
    expire_at_input_widget::ExpireAtWidget, input_widget::InputWidget,
    multiline_input_widget::MultilineInput,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use serde_json::json;

pub struct EditScreen {
    bin: Bin,
    title_input: InputWidget<'static>,
    content_input: MultilineInput<'static>,
    expire_at: ExpireAtWidget<'static>,
    files: Vec<EditFiles>,
    selected_file: ListState,
    focused_element: EditElements,
}

struct EditFiles {
    file: File,
    removed: bool,
}

enum EditElements {
    Title,
    Content,
    Files,
    Expire,
}

impl EditScreen {
    pub fn new(bin: Bin) -> Self {
        let files: Vec<EditFiles> = bin
            .files
            .iter()
            .map(|file| EditFiles {
                file: file.clone(),
                removed: false,
            })
            .collect();

        Self {
            bin: bin.clone(),
            title_input: InputWidget::new(
                bin.title,
                Style::default(),
                Style::default().fg(Color::Black).bg(Color::White),
            ),
            content_input: MultilineInput::new(bin.content),
            files,
            selected_file: ListState::default().with_selected(None),
            focused_element: EditElements::Title,
            expire_at: ExpireAtWidget::new(
                Style::default(),
                Style::default().fg(Color::Black).bg(Color::White),
            ),
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        self.edit_widget(chunks[0], f);
        self.help_widget(chunks[1], f);
    }

    fn edit_widget(&mut self, area: Rect, f: &mut Frame) {
        let chunks = if self.files.is_empty() {
            Layout::default()
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(area)
        } else {
            Layout::default()
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(area)
        };

        let title_style = if matches!(self.focused_element, EditElements::Title) {
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

        let content_style = if matches!(self.focused_element, EditElements::Content) {
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

        if !self.files.is_empty() {
            let files_style = if matches!(self.focused_element, EditElements::Files) {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Gray)
            };
            let files_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(files_style)
                .style(files_style)
                .title("Files");

            let files = self.files.iter().map(|file| {
                let mut file_lines = vec![
                    Line::from(file.file.name.as_str()),
                    Line::from(file.file.type_name.as_str()),
                ];

                if file.removed {
                    file_lines.push(
                        Line::from("Removed").style(Style::default().italic().fg(Color::Red)),
                    );
                    file_lines.push(Line::from("\n"));
                } else {
                    file_lines.push(Line::from("\n"));
                }

                let formatted = Text::from(file_lines);
                let style = if file.removed {
                    Style::default().fg(Color::Gray)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                ListItem::new(formatted).style(style)
            });
            let list = List::new(files)
                .style(Style::new().green())
                .highlight_symbol("-> ")
                .repeat_highlight_symbol(false)
                .block(files_block);
            f.render_stateful_widget(list, chunks[2], &mut self.selected_file);
        }

        let chunk = if !self.files.is_empty() {
            chunks[3]
        } else {
            chunks[2]
        };

        let expire_at_style = if matches!(self.focused_element, EditElements::Expire) {
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
        f.render_widget(&expire_at_widget, chunk);
    }

    fn help_widget(&self, area: Rect, f: &mut Frame) {
        let help_message = if matches!(self.focused_element, EditElements::Files) {
            "(Esc) to cancel edit / (Ctrl-S) to save / (r) to toggle file remove"
        } else {
            "(Esc) to cancel edit / (Ctrl-S) to save"
        };

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<BinActions> {
        match e.code {
            KeyCode::Esc => Some(BinActions::ChangeScreen(
                CurrentScreen::Show,
                Some(self.bin.clone()),
            )),
            KeyCode::Char('s') if e.modifiers == KeyModifiers::CONTROL => {
                let edited_title = self.title_input.content();
                let edited_content = self.content_input.content();
                let files: Vec<File> = self
                    .files
                    .iter()
                    .filter(|f| !f.removed)
                    .map(|f| f.file.clone())
                    .collect();
                let expire_at = self.expire_at.time();

                Some(BinActions::SendEvent(Some(json!({
                   "action": "edit",
                   "data": {
                        "id": self.bin.id,
                        "title": edited_title,
                        "content": edited_content,
                        "files": files,
                        "expire": {"time": expire_at.time, "unit": expire_at.unit.to_string()}
                   }
                }))))
            }
            KeyCode::Char('j') if matches!(self.focused_element, EditElements::Files) => {
                self.select_edit_next_file();
                None
            }
            KeyCode::Char('k') if matches!(self.focused_element, EditElements::Files) => {
                self.select_edit_previous_file();
                None
            }
            KeyCode::Char('r') if matches!(self.focused_element, EditElements::Files) => {
                if let Some(selected_index) = self.selected_file.selected() {
                    self.files[selected_index].removed = !self.files[selected_index].removed;
                }
                None
            }
            KeyCode::Tab => {
                match self.focused_element {
                    EditElements::Title => self.focused_element = EditElements::Content,
                    EditElements::Content => {
                        if self.files.is_empty() {
                            self.focused_element = EditElements::Expire
                        } else {
                            self.focused_element = EditElements::Files
                        }
                    }
                    EditElements::Files => self.focused_element = EditElements::Expire,
                    EditElements::Expire => self.focused_element = EditElements::Title,
                }
                None
            }
            _ => {
                match self.focused_element {
                    EditElements::Title => self.title_input.handle_key(e),
                    EditElements::Content => self.content_input.handle_key(e),
                    EditElements::Expire => self.expire_at.handle_key(e),
                    _ => {}
                }
                None
            }
        }
    }

    fn select_edit_previous_file(&mut self) {
        self.selected_file.select_previous();
    }

    fn select_edit_next_file(&mut self) {
        self.selected_file.select_next();
    }
}
