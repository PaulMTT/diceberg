use crate::api::http::management::client::ManagementClient;
use crate::api::store::catalog::dici::DiciCatalog;
use crate::mcp::tools::{DiciCallableTool, DiciToolBox};
use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, Default, Clone)]
pub struct DiciServerHandlerState {
    pub management_client: ManagementClient,
    pub dici_catalog: DiciCatalog,
}
#[derive(TypedBuilder, Default)]
pub struct DiciServerHandler {
    state: DiciServerHandlerState,
}
#[async_trait]
impl ServerHandler for DiciServerHandler {
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
