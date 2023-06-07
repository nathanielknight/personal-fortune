use axum::http::StatusCode;

use crate::AppError;

#[derive(Debug)]
pub struct Entry {
    pub slug: String,
    pub content: String,
    pub source: String,
    pub link: Option<String>,
}

pub fn init_db(conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    let init_stm = "
         CREATE TABLE IF NOT EXISTS entry(
             id INTEGER PRIMARY KEY,
             slug TEXT NOT NULL UNIQUE,
             content TEXT NOT NULL,
             source TEXT NOT NULL,
             link TEXT
         )";
    conn.execute(init_stm, [])?;
    Ok(())
}

fn row_to_entry(row: &rusqlite::Row) -> Result<Entry, rusqlite::Error> {
    let slug = row.get(0)?;
    let content = row.get(1)?;
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
        "SELECT slug, content, source, link FROM entry ORDER BY RANDOM() LIMIT 1",
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
        "SELECT slug, content, source, link FROM entry WHERE slug = ?",
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
        "SELECT source, link, content, slug FROM entrytext WHERE entrytext MATCH ?";
    log::debug!("Preparing search statement");
    let mut stmt = conn.prepare(SEARCH_STMT).map_err(database_error)?;
    log::debug!("Performing search");
    let search = stmt
        .query_map([&query], |row| {
            Ok(SearchResult {
                source: row.get(0)?,
                link: row.get(1)?,
                quote: row.get(2)?,
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

#[test]
fn smoketest_database() {
    let mut conn =
        rusqlite::Connection::open(":memory:").expect("Failed to open in-memory database");
    init_db(&mut conn).expect("Failed to initialize database");

    let entry = Entry {
        slug: String::from("slug"),
        content: String::from("entry"),
        source: String::from("source"),
        link: Some(String::from("link")),
    };

    conn.execute(
        r#"INSERT INTO entry (slug, content, source, link) VALUES (?, ?, ?, ?)"#,
        rusqlite::params![&entry.slug, &entry.content, &entry.source, &entry.link,],
    )
    .expect("Failed to insert test entry");

    let maybe_retrieved = get_entry(&mut conn, "slug").expect("Failed to retrieve entry");
    if let Some(retrieved) = maybe_retrieved {
        assert_eq!(entry.slug, retrieved.slug);
        assert_eq!(entry.content, retrieved.content);
        assert_eq!(entry.source, retrieved.source);
        assert_eq!(entry.link, retrieved.link);
    } else {
        panic!("Expected to retrieve an entry");
    }
}

fn database_error(e: rusqlite::Error) -> AppError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Database error: {:?}", e),
    )
}
