use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum SchemaArgs {
    /// Using core fxf identifier
    Core(SchemaCoreArgs),
    /// Using iceberg identifier
    Iceberg(SchemaIcebergArgs),
}

#[derive(Args)]
pub struct SchemaCoreArgs {
    /// The core four-by-four
    pub fxf: String,
}

#[derive(Args)]
pub struct SchemaIcebergArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
}

pub async fn handle_info_table_schema(asset: SchemaArgs) -> anyhow::Result<()> {
    let fields = match asset {
        SchemaArgs::Core(SchemaCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            asset.schema().await?
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
            asset.schema().await?
        }
    };
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
