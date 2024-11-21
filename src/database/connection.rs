use rusqlite::{Connection, Result};
use std::path::Path;

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

    pub fn query<T>(&self, query: &str, mapper: impl Fn(&rusqlite::Row) -> rusqlite::Result<T>) -> Result<Vec<T>> {
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], mapper)?;
        rows.collect()
    }
}