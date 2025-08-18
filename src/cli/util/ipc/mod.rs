pub mod print;
pub mod query;
use crate::cli::util::ipc::print::handle_util_ipc_print;
use crate::cli::util::ipc::query::{IpcQueryArgs, handle_util_ipc_query};
use anyhow::Result;
use clap::Subcommand;
#[derive(Subcommand, Clone)]
pub enum IpcCommand {
    Print,
    Query(IpcQueryArgs),
}
pub async fn handle_util_ipc(ipc_command: IpcCommand) -> Result<()> {
    match ipc_command {
        IpcCommand::Print => handle_util_ipc_print().await,
        IpcCommand::Query(args) => handle_util_ipc_query(args).await,
    }
}
