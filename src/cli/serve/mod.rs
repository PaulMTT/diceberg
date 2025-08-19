#[cfg(feature = "mcp")]
use crate::cli::serve::mcp::handle_serve_mcp;
use clap::Subcommand;
#[cfg(feature = "mcp")]
pub mod mcp;
#[derive(Subcommand, Clone)]
pub enum ServeCommand {
    #[cfg(feature = "mcp")]
    MCP,
}
pub async fn handle_serve(serve_command: ServeCommand) -> anyhow::Result<()> {
    match serve_command {
        #[cfg(feature = "mcp")]
        ServeCommand::MCP => handle_serve_mcp().await,
    }
}
