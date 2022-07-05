#[derive(Debug)]
pub struct Entry {
    id: u32,
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
    let id = row.get(0)?;
    let slug = row.get(1)?;
    let content = row.get(2)?;
    let source = row.get(3)?;
    let link: Option<String> = row.get(4)?;
    Ok(Entry {
        id,
        slug,
        content,
        source,
        link,
    })
}

pub fn random_entry(conn: &mut rusqlite::Connection) -> Result<Entry, rusqlite::Error> {
    let entry = conn.query_row(
        "SELECT * FROM entry ORDER BY RANDOM() LIMIT 1",
        [],
        row_to_entry,
    )?;
    Ok(entry)
}

pub fn get_entry(conn: &mut rusqlite::Connection, slug: &str) -> Result<Entry, rusqlite::Error> {
    let entry = conn.query_row("SELECT * FROM entry WHERE slug = ?", &[&slug], row_to_entry)?;
    Ok(entry)
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
) -> Result<Vec<SearchResult>, rusqlite::Error> {
    log::debug!("Searching query={}", &query);
    const SEARCH_STMT: &str =
        "SELECT source, link, content, slug FROM entrytext WHERE entrytext MATCH ?";
    log::debug!("Preparing search statement");
    let mut stmt = conn.prepare(SEARCH_STMT)?;
    log::debug!("Performing search");
    let search = stmt.query_map(&[&query], |row| {
        Ok(SearchResult {
            source: row.get(0)?,
            link: row.get(1)?,
            quote: row.get(2)?,
            slug: row.get(3)?,
        })
    })?;
    log::debug!("finished search");
    search.collect()
}

#[test]
fn smoketest_database() {
    let mut conn =
        rusqlite::Connection::open(":memory:").expect("Failed to open in-memory database");
    init_db(&mut conn).expect("Failed to initialize database");

    let entry = Entry {
        id: 0,
        slug: String::from("slug"),
        content: String::from("entry"),
        source: String::from("source"),
        link: Some(String::from("link")),
    };

    conn.execute(
        r#"INSERT INTO entry (id, slug, content, source, link) VALUES (?, ?, ?, ?, ?)"#,
        rusqlite::params![
            &entry.id,
            &entry.slug,
            &entry.content,
            &entry.source,
            &entry.link,
        ],
    )
    .expect("Failed to insert test entry");

    let retrieved = get_entry(&mut conn, "slug").expect("Failed to retrieve entry");
    assert_eq!(entry.id, retrieved.id);
    assert_eq!(entry.slug, retrieved.slug);
    assert_eq!(entry.content, retrieved.content);
    assert_eq!(entry.source, retrieved.source);
    assert_eq!(entry.link, retrieved.link);
}
