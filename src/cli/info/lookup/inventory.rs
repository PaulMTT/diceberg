use clap::Subcommand;

#[derive(Subcommand)]
pub enum InventoryLookupType {
    All,
}

pub async fn handle_lookup_inventory(
    inventory_lookup_type: InventoryLookupType,
) -> anyhow::Result<()> {
    todo!()
}
