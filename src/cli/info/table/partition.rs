use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::Context;

pub async fn handle_info_table_partition(asset_args: AssetArgs) -> anyhow::Result<()> {
    let table = match asset_args {
        AssetArgs::Core(args) => {
            let asset: DicebergCoreAsset = args.into();
            asset.table().await?
        }
        AssetArgs::Iceberg(args) => {
            let asset: DicebergIcebergAsset = args.into();
            asset.table().await?
        }
    };

    let metadata = table.metadata();
    serde_json::to_writer_pretty(std::io::stdout(), metadata.default_partition_spec())
        .context("failed to serialize partitions")
}
