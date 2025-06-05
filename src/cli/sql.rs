use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum SqlAsset {
    Core(SqlCoreArgs),
    Iceberg(SqlIcebergArgs),
}

#[derive(Args)]
pub struct SqlCoreArgs {
    #[arg(long)]
    pub fxf: String,

    #[arg(long)]
    pub query: String,
}

#[derive(Args)]
pub struct SqlIcebergArgs {
    #[arg(long)]
    pub location: String,

    #[arg(long)]
    pub schema_table: String,

    #[arg(long)]
    pub query: String,
}
