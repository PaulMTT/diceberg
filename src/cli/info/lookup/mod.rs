pub mod inventory;
pub mod registration;
use anyhow::Result;

use crate::cli::info::lookup::inventory::{InventoryLookupCommand, handle_lookup_inventory};
use crate::cli::info::lookup::registration::{
    RegistrationLookupCommand, handle_lookup_registration,
};
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum InfoLookupCommand {
    #[clap(subcommand)]
    Registration(RegistrationLookupCommand),

    #[clap(subcommand)]
    Inventory(InventoryLookupCommand),
}

pub async fn handle_lookup(info_lookup_command: InfoLookupCommand) -> Result<()> {
    match info_lookup_command {
        InfoLookupCommand::Registration(args) => handle_lookup_registration(args).await,
        InfoLookupCommand::Inventory(args) => handle_lookup_inventory(args).await,
    }
}
