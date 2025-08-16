pub mod ipc;

use crate::cli::util::ipc::{IpcCommand, handle_util_ipc};
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum UtilCommand {
    #[clap(subcommand)]
    IPC(IpcCommand),
}

pub async fn handle_util(util_command: UtilCommand) -> Result<()> {
    match util_command {
        UtilCommand::IPC(args) => handle_util_ipc(args).await,
    }
}
