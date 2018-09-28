use actix_web::{FromRequest, HttpRequest, HttpResponse, Path, Responder};

use model;
use view;

pub fn serve_entry(r: &HttpRequest) -> impl Responder {
    let slug = Path::<(String,)>::extract(r);
    match slug {
        Err(_) => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("No slug?".to_string()),
        Ok(s) => {
            let (slug,): (String,) = s.into_inner();
            match model::get_entry(&slug) {
                Err(_) => HttpResponse::NotFound()
                    .content_type("text/plain")
                    .body(format!("No entry '{}'", slug)),
                Ok(e) => view::Index::render_entry(&e),
            }
        }
    }
}
