use super::{AppActions, ScreenType};
use crate::tui::input_widget::InputWidget;
use crate::{phoenix::event::PhoenixEvent, tui::multiline_input_widget::MultilineInput};
use chrono::{DateTime, Local};
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

const ENDPOINT: &str = "http://localhost:4000";

/// ## Events
///
/// ### Get all bins
/// screen -> socket: `{"name": "bin", "payload": {"action": "get-all"}}`
///
/// socket -> screen: `{"name": "bin", "payload": {"action": "get-all", "data": [...]}}`
///
/// ### Edit bin
/// screen -> socket: `{"name": "bin", "payload": {"action": "edit", "data": {"id": 1, ...}}}`
///
/// ### Delete bin
/// screen -> socket: `{"name": "bin", "payload": {"action": "delete", "id": 1}}`
pub struct BinScreen {
    pub screen_sender: Sender<PhoenixEvent>,
    bins: Vec<Bin>,
    list_state: ListState,
    current_screen: CurrentScreen,
}

enum CurrentScreen {
    Main,
    // New(Bin)
    Show(Selected),
    Edit(EditSelected),
}

struct Selected {
    selected: Bin,
    selected_file: ListState,
}

struct EditSelected {
    selected: Bin,
    selected_file: ListState,
    title_input: InputWidget<'static>,
    content_input: MultilineInput<'static>,
    focused_element: EditElements,
}

