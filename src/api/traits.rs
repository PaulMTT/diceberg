use crate::api::client::DiciClient;
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::{SQLOptions, SessionContext};
use iceberg::spec::NestedFieldRef;
use iceberg::table::Table;
use iceberg::{Catalog, TableIdent};
use iceberg_catalog_glue::GlueCatalog;
use iceberg_datafusion::IcebergTableProvider;
use std::sync::Arc;

pub trait ClientSource {
    fn client(&self) -> &DiciClient;
}

pub trait CatalogSource {
    fn catalog(&self) -> impl Future<Output = Result<GlueCatalog>>;
}

pub trait TableIdentitySource {
    fn table_ident(&self) -> impl Future<Output = Result<TableIdent>>;
}

pub trait TableReferenceSource {
    fn table_reference(&self) -> impl Future<Output = Result<TableReference>>;
}

pub trait TableSource: TableIdentitySource + CatalogSource {
    fn table(&self) -> impl Future<Output = Result<Table>>;
    fn schema(&self) -> impl Future<Output = Result<Vec<NestedFieldRef>>>;
}

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

pub trait SqlAble: TableSource + TableReferenceSource {
    fn context(&self) -> impl Future<Output = Result<SessionContext>>;

    fn sql(&self, sql: &str) -> impl Future<Output = Result<DataFrame>>;

    fn sql_with_options(
        &self,
        sql: &str,
        options: SQLOptions,
    ) -> impl Future<Output = Result<DataFrame>>;
}

impl<T> TableSource for T
where
    T: TableIdentitySource + CatalogSource,
{
    async fn table(&self) -> Result<Table> {
        self.catalog()
            .await
            .context("Failed to construct catalog")?
            .load_table(
                &self
                    .table_ident()
                    .await
                    .context("Failed to construct table ident")?,
            )
            .await
            .context("Failed to load table")
    }

    async fn schema(&self) -> Result<Vec<NestedFieldRef>> {
        Ok(self
            .table()
            .await?
            .metadata()
            .current_schema()
            .as_struct()
            .fields()
            .to_vec())
    }
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

impl<T> CatalogSource for T
where
    T: ClientSource,
{
    async fn catalog(&self) -> Result<GlueCatalog> {
        self.client().catalog().await
    }
}
