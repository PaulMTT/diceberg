use crate::cli::sql::SqlOutputFormat;
use arrow_ipc::reader::StreamReader;
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
    let ctx = SessionContext::new();
    let reader = StreamReader::try_new(io::stdin(), None)?;
    let schema = reader.schema();
    let records: Vec<arrow::array::RecordBatch> =
        reader.collect::<arrow::error::Result<Vec<_>>>()?;
    let mem_table = MemTable::try_new(schema.clone(), vec![records])?;
    ctx.register_table("this", Arc::new(mem_table))?;
    match ipc_query_args {
        IpcQueryArgs { query, format } => {
            let df = ctx.sql(query.as_str()).await?;
            format.to_writer(io::stdout(), df).await
        }
    }
}
