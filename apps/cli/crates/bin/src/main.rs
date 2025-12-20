//! CLI binary

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("CLI v{}", env!("CARGO_PKG_VERSION"));

    let text = "hello world";
    println!("Original: {}", text);
    println!("Capitalized: {}", cli_core::capitalize(text));
    println!("Slugified: {}", cli_core::slugify("Hello World Example"));

    Ok(())
}
