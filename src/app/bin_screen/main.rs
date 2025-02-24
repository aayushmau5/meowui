use super::{AppActions, Bin, BinActions, CurrentScreen, ScreenType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct MainScreen {
    pub bins: Vec<Bin>,
    pub list_state: ListState,
}

impl MainScreen {
    pub fn new(bins: Vec<Bin>) -> Self {
        let bin_screen = Self {
            bins,
            list_state: ListState::default(),
        };
        bin_screen
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        self.menu_items_widget(chunks[0], f);
        self.help_widget(chunks[1], f);
    }

    // Keyboard event handler

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<BinActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(BinActions::App(AppActions::Quit)),
            KeyCode::Char('b') => Some(BinActions::App(AppActions::ChangeScreen(ScreenType::Main))),
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
            KeyCode::Char('l') | KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    let selected = self.bins[index].clone();
                    return Some(BinActions::ChangeScreen(
                        CurrentScreen::Show,
                        Some(selected),
                    ));
                }
                None
            }
            KeyCode::Char('n') => Some(BinActions::ChangeScreen(CurrentScreen::New, None)),
            _ => None,
        }
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    fn menu_items_widget(&mut self, area: Rect, f: &mut Frame) {
        let items = self.bins.iter().map(|item| {
            let mut text = Text::default();
            let formatted_date_time = item.expire_at.format("%d/%m/%Y %I:%M %p").to_string();
            text.extend([
                Span::raw(&item.title),
                Span::raw(format!("Expire at: {formatted_date_time}")).blue(),
                Span::raw(""),
            ]);
            ListItem::new(text)
        });

        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title("Bin");

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().reversed())
            .style(Style::new().green())
            .highlight_symbol("-> ")
            .repeat_highlight_symbol(false);

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn help_widget(&self, area: Rect, f: &mut Frame) {
        let help_message =
            "(q) to quit / (b) back to main menu / (n) to add a new bin / (l) to see a bin";

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }
}
