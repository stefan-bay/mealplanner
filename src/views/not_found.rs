use askama_axum::Template;

#[derive(Template)]
#[template(path = "not_found.html")]
pub struct NotFound {}

pub fn show() -> NotFound {
    NotFound {}
}
