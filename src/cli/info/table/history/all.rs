use crate::api::client::DiciAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::{Context, Result};
pub async fn handle_info_table_history_all(asset_args: AssetArgs) -> Result<()> {
    let asset: DiciAsset = match asset_args {
        AssetArgs::Core(core) => core.into(),
        AssetArgs::Iceberg(iceberg) => iceberg.into(),
    };
    let table = asset.table().await?;
    serde_json::to_writer_pretty(std::io::stdout(), table.metadata().history())
        .context("failed to serialize table history")
}
