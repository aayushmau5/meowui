use super::{AppActions, ScreenType};
use crate::phoenix::event::PhoenixEvent;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub struct TodosScreen {}

impl TodosScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black))
            .title("Todos Screen");
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
        println!("{payload}");
    }
}
