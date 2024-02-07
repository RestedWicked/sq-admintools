use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{ Html, IntoResponse, Response },
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[allow(unused_imports)]
use tracing::{ info, debug, error, warn };
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sq-admintools=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Router!");

    let static_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .route("/", get(sq_admin))
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

async fn sq_admin() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn sq_lootbox() -> impl IntoResponse {
    let template = LootboxTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "lootbox.html")]
struct LootboxTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

