use crossterm::event::KeyEvent;
use ratatui::Frame;

use super::AppActions;
pub struct NotesScreen {}

impl NotesScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame) {}

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        None
    }
}
