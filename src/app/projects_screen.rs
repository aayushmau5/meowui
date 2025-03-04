use std::{env::current_dir, path::Path};

use super::{AppActions, ScreenType};
use crate::{phoenix::event::PhoenixEvent, sqlite::Sqlite};
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, TableState},
    Frame,
};
use rusqlite::Result as SqliteResult;

pub struct ProjectsScreen {
    db: Sqlite,
    data: Vec<ProjectData>,
    table_state: TableState,
}

#[derive(Debug)]
struct ProjectData {
    id: i32,
    name: String,
    editor: String,
    path: String,
}

impl ProjectsScreen {
    pub fn new() -> Self {
        let sqlite = Sqlite::new();

        if !sqlite.check_table_exists("projects") {
            Self::create_projects_table(&sqlite);
        }

        let mut projects = Self {
            db: sqlite,
            data: vec![],
            table_state: TableState::default().with_selected(Some(0)),
        };
        projects.data = projects.get_projects();

        projects
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(f.area());

        self.main_widget(chunks[0], f);
        self.help_widget(chunks[1], f);
    }

    // UIs

    fn main_widget(&mut self, area: Rect, f: &mut Frame) {
        let rows: Vec<Row> = self
            .data
            .iter()
            .map(|d| Row::new([d.name.clone(), d.editor.clone(), d.path.clone()]))
            .collect();

        let widths = [Constraint::Min(1), Constraint::Min(1), Constraint::Max(50)];

        let block = Block::new()
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .style(Style::new().green())
            .title("Projects");

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .header(
                Row::new(vec!["Name", "Editor", "Path"])
                    .style(
                        Style::new()
                            .italic()
                            .bold()
                            .underline_color(Color::Blue)
                            .fg(Color::Blue),
                    )
                    .slow_blink()
                    .add_modifier(Modifier::UNDERLINED),
            )
            .row_highlight_style(Style::new().reversed())
            .cell_highlight_style(Style::new().blue())
            .block(block)
            .highlight_symbol("->");

        f.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn help_widget(&mut self, area: Rect, f: &mut Frame) {
        let help_message =
            "(q | Esc) to quit / (b) back to main menu / (n) to add new entry / (d) to delete an entry / (e) to edit an entry";

        let help_widget = Span::styled(help_message, Style::default().fg(Color::Blue));
        let help_widget = Paragraph::new(Line::from(help_widget)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(help_widget, area);
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
            KeyCode::Char('j') => {
                self.select_next();
                None
            }
            KeyCode::Char('k') => {
                self.select_previous();
                None
            }
            KeyCode::Char('n') => {
                // new
                self.select_previous();
                None
            }
            KeyCode::Char('d') => {
                // delete
                self.select_previous();
                None
            }
            KeyCode::Char('e') => {
                // edit
                self.select_previous();
                None
            }
            KeyCode::Enter => {
                if let Some(index) = self.table_state.selected() {
                    let selected = &self.data[index];

                    let project_path = Path::new(".")
                        .join(selected.path.clone())
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    info!("Path: {project_path}");

                    // maybe exit with command in stdout?

                    // what to do when editor is vim?
                    // std::process::Command::new("nvim")
                    //     .arg(project_path)
                    //     .spawn()
                    //     .expect("Error: Failed to run editor")
                    //     .wait()
                    //     .expect("Error: Editor returned a non-zero status");
                }
                None
            }
            _ => None,
        }
    }

    fn select_next(&mut self) {
        self.table_state.select_next();
    }

    fn select_previous(&mut self) {
        self.table_state.select_previous();
    }

    pub fn handle_socket_event(&self, event: PhoenixEvent) {
        println!("{event}");
    }

    // db stuff

    fn create_projects_table(db: &Sqlite) {
        let result = db.connection.execute(
            "CREATE TABLE projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                editor TEXT NOT NULL
            )",
            (),
        );

        // Panic if cannot create table
        result.unwrap();
    }

    fn get_projects(&self) -> Vec<ProjectData> {
        let mut stmt = self
            .db
            .connection
            .prepare("SELECT * FROM projects")
            .unwrap();

        let projects = stmt
            .query_map([], |row| {
                Ok(ProjectData {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    editor: row.get(3)?,
                })
            })
            .unwrap();

        projects.map(|v| v.unwrap()).collect()
    }
}
