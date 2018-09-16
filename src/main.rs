#![feature(proc_macro_non_items)]
extern crate actix_web;
extern crate maud;
extern crate rusqlite;

use actix_web::{server, App, fs};

mod model;
mod view;

const ADDRESS: &'static str = "127.0.0.1:6429";


fn main() -> Result<(), rusqlite::Error> {
    model::init_db()?;
    server::new(||
            App::new()
                .resource("/", |r| r.f(|_| view::Index::render_random()))
                .handler("/static", fs::StaticFiles::new("./static").unwrap())
        )
        .bind(ADDRESS)
        .unwrap()
        .run();
    Ok(())
}
