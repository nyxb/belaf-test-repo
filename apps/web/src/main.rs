//! Web application for the belaf test monorepo

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
        name: "web".to_string(),
        port: 3000,
        ..Default::default()
    };

    info!("Starting {} v{} on port {}", config.name, config.version, config.port);

    let title = utils::capitalize("welcome to the web app");
    let slug = utils::slugify(&title);

    println!("Page title: {}", title);
    println!("URL slug: {}", slug);

    info!("Web server ready");
    Ok(())
}
