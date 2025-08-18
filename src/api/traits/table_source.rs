use crate::api::traits::{CatalogSource, TableIdentitySource};
use anyhow::{Context, Result};
use iceberg::Catalog;
use iceberg::spec::NestedFieldRef;
use iceberg::table::Table;
pub trait TableSource: TableIdentitySource + CatalogSource {
    fn table(&self) -> impl Future<Output = Result<Table>>;
    fn schema(&self) -> impl Future<Output = Result<Vec<NestedFieldRef>>>;
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
