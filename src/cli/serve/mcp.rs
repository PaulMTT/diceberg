use crate::mcp::handler::DiciServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};
use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::{ServerRuntime, server_runtime},
};
pub async fn handle_serve_mcp() -> anyhow::Result<()> {
    run_mcp()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
async fn run_mcp() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "The data and insights cloud integration (DICI) model context protocol (MCP) server.".to_string(),
            version: "0.1.0".to_string(),
            title: Some("The data and insights cloud integration (DICI) model context protocol (MCP) server.".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: None,
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };
    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = DiciServerHandler::default();
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);
    if let Err(start_error) = server.start().await {
        eprintln!(
            "{}",
            start_error
                .rpc_error_message()
                .unwrap_or(&start_error.to_string())
        );
    }
    Ok(())
}
