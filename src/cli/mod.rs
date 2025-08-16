use crate::cli::ai::handle_ai;
use crate::cli::info::lookup::{InfoLookupCommand, handle_lookup};
use crate::cli::info::{InfoCommand, handle_info};
use crate::cli::serve::{ServeCommand, handle_serve};
use crate::cli::sql::{SqlCommand, handle_sql};
use crate::cli::util::{UtilCommand, handle_util};
use anyhow::Result;
use clap::{Parser, Subcommand};

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
    #[clap(subcommand)]
    Serve(ServeCommand),

    Ai,
}

impl DiciCli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Info(args) => handle_info(args).await,
            Commands::Sql(args) => handle_sql(args).await,
            Commands::Util(args) => handle_util(args).await,
            Commands::Serve(args) => handle_serve(args).await,
            Commands::Ai => handle_ai().await,
        }
    }
}

#[derive(Parser)]
#[command(version)]
pub struct InfoMcpCli {
    #[command(subcommand)]
    pub command: InfoLookupCommand,
}

impl InfoMcpCli {
    pub async fn run(self) -> Result<()> {
        handle_lookup(self.command).await
    }
}
