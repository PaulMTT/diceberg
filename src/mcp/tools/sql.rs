use crate::api::client::DiciAsset;
use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::traits::ManuallySqlAble;
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
    title = "Execute SQL Query on Dataset by FXF",
    description = "Executes a SQL query against a public dataset identified by FXF. \
Input: `fxf` (string, `xxxx-xxxx`), `sql` (string, must always use table name `this`). \
Output: JSON array of query results. \
Concepts: FXF resolves to an Inventory → Iceberg location mapping. All queries must refer only to `this`, \
which represents the resolved dataset table.",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
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
        let asset = DiciAsset::Core {
            asset: CoreAsset::builder().fxf(&self.fxf).build(),
            dici_client: state.dici_client.clone(),
            management_client: state.management_client.clone(),
        };
        let x: Vec<Value> = run_sql_and_return_json(&asset, &self.sql)
            .await
            .map_err(into_call_err)?;
        json_as_text(&x)
    }
}
#[mcp_tool(
    name = "asset_execute_sql_by_iceberg",
    title = "Execute SQL Query on Dataset by Iceberg Location",
    description = "Executes a SQL query directly against an Iceberg table. \
Input: `location` (string, `_` + 32 lowercase hex), `schema_table` (string), `sql` (string, must always use table name `this`). \
Output: JSON array of query results. \
Concepts: Direct access to Iceberg tables bypassing FXF resolution. All queries must refer only to `this`, \
which represents the bound Iceberg table.",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
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
        let asset = DiciAsset::Iceberg {
            asset: IcebergAsset::builder()
                .location(&self.location)
                .schema_table(&self.schema_table)
                .build(),
            client: state.dici_client.clone(),
        };
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
