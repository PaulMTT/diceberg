pub mod print;
pub mod query;

use crate::cli::util::ipc::print::handle_util_ipc_print;
use crate::cli::util::ipc::query::{handle_util_ipc_query, IpcQueryArgs};
use anyhow::Result;
use clap::Subcommand;
#[derive(Subcommand)]
pub enum IpcCommand {
    /// Read an arrow IPC dataframe from stdin and print it
    Print,
    /// Execute sql against an IPC dataframe from stdin
    Query(IpcQueryArgs),
}

pub async fn handle_util_ipc(ipc_command: IpcCommand) -> Result<()> {
    match ipc_command {
        IpcCommand::Print => handle_util_ipc_print().await,
        IpcCommand::Query(args) => handle_util_ipc_query(args).await,
    }
}
