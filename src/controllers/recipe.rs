use axum::{Extension, response::IntoResponse, routing::get, Router};

use crate::{views::recipe, ViewEngine};

async fn list(Extension(view_engine): Extension<ViewEngine>) -> impl IntoResponse {
    recipe::list(&view_engine)
}

pub fn routes() -> Router {
    Router::new().route("/recipes", get(list))
}
