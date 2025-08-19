#[cfg(feature = "ai")]
use crate::cli::ai::handle_ai;
use crate::cli::info::{handle_info, InfoCommand};
#[cfg(feature = "mcp")]
use crate::cli::serve::{handle_serve, ServeCommand};
use crate::cli::sql::{handle_sql, SqlCommand};
use crate::cli::util::{handle_util, UtilCommand};
use anyhow::Result;
use clap::{Parser, Subcommand};
#[cfg(feature = "ai")]
pub mod ai;
pub mod info;
pub mod serve;
pub mod sql;
pub mod util;
#[derive(Parser)]
#[command(version)]
pub struct DiciCli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand, Clone)]
pub enum Commands {
    #[clap(subcommand)]
    Info(InfoCommand),
    #[clap(subcommand)]
    Sql(SqlCommand),
    #[clap(subcommand)]
    Util(UtilCommand),
    #[cfg(feature = "mcp")]
    #[clap(subcommand)]
    Serve(ServeCommand),
    #[cfg(feature = "ai")]
    Ai,
}
impl DiciCli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Info(args) => handle_info(args).await,
            Commands::Sql(args) => handle_sql(args).await,
            Commands::Util(args) => handle_util(args).await,
            #[cfg(feature = "mcp")]
            Commands::Serve(args) => handle_serve(args).await,
            #[cfg(feature = "ai")]
            Commands::Ai => handle_ai().await,
        }
    }
}
