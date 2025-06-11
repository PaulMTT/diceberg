use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::cli::info::table::history::{handle_info_table_history, HistoryCommand};
use crate::cli::info::table::partition::handle_info_table_partition;
use crate::cli::info::table::schema::handle_info_table_schema;
use crate::cli::info::table::stats::{handle_info_table_stats, StatsCommand};
use clap::{Args, Subcommand};

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

#[derive(Subcommand)]
pub enum AssetArgs {
    /// Using core fxf identifier
    Core(CoreAssetArgs),
    /// Using iceberg identifier
    Iceberg(IcebergAssetArgs),
}

#[derive(Args)]
pub struct CoreAssetArgs {
    /// The core four-by-four
    pub fxf: String,
}

impl Into<DicebergCoreAsset> for CoreAssetArgs {
    fn into(self) -> DicebergCoreAsset {
        DicebergClient::default().core(CoreAsset::builder().fxf(self.fxf).build())
    }
}

#[derive(Args)]
pub struct IcebergAssetArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
}

impl Into<DicebergIcebergAsset> for IcebergAssetArgs {
    fn into(self) -> DicebergIcebergAsset {
        DicebergClient::default().iceberg(
            IcebergAsset::builder()
                .location(self.location)
                .schema_table(self.schema_table)
                .build(),
        )
    }
}

pub async fn handle_info_table(info_table_command: InfoTableCommand) -> anyhow::Result<()> {
    match info_table_command {
        InfoTableCommand::Schema(args) => handle_info_table_schema(args).await,
        InfoTableCommand::Partition(args) => handle_info_table_partition(args).await,
        InfoTableCommand::History(args) => handle_info_table_history(args).await,
        InfoTableCommand::Stats(args) => handle_info_table_stats(args).await,
    }
}
