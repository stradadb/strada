use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;
use std::sync::Mutex;

pub struct DatabaseState {
    conn: Mutex<Connection>,
}

async fn get_schema(data: web::Data<DatabaseState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    match crate::database::parser::SQLiteParser::parse_schema(&conn) {
        Ok(schema) => HttpResponse::Ok().json(schema),
        Err(_) => HttpResponse::InternalServerError().body("Failed to parse schema"),
    }
}

async fn execute_query(
    data: web::Data<DatabaseState>, 
    query: web::Json<String>
) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    match conn.execute(&query.into_inner(), []) {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        Err(_) => HttpResponse::BadRequest().body("Query execution failed"),
    }
}

pub async fn start_server(database_path: String) -> std::io::Result<()> {
    let conn = Connection::open(database_path).expect("Failed to open database");
    let db_state = web::Data::new(DatabaseState {
        conn: Mutex::new(conn),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .route("/schema", web::get().to(get_schema))
            .route("/query", web::post().to(execute_query))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}