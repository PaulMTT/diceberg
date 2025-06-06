use crate::cli::info::{handle_info, InfoKind};
use crate::cli::sql::{handle_sql, SqlAsset};
use clap::{Parser, Subcommand};

pub mod info;
pub mod sql;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct DiciCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(subcommand)]
    Info(InfoKind),
    #[clap(subcommand)]
    Sql(SqlAsset),
}

impl DiciCli {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Info(kind) => handle_info(kind).await?,
            Commands::Sql(asset) => handle_sql(asset).await?,
        }
        Ok(())
    }
}
