use crate::cli::info::schema::{handle_info_schema, SchemaAsset};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum TableLookupType {
    #[clap(subcommand)]
    Schema(SchemaAsset),
}

pub async fn handle_info_table(table_lookup_type: TableLookupType) -> anyhow::Result<()> {
    match table_lookup_type {
        TableLookupType::Schema(asset) => handle_info_schema(asset).await
    }
}