use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Error, Extension, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use handlebars::{Handlebars, TemplateError};

use mealplanner::{controllers, HbsViewEngine, ViewEngine};

pub fn register_templates(hbs: &mut Handlebars) -> Result<(), TemplateError> {
    // base
    hbs.register_template_file("base", "templates/base.hbs")?;

    // recipe
    hbs.register_template_file("recipe/list", "templates/recipe/list.hbs")?;

    // etc.
    hbs.register_template_file("404", "templates/404.hbs")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut hbs = Handlebars::new();

    hbs.set_strict_mode(true);
    hbs.set_dev_mode(true);

    match register_templates(&mut hbs) {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            panic!("Could not register templates");
        }
    };

    let view_engine: ViewEngine = Arc::new(HbsViewEngine::new(hbs));

    let app = Router::new().merge(
        controllers::recipe::routes()
    )
    .fallback(not_found)
    .layer(Extension(view_engine));

    // serve static files
    let app = app.nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5050));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}

async fn not_found(Extension(view_engine): Extension<ViewEngine>) -> impl IntoResponse {
    view_engine.render_with_code("404", &(), StatusCode::NOT_FOUND)
}
