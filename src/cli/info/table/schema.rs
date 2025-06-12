use crate::api::client::DiciAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::Context;

pub async fn handle_info_table_schema(asset_args: AssetArgs) -> anyhow::Result<()> {
    let asset: DiciAsset = match asset_args {
        AssetArgs::Core(args) => args.into(),
        AssetArgs::Iceberg(args) => args.into(),
    };
    let fields = asset.schema().await?;
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
