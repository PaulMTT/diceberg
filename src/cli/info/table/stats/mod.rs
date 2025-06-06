pub mod manifest_size;
pub mod table_size;

use crate::cli::info::table::schema::SchemaAsset;
use crate::cli::info::table::stats::manifest_size::handle_info_table_stats_manifest_size;
use crate::cli::info::table::stats::table_size::handle_info_table_stats_data_size;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum StatsType {
    #[clap(subcommand)]
    ManifestSize(SchemaAsset),
    #[clap(subcommand)]
    DataSize(SchemaAsset),
}

pub async fn handle_info_table_stats(stats_type: StatsType) -> anyhow::Result<()> {
    match stats_type {
        StatsType::ManifestSize(asset) => handle_info_table_stats_manifest_size(asset).await,
        StatsType::DataSize(asset) => handle_info_table_stats_data_size(asset).await,
    }
}
