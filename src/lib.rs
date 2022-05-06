//! Library module

use logging::Logging;

pub mod logging;

pub async fn run() -> anyhow::Result<()> {
    // Load environment variables from .env
    dotenv::dotenv().ok();

    // Initialize logging
    Logging::from_env().init()?;

    log::info!("Hello, Earth!");

    Ok(())
}
