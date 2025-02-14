use super::{AppActions, ScreenType};
use crate::phoenix::event::PhoenixEvent;
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
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
    bins: Vec<Bin>,
    list_state: ListState,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Bin {
    title: String,
    content: String,
    expire_at: String,
    files: Vec<File>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct File {}

impl BinScreen {
    pub fn new(screen_sender: Sender<PhoenixEvent>) -> Self {
        let bin_screen = Self {
            screen_sender,
            bins: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
        };
        bin_screen.push_event(Some(json!({"action": "get-all"})));
        bin_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        let items = self
            .bins
            .iter()
            .map(|item| ListItem::from(item.title.as_str()));

        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title("Bin");

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().reversed())
            .style(Style::new().green())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        f.render_stateful_widget(list, f.area(), &mut self.list_state);
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
            KeyCode::Char('h') | KeyCode::Left => {
                self.select_none();
                None
            }
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
            _ => None,
        }
    }

    pub fn handle_socket_event(&mut self, event: PhoenixEvent) {
        if let Some(payload) = event.payload {
            let action = payload["action"].as_str().unwrap();
            let data = payload["data"].clone();
            match action {
                "get-all" => {
                    let data: Result<Vec<Bin>, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        self.bins = data.unwrap();
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

    fn select_none(&mut self) {
        self.list_state.select(None);
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
}
