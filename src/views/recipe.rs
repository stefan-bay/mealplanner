#[allow(dead_code)]

use axum::response::IntoResponse;

use askama::Template;

use crate::{models::recipe::Model, HtmlTemplate};

#[derive(Template)]
#[template(path = "recipe/list.html")]
pub struct ListTemplate {
    recipes: Vec<RecipeDisplay>
}

pub struct RecipeDisplay {
    created_at: String,
    title: String,
}

impl RecipeDisplay {
    pub fn new(item: &Model) -> Self {
        RecipeDisplay {
            created_at: item.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            title: String::from(&item.title)
        }
    }
}

pub fn list(items: &[Model]) -> impl IntoResponse {
    let display_items: Vec<RecipeDisplay> = items.iter().map(RecipeDisplay::new).collect();

    let template = ListTemplate {
        recipes: display_items
    };

    HtmlTemplate(template)
}
