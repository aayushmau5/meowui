use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
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

pub enum TUIAction {
    Quit,
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
        self.event_loop(app)?; // TODO: call done() on error as well
        Self::done()
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

    fn event_loop(&mut self, mut app: App) -> Result<(), std::io::Error> {
        loop {
            app.receive_socket_events();
            self.terminal.draw(|f| app.render(f))?;
            if let Event::Key(e) = event::read()? {
                let response = match e {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => Some(TUIAction::Quit),
                    e => app.handle_key(e),
                };
                match response {
                    Some(TUIAction::Quit) => break Ok(()),
                    None => {}
                }
            }
        }
    }
}
