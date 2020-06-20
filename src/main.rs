use pencil::helpers;
use pencil::method::Get;
use pencil::{abort, Pencil, PencilResult, Request, Response};
use rusqlite;

mod model;

const ADDRESS: &str = "0.0.0.0:6429";

fn in_site_template(body: &str) -> String {
    format!(
        r#"<DOCTYPE html>
<html>
<head>
  <title>Quotations</title>
  <link rel="icon" type="image/png" href="static/favicon.png" />
  <link rel="stylesheet" href="/static/style.css" />
  <meta charset="utf-8">
</head>
<body>
  {}
</body>
</html>"#,
        body
    )
}

fn render_entry(entry: Result<model::Entry, rusqlite::Error>) -> PencilResult {
    match entry {
        Ok(e) => {
            let e_str: String = e.into();
            Ok(Response::from(in_site_template(&e_str)))
        }
        Err(_) => abort(500),
    }
}

fn random_entry(_: &mut Request) -> PencilResult {
    render_entry(model::random_entry())
}

fn entry(req: &mut Request) -> PencilResult {
    let entry_id = match req.view_args.get("id") {
        Some(id) => id,
        None => return abort(400),
    };
    match model::get_entry(entry_id) {
        Ok(e) => {
            let e_str: String = e.into();
            Ok(Response::from(in_site_template(&e_str)))
        }
        Err(_) => return abort(404),
    }
}

fn serve_static(req: &mut Request) -> PencilResult {
    let fname = match req.view_args.get("fname") {
        Some(s) => s,
        None => return abort(400),
    };
    let mut response = helpers::send_from_directory("static", fname, false)?;
    response.headers.set_raw(
        "cache-control",
        vec![b"max-age=3600".to_vec(), b"public".to_vec()],
    );
    Ok(response)

}

fn main() -> Result<(), rusqlite::Error> {
    model::init_db()?;
    let mut app = Pencil::new("/");

    app.route("/", &[Get], "index", random_entry);
    app.route("/entry/<id:string>", &[Get], "entry", entry);
    app.route("/static/<fname:path>", &[Get], "static", serve_static);

    println!("Serving personal-fortune on {}", ADDRESS);
    app.run(ADDRESS);
    Ok(())
}
