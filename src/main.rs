mod utils;
mod api;

use crate::utils::tracing::sq_trace;
use crate::utils::app::app;

#[allow(unused_imports)]
use tracing::{ info, debug, error, warn };

extern crate lazy_static;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sq_trace();
    app().await
}

