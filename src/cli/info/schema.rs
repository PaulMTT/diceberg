use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum SchemaAsset {
    Core(SchemaCoreArgs),
    Iceberg(SchemaIcebergArgs),
}

#[derive(Args)]
pub struct SchemaCoreArgs {
    pub fxf: String,
}

#[derive(Args)]
pub struct SchemaIcebergArgs {
    pub location: String,

    pub schema_table: String,
}

pub async fn handle_info_schema(asset: SchemaAsset) -> anyhow::Result<()> {
    match asset {
        SchemaAsset::Core(SchemaCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let fields = asset.schema().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &fields)
                .context("failed to serialize core schema")
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
            let fields = asset.schema().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &fields)
                .context("failed to serialize iceberg schema")
        }
    }
}
