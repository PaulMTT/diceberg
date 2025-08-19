use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, json_as_text};
use chrono::{DateTime, Utc};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};
use serde::{Deserialize, Serialize};
use serde_json;
#[mcp_tool(
    name = "get_date_time",
    title = "Get Current Date and Time",
    description = "Input: none. \
                   Output: The current UTC datetime (ISO-8601 string).",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true,
    meta = r#"{"version": "1.0"}"#
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetDateTimeTool {}
impl DiciCallableTool for GetDateTimeTool {
    async fn call_tool(
        &self,
        _state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let now: DateTime<Utc> = Utc::now();
        json_as_text(&now)
    }
}
