use crate::cli::info::{handle_info, InfoKind};
use crate::cli::sql::{handle_sql, SqlAsset};
use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Info(kind) => handle_info(kind).await?,
            Commands::Sql(asset) => handle_sql(asset).await?,
        }
        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    Info(InfoKind),
    #[clap(subcommand)]
    Sql(SqlAsset),
}
