//! API server for the belaf test monorepo

use anyhow::Result;
use config::AppConfig;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = AppConfig {
        name: "api".to_string(),
        port: 8080,
        ..Default::default()
    };

    info!("Starting {} v{} on port {}", config.name, config.version, config.port);

    // Simulate API startup
    println!("API server configuration:");
    println!("{}", config.to_json()?);

    info!("API server ready");
    Ok(())
}
