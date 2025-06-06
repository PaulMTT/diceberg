use crate::cli::info::table::history::{handle_info_table_history, HistoryType};
use crate::cli::info::table::partition::handle_info_table_partition;
use crate::cli::info::table::schema::{handle_info_table_schema, SchemaAsset};
use crate::cli::info::table::stats::{handle_info_table_stats, StatsType};
use clap::Subcommand;

pub mod history;
pub mod partition;
pub mod schema;
pub mod stats;

#[derive(Subcommand)]
pub enum TableLookupType {
    #[clap(subcommand)]
    Schema(SchemaAsset),
    #[clap(subcommand)]
    Partition(SchemaAsset),
    #[clap(subcommand)]
    History(HistoryType),
    #[clap(subcommand)]
    Stats(StatsType),
}

pub async fn handle_info_table(table_lookup_type: TableLookupType) -> anyhow::Result<()> {
    match table_lookup_type {
        TableLookupType::Schema(asset) => handle_info_table_schema(asset).await,
        TableLookupType::Partition(asset) => handle_info_table_partition(asset).await,
        TableLookupType::History(history) => handle_info_table_history(history).await,
        TableLookupType::Stats(stats) => handle_info_table_stats(stats).await,
    }
}
