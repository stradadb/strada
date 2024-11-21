use rusqlite::{Connection, Row, Result};

#[derive(Clone)] // Not necessary for `Database`, we will manually implement `Clone`
pub struct Database {
    db_path: String,
    conn: Option<Connection>, // We will not clone the `Connection` directly
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn: Some(conn) })
    }

    pub fn execute_query(&self, query: &str) -> Result<Vec<Vec<String>>> {
        let mut stmt = self.conn.prepare(query)?;
        let rows = stmt.query_map([], |row: &Row| {
            let mut row_vec = Vec::new();
            for col_idx in 0..row.column_count() {
                row_vec.push(row.get(col_idx)?);
            }
            Ok(row_vec)
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }
}