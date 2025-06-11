use crate::cli::sql::SqlOutputFormat;
use anyhow::Context;
use arrow_ipc::reader::StreamReader;
use arrow_ipc::writer::StreamWriter;
use arrow_json::ArrayWriter;
use clap::Args;
use datafusion::catalog::MemTable;
use datafusion::prelude::SessionContext;
use std::io;
use std::sync::Arc;

#[derive(Args)]
pub struct IpcQueryArgs {
    /// The sql query, the table identifier is 'this'
    pub query: String,
    /// The response format
    #[arg(short, long, value_enum, default_value_t)]
    pub format: SqlOutputFormat,
}

pub async fn handle_util_ipc_query(ipc_query_args: IpcQueryArgs) -> anyhow::Result<()> {
    let mut frames = Vec::new();
    let reader = StreamReader::try_new(io::stdin(), None)?;
    let schema = reader.schema();
    for batch in reader {
        let batch = batch?;
        frames.push(batch);
    }
    let mem_table = MemTable::try_new(schema, vec![frames])?;
    let ctx = SessionContext::new();
    ctx.register_table("this", Arc::new(mem_table))?;
    match ipc_query_args {
        IpcQueryArgs { query, format } => {
            let df = ctx.sql(query.as_str()).await?;
            let records: Vec<arrow::array::RecordBatch> = df.clone().collect().await?;
            match format {
                SqlOutputFormat::JSON => {
                    let mut writer = ArrayWriter::new(io::stdout());
                    writer.write_batches(&records.iter().collect::<Vec<_>>())?;
                    writer.finish().context("Failed to write JSON")
                }
                SqlOutputFormat::IPC => {
                    let schema = df.schema().as_arrow();
                    let mut writer = StreamWriter::try_new(io::stdout(), schema)?;
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
}
