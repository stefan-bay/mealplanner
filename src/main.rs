use std::{sync::Arc, fs};

use axum::{extract::State, http::StatusCode, response::{Html, IntoResponse}, Error, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use handlebars::{Handlebars, TemplateError};

use mealplanner::{AppContext, controllers};

pub fn register_templates(hbs: &mut Handlebars) -> Result<(), TemplateError> {
    hbs.register_template_file("recipe/list", "views/recipe/list.hbs")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut handlebars = Handlebars::new();

    handlebars.set_strict_mode(true);
    handlebars.set_dev_mode(true);

    match register_templates(&mut handlebars) {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            panic!("Could not register templates");
        }
    };

    let context = AppContext { hbs: Arc::new(handlebars) };

    let app = Router::new().merge(
        controllers::recipe::routes(context.clone())
    ).fallback(not_found).with_state(context);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5050));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service()).await.unwrap();

    Ok(())
}

async fn not_found(State(_context): State<AppContext>) -> impl IntoResponse {
    let contents = fs::read_to_string("views/404.html");
    match contents {
        Ok(html) => (StatusCode::NOT_FOUND, Html(html)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Html(String::from("Render Error")))
    }
}
