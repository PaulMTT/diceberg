use crate::cli::info::table::history::{handle_info_table_history, HistoryCommand};
use crate::cli::info::table::partition::handle_info_table_partition;
use crate::cli::info::table::schema::{handle_info_table_schema, AssetArgs};
use crate::cli::info::table::stats::{handle_info_table_stats, StatsCommand};
use clap::Subcommand;

pub mod history;
pub mod partition;
pub mod schema;
pub mod stats;

#[derive(Subcommand)]
pub enum InfoTableCommand {
    /// The schema of the table
    #[clap(subcommand)]
    Schema(AssetArgs),
    /// The partitions used in the table
    #[clap(subcommand)]
    Partition(AssetArgs),
    /// The table history
    #[clap(subcommand)]
    History(HistoryCommand),
    /// Table statistics
    #[clap(subcommand)]
    Stats(StatsCommand),
}

pub async fn handle_info_table(table_lookup_type: InfoTableCommand) -> anyhow::Result<()> {
    match table_lookup_type {
        InfoTableCommand::Schema(asset) => handle_info_table_schema(asset).await,
        InfoTableCommand::Partition(asset) => handle_info_table_partition(asset).await,
        InfoTableCommand::History(history) => handle_info_table_history(history).await,
        InfoTableCommand::Stats(stats) => handle_info_table_stats(stats).await,
    }
}
