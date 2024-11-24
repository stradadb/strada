mod database;

use database::start_server;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_path = env::var("STRADA_DATABASE_PATH")
        .unwrap_or_else(|_| "./strada.db".to_string());

    println!("Starting STRADA database server with: {}", database_path);
    start_server(database_path).await
}