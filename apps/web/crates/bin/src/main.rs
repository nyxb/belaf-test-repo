//! Web binary

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use web_core::{NavItem, PageData};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Web Server v{}", env!("CARGO_PKG_VERSION"));

    let pages = vec![PageData::home(), PageData::about()];
    let nav = vec![
        NavItem::new("Home", "/"),
        NavItem::new("About", "/about"),
        NavItem::new("Contact", "/contact"),
    ];

    println!("Available pages:");
    for page in &pages {
        println!("  - {} ({})", page.title, page.path);
    }

    println!("\nNavigation:");
    for item in &nav {
        println!("  - {} -> {}", item.label, item.href);
    }

    info!("Web server ready on port 3000");
    Ok(())
}
