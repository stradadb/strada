use rusqlite::Result;
use crate::database::Database;

pub fn parse_and_run_query(database: &Database, query: &str) -> Result<Vec<Vec<String>>, rusqlite::Error> {
    database.execute_query(query)
}