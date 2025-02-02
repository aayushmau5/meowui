use crossterm::event::KeyEvent;
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::widgets::{Block, List, ListDirection, ListItem, ListState};
use ratatui::Frame;

use super::todos_screen::TodosScreen;
use super::{AppActions, Screens};

pub struct MainScreen {
    list_items: Vec<&'static str>,
    list_state: ListState,
}

impl MainScreen {
    pub fn new() -> Self {
        Self {
            list_items: vec!["Todos", "Notes", "Phoenix", "Projects"],
            list_state: ListState::default(),
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let mut state = ListState::default();
        let items = self
            .list_items
            .iter()
            .map(|&todo_item| ListItem::from(todo_item));

        let list = List::new(items)
            .block(Block::bordered().title("Stuff"))
            .highlight_style(Style::new().reversed())
            .style(Style::new().white())
            .highlight_symbol(">>")
            .direction(ListDirection::BottomToTop)
            .repeat_highlight_symbol(true);
        f.render_stateful_widget(list, f.area(), &mut state);
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        Some(AppActions::ChangeScreen(Screens::Todos(TodosScreen::new())))
    }
}
