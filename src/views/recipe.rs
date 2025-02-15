use axum::response::IntoResponse;

use crate::ViewEngine;

pub fn list(view_engine: &ViewEngine) -> impl IntoResponse {
    view_engine.render("recipe/list", &())
}
