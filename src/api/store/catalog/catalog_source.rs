use iceberg_catalog_glue::GlueCatalog;
pub trait CatalogSource {
    fn catalog(&self) -> impl Future<Output = anyhow::Result<GlueCatalog>>;
}
