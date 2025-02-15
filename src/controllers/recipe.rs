use axum::http::StatusCode;
use axum::Extension;
use axum::{routing::get, Router};
use askama_axum::{IntoResponse, Response};

use sea_orm::{sea_query::Order, QueryOrder, DatabaseConnection, EntityTrait};

use crate::models::recipe::{Entity, Column};
use crate::views::recipe as view_recipe;

async fn list(Extension(db): Extension<DatabaseConnection>) -> Response {
    let item = Entity::find()
    .order_by(Column::Id, Order::Desc)
    .all(&db)
    .await;

    match item {
        Ok (models) => {
            println!("Found models! {:?}", models);
            view_recipe::list().into_response()
        },
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }

}

pub fn routes() -> Router {
    Router::new()
        .route("/recipes", get(list))
}
