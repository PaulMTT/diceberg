use clap::Subcommand;
use polars::io::SerReader;
use polars::prelude::IpcStreamReader;
use std::io;
use std::io::Read;

#[derive(Subcommand)]
pub enum IpcCommand {
    /// Read an arrow IPC dataframe from stdin and print it
    Print,
}

pub async fn handle_util_ipc(ipc_command: IpcCommand) -> anyhow::Result<()> {
    match ipc_command {
        IpcCommand::Print => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            let cursor = io::Cursor::new(buffer);
            let df = IpcStreamReader::new(cursor).finish()?;
            println!("{}", df);
            Ok(())
        }
    }
}
