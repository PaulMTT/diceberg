use anyhow::Result;
use datafusion::sql::TableReference;
use iceberg::TableIdent;
use iceberg_catalog_glue::GlueCatalog;
pub mod manually_sqlable;
pub mod sqlable;
pub mod table_source;
pub trait CatalogSource {
    fn catalog(&self) -> impl Future<Output = Result<GlueCatalog>>;
}
pub trait TableIdentitySource {
    fn table_ident(&self) -> impl Future<Output = Result<TableIdent>>;
}
pub trait TableReferenceSource {
    fn table_reference(&self) -> impl Future<Output = Result<TableReference>>;
}
