use std::net::SocketAddr;
use sea_orm::{Database, DbErr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use axum::{Extension, Router};
use askama_axum::{IntoResponse, Response};

use mealplanner::{controllers, views};

const DB_URL: &str = "sqlite:./mealplanner.db?mode=rwc";

#[tokio::main]
async fn main() -> Result<(), DbErr> {

    let db = Database::connect(DB_URL).await?;
    println!("Connected to database: {}", DB_URL);

    let app = Router::new().merge(
        controllers::recipe::routes()
    )
    .layer(Extension(db))
    .fallback(not_found);

    // serve static files
    let app = app.nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5050));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}

async fn not_found() -> Response {
    views::not_found::show().into_response()
}
