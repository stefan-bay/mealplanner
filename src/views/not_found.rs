use askama::Template;
use axum::response::IntoResponse;

use crate::HtmlTemplate;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFound {}

pub fn show() -> impl IntoResponse {
    let t = NotFound {};
    HtmlTemplate(t)
}
