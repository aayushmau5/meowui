use super::{AppActions, Bin, BinActions, CurrentScreen};
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use serde_json::json;

const ENDPOINT: &str = if cfg!(debug_assertions) {
    "http://localhost:4000"
} else {
    "https://phoenix.aayushsahu.com"
};

pub struct ShowScreen {
    bin: Bin,
    list_state: ListState,
}

impl ShowScreen {
    pub fn new(bin: Bin) -> Self {
        Self {
            bin,
            list_state: ListState::default().with_selected(None),
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        self.bin_widget(chunks[0], f);
        self.help_widget(chunks[1], f);
    }

    // Keyboard event handler

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<BinActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(BinActions::App(AppActions::Quit)),
            KeyCode::Char('b') => Some(BinActions::ChangeScreen(CurrentScreen::Main, None)),
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.bin.files.is_empty() {
                    self.select_next_file();
                }
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !self.bin.files.is_empty() {
                    self.select_previous_file();
                }
                None
            }
            KeyCode::Char('d') => {
                let event = Some(json!({"action": "delete", "data": {"id": self.bin.id}}));
                Some(BinActions::SendEvent(event))
            }
            KeyCode::Char('o') => {
                if !self.list_state.selected().is_none() {
                    let selected_index = self.list_state.selected().unwrap();
                    let selected_file = &self.bin.files[selected_index];

                    match open::that(format!("{}{}", ENDPOINT, selected_file.access_path)) {
                        Ok(()) => {}
                        Err(e) => info!("Failed to open file: {e}"),
                    }
                }
                None
            }
            KeyCode::Char('e') => {
                let selected = self.bin.clone();
                Some(BinActions::ChangeScreen(
                    CurrentScreen::Edit,
                    Some(selected),
                ))
            }
            _ => None,
        }
    }

    fn select_previous_file(&mut self) {
        self.list_state.select_previous();
    }

    fn select_next_file(&mut self) {
        self.list_state.select_next();
    }

    fn bin_widget(&mut self, area: Rect, f: &mut Frame) {
        if !self.bin.files.is_empty() {
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
            .title(format!("{} - Bin", self.bin.title));

        let formatted_date_time = self.bin.expire_at.format("%d/%m/%Y %I:%M %p").to_string();
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
            format!("{}", self.bin.content),
            Style::default().fg(Color::Magenta),
        );
        let content_paragraph = Paragraph::new(content_text).block(content_block);
        f.render_widget(content_paragraph, layout_chunks[1]);
    }

    fn bin_widget_with_files(&mut self, area: Rect, f: &mut Frame) {
        let layout_chunks = Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Min(1),
            ])
            .split(area);

        let top_block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title(format!("{} - Bin", self.bin.title));

        let formatted_date_time = self.bin.expire_at.format("%d/%m/%Y %I:%M %p").to_string();
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
            format!("{}", self.bin.content),
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
        let files = self.bin.files.iter().map(|file| {
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
        f.render_stateful_widget(list, layout_chunks[2], &mut self.list_state);
    }

    fn help_widget(&self, area: Rect, f: &mut Frame) {
        let help_message = if self.list_state.selected().is_none() {
            "(q) to quit / (b) back to bin menu / (d) to delete this bin / (e) to edit this bin"
        } else {
            "(q) to quit / (b) back to bin menu / (o) open file / (d) to delete this bin / (e) to edit this bin"
        };

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }
}
