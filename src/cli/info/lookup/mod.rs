pub mod inventory;
pub mod registration;

use crate::cli::info::lookup::inventory::{handle_lookup_inventory, InventoryLookupType};
use crate::cli::info::lookup::registration::{handle_lookup_registration, RegistrationLookupType};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum LookupType {
    #[clap(subcommand)]
    Registration(RegistrationLookupType),
    #[clap(subcommand)]
    Inventory(InventoryLookupType),
}

pub async fn handle_lookup(lookup_type: LookupType) -> anyhow::Result<()> {
    match lookup_type {
        LookupType::Registration(registration_lookup_type) => {
            handle_lookup_registration(registration_lookup_type).await
        }
        LookupType::Inventory(inventory_lookup_type) => {
            handle_lookup_inventory(inventory_lookup_type).await
        }
    }
}
