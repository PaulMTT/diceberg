use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::schema::{SchemaAsset, SchemaCoreArgs, SchemaIcebergArgs};
use anyhow::Context;
use serde_json::json;

pub async fn handle_info_table_stats_data_size(asset: SchemaAsset) -> anyhow::Result<()> {
    let table = match asset {
        SchemaAsset::Core(SchemaCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            asset.table().await?
        }
        SchemaAsset::Iceberg(SchemaIcebergArgs {
            location,
            schema_table,
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            asset.table().await?
        }
    };
    let metadata = table.metadata();

    let mut total_size: u64 = 0;

    for snapshot in metadata.snapshots() {
        let manifest_files = snapshot
            .load_manifest_list(table.file_io(), metadata)
            .await?;
        for manifest_file in manifest_files.consume_entries() {
            let manifest = manifest_file.load_manifest(table.file_io()).await?;
            for entry in manifest.entries() {
                total_size += entry.file_size_in_bytes();
            }
        }
    }
    let output = json!({ "data_size_bytes": total_size });
    serde_json::to_writer_pretty(std::io::stdout(), &output)
        .context("failed to serialize core schema")
}
