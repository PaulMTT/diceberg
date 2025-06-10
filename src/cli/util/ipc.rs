use clap::Subcommand;
use polars::io::SerReader;
use polars::prelude::IpcStreamReader;
use std::io;

#[derive(Subcommand)]
pub enum IpcCommand {
    /// Read an arrow IPC dataframe from stdin and print it
    Print,
}

pub async fn handle_util_ipc(ipc_command: IpcCommand) -> anyhow::Result<()> {
    match ipc_command {
        IpcCommand::Print => {
            let df = IpcStreamReader::new(io::stdin()).finish()?;
            println!("{}", df);
            Ok(())
        }
    }
}
