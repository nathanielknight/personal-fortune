use actix_web::{HttpResponse};
use maud::{PreEscaped, DOCTYPE, html};

mod index;

pub use view::index::Index;

fn respond_html(body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(body)
}

fn in_site_template(body: String) -> String {
    let templated = html!{
        (DOCTYPE)
        head {}
        body {
            (PreEscaped(body))
        }
    };
    templated.into_string()
}

pub trait View {
    type ViewModel;
    fn render(vm: &Self::ViewModel) -> HttpResponse;
}

