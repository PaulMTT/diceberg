use crate::api::traits::table_source::TableSource;
use anyhow::{Context, Result};
use datafusion::dataframe::DataFrame;
use datafusion::prelude::{SQLOptions, SessionContext};
use datafusion::sql::TableReference;
use iceberg::table::Table;
use iceberg_datafusion::IcebergTableProvider;
use std::sync::Arc;
pub trait ManuallySqlAble: TableSource {
    fn context_with_table_reference(
        &self,
        table_reference: TableReference,
    ) -> impl Future<Output = Result<SessionContext>>;
    fn sql_with_table_reference(
        &self,
        sql: &str,
        table_reference: TableReference,
    ) -> impl Future<Output = Result<DataFrame>>;
    fn sql_with_table_reference_and_options(
        &self,
        sql: &str,
        table_reference: TableReference,
        options: SQLOptions,
    ) -> impl Future<Output = Result<DataFrame>>;
}
impl<T> ManuallySqlAble for T
where
    T: TableSource,
{
    async fn context_with_table_reference(
        &self,
        table_reference: TableReference,
    ) -> Result<SessionContext> {
        let table: Table = self.table().await?;
        let ctx = SessionContext::new();
        let table_provider = Arc::new(IcebergTableProvider::try_new_from_table(table).await?);
        ctx.register_table(table_reference, table_provider)
            .context("Failed to register table")?;
        Ok(ctx)
    }
    async fn sql_with_table_reference(
        &self,
        sql: &str,
        table_reference: TableReference,
    ) -> Result<DataFrame> {
        self.context_with_table_reference(table_reference)
            .await
            .context("Failed to get session context")?
            .sql(sql)
            .await
            .context("Failed to execute query")
    }
    async fn sql_with_table_reference_and_options(
        &self,
        sql: &str,
        table_reference: TableReference,
        options: SQLOptions,
    ) -> Result<DataFrame> {
        self.context_with_table_reference(table_reference)
            .await
            .context("Failed to get session context")?
            .sql_with_options(sql, options)
            .await
            .context("Failed to execute query")
    }
}
