use askama::Template;

pub async fn sq_index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn sq_lootbox() -> LootboxTemplate {
    LootboxTemplate {}
}

#[derive(Template)]
#[template(path = "lootbox.html")]
pub struct LootboxTemplate;
