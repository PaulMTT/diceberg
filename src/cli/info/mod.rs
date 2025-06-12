use crate::cli::info::lookup::{handle_lookup, InfoLookupCommand};
use crate::cli::info::table::{handle_info_table, InfoTableCommand};
use anyhow::Result;
use clap::Subcommand;

pub mod lookup;
pub mod table;

#[derive(Subcommand)]
pub enum InfoCommand {
    /// Table level information
    #[clap(subcommand)]
    Table(InfoTableCommand),
    /// DICI metadata from the management API
    #[clap(subcommand)]
    Lookup(InfoLookupCommand),
}

pub async fn handle_info(info_command: InfoCommand) -> Result<()> {
    match info_command {
        InfoCommand::Table(args) => handle_info_table(args).await,
        InfoCommand::Lookup(args) => handle_lookup(args).await,
    }
}
