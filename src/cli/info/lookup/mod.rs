use clap::Subcommand;

#[derive(Subcommand)]
pub enum LookupType {
    Registration,
    Inventory,
}

pub async fn handle_lookup(lookup_type: LookupType) -> anyhow::Result<()> {
    todo!()
}
