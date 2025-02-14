use std::sync::Arc;
use handlebars::Handlebars;

#[derive(Clone)]
pub struct AppContext {
    pub hbs: Arc<Handlebars<'static>>,
}

pub mod controllers;
pub mod views;
