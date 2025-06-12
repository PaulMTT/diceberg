use crate::cli::info::{handle_info, InfoCommand};
use crate::cli::sql::{handle_sql, SqlCommand};
use crate::cli::util::{handle_util, UtilCommand};
use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod info;
pub mod sql;
pub mod util;

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
    Sql(SqlCommand),
    /// Various utilities and helpers
    #[clap(subcommand)]
    Util(UtilCommand),
}

impl DiciCli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Info(args) => handle_info(args).await,
            Commands::Sql(args) => handle_sql(args).await,
            Commands::Util(args) => handle_util(args).await,
        }
    }
}
