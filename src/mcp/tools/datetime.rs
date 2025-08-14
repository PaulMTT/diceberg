use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{json_as_text, DiciCallableTool};
use chrono::{DateTime, Utc};
use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use serde::{Deserialize, Serialize};
use serde_json;

#[mcp_tool(
    name = "get_date_time",
    title = "Get Current UTC DateTime",
    description = "Retrieve the current date and time in UTC.\n\n\
                   Output:\n  - `utc`: ISO-8601 / RFC3339 UTC timestamp.\n  - `epoch_seconds`: integer (seconds since Unix epoch).",
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
