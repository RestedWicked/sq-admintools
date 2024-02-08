use crate::utils::routes::*;

use anyhow::Context;
use axum::{
    routing::get,
    Router,
};
#[allow(unused_imports)]
use tracing::{ info, debug, error, warn };

use tower_http::services::ServeDir;

pub async fn app() -> anyhow::Result<()> {
    info!("Initializing Router!");

    let static_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .route("/", get(sq_index))
        .route("/lootbox", get(sq_lootbox))
        .nest_service(
            "/static",
            ServeDir::new(format!("{}/static", static_path.to_str().unwrap())),
        );
    
    info!("Now listening on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .context("Error while starting server")?;
    Ok(())
}
