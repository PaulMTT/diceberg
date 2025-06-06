use crate::api::management::client::DiciManagementClient;
use anyhow::Context;
use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum InventoryLookupType {
    All,
    Fxf(FxfArgs),
    Iceberg(IcebergArgs),
}

#[derive(Args)]
pub struct FxfArgs {
    fxf: String,
}

#[derive(Args)]
pub struct IcebergArgs {
    location: String,
}

pub async fn handle_lookup_inventory(
    inventory_lookup_type: InventoryLookupType,
) -> anyhow::Result<()> {
    let dici_management_client = DiciManagementClient::default();
    match inventory_lookup_type {
        InventoryLookupType::All => {
            let inventories = dici_management_client.fetch_inventories().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize registrations")
        }
        InventoryLookupType::Fxf(FxfArgs { fxf }) => {
            let inventories = dici_management_client.fetch_inventory_by_fxf(fxf).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize registrations")
        }
        InventoryLookupType::Iceberg(IcebergArgs { location }) => {
            let inventories = dici_management_client
                .fetch_inventories_by_iceberg_location(location)
                .await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize registrations")
        }
    }
}
