use ratatui::text::{Line, Span};
use ratatui::Frame;

pub struct App {
    pub screen: Screens,
}

pub enum Screens {
    Main,
    Todos,
    Phoenix,
    Notes,
    Projects,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screens::Main,
        }
    }

    pub fn render(&self, f: &mut Frame) {
        match self.screen {
            Screens::Main => {
                // f.render_widget(widget, area);
            }
            _ => {}
        }
    }
}
