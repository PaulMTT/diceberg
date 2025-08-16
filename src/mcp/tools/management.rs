use crate::api::management::inventory::Inventory;
use crate::api::management::registration::Registration;
use crate::api::management::version::GitConfig;
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
    description = "Looks up a single Inventory record by its FXF identifier (format `xxxx-xxxx`). \
Input: `fxf` (string). \
Output: JSON object representing the Inventory, including domain, Iceberg location, schema table, FXF, created/updated timestamps. \
Concepts: Inventory links a public FXF to an Iceberg location within a domain.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    description = "Finds all Inventory records referencing the same Iceberg location. \
Input: `iceberg_location` (string, `_` + 32 lowercase hex). \
Output: JSON array of Inventory objects. \
Concepts: Used to discover all public datasets backed by the same physical Iceberg table.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    description = "Lists all Inventory records belonging to a specific domain. \
Input: `domain` (string). \
Output: JSON array of Inventory objects. \
Concepts: Domain is the top-level scope for datasets; this lists all public datasets in that tenant.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    name = "registration_list_by_path",
    title = "List Registrations by Path",
    description = "Lists all Registration records with a given canonical path. \
Input: `path` (string). \
Output: JSON array of Registration objects, each with path, Iceberg location, metadata, created/updated timestamps. \
Concepts: Path deterministically maps to Iceberg location; may have multiple metadata variants.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    description = "Retrieves the Registration record for a given Iceberg location. \
Input: `iceberg_location` (string, `_` + 32 lowercase hex). \
Output: JSON object with path, Iceberg location, metadata, timestamps. \
Concepts: Iceberg location is the bridge between Registration and Inventory.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    description = "Searches for Registration records with a specific path and metadata filters. \
Input: `path` (string), `metadata` (map<string,string>, optional). \
Output: JSON array of Registration objects. \
Concepts: Enables refined search within Registrations, especially to filter by domain or other metadata.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    title = "Get Git Configuration and Build Metadata",
    description = "Retrieves the current Git configuration, commit, and build metadata. \
Input: none. \
Output: JSON object with branch, build info, commit details, tags, remote origin, and total commits. \
Concepts: GitConfig provides temporal context to dataset changes and system state.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
    name = "inventory_list_updated_since",
    title = "List Inventories Updated Since Timestamp",
    description = "Lists all Inventory records updated on or after the given timestamp. \
Input: `since` (ISO-8601 UTC string). \
Output: JSON array of Inventory objects. \
Concepts: Used to find datasets changed within a time window; can correlate with GitConfig or external events.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
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
