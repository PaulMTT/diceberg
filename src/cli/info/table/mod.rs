use clap::Subcommand;
use crate::cli::info::table::schema::{handle_info_schema, SchemaAsset};

pub mod schema;

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