// Screens
mod bin_screen;
mod main_screen;
mod notes_screen;
mod projects_screen;
mod todos_screen;

use super::tui::TUIAction;
use crate::phoenix::event::PhoenixEvent;
use bin_screen::BinScreen;
use cli_log::info;
use crossterm::event::KeyEvent;
use main_screen::MainScreen;
use notes_screen::NotesScreen;
use projects_screen::ProjectsScreen;
use ratatui::Frame;
use todos_screen::TodosScreen;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct App {
    pub screen: Screens,
    pub socket_receiver: Receiver<PhoenixEvent>,
    pub screen_sender: Sender<PhoenixEvent>,
}

pub enum ScreenType {
    Main,
    Todos,
    Bin,
    Notes,
    Projects,
}

pub enum Screens {
    Main(MainScreen),
    Todos(TodosScreen),
    Bin(BinScreen),
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
            Screens::Bin(bin_screen) => bin_screen.render(f),
            Screens::Projects(projects_screen) => projects_screen.render(f),
        }
    }

    fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match self {
            Screens::Main(main_screen) => main_screen.handle_key(e),
            Screens::Todos(todos_screen) => todos_screen.handle_key(e),
            Screens::Notes(notes_screen) => notes_screen.handle_key(e),
            Screens::Bin(bin_screen) => bin_screen.handle_key(e),
            Screens::Projects(projects_screen) => projects_screen.handle_key(e),
        }
    }

    fn handle_socket_event(&mut self, payload: PhoenixEvent) {
        match self {
            Screens::Main(main_screen) => main_screen.handle_socket_event(payload),
            Screens::Todos(todos_screen) => todos_screen.handle_socket_event(payload),
            Screens::Notes(notes_screen) => notes_screen.handle_socket_event(payload),
            Screens::Bin(bin_screen) => bin_screen.handle_socket_event(payload),
            Screens::Projects(projects_screen) => projects_screen.handle_socket_event(payload),
        }
    }
}

impl App {
    pub fn new(
        socket_receiver: Receiver<PhoenixEvent>,
        screen_sender: Sender<PhoenixEvent>,
    ) -> Self {
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
                    ScreenType::Bin => Screens::Bin(BinScreen::new(self.screen_sender.clone())),
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
            // TODO: filter between main app events and screen specific events(here? or in each screen?)
            info!("receive_socket_event: {event_payload}");
            self.screen.handle_socket_event(event_payload);
        }
    }
}
