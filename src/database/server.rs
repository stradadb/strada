use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::auth::{create_token, validate_token};
use std::sync::Mutex;
use crate::database::connection::DatabaseConnection;

pub struct DatabaseState {
    conn: Mutex<DatabaseConnection>,
}

async fn get_schema(data: web::Data<DatabaseState>) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    match crate::database::parser::SQLiteParser::parse_schema(&conn.get_connection()) {
        Ok(schema) => HttpResponse::Ok().json(schema),
        Err(_) => HttpResponse::InternalServerError().body("Failed to parse schema"),
    }
}

async fn execute_query(
    data: web::Data<DatabaseState>, 
    query: web::Json<String>
) -> impl Responder {
    let conn = data.conn.lock().unwrap();
    match conn.execute(&query.into_inner()) {
        Ok(rows_affected) => HttpResponse::Ok().json(rows_affected),
        Err(err) => {
            eprintln!("Query execution failed: {}", err);
            HttpResponse::BadRequest().body(format!("Query execution failed: {}", err))
        }
    }
}

pub async fn start_server(database_path: String) -> std::io::Result<()> {
    let conn = DatabaseConnection::new(database_path).expect("Failed to open database");
    let db_state = web::Data::new(DatabaseState {
        conn: Mutex::new(conn),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected_route))
            .route("/schema", web::get().to(get_schema))
            .route("/query", web::post().to(execute_query))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}