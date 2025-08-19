use crate::api::store::asset::core::CoreAsset;
use crate::api::store::asset::dici::{CoreArgs, DiciAsset, IcebergArgs};
use crate::api::store::asset::iceberg::IcebergAsset;
use crate::api::store::asset::traits::schema_source::SchemaSource;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, into_call_err, json_as_text};
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::CallToolResult;
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use serde::{Deserialize, Serialize};
#[mcp_tool(
    name = "asset_get_schema_by_fxf",
    title = "Get Asset Schema by FXF",
    description = "Input: { fxf } – FXF identifier of the dataset. \
                   Output: The schema of the dataset (list of fields with names and types).",
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
    title = "Get Asset Schema by Iceberg Location",
    description = "Input: { location, schema_table } – Iceberg location and schemaTable of the dataset. \
                   Output: The schema of the dataset (list of fields with names and types).",
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
