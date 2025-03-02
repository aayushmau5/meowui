use super::{AppActions, ScreenType};
use crate::{phoenix::event::PhoenixEvent, sqlite::Sqlite};
use cli_log::info;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, Row, Table},
    Frame,
};
use rusqlite::Result as SqliteResult;

const TABLE: &str = "projects";

pub struct ProjectsScreen {
    db: Sqlite,
    data: Vec<ProjectData>,
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

        let mut projects = Self {
            db: sqlite,
            data: vec![],
        };

        if !projects.db.check_table_exists("projects") {
            projects.create_projects_table();
        }

        projects.data = projects.get_projects();
        projects
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

    // pub fn add_project(&self, ProjectData) {}

    pub fn render(&mut self, f: &mut Frame) {
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
                            .bold()
                            .underline_color(Color::Blue)
                            .fg(Color::Blue),
                    )
                    .slow_blink()
                    .add_modifier(Modifier::UNDERLINED),
            )
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .block(block)
            .highlight_symbol(">>");

        f.render_widget(table, f.area());
    }

    pub fn handle_key(&mut self, e: KeyEvent) -> Option<AppActions> {
        match e.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(AppActions::Quit),
            KeyCode::Char('b') => Some(AppActions::ChangeScreen(ScreenType::Main)),
            _ => None,
        }
    }

    pub fn handle_socket_event(&self, event: PhoenixEvent) {
        println!("{event}");
    }

    // db stuff
    pub fn create_projects_table(&self) {
        let result = self.db.connection.execute(
            "CREATE TABLE projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                editor TEXT NOT NULL
            )",
            (),
        );

        match result {
            Ok(r) => info!("Table creation: {r}"),
            Err(e) => info!("{e}"),
        }
    }
}
