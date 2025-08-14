use crate::api::client::DiciClient;
use crate::api::management::client::ManagementClient;
use crate::mcp::tools::{DiciCallableTool, DiciToolBox};
use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult,
    RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Clone)]
pub struct DiciServerHandlerState {
    pub management_client: ManagementClient,
    pub dici_client: DiciClient,
}
#[derive(TypedBuilder, Default)]
pub struct DiciServerHandler {
    state: DiciServerHandlerState,
}

#[async_trait]
impl ServerHandler for DiciServerHandler {
    /// List available tools
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: DiciToolBox::tools(),
        })
    }

    /// Handle call to the DateTime tool
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        let tool_box: DiciToolBox =
            DiciToolBox::try_from(request.params).map_err(CallToolError::new)?;
        tool_box.call_tool(&self.state).await
    }
}
