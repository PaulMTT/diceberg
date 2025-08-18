use crate::api::store::asset::traits::table_source::TableSource;
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::{SQLOptions, SessionContext};
use iceberg::table::Table;
use iceberg_datafusion::IcebergTableProvider;
use std::sync::Arc;
pub trait TableReferenceSource {
    fn table_reference(&self) -> impl Future<Output = Result<TableReference>>;
}
pub trait SqlAble: TableSource + TableReferenceSource {
    fn context(&self) -> impl Future<Output = Result<SessionContext>>;
    fn sql(&self, sql: &str) -> impl Future<Output = Result<DataFrame>>;
    fn sql_with_options(
        &self,
        sql: &str,
        options: SQLOptions,
    ) -> impl Future<Output = Result<DataFrame>>;
}
impl<T> SqlAble for T
where
    T: TableSource + TableReferenceSource,
{
    async fn context(&self) -> Result<SessionContext> {
        let table: Table = self.table().await?;
        let ctx = SessionContext::new();
        let table_provider = Arc::new(IcebergTableProvider::try_new_from_table(table).await?);
        ctx.register_table(
            self.table_reference()
                .await
                .context("Failed to get table reference")?,
            table_provider,
        )
        .context("Failed to register table")?;
        Ok(ctx)
    }
    async fn sql(&self, sql: &str) -> Result<DataFrame> {
        self.context()
            .await
            .context("Failed to get session context")?
            .sql(sql)
            .await
            .context("Failed to execute query")
    }
    async fn sql_with_options(&self, sql: &str, options: SQLOptions) -> Result<DataFrame> {
        self.context()
            .await
            .context("Failed to get session context")?
            .sql_with_options(sql, options)
            .await
            .context("Failed to execute query")
    }
}
