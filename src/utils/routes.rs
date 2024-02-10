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

pub async fn sq_test() -> TestTemplate {
    TestTemplate {}
}

#[derive(Template)]
#[template(path = "test.html")]
pub struct TestTemplate;

