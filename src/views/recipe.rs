use axum::{http::StatusCode, response::{Html, IntoResponse}};
use handlebars::Handlebars;

pub fn list(hbs: &Handlebars) -> impl IntoResponse {
    let resp = hbs.render("recipe/list", &());
    if resp.is_err() {
        println!("{:?}", resp)
    }
    match resp {
        Ok(html) => (StatusCode::OK, Html(html)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Html(String::from("Render Error")))
    }
}
