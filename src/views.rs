use crate::model::{Entry, SearchResult};
use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct QuoteViewModel<'a> {
    entry: &'a Entry,
    query: &'a str,
}

pub(crate) fn quote(entry: &Entry) -> String {
    let e = QuoteViewModel { entry, query: "" };
    e.render().expect("Template rendering error in entry")
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchViewModel<'a> {
    query: &'a str,
    results: &'a [SearchResult],
}

pub(crate) fn search_results(query: &str, results: &[SearchResult]) -> String {
    let r = SearchViewModel { query, results };
    log::debug!("Rendering search results: q={}", query);
    r.render().expect("Template rendering error in search")
}
