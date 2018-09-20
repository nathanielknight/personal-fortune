#![feature(proc_macro_non_items)]
extern crate actix_web;
extern crate maud;
extern crate rusqlite;

use actix_web::{server, App};

mod model;
mod view;

const ADDRESS: &str = "127.0.0.1:6429";


fn main() -> Result<(), rusqlite::Error> {
    model::init_db()?;
    server::new(|| App::new().resource("/", |r| r.f(|_| {view::Index::render_random()})))
        .bind(ADDRESS)
        .unwrap_or_else(|_| panic!("Couldn't bind to {}", ADDRESS))
        .run();
    Ok(())
}
