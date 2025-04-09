use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::Html;

use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;

mod model;
mod views;

pub(crate) type AppError = (StatusCode, String);

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    log::info!("Loading configuration");
    let host = env::var("PERSONAL_FORTUNE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PERSONAL_FORTUNE_PORT").unwrap_or_else(|_| "6429".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid HOST and PORT format");

    let app = newapp();
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    Ok(())
}

fn newapp() -> axum::Router {
    use axum::routing::get;

    axum::Router::new()
        .route("/", get(random_entry))
        .route("/entry/:slug", get(entry))
        .route("/search", get(search))
}

// -------------------------------------------------------------------------
// Handlers

async fn random_entry() -> Result<Html<String>, AppError> {
    let mut conn = db_connection()?;
    let entry = model::random_entry(&mut conn);
    match entry {
        Ok(entry) => Ok(render_entry(entry)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to retrieve random entry.".to_owned(),
        )),
    }
}

async fn entry(Path(slug): Path<String>) -> Result<Html<String>, AppError> {
    let mut conn = db_connection()?;
    match model::get_entry(&mut conn, &slug) {
        Ok(Some(e)) => Ok(render_entry(e)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "No such entry.".to_owned())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {:?}", e))),
    }
}

fn render_search(query: &str, results: &[model::SearchResult]) -> Result<Html<String>, AppError> {
    let body = views::search_results(query, results);
    Ok(Html(body))
}

async fn search(
    Query(queryparams): Query<HashMap<String, String>>,
) -> Result<Html<String>, AppError> {
    let mut conn = db_connection()?;
    let q = queryparams.get("q").unwrap_or(&String::from("")).to_owned();
    log::debug!("searchquery = {:?}", q);
    let results = model::search(&mut conn, &q)?;
    render_search(&q, &results)
}

// -------------------------------------------------------------------------
// Utils

fn db_connection() -> Result<rusqlite::Connection, AppError> {
    let db_path_str = env::var("PERSONAL_FORTUNE_DB_PATH").map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database path not set".to_owned(),
        )
    })?;
    use std::path;
    let db_path = path::Path::new(&db_path_str);
    rusqlite::Connection::open(db_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't connect to database: {:?}", e),
        )
    })
}

fn render_entry(entry: model::Entry) -> Html<String> {
    let body = views::quote(&entry);
    Html(body)
}
