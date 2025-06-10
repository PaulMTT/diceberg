use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::base::DicebergClient;
use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::SqlAble;
use anyhow::Context;
use arrow::array::RecordBatch;
use arrow_ipc::writer::StreamWriter;
use arrow_json::ArrayWriter;
use clap::{Args, Subcommand, ValueEnum};
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
    /// The response format
    #[arg(short, long, value_enum, default_value_t)]
    pub format: SqlOutputFormat
}

#[derive(Args)]
pub struct SqlIcebergArgs {
    /// The iceberg location
    pub location: String,
    /// The iceberg schema-table
    pub schema_table: String,
    /// The sql statement
    pub query: String,
    /// The response format
    #[arg(short, long, value_enum, default_value_t)]
    pub format: SqlOutputFormat
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SqlOutputFormat{
    JSON,
    IPC
}

impl Default for SqlOutputFormat {
    fn default() -> Self {
        Self::JSON
    }
}

pub async fn handle_sql(sql_command: SqlArgs) -> anyhow::Result<()> {
    match sql_command {
        SqlArgs::Core(SqlCoreArgs { fxf, query, format }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let df = asset.sql(query.as_str()).await?;
            let records: Vec<RecordBatch> = df.clone().collect().await?;
            match format {
                SqlOutputFormat::JSON => {
                    let mut writer = ArrayWriter::new(io::stdout());
                    writer.write_batches(&records.iter().collect::<Vec<_>>())?;
                    writer
                        .finish()
                        .context("failed to write core records to stdout")
                }
                SqlOutputFormat::IPC => {
                    let schema = df.schema().as_arrow();
                    // create a new writer, the schema must be known in advance
                    let mut writer = StreamWriter::try_new(io::stdout(), schema)?;
                    for record in records{
                        writer.write(&record).context("Failed to write an ipc batch")?;
                    }
                    writer.finish().context("Failed to write ipc")
                }
            }
        }
        SqlArgs::Iceberg(SqlIcebergArgs {
            location,
            schema_table,
            query,
            format
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            let df = asset.sql(query.as_str()).await?;
            let records: Vec<RecordBatch> = df.clone().collect().await?;
            match format {
                SqlOutputFormat::JSON => {
                    let mut writer = ArrayWriter::new(io::stdout());
                    writer.write_batches(&records.iter().collect::<Vec<_>>())?;
                    writer
                        .finish()
                        .context("failed to write core records to stdout")
                }
                SqlOutputFormat::IPC => {
                    let schema = df.schema().as_arrow();
                    // create a new writer, the schema must be known in advance
                    let mut writer = StreamWriter::try_new(io::stdout(), schema)?;
                    for record in records{
                        writer.write(&record).context("Failed to write an ipc batch")?;
                    }
                    writer.finish().context("Failed to write ipc")
                }
            }
        }
    }
}
