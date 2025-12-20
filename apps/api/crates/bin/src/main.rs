//! API binary

use anyhow::Result;
use api_core::{ApiResponse, HealthCheck};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("API Server v{}", env!("CARGO_PKG_VERSION"));

    let health = HealthCheck::default();
    let response = ApiResponse::success(health);

    println!("Health check response:");
    println!("{}", serde_json::to_string_pretty(&response)?);

    info!("API server ready on port 8080");
    Ok(())
}
