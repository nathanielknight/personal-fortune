use axum::http::StatusCode;

use crate::AppError;

#[derive(Debug)]
pub struct Entry {
    pub slug: String,
    pub content: String,
    pub source: String,
    pub link: Option<String>,
}

fn row_to_entry(row: &rusqlite::Row) -> Result<Entry, rusqlite::Error> {
    let slug = row.get(0)?;
    let raw_content: String = row.get(1)?;
    let content = markdown::to_html(&raw_content);
    let source = row.get(2)?;
    let link: Option<String> = row.get(3)?;
    Ok(Entry {
        slug,
        content,
        source,
        link,
    })
}

pub fn random_entry(conn: &mut rusqlite::Connection) -> Result<Entry, rusqlite::Error> {
    let entry = conn.query_row(
        "SELECT slug, content, source, link FROM quotes_quote ORDER BY RANDOM() LIMIT 1",
        [],
        row_to_entry,
    )?;
    Ok(entry)
}

pub fn get_entry(
    conn: &mut rusqlite::Connection,
    slug: &str,
) -> Result<Option<Entry>, rusqlite::Error> {
    use rusqlite::OptionalExtension;
    conn.query_row(
        "SELECT slug, content, source, link FROM quotes_quote WHERE slug = ?",
        [&slug],
        row_to_entry,
    )
    .optional()
}

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub source: String,
    pub link: Option<String>,
    pub quote: String,
    pub slug: String,
}

pub(crate) fn search(
    conn: &mut rusqlite::Connection,
    query: &str,
) -> Result<Vec<SearchResult>, AppError> {
    log::debug!("Searching query={}", &query);
    const SEARCH_STMT: &str =
        "SELECT source, link, content, slug FROM quotes_quoteindex WHERE quotes_quoteindex MATCH ?";
    log::debug!("Preparing search statement");
    let mut stmt = conn.prepare(SEARCH_STMT).map_err(database_error)?;
    log::debug!("Performing search");
    let search = stmt
        .query_map([&query], |row| {
            let raw_source: String = row.get(0)?;
            let source = markdown::to_html(&raw_source);
            let raw_content: String = row.get(2)?;
            let content = markdown::to_html(&raw_content);
            Ok(SearchResult {
                source,
                link: row.get(1)?,
                quote: content,
                slug: row.get(3)?,
            })
        })
        .map_err(database_error)?;
    log::debug!("finished search");
    search
        .into_iter()
        .collect::<Result<Vec<SearchResult>, rusqlite::Error>>()
        .map_err(database_error)
}

fn database_error(e: rusqlite::Error) -> AppError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {:?}", e),
    )
}
