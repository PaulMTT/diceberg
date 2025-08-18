use crate::api::store::asset::dici::DiciAsset;
use crate::api::store::asset::traits::table_source::SchemaSource;
use crate::cli::info::table::AssetArgs;
use anyhow::{Context, Result};
pub async fn handle_info_table_schema(asset_args: AssetArgs) -> Result<()> {
    let asset: DiciAsset = match asset_args {
        AssetArgs::Core(args) => args.into(),
        AssetArgs::Iceberg(args) => args.into(),
    };
    let fields = asset.schema().await?;
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
