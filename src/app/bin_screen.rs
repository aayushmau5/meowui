mod edit;
mod main;
mod new;
mod show;

use super::{AppActions, ScreenType};
use crate::phoenix::event::{PhoenixEvent, StatusEvent};
use chrono::{DateTime, Local};
use cli_log::info;
use crossterm::event::KeyEvent;
use edit::EditScreen;
use main::MainScreen;
use new::NewScreen;
use ratatui::Frame;
use serde_json::{json, Value};
use show::ShowScreen;
use tokio::sync::mpsc::Sender;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Bin {
    id: u64,
    title: String,
    content: String,
    expire_at: DateTime<Local>,
    files: Vec<File>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct File {
    id: String,
    name: String,
    access_path: String,
    #[serde(rename = "type")]
    type_name: String,
}

pub enum BinActions {
    SendEvent(Option<Value>),
    App(AppActions),
    ChangeScreen(CurrentScreen, Option<Bin>),
}

pub enum CurrentScreen {
    Main,
    New,
    Show,
    Edit,
}

enum Screens {
    Main(MainScreen),
    New(NewScreen),
    Show(ShowScreen),
    Edit(EditScreen),
}

impl Screens {
    fn render(&mut self, f: &mut Frame) {
        match self {
            Screens::Main(main_screen) => main_screen.render(f),
            Screens::New(new_screen) => new_screen.render(f),
            Screens::Show(show_screen) => show_screen.render(f),
            Screens::Edit(edit_screen) => edit_screen.render(f),
        }
    }

    fn handle_key(&mut self, e: KeyEvent) -> Option<BinActions> {
        match self {
            Screens::Main(main_screen) => main_screen.handle_key(e),
            Screens::New(new_screen) => new_screen.handle_key(e),
            Screens::Show(show_screen) => show_screen.handle_key(e),
            Screens::Edit(edit_screen) => edit_screen.handle_key(e),
        }
    }
}

/// ## Events
///
/// ### Get all bins
/// screen -> socket: `{"name": "bin", "payload": {"action": "get-all"}}`
///
/// screen <- socket: `{"name": "bin", "payload": {"action": "get-all", "data": [...]}}`
///
/// ### Edit bin
/// screen -> socket: `{"name": "bin", "payload": {"action": "edit", "data": {"id": 1, ...}}}`
///
/// screen <- socket: `{"name": "bin", "payload": {"action": "edit", "data": {"id": 1, ...}}}`
///
/// ### Delete bin
/// screen -> socket: `{"name": "bin", "payload": {"action": "delete", "id": 1}}`
///
pub struct BinScreen {
    pub screen_sender: Sender<PhoenixEvent>,
    current_screen: CurrentScreen,
    screen: Screens,
}

impl BinScreen {
    pub fn new(screen_sender: Sender<PhoenixEvent>) -> Self {
        let bin_screen = Self {
            screen_sender,
            current_screen: CurrentScreen::Main,
            screen: Screens::Main(MainScreen::new(Vec::new())),
        };

        bin_screen.push_event(Some(json!({"action": "get-all"})));
        bin_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        self.screen.render(f);
    }

    // Keyboard event handler

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match self.screen.handle_key(e) {
            Some(action) => match action {
                BinActions::ChangeScreen(screen_type, data) => match screen_type {
                    CurrentScreen::Main => {
                        self.change_to_main_screen();
                        None
                    }
                    CurrentScreen::New => {
                        self.current_screen = CurrentScreen::New;
                        self.screen = Screens::New(NewScreen::new());
                        None
                    }
                    CurrentScreen::Show => {
                        self.current_screen = CurrentScreen::Show;
                        self.screen = Screens::Show(ShowScreen::new(data.unwrap()));
                        None
                    }
                    CurrentScreen::Edit => {
                        self.current_screen = CurrentScreen::Edit;
                        self.screen = Screens::Edit(EditScreen::new(data.unwrap()));
                        None
                    }
                },
                BinActions::SendEvent(value) => {
                    self.push_event(value);
                    None
                }
                BinActions::App(action) => Some(action),
            },
            None => None,
        }
    }

    // Event handlers

    pub fn handle_socket_event(&mut self, event: PhoenixEvent) {
        if let Some(payload) = event.payload {
            let action = payload["action"].as_str().unwrap();
            let data = payload["data"].clone();
            info!("{data:#?}");
            match action {
                "get-all" => {
                    let data: Result<Vec<Bin>, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        if let Screens::Main(main_screen) = &mut self.screen {
                            main_screen.bins = data.unwrap();
                            main_screen.select_first();
                        }
                    }
                }
                "new" => {
                    let data: Result<StatusEvent, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        let data = data.unwrap();
                        if data.status.as_str() == "OK" {
                            // on successful creation of new bin
                            // move to main screen
                            if !matches!(self.current_screen, CurrentScreen::Main) {
                                self.change_to_main_screen();
                            }
                        } else {
                            info!("{}", data.message.unwrap());
                        }
                    }
                }
                "delete" => {
                    let data: Result<StatusEvent, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        let data = data.unwrap();
                        if data.status.as_str() == "OK" {
                            // on successful deletion
                            // move to main screen
                            if !matches!(self.current_screen, CurrentScreen::Main) {
                                self.change_to_main_screen();
                            }
                        } else {
                            info!("{}", data.message.unwrap());
                        }
                    }
                }
                "edit" => {
                    let data: Result<Bin, serde_json::Error> = serde_json::from_value(data);
                    if data.is_ok() {
                        let bin = data.unwrap();
                        self.current_screen = CurrentScreen::Show;
                        self.screen = Screens::Show(ShowScreen::new(bin));
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

    // Used frequently. Moved into a helper function.
    /// Change to the main screen.
    fn change_to_main_screen(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.screen = Screens::Main(MainScreen::new(Vec::new()));
        self.push_event(Some(json!({"action": "get-all"})));
    }
}
