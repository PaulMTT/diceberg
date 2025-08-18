use crate::api::store::asset::dici::DiciAsset;
use crate::api::store::asset::traits::table_source::TableSource;
use crate::cli::info::table::AssetArgs;
use anyhow::{Context, Result};
use serde_json::json;
pub async fn handle_info_table_stats_manifest_size(asset_args: AssetArgs) -> Result<()> {
    let asset: DiciAsset = match asset_args {
        AssetArgs::Core(args) => args.into(),
        AssetArgs::Iceberg(args) => args.into(),
    };
    let table = asset.table().await?;
    let metadata = table.metadata();
    let mut total_size: u64 = 0;
    for snapshot in metadata.snapshots() {
        let manifest_files = snapshot
            .load_manifest_list(table.file_io(), metadata)
            .await?;
        for manifest_file in manifest_files.consume_entries() {
            total_size += manifest_file.manifest_length as u64;
        }
    }
    let output = json!({ "manifest_size_bytes": total_size });
    serde_json::to_writer_pretty(std::io::stdout(), &output)
        .context("failed to serialize manifest_size_bytes")
}
