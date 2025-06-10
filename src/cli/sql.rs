use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::SqlAble;
use anyhow::Context;
use arrow_json::ArrayWriter;
use clap::{Args, Subcommand};
use std::io;

#[derive(Subcommand)]
pub enum SqlArgs {
    /// Using the core fxf identifier
    Core(SqlCoreArgs),
    /// Using the iceberg identifier
    Iceberg(SqlIcebergArgs),
}

#[derive(Args)]
pub struct SqlCoreArgs {
    /// The core four-by-four
    pub fxf: String,
    /// The sql statement
    pub query: String,
}

#[derive(Args)]
pub struct SqlIcebergArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
    /// The sql statement
    pub query: String,
}

pub async fn handle_sql(sql_command: SqlArgs) -> anyhow::Result<()> {
    match sql_command {
        SqlArgs::Core(SqlCoreArgs { fxf, query }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let records = asset.sql(query.as_str()).await?.collect().await?;
            let mut writer = ArrayWriter::new(io::stdout());
            writer.write_batches(&records.iter().collect::<Vec<_>>())?;
            writer
                .finish()
                .context("failed to write core records to stdout")
        }
        SqlArgs::Iceberg(SqlIcebergArgs {
            location,
            schema_table,
            query,
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            let records = asset.sql(query.as_str()).await?.collect().await?;
            let mut writer = ArrayWriter::new(io::stdout());
            writer.write_batches(&records.iter().collect::<Vec<_>>())?;
            writer
                .finish()
                .context("failed to write iceberg records to stdout")
        }
    }
}
