use super::{AppActions, ScreenType};
use crate::phoenix::event::PhoenixEvent;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::widgets::{Block, List, ListItem, ListState};
use ratatui::Frame;

enum Menu {
    Todos,
    Notes,
    Phoenix,
    Projects,
}

impl Menu {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Notes" => Some(Menu::Notes),
            "Phoenix" => Some(Menu::Phoenix),
            "Projects" => Some(Menu::Projects),
            "Todos" => Some(Menu::Todos),
            _ => None,
        }
    }

    fn items() -> Vec<&'static str> {
        vec!["Notes", "Phoenix", "Projects", "Todos"]
    }
}

pub struct MainScreen {
    list_items: Vec<&'static str>,
    list_state: ListState,
}

impl MainScreen {
    pub fn new() -> Self {
        Self {
            list_items: Menu::items(),
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let items = self
            .list_items
            .iter()
            .map(|&todo_item| ListItem::from(todo_item));

        let list = List::new(items)
            .block(Block::bordered().title("MeowUI"))
            .highlight_style(Style::new().reversed())
            .style(Style::new().green())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        f.render_stateful_widget(list, f.area(), &mut self.list_state);
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('h') | KeyCode::Left => {
                self.select_none();
                None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                None
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.select_first();
                None
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.select_last();
                None
            }
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                if let Some(i) = self.list_state.selected() {
                    let item = self.list_items[i];
                    let screen = match Menu::from_str(item)? {
                        Menu::Notes => ScreenType::Notes,
                        Menu::Phoenix => ScreenType::Phoenix,
                        Menu::Todos => ScreenType::Todos,
                        Menu::Projects => ScreenType::Projects,
                    };
                    Some(AppActions::ChangeScreen(screen))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn select_none(&mut self) {
        self.list_state.select(None);
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }
    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    pub fn handle_socket_event(&self, payload: PhoenixEvent) {
        println!("{payload}");
    }
}
