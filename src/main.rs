use std::env;

use rusqlite;
use simple_server::{Request, ResponseBuilder, ResponseResult, Server};

mod model;

fn in_site_template(body: &str) -> String {
    format!(
        r#"<DOCTYPE html>
<html>
<head>
  <title>Quotations</title>
  <link rel="icon" type="image/png" href="static/favicon.png" />
  <meta charset="utf-8">
  <style>
    body {{
        max-width: 33em;
        margin: auto;
        margin-top: 2em;
        font-size: 14pt;
        font-family: 'Trebuchet MS', Verdana, sans-serif;
    }}
    a {{
        font-weight: normal;
        color: gray;
        font-style: italic;
    }}
    a:hover {{
        color: orange;
    }}
    nav p {{
        font-size: 70%;
    }}
  </style>
</head>
<body>
  {}
</body>
</html>"#,
        body
    )
}

fn abort(code: u16, mut rb: ResponseBuilder) -> ResponseResult {
    Ok(rb.status(code).body(vec![])?)
}

fn render_entry(entry: model::Entry, mut rb: ResponseBuilder) -> ResponseResult {
    let e_str: String = entry.into();
    Ok(rb
        .status(200)
        .header("content-type", "text/html; charset=UTF-8")
        .body(in_site_template(&e_str).as_bytes().to_vec())?)
}

fn random_entry(rb: ResponseBuilder) -> ResponseResult {
    match model::random_entry() {
        Ok(entry) => render_entry(entry, rb),
        Err(_) => abort(500, rb),
    }
}

fn entry(req: &Request<Vec<u8>>, rb: ResponseBuilder) -> ResponseResult {
    let path = req.uri().path();
    assert!(req.uri().path().starts_with("/entry/"));
    let entry_id = &path[7..path.len()];
    match model::get_entry(entry_id) {
        Ok(e) => render_entry(e, rb),
        Err(_) => return abort(404, rb),
    }
}

fn log_request(req: &Request<Vec<u8>>) {
    let method = req.method();
    let path = req.uri().path();
    println!("{} {}", method, path);
}

fn handler(request: Request<Vec<u8>>, responsebuilder: ResponseBuilder) -> ResponseResult {
    log_request(&request);
    let path = request.uri().path();
    if path == "/" {
        random_entry(responsebuilder)
    } else if path.starts_with("/entry/") {
        entry(&request, responsebuilder)
    } else {
        abort(404, responsebuilder)
    }
}

fn main() -> Result<(), rusqlite::Error> {
    use std::path;
    println!("Starting Personal Fortune Server");

    println!("Loading configuration");
    model::init_db()?;
    let host = env::var("WTIIRN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "6429".to_string());

    println!("Initializing server");
    let mut server = Server::new(handler);
    server.set_static_directory(path::Path::new("static"));

    println!("Serving personal-fortune on {}:{}", host, port);
    server.listen(&host, &port);
}
