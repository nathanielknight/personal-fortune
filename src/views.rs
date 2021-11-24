use crate::model::{Entry, SearchResult};
use askama::Template;

#[derive(Template)]
#[template(path = "quote.html")]
struct QuoteViewModel<'a> {
    entry: &'a Entry,
}

pub(crate) fn quote(entry: &Entry) -> String {
    let e = QuoteViewModel { entry };
    e.render().expect("Template rendering error in entry")
}
