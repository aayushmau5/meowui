use crossterm::event::KeyEvent;
use ratatui::Frame;

// Screens
mod main_screen;
mod notes_screen;
mod phoenix_screen;
mod projects_screen;
mod todos_screen;

use main_screen::MainScreen;
use notes_screen::NotesScreen;
use phoenix_screen::PhoenixScreen;
use projects_screen::ProjectsScreen;
use todos_screen::TodosScreen;

pub struct App {
    pub screen: Screens,
}

pub enum Screens {
    Main(MainScreen),
    Todos(TodosScreen),
    Phoenix(PhoenixScreen),
    Notes(NotesScreen),
    Projects(ProjectsScreen),
}

pub enum AppActions {
    ChangeScreen(Screens),
}

impl Screens {
    fn render(&mut self, f: &mut Frame) {
        match self {
            Screens::Main(main_screen) => main_screen.render(f),
            Screens::Todos(todos_screen) => todos_screen.render(f),
            Screens::Notes(notes_screen) => notes_screen.render(f),
            Screens::Phoenix(phoenix_screen) => phoenix_screen.render(f),
            Screens::Projects(projects_screen) => projects_screen.render(f),
        }
    }
    fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match self {
            Screens::Main(main_screen) => main_screen.handle_key(e),
            Screens::Todos(todos_screen) => todos_screen.handle_key(e),
            Screens::Notes(notes_screen) => notes_screen.handle_key(e),
            Screens::Phoenix(phoenix_screen) => phoenix_screen.handle_key(e),
            Screens::Projects(projects_screen) => projects_screen.handle_key(e),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screens::Main(MainScreen::new()),
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        self.screen.render(f);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let key_response = self.screen.handle_key(key);
        match key_response {
            Some(AppActions::ChangeScreen(screen)) => self.change_screen(screen),
            None => {}
        }
    }

    pub fn change_screen(&mut self, screen: Screens) {
        self.screen = screen;
    }
}
