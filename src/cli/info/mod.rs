use crate::cli::info::lookup::{handle_lookup, InfoLookupCommand};
use crate::cli::info::table::{handle_info_table, InfoTableCommand};
use clap::Subcommand;

pub mod lookup;
pub mod table;

#[derive(Subcommand)]
pub enum InfoCommand {
    #[clap(subcommand)]
    Table(InfoTableCommand),
    #[clap(subcommand)]
    Lookup(InfoLookupCommand),
}

pub async fn handle_info(kind: InfoCommand) -> anyhow::Result<()> {
    match kind {
        InfoCommand::Table(lookup) => handle_info_table(lookup).await,
        InfoCommand::Lookup(lookup) => handle_lookup(lookup).await,
    }
}
