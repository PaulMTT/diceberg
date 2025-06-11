use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum AssetArgs {
    /// Using core fxf identifier
    Core(CoreAssetArgs),
    /// Using iceberg identifier
    Iceberg(IcebergAssetArgs),
}

#[derive(Args)]
pub struct CoreAssetArgs {
    /// The core four-by-four
    pub fxf: String,
}

impl Into<DicebergCoreAsset> for CoreAssetArgs {
    fn into(self) -> DicebergCoreAsset {
        DicebergClient::default().core(CoreAsset::builder().fxf(self.fxf).build())
    }
}

#[derive(Args)]
pub struct IcebergAssetArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
}

impl Into<DicebergIcebergAsset> for IcebergAssetArgs {
    fn into(self) -> DicebergIcebergAsset {
        DicebergClient::default().iceberg(
            IcebergAsset::builder()
                .location(self.location)
                .schema_table(self.schema_table)
                .build(),
        )
    }
}

pub async fn handle_info_table_schema(asset: AssetArgs) -> anyhow::Result<()> {
    let fields = match asset {
        AssetArgs::Core(args) => {
            let asset: DicebergCoreAsset = args.into();
            asset.schema().await?
        }
        AssetArgs::Iceberg(args) => {
            let asset: DicebergIcebergAsset = args.into();
            asset.schema().await?
        }
    };
    serde_json::to_writer_pretty(std::io::stdout(), &fields).context("failed to serialize schema")
}
