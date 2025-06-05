use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum InfoKind {
    #[clap(subcommand)]
    Schema(InfoAsset),
}

#[derive(Subcommand)]
pub enum InfoAsset {
    Core(InfoCoreArgs),
    Iceberg(InfoIcebergArgs),
}

#[derive(Args)]
pub struct InfoCoreArgs {
    pub fxf: String,
}

#[derive(Args)]
pub struct InfoIcebergArgs {
    pub location: String,

    pub schema_table: String,
}

pub async fn handle_info(kind: InfoKind) -> anyhow::Result<()> {
    match kind {
        InfoKind::Schema(asset) => handle_info_schema(asset).await?,
    }
    Ok(())
}

pub async fn handle_info_schema(asset: InfoAsset) -> anyhow::Result<()> {
    match asset {
        InfoAsset::Core(InfoCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let fields = asset.schema().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &fields)?;
        }
        InfoAsset::Iceberg(InfoIcebergArgs {
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
            serde_json::to_writer_pretty(std::io::stdout(), &fields)?;
        }
    }
    Ok(())
}
