use crate::api::management::inventory::Inventory;
use crate::api::management::registration::Registration;
use crate::api::management::version::GitConfig;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{into_call_err, json_as_text, DiciCallableTool};
use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[mcp_tool(
    name = "inventory_get_by_fxf",
    title = "Get Inventory by FXF",
    description = "Retrieve a single inventory record using its FXF (FourByFour) Socrata dataset identifier.\n\n\
                   Input:\n  - `fxf`: string in format `XXXX-XXXX` (8 alphanumeric, split 4–4, case-insensitive).\n\n\
                   Output:\n  - Complete Inventory object with:\n    \
                       • id.domain.domain (string)\n    \
                       • id.icebergLocation.icebergLocation (string, `_` + 32 lowercase hex)\n    \
                       • id.schemaTable.schemaTable (string)\n    \
                       • fourByFour.fourByFour (string, FXF)\n    \
                       • createdAt / updatedAt (ISO-8601 UTC)",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryGetByFxf {
    /// FXF identifier in the format `XXXX-XXXX`.
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
    description = "List all inventory records for a given iceberg location.\n\n\
                   Input:\n  - `iceberg_location`: string in format `_` + 32 lowercase hex (md5 of registration path).\n\n\
                   Output:\n  - Array of Inventory objects, each with:\n    \
                       • id.domain.domain\n    \
                       • id.icebergLocation.icebergLocation\n    \
                       • id.schemaTable.schemaTable\n    \
                       • fourByFour.fourByFour\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListByIcebergLocation {
    /// Iceberg location string in the format `_` followed by 32 lowercase hex characters.
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
    description = "List all inventory records for a given Socrata domain.\n\n\
                   Input:\n  - `domain`: string (case-sensitive, matches registration metadata.domain).\n\n\
                   Output:\n  - Array of Inventory objects, each with:\n    \
                       • id.domain.domain\n    \
                       • id.icebergLocation.icebergLocation\n    \
                       • id.schemaTable.schemaTable\n    \
                       • fourByFour.fourByFour\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListByDomain {
    /// Domain string to match exactly (case-sensitive).
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
    description = "Retrieve all registrations that match the given canonical path.\n\n\
                   Input:\n  - `path`: string (any valid format, e.g. `s3://bucket/raw/path`).\n\n\
                   Output:\n  - Array of Registration objects, each with:\n    \
                       • id.path\n    \
                       • icebergLocation.icebergLocation\n    \
                       • metadata (map<string,string>)\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationListByPath {
    /// Canonical path for the registration.
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
    description = "Retrieve a single registration by its iceberg location identifier.\n\n\
                   Input:\n  - `iceberg_location`: string in format `_` + 32 lowercase hex.\n\n\
                   Output:\n  - Registration object with:\n    \
                       • id.path\n    \
                       • icebergLocation.icebergLocation\n    \
                       • metadata (map<string,string>)\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationGetByIcebergLocation {
    /// Iceberg location string in the format `_` followed by 32 lowercase hex characters.
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
    description = "Query for registrations matching a given path and exact metadata key–value pairs.\n\n\
                   Input:\n  - `path`: string (any format).\n  - `metadata`: map<string,string> (all key–value pairs must match exactly).\n\n\
                   Output:\n  - Array of Registration objects, each with:\n    \
                       • id.path\n    \
                       • icebergLocation.icebergLocation\n    \
                       • metadata\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RegistrationQueryByPathAndMetadata {
    /// Path string for the registration.
    pub path: String,
    /// Metadata filters that must match exactly.
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
    title = "Get Git Version Info",
    description = "Retrieve current build and Git commit metadata from the /version endpoint.\n\n\
                   Input:\n  - No parameters.\n\n\
                   Output:\n  - GitConfig object with fields including:\n    \
                       • branch\n    \
                       • build (host, time, user { email, name }, version, number?)\n    \
                       • closest.tag (name?, commit.count?)\n    \
                       • commit (author.time, committer.time, id { abbrev, describe, describeShort, full }, message { full, short }, time, user { email, name })\n    \
                       • dirty\n    \
                       • local.branch (ahead, behind)\n    \
                       • remote.origin.url\n    \
                       • tag?, tags?\n    \
                       • total.commit.count",
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
    title = "List Inventories Updated Since",
    description = "List all inventory records whose updatedAt is greater than or equal to the given timestamp.\n\n\
                   Input:\n  - `since`: string in ISO-8601 / RFC3339 format (UTC), e.g. `2025-08-14T00:00:00Z`.\n\n\
                   Output:\n  - Array of Inventory objects, each with:\n    \
                       • id.domain.domain\n    \
                       • id.icebergLocation.icebergLocation\n    \
                       • id.schemaTable.schemaTable\n    \
                       • fourByFour.fourByFour\n    \
                       • createdAt / updatedAt",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InventoryListUpdatedSince {
    /// ISO-8601 UTC timestamp to filter updatedAt >= this value.
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
