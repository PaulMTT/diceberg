use anyhow::Result;
use clap::Parser;
use diceberg::cli::commands::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(Cli::parse().run().await?)
}
