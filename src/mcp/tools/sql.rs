use crate::api::store::asset::core::CoreAsset;
use crate::api::store::asset::dici::{CoreArgs, DiciAsset, IcebergArgs};
use crate::api::store::asset::iceberg::IcebergAsset;
use crate::api::store::asset::traits::manually_sqlable::ManuallySqlAble;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{DiciCallableTool, into_call_err, json_as_text};
use arrow_json::ArrayWriter;
use datafusion::prelude::SQLOptions;
use datafusion::sql::TableReference;
use rust_mcp_sdk::macros::{JsonSchema, mcp_tool};
use rust_mcp_sdk::schema::CallToolResult;
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[mcp_tool(
    name = "asset_execute_sql_by_fxf",
    title = "Execute SQL on Asset by FXF",
    description = "Input: { fxf, sql } – FXF identifier of the dataset and an SQL query string. \
                   The SQL must reference the dataset as the table name \"this\". \
                   Output: Query results as JSON values."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetExecuteSqlByFxf {
    pub fxf: String,
    pub sql: String,
}
impl DiciCallableTool for AssetExecuteSqlByFxf {
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
        let x: Vec<Value> = run_sql_and_return_json(&asset, &self.sql)
            .await
            .map_err(into_call_err)?;
        json_as_text(&x)
    }
}
#[mcp_tool(
    name = "asset_execute_sql_by_iceberg",
    title = "Execute SQL on Asset by Iceberg Location",
    description = "Input: { location, schema_table, sql } – Iceberg location, schemaTable, and an SQL query string. \
                   The SQL must reference the dataset as the table name \"this\". \
                   Output: Query results as JSON values."
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetExecuteSqlByIceberg {
    pub location: String,
    pub schema_table: String,
    pub sql: String,
}
impl DiciCallableTool for AssetExecuteSqlByIceberg {
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
        let x: Vec<Value> = run_sql_and_return_json(&asset, &self.sql)
            .await
            .map_err(into_call_err)?;
        json_as_text(&x)
    }
}
async fn run_sql_and_return_json<T>(asset: &DiciAsset, sql: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let options = SQLOptions::new()
        .with_allow_ddl(false)
        .with_allow_dml(false)
        .with_allow_statements(false);
    let table_reference = TableReference::Bare {
        table: "this".into(),
    };
    let dataframe = asset
        .sql_with_table_reference_and_options(sql, table_reference, options)
        .await?;
    let results = dataframe.collect().await?;
    let mut buf = Vec::new();
    {
        let mut writer = ArrayWriter::new(&mut buf);
        for batch in &results {
            writer.write(batch)?;
        }
        writer.finish()?;
    }
    let json_values: T = serde_json::from_slice(&buf)?;
    Ok(json_values)
}
