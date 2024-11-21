use warp::Filter;
use crate::database::Database;
use crate::parser::parse_and_run_query;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: Option<Vec<Vec<String>>>,
    error: Option<String>,
}

pub async fn start_server(db: Database) {
    let db_filter = warp::any().map(move || db.clone());

    let run_query = warp::path("run_query")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_filter)
        .map(|query: String, db: Database| {
            match parse_and_run_query(&db, &query) {
                Ok(data) => warp::reply::json(&ApiResponse {
                    success: true,
                    data: Some(data),
                    error: None,
                }),
                Err(err) => warp::reply::json(&ApiResponse {
                    success: false,
                    data: None,
                    error: Some(err.to_string()),
                }),
            }
        });

    warp::serve(run_query)
        .run(([127, 0, 0, 1], 3030))
        .await;
}