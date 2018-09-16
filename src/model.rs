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

const DB_PATH: &'static str = "./fortunes.sqlite";

pub fn init_db() -> Result<(), rusqlite::Error> {
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    let init_stm =  "
         CREATE TABLE IF NOT EXISTS entry(
             id INTEGER PRIMARY KEY,
             slug TEXT NOT NULL,
             content TEXT NOT NULL,
             source TEXT NOT NULL,
             link TEXT
         )";
    conn.execute(&init_stm, &[])?;
    Ok(())
}

pub fn random_entry() -> Result<Entry, rusqlite::Error> {
    let db_path = path::Path::new(DB_PATH);
    let conn = rusqlite::Connection::open(&db_path)?;
    fn row_to_entry(row: &rusqlite::Row) -> Result<Entry, rusqlite::Error> {
        let id = row.get_checked(0)?;
        let slug = row.get_checked(1)?;
        let content = row.get_checked(2)?;
        let source = row.get_checked(3)?;
        let link: Option<String> = row.get_checked(4)?;
        Ok(Entry{
            id: id,
            slug: slug,
            content: content,
            source: source,
            link: link,
        })
    }
    let entry = conn.query_row("SELECT * FROM entry ORDER BY RANDOM() LIMIT 1", &[], row_to_entry)??;
    Ok(entry)
}
