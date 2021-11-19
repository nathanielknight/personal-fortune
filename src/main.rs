use anyhow;
use std::env;

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

fn render_entry(entry: model::Entry) -> tide::Response {
    let e_str: String = entry.into();
    tide::Response::builder(200)
        .header("content-type", "text/html; charset=UTF-8")
        .body(in_site_template(&e_str).as_bytes().to_vec())
        .build()
}

fn abort(status: tide::StatusCode, msg: &str) -> AppResponse {
    let resp = tide::Response::builder(status)
        .header("content-type", "text/plain; charset=UTF-8")
        .body(msg)
        .build();
    Ok(resp)
}

async fn random_entry(_req: tide::Request<()>) -> AppResponse {
    match model::random_entry() {
        Ok(entry) => Ok(render_entry(entry)),
        Err(_) => abort(
            tide::StatusCode::InternalServerError,
            "Failed to retrieve random entry.",
        ),
    }
}

async fn entry(req: tide::Request<()>) -> AppResponse {
    let slug = req.param("slug")?;
    match model::get_entry(slug) {
        Ok(e) => Ok(render_entry(e)),
        Err(_) => abort(tide::StatusCode::NotFound, "No such entry."),
    }
}

async fn search(req: tide::Request<()>) -> AppResponse {
    let query = req.param("q")?;
    let results = model::search(query)?;
    unimplemented!()
}

type AppResponse = tide::Result<tide::Response>;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("Initializing");
    model::init_db()?;
    let mut app = tide::new();
    app.at("").get(random_entry);
    app.at("entry/:slug").get(entry);
    app.at("search").get(search);

    println!("Loading configuration");
    let host = env::var("WTIIRN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "6429".to_string());

    println!("Serving personal-fortune on {}:{}", host, port);
    app.listen(format!("{}:{}", host, port)).await?;
    Ok(())
}
