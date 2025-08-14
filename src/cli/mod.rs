use crate::cli::ai::handle_ai;
use crate::cli::info::lookup::{handle_lookup, InfoLookupCommand};
use crate::cli::info::{handle_info, InfoCommand};
use crate::cli::serve::{handle_serve, ServeCommand};
use crate::cli::sql::{handle_sql, SqlCommand};
use crate::cli::util::{handle_util, UtilCommand};
use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod ai;
pub mod info;
pub mod serve;
pub mod sql;
pub mod util;

/// A CLI tool to interact with and execute sql against DICI assets
#[derive(Parser)]
#[command(version)]
pub struct DiciCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
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
    #[clap(subcommand)]
    /// Serve
    Serve(ServeCommand),
    /// The AI repl
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
