use crate::api::client::core_scope::DicebergCoreAsset;
use crate::api::client::iceberg_scope::DicebergIcebergAsset;
use crate::api::traits::SqlAble;
use crate::cli::info::table::{CoreAssetArgs, IcebergAssetArgs};
use anyhow::Context;
use arrow::array::RecordBatch;
use arrow_ipc::writer::StreamWriter;
use arrow_json::ArrayWriter;
use clap::{Args, Subcommand, ValueEnum};
use datafusion::prelude::DataFrame;
use std::io;
use std::io::Write;

#[derive(Subcommand)]
pub enum SqlCommand {
    /// Using the core fxf identifier
    Core(SqlCoreArgs),
    /// Using the iceberg identifier
    Iceberg(SqlIcebergArgs),
}

#[derive(Args)]
pub struct SqlArgs {
    /// The sql statement
    pub query: String,
    /// The response format
    #[arg(short, long, value_enum, default_value_t)]
    pub format: SqlOutputFormat,
}

#[derive(Args)]
pub struct SqlCoreArgs {
    #[clap(flatten)]
    pub core: CoreAssetArgs,
    #[clap(flatten)]
    pub sql: SqlArgs,
}

#[derive(Args)]
pub struct SqlIcebergArgs {
    #[clap(flatten)]
    pub iceberg: IcebergAssetArgs,
    #[clap(flatten)]
    pub sql: SqlArgs,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SqlOutputFormat {
    JSON,
    IPC,
}

impl SqlOutputFormat {
    pub async fn to_writer<W: Write>(&self, writer: W, df: DataFrame) -> anyhow::Result<()> {
        let records: Vec<RecordBatch> = df.clone().collect().await?;
        match self {
            SqlOutputFormat::JSON => {
                let mut writer = ArrayWriter::new(writer);
                writer.write_batches(&records.iter().collect::<Vec<_>>())?;
                writer.finish().context("Failed to write JSON")
            }
            SqlOutputFormat::IPC => {
                let schema = df.schema().as_arrow();
                let mut writer = StreamWriter::try_new(writer, schema)?;
                for record in records {
                    writer
                        .write(&record)
                        .context("Failed to write an IPC batch")?;
                }
                writer.finish().context("Failed to write IPC")
            }
        }
    }
}

impl Default for SqlOutputFormat {
    fn default() -> Self {
        Self::JSON
    }
}

pub async fn handle_sql(sql_command: SqlCommand) -> anyhow::Result<()> {
    match sql_command {
        SqlCommand::Core(SqlCoreArgs {
            core,
            sql: SqlArgs { query, format },
        }) => {
            let asset: DicebergCoreAsset = core.into();
            let df = asset.sql(query.as_str()).await?;
            format.to_writer(io::stdout(), df).await
        }
        SqlCommand::Iceberg(SqlIcebergArgs {
            iceberg,
            sql: SqlArgs { query, format },
        }) => {
            let asset: DicebergIcebergAsset = iceberg.into();
            let df = asset.sql(query.as_str()).await?;
            format.to_writer(io::stdout(), df).await
        }
    }
}
