use anyhow::Result;
use polars::io::SerReader;
use polars::prelude::IpcStreamReader;
use std::io;
pub async fn handle_util_ipc_print() -> Result<()> {
    let df = IpcStreamReader::new(io::stdin()).finish()?;
    println!("{}", df);
    Ok(())
}
