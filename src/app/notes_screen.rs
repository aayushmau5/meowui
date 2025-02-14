use super::{AppActions, ScreenType};
use crate::phoenix::event::PhoenixEvent;
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use serde_json::json;
use tokio::sync::mpsc::Sender;

/// ## Events
///
/// ### Get Workspaces
/// screen -> socket: `{"name": "notes", "payload": {"action": "get-workspaces"}}`
///
/// socket -> screen: `{"name": "notes", "payload": {"action": "get-workspaces", "data": [...]}}`

pub struct NotesScreen {
    pub screen_sender: Sender<PhoenixEvent>,
}

impl NotesScreen {
    pub fn new(screen_sender: Sender<PhoenixEvent>) -> Self {
        let notes_screen = Self { screen_sender };
        notes_screen.push_event();
        notes_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title("Notes");
        f.render_widget(block, f.area());
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
            _ => None,
        }
    }

    pub fn handle_socket_event(&self, event: PhoenixEvent) {
        println!("{event}");
    }

    fn push_event(&self) {
        match self.screen_sender.try_send(PhoenixEvent {
            name: "notes".to_string(),
            payload: Some(json!({"action": "get-all"})),
        }) {
            Ok(()) => info!("sent message"),
            Err(e) => info!("{e}"),
        }
    }
}
