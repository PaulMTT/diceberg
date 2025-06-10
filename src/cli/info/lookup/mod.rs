pub mod inventory;
pub mod registration;

use crate::cli::info::lookup::inventory::{handle_lookup_inventory, InventoryLookupType};
use crate::cli::info::lookup::registration::{handle_lookup_registration, RegistrationLookupType};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum InfoLookupCommand {
    /// Find registrations
    #[clap(subcommand)]
    Registration(RegistrationLookupType),
    /// Find inventories
    #[clap(subcommand)]
    Inventory(InventoryLookupType),
}

pub async fn handle_lookup(lookup_type: InfoLookupCommand) -> anyhow::Result<()> {
    match lookup_type {
        InfoLookupCommand::Registration(registration_lookup_type) => {
            handle_lookup_registration(registration_lookup_type).await
        }
        InfoLookupCommand::Inventory(inventory_lookup_type) => {
            handle_lookup_inventory(inventory_lookup_type).await
        }
    }
}
