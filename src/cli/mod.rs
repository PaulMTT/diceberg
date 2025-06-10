use crate::cli::info::{handle_info, InfoCommand};
use crate::cli::sql::{handle_sql, SqlArgs};
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
    Info(InfoCommand),
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
