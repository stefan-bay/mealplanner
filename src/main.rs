use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use axum::{Error, Router};
use askama_axum::{IntoResponse, Response};

use mealplanner::{controllers, views};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let app = Router::new().merge(
        controllers::recipe::routes()
    )
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
