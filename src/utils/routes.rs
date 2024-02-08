use crate::utils::template::HtmlTemplate;

use askama::Template;
use axum::response::IntoResponse;

pub async fn sq_index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn sq_lootbox() -> impl IntoResponse {
    let template = LootboxTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "lootbox.html")]
struct LootboxTemplate;
