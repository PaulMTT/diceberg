use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::schema::{SchemaAsset, SchemaCoreArgs, SchemaIcebergArgs};
use anyhow::Context;

pub async fn handle_info_table_partition(asset: SchemaAsset) -> anyhow::Result<()> {
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
    serde_json::to_writer_pretty(std::io::stdout(), metadata.default_partition_spec())
        .context("failed to serialize partitions")
}
