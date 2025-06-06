use crate::cli::info::table::history::all::handle_info_table_history_all;
use crate::cli::info::table::history::snapshot::handle_info_table_snapshot;
use crate::cli::info::table::schema::SchemaAsset;
use clap::Subcommand;
use snapshot::SnapshotAsset;

mod all;
pub mod snapshot;

#[derive(Subcommand)]
pub enum HistoryType {
    #[clap(subcommand)]
    All(SchemaAsset),
    #[clap(subcommand)]
    Snapshot(SnapshotAsset),
}

pub async fn handle_info_table_history(history: HistoryType) -> anyhow::Result<()> {
    match history {
        HistoryType::All(asset) => handle_info_table_history_all(asset).await,
        HistoryType::Snapshot(snapshot) => handle_info_table_snapshot(snapshot).await,
    }
}
