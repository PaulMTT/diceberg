pub mod ipc;

use crate::cli::util::ipc::{handle_util_ipc, IpcCommand};
use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum UtilCommand {
    /// Arrow IPC utilities
    #[clap(subcommand)]
    IPC(IpcCommand),
}

pub async fn handle_util(util_command: UtilCommand) -> Result<()> {
    match util_command {
        UtilCommand::IPC(args) => handle_util_ipc(args).await,
    }
}
