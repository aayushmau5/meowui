use cli_log::info;
use rusqlite::{Connection, Error, Params, Result};

#[derive(Debug)]
pub struct Sqlite {
    pub connection: Connection,
}

const DB: &str = "./meowui.db";

impl Sqlite {
    pub fn new() -> Self {
        let conn = Connection::open(DB.to_string()).unwrap();
        Self { connection: conn }
    }

    pub fn execute_query(&self, query: &str) -> Result<usize> {
        self.connection.execute(query, [])
    }

    pub fn execute_query_with_params(&self, query: &str, params: impl Params) -> Result<usize> {
        self.connection.execute(query, params)
    }

    pub fn check_table_exists(&self, table_name: &str) -> bool {
        let result: Result<String> = self.connection.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?1;",
            [table_name],
            |row| row.get(0),
        );
        match result {
            Ok(_) => true,
            Err(e) => match e {
                Error::QueryReturnedNoRows => false,
                _ => false,
            },
        }
    }
}
