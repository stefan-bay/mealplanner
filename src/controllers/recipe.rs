use axum::{extract::State, response::IntoResponse, routing::get, Router};

use crate::{AppContext, views::recipe};

async fn list(State(context): State<AppContext>) -> impl IntoResponse {
    recipe::list(&context.hbs)
}

// pub fn routes(context: &AppContext) -> Router {
pub fn routes(context: AppContext) -> Router<AppContext> {
    Router::new().route("/recipes", get(list)).with_state(context.clone())
}
