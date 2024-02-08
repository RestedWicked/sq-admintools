mod utils;

use crate::utils::tracing::sq_trace;
use crate::utils::app::app;

#[allow(unused_imports)]
use tracing::{ info, debug, error, warn };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    sq_trace();
    app().await
}

