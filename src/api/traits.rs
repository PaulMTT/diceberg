use crate::api::client::base::DicebergClient;
use anyhow::{Context, Result};
use iceberg::spec::NestedFieldRef;
use iceberg::table::Table;
use iceberg::{Catalog, TableIdent};
use iceberg_catalog_glue::GlueCatalog;

pub trait ClientSource {
    fn client(&self) -> &DicebergClient;
}

pub trait CatalogSource {
    fn catalog(&self) -> impl Future<Output=Result<GlueCatalog>>;
}

pub trait TableIdentity {
    fn table_ident(&self) -> impl Future<Output=Result<TableIdent>>;
}

pub trait TableSource: TableIdentity + CatalogSource {
    fn table(&self) -> impl Future<Output=Result<Table>>;
    fn schema(&self) -> impl Future<Output=Result<Vec<NestedFieldRef>>>;
}

impl<T> TableSource for T
where
    T: TableIdentity + CatalogSource,
{
    async fn table(&self) -> Result<Table> {
        self.catalog()
            .await.context("Failed to construct catalog")?
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
        Ok(
            self.table().await?
                .metadata()
                .current_schema()
                .as_struct()
                .fields()
                .to_vec()
        )
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