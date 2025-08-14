pub mod manifest_size;
pub mod table_size;

use crate::cli::info::table::stats::manifest_size::handle_info_table_stats_manifest_size;
use crate::cli::info::table::stats::table_size::handle_info_table_stats_data_size;
use crate::cli::info::table::AssetArgs;
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum StatsCommand {
    /// The size of the metadata files
    #[clap(subcommand)]
    ManifestSize(AssetArgs),
    /// The size of the data files
    #[clap(subcommand)]
    DataSize(AssetArgs),
}

pub async fn handle_info_table_stats(stats_command: StatsCommand) -> Result<()> {
    match stats_command {
        StatsCommand::ManifestSize(args) => handle_info_table_stats_manifest_size(args).await,
        StatsCommand::DataSize(args) => handle_info_table_stats_data_size(args).await,
    }
}
