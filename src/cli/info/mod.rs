use crate::cli::info::lookup::{handle_lookup, LookupType};
use crate::cli::info::schema::{handle_info_schema, SchemaAsset};
use clap::Subcommand;

pub mod lookup;
pub mod schema;

#[derive(Subcommand)]
pub enum InfoKind {
    #[clap(subcommand)]
    Schema(SchemaAsset),
    #[clap(subcommand)]
    Lookup(LookupType),
}

pub async fn handle_info(kind: InfoKind) -> anyhow::Result<()> {
    match kind {
        InfoKind::Schema(asset) => handle_info_schema(asset).await?,
        InfoKind::Lookup(lookup) => handle_lookup(lookup).await?,
    }
    Ok(())
}
