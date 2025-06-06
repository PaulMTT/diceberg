use anyhow::Result;
use clap::Parser;
use diceberg::cli::DiciCli;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(DiciCli::parse().run().await?)
}
