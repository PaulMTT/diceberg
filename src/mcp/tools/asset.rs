use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::DiciAsset;
use crate::api::traits::TableSource;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{into_call_err, json_as_text, DiciCallableTool};
use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::CallToolResult;
use serde::{Deserialize, Serialize};

#[mcp_tool(
    name = "asset_get_schema_by_fxf",
    title = "Get Asset Table Schema by FXF",
    description = "Retrieve the table schema for an asset identified by its Core FXF (FourByFour) identifier.\n\n\
                   Input:\n  - `fxf`: string in format `XXXX-XXXX` (8 alphanumeric, split 4–4, case-insensitive).\n\n\
                   Output:\n  - Array of field definitions (as returned by asset.schema()), pretty-printed JSON.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetGetSchemaByFxf {
    /// Core FXF identifier in format `XXXX-XXXX`.
    pub fxf: String,
}

impl DiciCallableTool for AssetGetSchemaByFxf {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let asset = DiciAsset::Core {
            asset: CoreAsset::builder().fxf(&self.fxf).build(),
            dici_client: state.dici_client.clone(),
            management_client: state.management_client.clone(),
        };

        let fields = asset.schema().await.map_err(into_call_err)?;
        json_as_text(&fields)
    }
}

#[mcp_tool(
    name = "asset_get_schema_by_iceberg",
    title = "Get Asset Table Schema by Iceberg Location",
    description = "Retrieve the table schema for an asset identified by its Iceberg location and schema table.\n\n\
                   Input:\n  - `location`: string in format `_` + 32 lowercase hex (md5 of registration path).\n  - `schema_table`: string.\n\n\
                   Output:\n  - Array of field definitions (as returned by asset.schema()), pretty-printed JSON.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetGetSchemaByIceberg {
    /// Iceberg location string in the format `_` followed by 32 lowercase hex characters.
    pub location: String,
    /// Iceberg schema table name.
    pub schema_table: String,
}

impl DiciCallableTool for AssetGetSchemaByIceberg {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let asset = DiciAsset::Iceberg {
            asset: IcebergAsset::builder()
                .location(&self.location)
                .schema_table(&self.schema_table)
                .build(),
            client: state.dici_client.clone(),
        };

        let fields = asset.schema().await.map_err(into_call_err)?;
        json_as_text(&fields)
    }
}
