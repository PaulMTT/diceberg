use crate::api::dici::config::DiciConfig;
use crate::api::traits::CatalogSource;
use anyhow::Context;
use anyhow::Result;
use iceberg_catalog_glue::{GlueCatalog, GlueCatalogConfig};
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, Clone)]
pub struct DiciCatalog {
    #[builder(default)]
    config: DiciConfig,
}
impl Default for DiciCatalog {
    fn default() -> Self {
        Self::builder().build()
    }
}
impl CatalogSource for DiciCatalog {
    async fn catalog(&self) -> Result<GlueCatalog> {
        GlueCatalog::new(
            GlueCatalogConfig::builder()
                .warehouse(self.config.warehouse.clone())
                .build(),
        )
        .await
        .context("Failed to construct glue catalog")
    }
}
