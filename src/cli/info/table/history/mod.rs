use crate::cli::info::table::history::all::handle_info_table_history_all;
use crate::cli::info::table::history::snapshot::handle_info_table_snapshot;
use crate::cli::info::table::schema::AssetArgs;
use clap::Subcommand;
use snapshot::SnapshotCommand;

mod all;
pub mod snapshot;

#[derive(Subcommand)]
pub enum HistoryCommand {
    /// List all snapshots
    #[clap(subcommand)]
    All(AssetArgs),
    /// Get details of a specific snapshot
    #[clap(subcommand)]
    Snapshot(SnapshotCommand),
}

pub async fn handle_info_table_history(history: HistoryCommand) -> anyhow::Result<()> {
    match history {
        HistoryCommand::All(asset) => handle_info_table_history_all(asset).await,
        HistoryCommand::Snapshot(snapshot) => handle_info_table_snapshot(snapshot).await,
    }
}
