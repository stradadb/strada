use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: String,
}

pub struct SQLiteParser;

impl SQLiteParser {
    pub fn parse_schema(conn: &Connection) -> Result<Vec<TableSchema>> {
        let mut schemas = Vec::new();
        
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_names = stmt.query_map([], |row| row.get::<_, String>(0))?;

        for table_name_result in table_names {
            let table_name = table_name_result?;
            let columns = Self::parse_table_columns(conn, &table_name)?;
            
            schemas.push(TableSchema {
                name: table_name,
                columns,
            });
        }

        Ok(schemas)
    }

    fn parse_table_columns(conn: &Connection, table_name: &str) -> Result<Vec<ColumnSchema>> {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
        let columns = stmt.query_map([], |row| {
            Ok(ColumnSchema {
                name: row.get(1)?,
                data_type: row.get(2)?,
            })
        })?;

        columns.collect()
    }
}