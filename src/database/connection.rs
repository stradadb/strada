use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug)]
pub struct DatabaseConnection {
    conn: Connection,
}

impl DatabaseConnection {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(DatabaseConnection { conn })
    }

    pub fn execute(&self, query: &str) -> Result<usize> {
        self.conn.execute(query, [])
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}