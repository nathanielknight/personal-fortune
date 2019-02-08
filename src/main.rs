use pencil::method::Get;
use pencil::{abort, Pencil, PencilResult, Request, Response};
use rusqlite;

mod model;

const ADDRESS: &str = "127.0.0.1:6429";

fn in_site_template(body: &str) -> String {
    format!(
        r#"<DOCTYPE html>
<html>
<head>
  <title>Personal Fortune</title>
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
        None => return abort(404),
    };
    match model::get_entry(entry_id) {
        Ok(e) => {
            let e_str: String = e.into();
            Ok(Response::from(in_site_template(&e_str)))
        }
        Err(_) => return abort(404),
    }
}

fn main() -> Result<(), rusqlite::Error> {
    model::init_db()?;
    let mut app = Pencil::new("/");

    app.route("/", &[Get], "index", random_entry);
    app.route("/entry/<id:string>", &[Get], "entry", entry);

    println!("Serving personal-fortune on {}", ADDRESS);
    app.run(ADDRESS);
    Ok(())
}
