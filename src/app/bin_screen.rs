use super::{AppActions, ScreenType};
use crate::phoenix::event::PhoenixEvent;
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

/// ## Events
///
/// ### Get all bins
/// screen -> socket: `{"name": "bin", "payload": {"action": "get-all"}}`
///
/// socket -> screen: `{"name": "bin", "payload": {"action": "get-all", "data": [...]}}`
pub struct BinScreen {
    pub screen_sender: Sender<PhoenixEvent>,
}

impl BinScreen {
    pub fn new(screen_sender: Sender<PhoenixEvent>) -> Self {
        let bin_screen = Self { screen_sender };
        bin_screen.push_event(Some(json!({"action": "get-all"})));
        bin_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black))
            .title("Bin Screen");
        f.render_widget(block, f.area());
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
            _ => None,
        }
    }

    pub fn handle_socket_event(&self, payload: PhoenixEvent) {
        info!("{payload}");
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
}
