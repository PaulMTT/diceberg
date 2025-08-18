use crate::api::client::DiciAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::{Context, Result};
pub async fn handle_info_table_partition(asset_args: AssetArgs) -> Result<()> {
    let asset: DiciAsset = match asset_args {
        AssetArgs::Core(args) => args.into(),
        AssetArgs::Iceberg(args) => args.into(),
    };
    let table = asset.table().await?;
    let metadata = table.metadata();
    serde_json::to_writer_pretty(std::io::stdout(), metadata.default_partition_spec())
        .context("failed to serialize partitions")
}
