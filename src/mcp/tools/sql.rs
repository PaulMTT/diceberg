use crate::api::client::asset::{CoreAsset, IcebergAsset};
use crate::api::client::DiciAsset;
use crate::api::traits::SqlAble;
use crate::mcp::handler::DiciServerHandlerState;
use crate::mcp::tools::{into_call_err, json_as_text, DiciCallableTool};
use arrow_json::ArrayWriter;
use datafusion::prelude::SQLOptions;
use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::schema_utils::CallToolError;
use rust_mcp_sdk::schema::CallToolResult;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[mcp_tool(
    name = "asset_execute_sql_by_fxf",
    title = "Execute SQL Query on Dataset by FXF",
    description = "Executes a SQL query against a public dataset identified by FXF. \
Input: `fxf` (string, `xxxx-xxxx`), `sql` (string, must use quoted `<fxf>` as table name). \
Output: JSON array of query results. \
Concepts: FXF resolves to an Inventory → Iceberg location mapping, enabling query execution on the underlying table.",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetExecuteSqlByFxf {
    /// Core FXF identifier in format `XXXX-XXXX`.
    pub fxf: String,
    /// SQL query to execute (use quoted `<fxf>` as the table name).
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
Input: `location` (string, `_` + 32 lowercase hex), `schema_table` (string), `sql` (string, must reference `<schema_table>` only). \
Output: JSON array of query results. \
Concepts: Direct access to Iceberg tables bypassing FXF resolution; suitable for internal workflows.",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AssetExecuteSqlByIceberg {
    /// Iceberg location string in the format `_` followed by 32 lowercase hex characters.
    pub location: String,
    /// Iceberg schema table name.
    pub schema_table: String,
    /// SQL query to execute (must reference `<schema_table>` only).
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
    let dataframe = asset.sql_with_options(sql, options).await?;
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
