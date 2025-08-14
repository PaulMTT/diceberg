pub mod inventory;
pub mod registration;
use anyhow::Result;

use crate::cli::info::lookup::inventory::{handle_lookup_inventory, InventoryLookupCommand};
use crate::cli::info::lookup::registration::{
    handle_lookup_registration, RegistrationLookupCommand,
};
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum InfoLookupCommand {
    /// Find registrations
    #[clap(subcommand)]
    Registration(RegistrationLookupCommand),
    /// Find inventories
    #[clap(subcommand)]
    Inventory(InventoryLookupCommand),
}

pub async fn handle_lookup(info_lookup_command: InfoLookupCommand) -> Result<()> {
    match info_lookup_command {
        InfoLookupCommand::Registration(args) => handle_lookup_registration(args).await,
        InfoLookupCommand::Inventory(args) => handle_lookup_inventory(args).await,
    }
}
