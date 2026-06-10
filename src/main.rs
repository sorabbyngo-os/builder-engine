//! builder-engine — Sorabbyngo OS image build system

mod manifest;
mod rootfs;
mod iso;
mod ota;
mod signing;
mod cache;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("builder-engine starting");
    Ok(())
}
