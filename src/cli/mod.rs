#[cfg(feature = "ai")]
use crate::cli::ai::handle_ai;
use crate::cli::info::{InfoCommand, handle_info};
#[cfg(feature = "mcp")]
use crate::cli::serve::{ServeCommand, handle_serve};
use crate::cli::sql::{SqlCommand, handle_sql};
use crate::cli::util::{UtilCommand, handle_util};
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
