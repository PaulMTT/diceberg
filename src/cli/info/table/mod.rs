use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::{DiciAsset, DiciClient};
use crate::api::management::client::ManagementClient;
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
        DiciAsset::Core {
            asset: CoreAsset::builder().fxf(self.fxf).build(),
            dici_client: DiciClient::default(),
            management_client: ManagementClient::default(),
        }
    }
}
#[derive(Args, Clone)]
pub struct IcebergAssetArgs {
    pub location: String,
    pub schema_table: String,
}
impl Into<DiciAsset> for IcebergAssetArgs {
    fn into(self) -> DiciAsset {
        DiciAsset::Iceberg {
            asset: IcebergAsset::builder()
                .location(self.location)
                .schema_table(self.schema_table)
                .build(),
            client: DiciClient::default(),
        }
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
