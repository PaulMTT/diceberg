use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::Context;

pub async fn handle_info_table_schema(asset_args: AssetArgs) -> anyhow::Result<()> {
    let fields = match asset_args {
        AssetArgs::Core(args) => {
            let asset: DicebergCoreAsset = args.into();
            asset.schema().await?
        }
        AssetArgs::Iceberg(args) => {
            let asset: DicebergIcebergAsset = args.into();
            asset.schema().await?
        }
    };
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
