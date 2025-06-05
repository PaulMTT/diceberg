use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::SqlAble;
use arrow_json::ArrayWriter;
use clap::{Args, Subcommand};
use std::io;

#[derive(Subcommand)]
pub enum SqlAsset {
    Core(SqlCoreArgs),
    Iceberg(SqlIcebergArgs),
}

#[derive(Args)]
pub struct SqlCoreArgs {
    pub fxf: String,
    pub query: String,
}

#[derive(Args)]
pub struct SqlIcebergArgs {
    pub location: String,
    pub schema_table: String,
    pub query: String,
}

pub async fn handle_sql(sql_command: SqlAsset) -> anyhow::Result<()> {
    match sql_command {
        SqlAsset::Core(SqlCoreArgs { fxf, query }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let records = asset.sql(query.as_str()).await?.collect().await?;
            let mut writer = ArrayWriter::new(io::stdout());
            writer.write_batches(&records.iter().collect::<Vec<_>>())?;
            writer.finish()?;
        }
        SqlAsset::Iceberg(SqlIcebergArgs {
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
            writer.finish()?;
        }
    }
    Ok(())
}
