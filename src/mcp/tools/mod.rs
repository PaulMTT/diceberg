use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::asset::{AssetGetSchemaByFxf, AssetGetSchemaByIceberg};
use crate::mcp::tools::datetime::GetDateTimeTool;
use crate::mcp::tools::management::{
    InventoryGetByFxf, InventoryGetById, InventoryListByDomain, InventoryListByIcebergLocation,
    InventoryListByIcebergLocationAndTable, InventoryListUpdatedSince,
    RegistrationGetByIcebergLocation, RegistrationListByPath, RegistrationQueryByPathAndMetadata,
    VersionGet,
};
use crate::mcp::tools::sql::{AssetExecuteSqlByFxf, AssetExecuteSqlByIceberg};
use arrow::record_batch::RecordBatch;
use arrow_json::ArrayWriter;
use rust_mcp_sdk::schema::schema_utils::{CallToolError, SdkError};
use rust_mcp_sdk::schema::{CallToolResult, TextContent};
use rust_mcp_sdk::tool_box;
use serde::Serialize;
use serde_json::Value;
pub mod asset;
pub mod datetime;
pub mod management;
pub mod sql;
pub fn into_call_err<E: std::fmt::Display>(e: E) -> CallToolError {
    CallToolError::new(SdkError::internal_error().with_message(&e.to_string()))
}
pub fn json_as_text<T: Serialize>(value: &T) -> Result<CallToolResult, CallToolError> {
    let pretty_json = serde_json::to_string_pretty(value).map_err(into_call_err)?;
    Ok(CallToolResult::text_content(vec![TextContent::from(
        pretty_json,
    )]))
}
pub fn record_batches_to_json_values(batches: &[RecordBatch]) -> anyhow::Result<Vec<Value>> {
    let mut buf = Vec::new();
    {
        let mut writer = ArrayWriter::new(&mut buf);
        for batch in batches {
            writer.write(batch)?;
        }
        writer.finish()?;
    }
    let values: Vec<Value> = serde_json::from_slice(&buf)?;
    Ok(values)
}
pub trait DiciCallableTool {
    fn call_tool(
        &self,
        state: &DiciServerHandlerState,
    ) -> impl Future<Output = anyhow::Result<CallToolResult, CallToolError>>;
}
#[macro_export]
macro_rules! tool_box_with_dispatch {
    ($name:ident, [ $( $variant:ident ),+ $(,)? ]) => {
        tool_box!($name, [ $( $variant ),+ ]);
        impl $crate::mcp::tools::DiciCallableTool for $name {
            fn call_tool(
                &self,
                state: &$crate::mcp::handler::DiciServerHandlerState,
            ) -> impl std::future::Future<
                Output = Result<
                    rust_mcp_sdk::schema::CallToolResult,
                    rust_mcp_sdk::schema::schema_utils::CallToolError
                >
            > {
                async move {
                    match self {
                        $(
                            Self::$variant(inner) =>
                                inner.call_tool(state).await,
                        )+
                    }
                }
            }
        }
    };
}
tool_box_with_dispatch!(
    DiciToolBox,
    [
        GetDateTimeTool,
        InventoryGetByFxf,
        InventoryListByIcebergLocation,
        InventoryListByDomain,
        InventoryListUpdatedSince,
        InventoryListByIcebergLocationAndTable,
        InventoryGetById,
        RegistrationGetByIcebergLocation,
        RegistrationListByPath,
        RegistrationQueryByPathAndMetadata,
        VersionGet,
        AssetGetSchemaByFxf,
        AssetGetSchemaByIceberg,
        AssetExecuteSqlByIceberg,
        AssetExecuteSqlByFxf
    ]
);
