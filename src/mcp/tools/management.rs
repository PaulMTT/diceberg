use crate::api::http::management::model::inventory::Inventory;
use crate::api::http::management::model::registration::Registration;
use crate::api::http::management::model::version::GitConfig;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, into_call_err, json_as_text};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::{CallToolResult, schema_utils::CallToolError};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
#[mcp_tool(
    name = "inventory_get_by_fxf",
    title = "Get Inventory by FXF",
    description = "Input: { fxf } – fourByFour ID. \
                   Output: Inventory object."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryGetByFxf {
    pub fxf: String,
}
impl DiciCallableTool for InventoryGetByFxf {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let client = &state.management_client;
        let inv: Inventory = client
            .fetch_inventory_by_fxf(self.fxf.clone())
            .await
            .map_err(into_call_err)?;
        json_as_text(&inv)
    }
}
#[mcp_tool(
    name = "inventory_list_by_iceberg_location",
    title = "List Inventories by Iceberg Location",
    description = "Input: { iceberg_location } – internal Iceberg identifier. \
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
    name = "inventory_list_by_domain",
    title = "List Inventories by Domain",
    description = "Input: { domain } – domain name. \
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
    name = "inventory_list_updated_since",
    title = "List Inventories Updated Since",
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
    name = "registration_list_by_path",
    title = "List Registrations by Path",
    description = "Input: { path } – canonical dataset path. \
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
    name = "registration_get_by_iceberg_location",
    title = "Get Registration by Iceberg Location",
    description = "Input: { iceberg_location } – internal Iceberg identifier. \
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
    name = "registration_query_by_path_and_metadata",
    title = "Query Registrations by Path and Metadata",
    description = "Input: { path, metadata } – dataset path and metadata filters. \
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
    name = "version_get",
    title = "Get Build Version",
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
