use std::path;

use rusqlite;

#[derive(Debug)]
pub struct Entry {
    id: u32,
    pub slug: String,
    pub content: String,
    pub source: String,
    pub link: Option<String>,
}

const DB_PATH: &str = "./fortunes.sqlite";

pub fn init_db() -> Result<(), rusqlite::Error> {
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    let init_stm = "
         CREATE TABLE IF NOT EXISTS entry(
             id INTEGER PRIMARY KEY,
             slug TEXT NOT NULL UNIQUE,
             content TEXT NOT NULL,
             source TEXT NOT NULL,
             link TEXT
         )";
    conn.execute(init_stm, &[])?;
    Ok(())
}

fn row_to_entry(row: &rusqlite::Row) -> Result<Entry, rusqlite::Error> {
    let id = row.get_checked(0)?;
    let slug = row.get_checked(1)?;
    let content = row.get_checked(2)?;
    let source = row.get_checked(3)?;
    let link: Option<String> = row.get_checked(4)?;
    Ok(Entry {
        id,
        slug,
        content,
        source,
        link,
    })
}

pub fn random_entry() -> Result<Entry, rusqlite::Error> {
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    let entry = conn.query_row(
        "SELECT * FROM entry ORDER BY RANDOM() LIMIT 1",
        &[],
        row_to_entry,
    )??;
    Ok(entry)
}

pub fn get_entry(slug: &str) -> Result<Entry, rusqlite::Error> {
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    let entry = conn.query_row("SELECT * FROM entry WHERE slug = ?", &[&slug], row_to_entry)??;
    Ok(entry)
}

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub source: String,
    pub link: Option<String>,
    pub quote: String,
    pub slug: String,
}

pub(crate) fn search(query: &str) -> Result<Vec<SearchResult>, rusqlite::Error> {
    log::debug!("Searching query={}", &query);
    const SEARCH_STMT: &str =
        "SELECT source, link, content, slug FROM entrytext WHERE entrytext MATCH ?";
    log::debug!("Connecting to DB");
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    log::debug!("Preparing search statement");
    let mut stmt = conn.prepare(SEARCH_STMT)?;
    log::debug!("Performing search");
    let search = stmt.query_map(&[&query], |row| SearchResult {
        source: row.get(0),
        link: row.get(1),
        quote: row.get(2),
        slug: row.get(3),
    })?;
    log::debug!("finished search");
    search.collect()
}
