use crate::cli::info::lookup::{InfoLookupCommand, handle_lookup};
use crate::cli::info::table::{InfoTableCommand, handle_info_table};
use anyhow::Result;
use clap::Subcommand;

pub mod lookup;
pub mod table;

#[derive(Subcommand, Clone)]
pub enum InfoCommand {
    #[clap(subcommand)]
    Table(InfoTableCommand),

    #[clap(subcommand)]
    Lookup(InfoLookupCommand),
}

pub async fn handle_info(info_command: InfoCommand) -> Result<()> {
    match info_command {
        InfoCommand::Table(args) => handle_info_table(args).await,
        InfoCommand::Lookup(args) => handle_lookup(args).await,
    }
}
