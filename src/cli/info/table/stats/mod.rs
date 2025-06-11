pub mod manifest_size;
pub mod table_size;

use crate::cli::info::table::schema::AssetArgs;
use crate::cli::info::table::stats::manifest_size::handle_info_table_stats_manifest_size;
use crate::cli::info::table::stats::table_size::handle_info_table_stats_data_size;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum StatsCommand {
    /// The size of the metadata files
    #[clap(subcommand)]
    ManifestSize(AssetArgs),
    /// The size of the data files
    #[clap(subcommand)]
    DataSize(AssetArgs),
}

pub async fn handle_info_table_stats(stats_type: StatsCommand) -> anyhow::Result<()> {
    match stats_type {
        StatsCommand::ManifestSize(asset) => handle_info_table_stats_manifest_size(asset).await,
        StatsCommand::DataSize(asset) => handle_info_table_stats_data_size(asset).await,
    }
}
