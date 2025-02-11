use tokio::sync::broadcast::Receiver;
use tokio::sync::mpsc::Sender;

use crossterm::event::KeyEvent;
use ratatui::Frame;

// Screens
mod main_screen;
mod notes_screen;
mod phoenix_screen;
mod projects_screen;
mod todos_screen;

use super::tui::TUIAction;
use main_screen::MainScreen;
use notes_screen::NotesScreen;
use phoenix_screen::PhoenixScreen;
use projects_screen::ProjectsScreen;
use todos_screen::TodosScreen;

pub struct App {
    pub screen: Screens,
    pub socket_receiver: Receiver<String>,
    pub screen_sender: Sender<String>,
}

pub enum ScreenType {
    Main,
    Todos,
    Phoenix,
    Notes,
    Projects,
}

pub enum Screens {
    Main(MainScreen),
    Todos(TodosScreen),
    Phoenix(PhoenixScreen),
    Notes(NotesScreen),
    Projects(ProjectsScreen),
}

pub enum AppActions {
    ChangeScreen(ScreenType),
    Quit,
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

    fn handle_socket_event(&mut self, payload: String) {
        match self {
            Screens::Main(main_screen) => main_screen.handle_socket_event(payload),
            Screens::Todos(todos_screen) => todos_screen.handle_socket_event(payload),
            Screens::Notes(notes_screen) => notes_screen.handle_socket_event(payload),
            Screens::Phoenix(phoenix_screen) => phoenix_screen.handle_socket_event(payload),
            Screens::Projects(projects_screen) => projects_screen.handle_socket_event(payload),
        }
    }
}

impl App {
    pub fn new(socket_receiver: Receiver<String>, screen_sender: Sender<String>) -> Self {
        Self {
            screen: Screens::Main(MainScreen::new()),
            socket_receiver,
            screen_sender,
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        self.screen.render(f);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<TUIAction> {
        let key_response = self.screen.handle_key(key);
        match key_response {
            Some(AppActions::ChangeScreen(screen)) => {
                let screen = match screen {
                    ScreenType::Main => Screens::Main(MainScreen::new()),
                    ScreenType::Notes => {
                        Screens::Notes(NotesScreen::new(self.screen_sender.clone()))
                    }
                    ScreenType::Phoenix => Screens::Phoenix(PhoenixScreen::new()),
                    ScreenType::Projects => Screens::Projects(ProjectsScreen::new()),
                    ScreenType::Todos => Screens::Todos(TodosScreen::new()),
                };
                self.change_screen(screen);
                None
            }
            Some(AppActions::Quit) => Some(TUIAction::Quit),
            None => None,
        }
    }

    pub fn change_screen(&mut self, screen: Screens) {
        self.screen = screen;
    }

    pub fn receive_socket_events(&mut self) {
        if let Ok(event_payload) = self.socket_receiver.try_recv() {
            // TODO: filter between main app events and screen specific events
            self.screen.handle_socket_event(event_payload);
        }
    }
}
