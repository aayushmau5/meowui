use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

use crate::app::App;

type Term = Terminal<CrosstermBackend<std::io::Stdout>>;
pub struct TUI {
    terminal: Term,
}

impl TUI {
    pub fn new() -> Self {
        if let Ok(terminal) = Self::setup_terminal() {
            Self { terminal }
        } else {
            panic!("Cannot setup terminal")
        }
    }

    pub fn run(&mut self, app: App) -> Result<(), std::io::Error> {
        Self::init()?;
        self.event_loop(app)?;
        Self::done()?;
        Ok(())
    }

    fn setup_terminal() -> Result<Term, std::io::Error> {
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend)
    }

    fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)
    }

    fn done() -> Result<(), std::io::Error> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()
    }

    fn event_loop(&mut self, app: App) -> Result<(), std::io::Error> {
        loop {
            self.terminal.draw(|f| app.render(f))?;
            if let Event::Key(e) = event::read()? {
                match (e.code, e.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}

// reading, handling key events, quitting
// rendering ui