enum EditElements {
    Title,
    Content,
    Files,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Bin {
    id: u64,
    title: String,
    content: String,
    expire_at: DateTime<Local>,
    files: Vec<File>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct File {
    name: String,
    access_path: String,
    #[serde(rename = "type")]
    type_name: String,
}

impl BinScreen {
    pub fn new(screen_sender: Sender<PhoenixEvent>) -> Self {
        let bin_screen = Self {
            screen_sender,
            bins: Vec::new(),
            list_state: ListState::default(),
            current_screen: CurrentScreen::Main,
        };
        bin_screen.push_event(Some(json!({"action": "get-all"})));
        bin_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        match self.current_screen {
            CurrentScreen::Main => self.menu_items_widget(chunks[0], f),
            CurrentScreen::Show(_) => self.bin_widget(chunks[0], f),
            CurrentScreen::Edit(_) => self.edit_bin_widget(chunks[0], f),
        }

        self.help_widget(chunks[1], f);
    }

    fn menu_items_widget(&mut self, area: Rect, f: &mut Frame) {
        let items = self.bins.iter().map(|item| {
            let mut text = Text::default();
            let formatted_date_time = item.expire_at.format("%d/%m/%Y %I:%M %p").to_string();
            text.extend([
                Span::raw(&item.title),
                Span::raw(format!("Expire at: {formatted_date_time}")).blue(),
                Span::raw(""),
            ]);
            ListItem::new(text)
        });

        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title("Bin");

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(Color::Green))
            .style(Style::new().green())
            .highlight_symbol("-> ")
            .repeat_highlight_symbol(false);

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn bin_widget(&mut self, area: Rect, f: &mut Frame) {
        if let CurrentScreen::Show(selected) = &self.current_screen {
            if !selected.selected.files.is_empty() {
                return self.bin_widget_with_files(area, f);
            }

            let layout_chunks = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(1)])
                .split(area);

            let top_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::new().green())
                .title(format!("{} - Bin", selected.selected.title));

            let formatted_date_time = selected
                .selected
                .expire_at
                .format("%d/%m/%Y %I:%M %p")
                .to_string();
            let expire_text = Text::style(
                format!("Expire at: {}\n\n", formatted_date_time).into(),
                Style::default().fg(Color::Blue),
            );
            let expire_paragraph = Paragraph::new(expire_text).block(top_block);
            f.render_widget(expire_paragraph, layout_chunks[0]);

            let content_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::new().green());
            let content_text = Text::styled(
                format!("{}", selected.selected.content),
                Style::default().fg(Color::Magenta),
            );
            let content_paragraph = Paragraph::new(content_text).block(content_block);
            f.render_widget(content_paragraph, layout_chunks[1]);
        }
    }

    fn bin_widget_with_files(&mut self, area: Rect, f: &mut Frame) {
        let layout_chunks = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Min(1),
            ])
            .split(area);

        if let CurrentScreen::Show(selected) = &mut self.current_screen {
            let top_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::new().green())
                .title(format!("{} - Bin", selected.selected.title));

            let formatted_date_time = selected
                .selected
                .expire_at
                .format("%d/%m/%Y %I:%M %p")
                .to_string();
            let expire_text = Text::style(
                format!("Expire at: {}\n\n", formatted_date_time).into(),
                Style::default().fg(Color::Blue),
            );
            let expire_paragraph = Paragraph::new(expire_text).block(top_block);
            f.render_widget(expire_paragraph, layout_chunks[0]);

            let content_block = Block::new()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::new().green());
            let content_text = Text::styled(
                format!("{}", selected.selected.content),
                Style::default().fg(Color::Magenta),
            );
            let content_paragraph = Paragraph::new(content_text).block(content_block);
            f.render_widget(content_paragraph, layout_chunks[1]);

            let file_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Green))
                .style(Style::new().green())
                .padding(Padding::top(1))
                .title("Files:");
            let files = selected.selected.files.iter().map(|file| {
                let formatted = Text::from(vec![
                    Line::from(file.name.as_str()),
                    Line::from(file.type_name.as_str()).style(Style::default().fg(Color::Blue)),
                    Line::from("\n"),
                ]);
                ListItem::new(formatted).style(Style::default().fg(Color::Cyan))
            });
            let list = List::new(files)
                .highlight_style(Style::new().bg(Color::Green))
                .style(Style::new().green())
                .highlight_symbol("-> ")
                .repeat_highlight_symbol(false)
                .block(file_block);
            f.render_stateful_widget(list, layout_chunks[2], &mut selected.selected_file);
        }
    }

    fn help_widget(&self, area: Rect, f: &mut Frame) {
        let help_message = match &self.current_screen {
            CurrentScreen::Main => {
                "(q) to quit / (b) back to main menu / (n) to add a new bin / (l) to see a bin"
            }
            CurrentScreen::Show(selected) => {
                if selected.selected_file.selected().is_none() {
                    "(q) to quit / (b) back to bin menu / (d) to delete this bin / (e) to edit this bin"
                } else {
                    "(q) to quit / (b) back to bin menu / (o) open file / (d) to delete this bin / (e) to edit this bin"
                }
            }
            CurrentScreen::Edit(_) => "(Esc) to cancel edit / (Ctrl-S) to save",
        };

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }

    fn edit_bin_widget(&mut self, area: Rect, f: &mut Frame) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(4), Constraint::Min(1)])
            .split(area);

        if let CurrentScreen::Edit(edit_selected) = &self.current_screen {
            let title_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .style(Style::new().green())
                .title("Title");
            let title_widget = edit_selected.title_input.clone().block(title_block);
            f.render_widget(&title_widget, chunks[0]);

            let content_block = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta))
                .style(Style::new().green())
                .title("Content");
            let content_widget = edit_selected.content_input.clone().block(content_block);
            f.render_widget(&content_widget, chunks[1]);
        }
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match &self.current_screen {
            CurrentScreen::Main => match e.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
                KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
                KeyCode::Char('j') | KeyCode::Down => {
                    self.select_next();
                    None
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.select_previous();
                    None
                }
                KeyCode::Char('g') | KeyCode::Home => {
                    self.select_first();
                    None
                }
                KeyCode::Char('G') | KeyCode::End => {
                    self.select_last();
                    None
                }
                KeyCode::Char('l') | KeyCode::Enter => {
                    if let Some(index) = self.list_state.selected() {
                        let selected = self.bins[index].clone();
                        self.current_screen = CurrentScreen::Show(Selected {
                            selected,
                            selected_file: ListState::default().with_selected(None),
                        });
                    }
                    None
                }
                _ => None,
            },
            CurrentScreen::Show(selected) => match e.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
                KeyCode::Char('b') => {
                    self.current_screen = CurrentScreen::Main;
                    None
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !selected.selected.files.is_empty() {
                        self.select_next_file();
                    }
                    None
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if !selected.selected.files.is_empty() {
                        self.select_previous_file();
                    }
                    None
                }
                KeyCode::Char('o') => {
                    if !selected.selected_file.selected().is_none() {
                        let selected_index = selected.selected_file.selected().unwrap();
                        let selected_file = &selected.selected.files[selected_index];

                        match open::that(format!("{}{}", ENDPOINT, selected_file.access_path)) {
                            Ok(()) => {}
                            Err(e) => info!("Failed to open file: {e}"),
                        }
                    }
                    None
                }
                KeyCode::Char('e') => {
                    self.current_screen = CurrentScreen::Edit(EditSelected {
                        selected: selected.selected.clone(),
                        selected_file: ListState::default().with_selected(None),
                        title_input: InputWidget::new(
                            String::new(),
                            Style::default(),
                            Style::default().fg(Color::Black).bg(Color::White),
                        ),
                        content_input: MultilineInput::new(String::new()),
                        focused_element: EditElements::Title,
                    });
                    None
                }
                _ => None,
            },
            CurrentScreen::Edit(selected) => match e.code {
                KeyCode::Esc => {
                    // TODO: get to show state without any changes
                    self.current_screen = CurrentScreen::Show(Selected {
                        selected: selected.selected.clone(),
                        selected_file: ListState::default().with_selected(None),
                    });
                    None
                }
                KeyCode::Char('s') if e.modifiers == KeyModifiers::CONTROL => {
                    if let CurrentScreen::Edit(edit_selected) = &self.current_screen {
                        let val = edit_selected.title_input.content();
                        info!("Input: {val}");
                    }

                    self.current_screen = CurrentScreen::Show(Selected {
                        selected: selected.selected.clone(),
                        selected_file: ListState::default().with_selected(None),
                    });
                    None
                }
                KeyCode::Tab => {
                    if let CurrentScreen::Edit(edit_selected) = &mut self.current_screen {
                        match edit_selected.focused_element {
                            EditElements::Title => {
                                edit_selected.focused_element = EditElements::Content
                            }
                            EditElements::Content => {
                                edit_selected.focused_element = EditElements::Files
                            }
                            EditElements::Files => {
                                edit_selected.focused_element = EditElements::Title
                            }
                        }
                    }
                    None
                }
                _ => {
                    if let CurrentScreen::Edit(edit_selected) = &mut self.current_screen {
                        match edit_selected.focused_element {
                            EditElements::Title => edit_selected.title_input.handle_key(e),
                            EditElements::Content => edit_selected.content_input.handle_key(e),
                            _ => {}
                        }
                    }
                    None
                }
            },
        }
    }

    pub fn handle_socket_event(&mut self, event: PhoenixEvent) {
        if let Some(payload) = event.payload {
            let action = payload["action"].as_str().unwrap();
            let data = payload["data"].clone();
            info!("{data:#?}");
            match action {
                // TODO: optimization: get only title and expire_at field for bin list
                "get-all" => {
                    let data: Result<Vec<Bin>, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        self.bins = data.unwrap();
                        self.select_first();
                    }
                }
                e => info!("Unhandled action: {e}"),
            }
        }
    }

    fn push_event(&self, payload: Option<Value>) {
        match self.screen_sender.try_send(PhoenixEvent {
            name: "bin".to_string(),
            payload,
        }) {
            Ok(()) => info!("sent message"),
            Err(e) => info!("{e}"),
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    fn select_previous_file(&mut self) {
        match &mut self.current_screen {
            CurrentScreen::Show(selected) => selected.selected_file.select_previous(),
            CurrentScreen::Edit(selected) => selected.selected_file.select_previous(),
            _ => {}
        }
    }

    fn select_next_file(&mut self) {
        match &mut self.current_screen {
            CurrentScreen::Show(selected) => selected.selected_file.select_next(),
            CurrentScreen::Edit(selected) => selected.selected_file.select_next(),
            _ => {}
        }
    }
}
