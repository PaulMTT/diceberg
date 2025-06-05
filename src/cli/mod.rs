use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Info {
        #[clap(subcommand)]
        kind: InfoKind,
    },
    Sql {
        #[clap(subcommand)]
        asset: SqlAsset,
    },
}

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
