use crate::api::store::catalog::catalog_source::CatalogSource;
use anyhow::{Context, Result};
use iceberg::table::Table;
use iceberg::{Catalog, TableIdent};
pub trait TableIdentitySource {
    fn table_ident(&self) -> impl Future<Output = Result<TableIdent>>;
}
pub trait TableSource: TableIdentitySource + CatalogSource {
    fn table(&self) -> impl Future<Output = Result<Table>>;
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
}
