use crate::cli::serve::mcp::handle_serve_mcp;
use clap::Subcommand;

pub mod mcp;

#[derive(Subcommand, Clone)]
pub enum ServeCommand {
    MCP,
}

pub async fn handle_serve(serve_command: ServeCommand) -> anyhow::Result<()> {
    match serve_command {
        ServeCommand::MCP => handle_serve_mcp().await,
    }
}
