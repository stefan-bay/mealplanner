use axum::{routing::get, Router};
use askama_axum::{IntoResponse, Response};

use crate::views::recipe;

async fn list() -> Response {
    recipe::list().into_response()
}

pub fn routes() -> Router {
    Router::new()
        .route("/recipes", get(list))
}
