use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{Extension, Json};
use axum::{routing, Router};

use sea_orm::{sea_query::Order, QueryOrder, DatabaseConnection, EntityTrait};

use crate::models::recipe::{AddBody, Column, Entity, Model};
use crate::views::recipe as view_recipe;

async fn list(Extension(db): Extension<DatabaseConnection>) -> Response {
    let items_res = Entity::find()
    .order_by(Column::CreatedAt, Order::Desc)
    .all(&db)
    .await;

    match items_res {
        Ok (items) => view_recipe::list(&items).into_response(),
        Err(e) => {
            println!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "DB Error").into_response()
        }
    }
}

// TODO: use Form instead of Json
async fn add(Extension(db): Extension<DatabaseConnection>, Json(body): Json<AddBody>) -> Response {
    let created = match Model::create(&db, &body).await {
        Ok(_) => true,
        Err(_) => false
    };

    if !created {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Could not add to db").into_response();
    }

    // TODO: show newly created recipe, not the list
    Redirect::to("/recipes/").into_response()
}

pub fn routes() -> Router {
    Router::new()
        .route("/recipes/", routing::get(list))
        .route("/recipes/new", routing::post(add))
}
