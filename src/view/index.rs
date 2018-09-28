pub struct Index {}

use actix_web::HttpResponse;
use maud::html;
use model;
use view::{in_site_template, respond_html, View};

impl View for Index {
    type ViewModel = model::Entry;
    fn render(vm: &Self::ViewModel) -> HttpResponse {
        let slug_url = format!("/entry/{}", vm.slug);
        let body = html!{
            blockquote {
                (vm.content)
            }
            p {cite { (vm.source) }}
            @if let Some(url) = &vm.link {
                p {a href=(url) { "↪ link" }}
            }
            p {a href=(slug_url) { "♾ permalink" } }
        };
        respond_html(in_site_template(body.into_string()))
    }
}

impl Index {
    pub fn render_random() -> Option<HttpResponse> {
        let entry = model::random_entry();
        entry.ok().map(|e| Index::render(&e))
    }

    pub fn render_entry(entry: &model::Entry) -> HttpResponse {
        Index::render(entry)
    }
}
