use crate::cli::info::lookup::{handle_lookup, LookupType};
use crate::cli::info::table::{handle_info_table, TableLookupType};
use clap::Subcommand;

pub mod lookup;
pub mod table;

#[derive(Subcommand)]
pub enum InfoKind {
    #[clap(subcommand)]
    Table(TableLookupType),
    #[clap(subcommand)]
    Lookup(LookupType),
}

pub async fn handle_info(kind: InfoKind) -> anyhow::Result<()> {
    match kind {
        InfoKind::Table(lookup) => handle_info_table(lookup).await,
        InfoKind::Lookup(lookup) => handle_lookup(lookup).await,
    }
}
