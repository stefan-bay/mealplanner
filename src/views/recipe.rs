use askama_axum::Template;

#[derive(Template)]
#[template(path = "recipe/list.html")]
pub struct List {}

pub fn list() -> List {
    List {}
}
