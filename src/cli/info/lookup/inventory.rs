use crate::api::management::client::ManagementClient;
use anyhow::Context;
use clap::{Args, Subcommand};
#[derive(Subcommand, Clone)]
pub enum InventoryLookupCommand {
    All,
    Fxf(FxfArgs),
    Iceberg(IcebergArgs),
}
#[derive(Args, Clone)]
pub struct FxfArgs {
    fxf: String,
}
#[derive(Args, Clone)]
pub struct IcebergArgs {
    location: String,
}
pub async fn handle_lookup_inventory(
    inventory_lookup_command: InventoryLookupCommand,
) -> anyhow::Result<()> {
    let dici_management_client = ManagementClient::default();
    match inventory_lookup_command {
        InventoryLookupCommand::All => {
            let inventories = dici_management_client.fetch_inventories().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize inventories")
        }
        InventoryLookupCommand::Fxf(FxfArgs { fxf }) => {
            let inventories = dici_management_client.fetch_inventory_by_fxf(fxf).await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize inventory")
        }
        InventoryLookupCommand::Iceberg(IcebergArgs { location }) => {
            let inventories = dici_management_client
                .fetch_inventories_by_iceberg_location(location)
                .await?;
            serde_json::to_writer_pretty(std::io::stdout(), &inventories)
                .context("failed to serialize inventories")
        }
    }
}
