use serde_qs::Config;
use serde::Deserialize;
use axum::{
    routing::post,
    body::Bytes,
    Router
};
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TestVec {
    id: Vec<String>,
    teststruct: Vec<TestStruct>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TestStruct {
    city: String,
}
/*
async fn filter_test(
    QsQuery(info): QsQuery<TestVec>
) -> impl IntoResponse {
    info.id
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ")

}
*/

async fn filter_test(body: Bytes) {
    let config = Config::new(5, false);
    println!("{:#?}",body);
    let ids: TestVec = config.deserialize_bytes(&body).unwrap();
    println!("{:#?}", ids);
}

pub async fn test_query() -> Router {
    Router::<()>::new()
        .route("/query", post(filter_test))
}
