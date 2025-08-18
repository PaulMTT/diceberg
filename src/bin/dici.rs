use anyhow::Result;
use clap::Parser;
use diceberg::cli::DiciCli;
#[tokio::main]
async fn main() -> Result<()> {
    DiciCli::parse().run().await
}
