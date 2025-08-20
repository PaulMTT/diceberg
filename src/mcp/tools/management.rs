use crate::api::http::management::model::inventory::Inventory;
use crate::api::http::management::model::registration::Registration;
use crate::api::http::management::model::sync::IcebergLocationSync;
use crate::api::http::management::model::version::GitConfig;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, into_call_err, json_as_text};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
#[mcp_tool(
    name = "get_inventory_by_four_by_four",
    title = "Get inventory by a fourByFour",
    description = "Input: { four_by_four } – The fourByFour. \
                   Output: Inventory object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryGetByFxf {
    pub four_by_four: String,
}
impl DiciCallableTool for InventoryGetByFxf {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let inv: Inventory = client
            .fetch_inventory_by_fxf(self.four_by_four.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&inv)
    }
}
#[mcp_tool(
    name = "list_inventory_by_iceberg_location",
    title = "List inventories by icebergLocation",
    description = "Input: { iceberg_location } – The icebergLocation. \
                   Output: List of Inventory objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListByIcebergLocation {
    pub iceberg_location: String,
}
impl DiciCallableTool for InventoryListByIcebergLocation {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let list: Vec<Inventory> = client
            .fetch_inventories_by_iceberg_location(self.iceberg_location.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&list)
    }
}
#[mcp_tool(
    name = "list_inventory_by_domain",
    title = "List inventories by domain",
    description = "Input: { domain } – The domain. \
                   Output: List of Inventory objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListByDomain {
    pub domain: String,
}
impl DiciCallableTool for InventoryListByDomain {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let list: Vec<Inventory> = client
            .fetch_inventories_by_domain(self.domain.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&list)
    }
}
#[mcp_tool(
    name = "list_inventory_updated_since",
    title = "List inventories updates since a datetime",
    description = "Input: { since } – ISO-8601 datetime. \
                   Output: List of Inventory objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListUpdatedSince {
    pub since: String,
}
impl DiciCallableTool for InventoryListUpdatedSince {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        use chrono::{DateTime, Utc};
        let client = &state.management_client;
        let since_dt: DateTime<Utc> = self
            .since
            .parse()
            .map_err(|e| into_call_err(format!("Invalid datetime format for 'since': {}", e)))?;
        let list: Vec<Inventory> = client
            .fetch_inventories_updated_since(since_dt)
            .await
            .map_err(into_call_err)?;
        json_as_text(&list)
    }
}
#[mcp_tool(
    name = "list_inventory_by_iceberg_location_and_table",
    title = "List inventories by icebergLocation and schemaTable",
    description = "Input: { iceberg_location, schema_table } – The icebergLocation and schemaTable. \
                   Output: List of Inventory objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListByIcebergLocationAndTable {
    pub iceberg_location: String,
    pub schema_table: String,
}
impl DiciCallableTool for InventoryListByIcebergLocationAndTable {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let list: Vec<Inventory> = client
            .fetch_inventories_by_iceberg_location_and_table(
                self.iceberg_location.clone(),
                self.schema_table.clone(),
            )
            .await
            .map_err(into_call_err)?;
        json_as_text(&list)
    }
}
#[mcp_tool(
    name = "get_inventory_by_id",
    title = "Get inventory by domain, icebergLocation, and schemaTable",
    description = "Input: { domain, iceberg_location, schema_table } – The domain, icebergLocation, and schemaTable. \
                   Output: Inventory object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryGetById {
    pub domain: String,
    pub iceberg_location: String,
    pub schema_table: String,
}
impl DiciCallableTool for InventoryGetById {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let inv: Inventory = client
            .fetch_inventory_by_id(
                self.domain.clone(),
                self.iceberg_location.clone(),
                self.schema_table.clone(),
            )
            .await
            .map_err(into_call_err)?;
        json_as_text(&inv)
    }
}
#[mcp_tool(
    name = "list_registrations_by_path",
    title = "List registrations by path",
    description = "Input: { path } – The registration path. \
                   Output: List of Registration objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationListByPath {
    pub path: String,
}
impl DiciCallableTool for RegistrationListByPath {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let regs: Vec<Registration> = client
            .fetch_registrations_by_path(self.path.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&regs)
    }
}
#[mcp_tool(
    name = "get_registration_by_iceberg_location",
    title = "Get a registration by icebergLocation",
    description = "Input: { iceberg_location } – The icebergLocation. \
                   Output: Registration object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationGetByIcebergLocation {
    pub iceberg_location: String,
}
impl DiciCallableTool for RegistrationGetByIcebergLocation {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let reg: Registration = client
            .fetch_registration_by_iceberg_location(self.iceberg_location.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&reg)
    }
}
#[mcp_tool(
    name = "list_registration_by_path_and_metadata",
    title = "List registrations by path and metadata",
    description = "Input: { path, metadata } – The registration path and metadata key-value filters. \
                   Output: List of Registration objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationQueryByPathAndMetadata {
    pub path: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}
impl DiciCallableTool for RegistrationQueryByPathAndMetadata {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let regs: Vec<Registration> = client
            .fetch_registrations_by_path_and_metadata(self.path.clone(), self.metadata.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&regs)
    }
}
#[mcp_tool(
    name = "get_dici_management_information",
    title = "Get the deployment information of dici management",
    description = "Input: none. \
                   Output: GitConfig object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct VersionGet {}
impl DiciCallableTool for VersionGet {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let version_info: GitConfig = client.fetch_version().await.map_err(into_call_err)?;
        json_as_text(&version_info)
    }
}
#[mcp_tool(
    name = "sync_table",
    title = "Sync an iceberg table",
    description = "Input: { iceberg_location, schema_table } – The icebergLocation and schemaTable. \
                   Output: List of Inventory objects."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SyncTable {
    pub iceberg_location: String,
    pub schema_table: String,
}
impl DiciCallableTool for SyncTable {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let list: Vec<Inventory> = client
            .sync_table(self.iceberg_location.clone(), self.schema_table.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&list)
    }
}
#[mcp_tool(
    name = "sync_table_domain",
    title = "Sync an iceberg table with explicit domain",
    description = "Input: { domain, iceberg_location, schema_table } – The domain, icebergLocation, and schemaTable. \
                   Output: Inventory object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SyncTableDomain {
    pub domain: String,
    pub iceberg_location: String,
    pub schema_table: String,
}
impl DiciCallableTool for SyncTableDomain {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let inv: Inventory = client
            .sync_table_domain(
                self.domain.clone(),
                self.iceberg_location.clone(),
                self.schema_table.clone(),
            )
            .await
            .map_err(into_call_err)?;
        json_as_text(&inv)
    }
}
#[mcp_tool(
    name = "sync_iceberg_location",
    title = "Sync an entire iceberg location",
    description = "Input: { iceberg_location } – The icebergLocation. \
                   Output: IcebergLocationSync object with successes and failures."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SyncIcebergLocation {
    pub iceberg_location: String,
}
impl DiciCallableTool for SyncIcebergLocation {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let sync_result: IcebergLocationSync = client
            .sync_iceberg_location(self.iceberg_location.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&sync_result)
    }
}
