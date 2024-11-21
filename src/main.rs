mod database;
mod parser;
mod server;

use database::Database;
use server::start_server;
use tokio;

#[tokio::main]
async fn main() {
    let db = Database::new("strada.db").expect("Failed to open the database");
    start_server(db).await;
}