use crate::cli::info::table::history::all::handle_info_table_history_all;
use crate::cli::info::table::history::snapshot::handle_info_table_snapshot;
use crate::cli::info::table::AssetArgs;
use anyhow::Result;
use clap::Subcommand;
use snapshot::SnapshotCommand;

mod all;
pub mod snapshot;

#[derive(Subcommand, Clone)]
pub enum HistoryCommand {
    /// List all snapshots
    #[clap(subcommand)]
    All(AssetArgs),
    /// Get details of a specific snapshot
    #[clap(subcommand)]
    Snapshot(SnapshotCommand),
}

pub async fn handle_info_table_history(history_command: HistoryCommand) -> Result<()> {
    match history_command {
        HistoryCommand::All(args) => handle_info_table_history_all(args).await,
        HistoryCommand::Snapshot(args) => handle_info_table_snapshot(args).await,
    }
}
