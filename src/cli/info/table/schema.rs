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

impl Into<DicebergCoreAsset> for SchemaCoreArgs {
    fn into(self) -> DicebergCoreAsset {
        DicebergClient::default().core(CoreAsset::builder().fxf(self.fxf).build())
    }
}

#[derive(Args)]
pub struct SchemaIcebergArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
}

impl Into<DicebergIcebergAsset> for SchemaIcebergArgs {
    fn into(self) -> DicebergIcebergAsset {
        DicebergClient::default().iceberg(
            IcebergAsset::builder()
                .location(self.location)
                .schema_table(self.schema_table)
                .build(),
        )
    }
}

pub async fn handle_info_table_schema(asset: SchemaArgs) -> anyhow::Result<()> {
    let fields = match asset {
        SchemaArgs::Core(args) => {
            let asset: DicebergCoreAsset = args.into();
            asset.schema().await?
        }
        SchemaArgs::Iceberg(args) => {
            let asset: DicebergIcebergAsset = args.into();
            asset.schema().await?
        }
    };
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
