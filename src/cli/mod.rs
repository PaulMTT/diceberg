use crate::cli::info::{handle_info, InfoCommand};
use crate::cli::sql::{handle_sql, SqlArgs};
use clap::{Parser, Subcommand};

pub mod info;
pub mod sql;

/// A CLI tool to interact with and execute sql against DICI assets
#[derive(Parser)]
#[command(version)]
pub struct DiciCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Information about DICI assets
    #[clap(subcommand)]
    Info(InfoCommand),
    /// Execute sql against a DICI asset
    #[clap(subcommand)]
    Sql(SqlArgs),
}

impl DiciCli {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Info(kind) => handle_info(kind).await,
            Commands::Sql(asset) => handle_sql(asset).await,
        }
    }
}
