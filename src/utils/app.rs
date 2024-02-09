use crate::utils::routes::*;
use crate::utils::settings::Settings;
use crate::api::lootbox_gen::gen_lootbox;


use anyhow::Context;
#[allow(unused_imports)]
use axum::{
    routing::{
        get,
        post,
    },
    Router,
};
use lazy_static::lazy_static;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[allow(unused_imports)]
use tracing::{ info, debug, error, warn };


lazy_static! {
    static ref SETTINGS: Settings = match Settings::new() {
        Some(s) => s,
        _ => {
            warn!("Failed to parse settings, defaults will be used instead");
            Settings::from_str("").unwrap()
        }
    };
}

async fn api() -> Router {
    Router::new()
        .route("/gen_lootbox", post(gen_lootbox))
}
/*
async fn gen_lootbo() {
    info!("Lootbo!")
}
*/
pub async fn app() -> anyhow::Result<()> {
    info!("Initializing Router!");

    let static_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .nest("/api", api().await)
        // Routes
        .route("/", get(sq_index))
        .route("/lootbox", get(sq_lootbox))
        // File Server
        .nest_service(
            "/static",
            ServeDir::new(format!("{}/static", static_path.to_str().unwrap())),
        );

    let address: SocketAddr = format!("{}:{}", SETTINGS.ip, SETTINGS.port)
        .parse()
        .unwrap();

    info!("Now listening on http://{}", address);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .context("Error while starting server")?;
    Ok(())
}
