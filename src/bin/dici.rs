use anyhow::Result;
use clap::Parser;
use diceberg::cli::commands::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}
