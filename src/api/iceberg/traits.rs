use crate::api::iceberg::client::DicebergClient;
use anyhow::{Context, Result};
use iceberg::table::Table;
use iceberg::{Catalog, TableIdent};
use iceberg_catalog_glue::GlueCatalog;

pub trait ClientSource {
    fn client(&self) -> &DicebergClient;
}

pub trait CatalogSource {
    fn catalog(&self) -> impl Future<Output=Result<GlueCatalog>> + Send;
}

pub trait TableIdentity {
    fn table_ident(&self) -> impl Future<Output=Result<TableIdent>> + Send;
}

pub trait TableSource: TableIdentity + CatalogSource {
    fn table(&self) -> impl Future<Output=Result<Table>> + Send;
}

impl<T> TableSource for T
where
    T: TableIdentity + CatalogSource + Sync,
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
}


impl<T> CatalogSource for T
where
    T: ClientSource + Sync,
{
    async fn catalog(&self) -> Result<GlueCatalog> {
        self.client().catalog().await
    }
}