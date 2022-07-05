use std::env;

use tide::convert::Deserialize;

mod model;
mod views;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();
    log::info!("Initializing");
    let mut conn = db_connection()?;
    model::init_db(&mut conn)?;
    let mut app = tide::new();
    app.at("").get(random_entry);
    app.at("entry/:slug").get(entry);
    app.at("search").get(search);

    log::info!("Loading configuration");
    let host = env::var("WTIIRN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "6429".to_string());

    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}

// -------------------------------------------------------------------------
// Handlers

async fn random_entry(_req: tide::Request<()>) -> AppResponse {
    let mut conn = db_connection()?;
    match model::random_entry(&mut conn) {
        Ok(entry) => Ok(render_entry(entry)),
        Err(_) => abort(
            tide::StatusCode::InternalServerError,
            "Failed to retrieve random entry.",
        ),
    }
}

async fn entry(req: tide::Request<()>) -> AppResponse {
    let slug = req.param("slug")?;
    let mut conn = db_connection()?;
    match model::get_entry(&mut conn, slug) {
        Ok(e) => Ok(render_entry(e)),
        Err(_) => abort(tide::StatusCode::NotFound, "No such entry."),
    }
}

fn render_search(query: &str, results: &[model::SearchResult]) -> AppResponse {
    let body = views::search_results(query, results);
    let resp = tide::Response::builder(200)
        .body(body.as_bytes())
        .header("content-type", "text/html; charset=UTF-8")
        .build();
    Ok(resp)
}

#[derive(Deserialize, Debug, Default)]
struct SearchQuery {
    q: Option<String>,
}

async fn search(req: tide::Request<()>) -> AppResponse {
    let searchquery: SearchQuery = req.query()?;
    let mut conn = db_connection()?;
    log::debug!("searchquery = {:?}", searchquery);
    let query = searchquery.q;
    let results = match query {
        Some(ref q) => model::search(&mut conn, q)?,
        None => Vec::new(),
    };
    render_search(&query.unwrap_or_default(), &results)
}

type AppResponse = tide::Result<tide::Response>;

// -------------------------------------------------------------------------
// Utils
const DB_PATH: &str = "./fortunes.sqlite";

fn db_connection() -> Result<rusqlite::Connection, rusqlite::Error> {
    use std::path;
    let db_path = path::Path::new(DB_PATH);
    rusqlite::Connection::open(&db_path)
}

fn render_entry(entry: model::Entry) -> tide::Response {
    tide::Response::builder(200)
        .header("content-type", "text/html; charset=UTF-8")
        .body(views::quote(&entry).as_bytes().to_vec())
        .build()
}

fn abort(status: tide::StatusCode, msg: &str) -> AppResponse {
    let resp = tide::Response::builder(status)
        .header("content-type", "text/plain; charset=UTF-8")
        .body(msg)
        .build();
    Ok(resp)
}
