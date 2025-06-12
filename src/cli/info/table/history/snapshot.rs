use crate::api::client::DiciAsset;
use crate::api::traits::TableSource;
use crate::cli::info::table::{CoreAssetArgs, IcebergAssetArgs};
use anyhow::{Context, Result};
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum SnapshotCommand {
    /// Using core fxf identifier
    Core(SnapshotCoreArgs),
    /// Using iceberg identifier
    Iceberg(SnapshotIcebergArgs),
}

#[derive(Args)]
pub struct SnapshotArgs {
    /// The snapshot number
    pub snapshot: i64,
}

#[derive(Args)]
pub struct SnapshotCoreArgs {
    #[clap(flatten)]
    pub core: CoreAssetArgs,
    #[clap(flatten)]
    pub snapshot: SnapshotArgs,
}

#[derive(Args)]
pub struct SnapshotIcebergArgs {
    #[clap(flatten)]
    pub iceberg: IcebergAssetArgs,
    #[clap(flatten)]
    pub snapshot: SnapshotArgs,
}

pub async fn handle_info_table_snapshot(snapshot_command: SnapshotCommand) -> Result<()> {
    let (asset, snapshot): (DiciAsset, i64) = match snapshot_command {
        SnapshotCommand::Core(SnapshotCoreArgs {
            core,
            snapshot: SnapshotArgs { snapshot },
        }) => (core.into(), snapshot),
        SnapshotCommand::Iceberg(SnapshotIcebergArgs {
            iceberg,
            snapshot: SnapshotArgs { snapshot },
        }) => (iceberg.into(), snapshot),
    };
    let table = asset.table().await?;
    let snapshot = table
        .metadata()
        .snapshot_by_id(snapshot)
        .context("Failed to find the snapshot by id")?;
    serde_json::to_writer_pretty(std::io::stdout(), snapshot)
        .context("failed to serialize core snapshot")
}
