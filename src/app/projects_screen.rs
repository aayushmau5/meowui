use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::{main_screen::MainScreen, AppActions, Screens};

pub struct ProjectsScreen {}

impl ProjectsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame) {
        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black))
            .title("Projects Screen");
        f.render_widget(block, f.area());
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(Screens::Main(MainScreen::new()))),
            _ => None,
        }
    }
}
