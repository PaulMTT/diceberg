use crate::api::dici::asset::DiciAsset;
use crate::cli::info::table::history::{HistoryCommand, handle_info_table_history};
use crate::cli::info::table::partition::handle_info_table_partition;
use crate::cli::info::table::schema::handle_info_table_schema;
use crate::cli::info::table::stats::{StatsCommand, handle_info_table_stats};
use anyhow::Result;
use clap::{Args, Subcommand};
pub mod history;
pub mod partition;
pub mod schema;
pub mod stats;
#[derive(Subcommand, Clone)]
pub enum InfoTableCommand {
    #[clap(subcommand)]
    Schema(AssetArgs),
    #[clap(subcommand)]
    Partition(AssetArgs),
    #[clap(subcommand)]
    History(HistoryCommand),
    #[clap(subcommand)]
    Stats(StatsCommand),
}
#[derive(Subcommand, Clone)]
pub enum AssetArgs {
    Core(CoreAssetArgs),
    Iceberg(IcebergAssetArgs),
}
#[derive(Args, Clone)]
pub struct CoreAssetArgs {
    pub fxf: String,
}
impl Into<DiciAsset> for CoreAssetArgs {
    fn into(self) -> DiciAsset {
        DiciAsset::core(self.fxf)
    }
}
#[derive(Args, Clone)]
pub struct IcebergAssetArgs {
    pub location: String,
    pub schema_table: String,
}
impl Into<DiciAsset> for IcebergAssetArgs {
    fn into(self) -> DiciAsset {
        DiciAsset::iceberg(self.location, self.schema_table)
    }
}
pub async fn handle_info_table(info_table_command: InfoTableCommand) -> Result<()> {
    match info_table_command {
        InfoTableCommand::Schema(args) => handle_info_table_schema(args).await,
        InfoTableCommand::Partition(args) => handle_info_table_partition(args).await,
        InfoTableCommand::History(args) => handle_info_table_history(args).await,
        InfoTableCommand::Stats(args) => handle_info_table_stats(args).await,
    }
}
