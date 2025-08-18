use crate::api::dici::asset::{CoreArgs, DiciAsset, IcebergArgs};
use crate::api::dici::core::CoreAsset;
use crate::api::dici::iceberg::IcebergAsset;
use crate::api::traits::table_source::TableSource;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, into_call_err, json_as_text};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::CallToolResult;
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use serde::{Deserialize, Serialize};
#[mcp_tool(
    name = "asset_get_schema_by_fxf",
    title = "Get Dataset Schema by FXF",
    description = "Given a public Four-by-Four (FXF) dataset identifier (format `xxxx-xxxx`), \
retrieves the table schema for the dataset within its domain. \
Input: `fxf` (string, FXF). \
Output: JSON array of schema fields with name, type, and nullability. \
Concepts: FXF uniquely identifies an Inventory; schema is resolved via Iceberg location mapping.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetGetSchemaByFxf {
    pub fxf: String,
}
impl DiciCallableTool for AssetGetSchemaByFxf {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let asset: DiciAsset = CoreArgs::builder()
            .asset(CoreAsset::builder().fxf(&self.fxf).build())
            .dici_catalog(state.dici_catalog.clone())
            .management_client(state.management_client.clone())
            .build()
            .into();
        let fields = asset.schema().await.map_err(into_call_err)?;
        json_as_text(&fields)
    }
}
#[mcp_tool(
    name = "asset_get_schema_by_iceberg",
    title = "Get Dataset Schema by Iceberg Location",
    description = "Retrieves the schema for a dataset directly from its Iceberg table reference. \
Input: `location` (string, Iceberg location `_` + 32 lowercase hex), `schema_table` (string, lowercase table name). \
Output: JSON array of schema fields with name, type, and nullability. \
Concepts: Iceberg location uniquely identifies a Registration and Inventory mapping; schema_table specifies the table within that location.",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetGetSchemaByIceberg {
    pub location: String,
    pub schema_table: String,
}
impl DiciCallableTool for AssetGetSchemaByIceberg {
    async fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> Result<CallToolResult, CallToolError> {
        let asset: DiciAsset = IcebergArgs::builder()
            .asset(
                IcebergAsset::builder()
                    .location(&self.location)
                    .schema_table(&self.schema_table)
                    .build(),
            )
            .dici_catalog(state.dici_catalog.clone())
            .build()
            .into();
        let fields = asset.schema().await.map_err(into_call_err)?;
        json_as_text(&fields)
    }
}
