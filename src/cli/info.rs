use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum InfoKind {
    Schema {
        #[clap(subcommand)]
        asset: InfoAsset,
    },
}

#[derive(Subcommand)]
pub enum InfoAsset {
    Core(InfoCoreArgs),
    Iceberg(InfoIcebergArgs),
}

#[derive(Args)]
pub struct InfoCoreArgs {
    #[arg(long)]
    pub fxf: String,
}

#[derive(Args)]
pub struct InfoIcebergArgs {
    #[arg(long)]
    pub location: String,

    #[arg(long)]
    pub schema_table: String,
}
