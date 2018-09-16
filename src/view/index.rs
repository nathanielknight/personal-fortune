pub struct Index {}

use view::{in_site_template, respond_html, View};
use model;
use actix_web::HttpResponse;
use maud::html;


impl View for Index {
    type ViewModel = model::Entry;
    fn render(vm: &Self::ViewModel) -> HttpResponse {
        let body = html!{
            blockquote {
                (vm.content)
            }
            p {cite { (vm.source) }}
            @if let Some(url) = &vm.link {
                p {a href=(url) { "link" }}
            }

        };
        respond_html(in_site_template(body.into_string()))
    }
}

impl Index {
    pub fn render_random() -> Option<HttpResponse> {
        let entry = model::random_entry();
        entry.ok().map(|e| Index::render(&e))
    }
}
