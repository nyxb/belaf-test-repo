//! CLI application for the belaf test monorepo

use anyhow::Result;
use config::AppConfig;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let config = AppConfig::new("cli");

    info!("Starting {} v{}", config.name, config.version);

    let message = "hello world";
    let capitalized = utils::capitalize(message);
    let slugified = utils::slugify("Hello World Example");

    println!("Original: {}", message);
    println!("Capitalized: {}", capitalized);
    println!("Slugified: {}", slugified);

    info!("CLI completed successfully");
    Ok(())
}
