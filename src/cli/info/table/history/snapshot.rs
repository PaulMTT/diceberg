use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::TableSource;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum SnapshotArgs {
    /// Using core fxf identifier
    Core(SnapshotCoreArgs),
    /// Using iceberg identifier
    Iceberg(SnapshotIcebergArgs),
}

#[derive(Args)]
pub struct SnapshotCoreArgs {
    /// The core four-by-four
    pub fxf: String,
    /// The snapshot number
    pub snapshot: i64,
}

#[derive(Args)]
pub struct SnapshotIcebergArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
    /// The snapshot number
    pub snapshot: i64,
}

pub async fn handle_info_table_snapshot(asset: SnapshotArgs) -> anyhow::Result<()> {
    match asset {
        SnapshotArgs::Core(SnapshotCoreArgs { fxf, snapshot }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let table = asset.table().await?;
            let snapshot = table
                .metadata()
                .snapshot_by_id(snapshot)
                .context("Failed to find the snapshot by id")?;
            serde_json::to_writer_pretty(std::io::stdout(), snapshot)
                .context("failed to serialize core snapshot")
        }
        SnapshotArgs::Iceberg(SnapshotIcebergArgs {
            location,
            schema_table,
            snapshot,
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            let table = asset.table().await?;
            let snapshot = table
                .metadata()
                .snapshot_by_id(snapshot)
                .context("Failed to find the snapshot by id")?;
            serde_json::to_writer_pretty(std::io::stdout(), snapshot)
                .context("failed to serialize iceberg snapshot")
        }
    }
}
