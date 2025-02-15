use std::sync::Arc;
use axum::{http::StatusCode, response::{Html, IntoResponse}};
use handlebars::Handlebars;
use serde::Serialize;

pub mod controllers;
pub mod views;

pub type ViewEngine = Arc<HbsViewEngine>;

pub struct HbsViewEngine {
    hbs: Arc<Handlebars<'static>>
}

impl HbsViewEngine {
    pub fn new(hbs: Handlebars<'static>) -> Self {
        Self {
            hbs: Arc::new(hbs),
        }
    }

    pub fn render<T>(&self, template: &str, data: &T) -> impl IntoResponse
    where T: Serialize {
        self.render_with_code(template, data, StatusCode::OK)
    }

    pub fn render_with_code<T>(&self, template: &str, data: &T, code: StatusCode) -> impl IntoResponse
    where T: Serialize {
        match self.hbs.render(template, data) {
            Ok(html) => (code, Html(html)),
            Err(e) => {
                println!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(String::from("Render Error")))
            }
        }
    }

}
