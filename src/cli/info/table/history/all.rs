use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::schema::{SchemaArgs, SchemaCoreArgs, SchemaIcebergArgs};
use anyhow::Context;

pub async fn handle_info_table_history_all(asset: SchemaArgs) -> anyhow::Result<()> {
    let table = match asset {
        SchemaArgs::Core(SchemaCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            asset.table().await?
        }
        SchemaArgs::Iceberg(SchemaIcebergArgs {
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
    serde_json::to_writer_pretty(std::io::stdout(), table.metadata().history())
        .context("failed to serialize table history")
}
