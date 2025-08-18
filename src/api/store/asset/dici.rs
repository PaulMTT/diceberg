use crate::api::http::management::client::ManagementClient;
use crate::api::store::asset::core::{CoreAsset, CoreFxf};
use crate::api::store::asset::iceberg::{IcebergAsset, IcebergLocation, IcebergSchemaTable};
use crate::api::store::asset::traits::sqlable::TableReferenceSource;
use crate::api::store::asset::traits::table_source::TableIdentitySource;
use crate::api::store::catalog::catalog_source::CatalogSource;
use crate::api::store::catalog::dici::DiciCatalog;
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use iceberg::TableIdent;
use iceberg_catalog_glue::GlueCatalog;
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, Clone)]
pub struct CoreArgs {
    asset: CoreAsset,
    #[builder(default)]
    dici_catalog: DiciCatalog,
    #[builder(default)]
    management_client: ManagementClient,
}
impl Into<DiciAsset> for CoreArgs {
    fn into(self) -> DiciAsset {
        DiciAsset::Core(self)
    }
}
#[derive(TypedBuilder, Clone)]
pub struct IcebergArgs {
    asset: IcebergAsset,
    #[builder(default)]
    dici_catalog: DiciCatalog,
}
impl Into<DiciAsset> for IcebergArgs {
    fn into(self) -> DiciAsset {
        DiciAsset::Iceberg(self)
    }
}
pub enum DiciAsset {
    Core(CoreArgs),
    Iceberg(IcebergArgs),
}
impl DiciAsset {
    pub fn core(fxf: CoreFxf) -> Self {
        CoreArgs::builder()
            .asset(CoreAsset::builder().fxf(fxf).build())
            .build()
            .into()
    }
    pub fn iceberg(
        iceberg_location: IcebergLocation,
        iceberg_schema_table: IcebergSchemaTable,
    ) -> Self {
        IcebergArgs::builder()
            .asset(
                IcebergAsset::builder()
                    .location(iceberg_location)
                    .schema_table(iceberg_schema_table)
                    .build(),
            )
            .build()
            .into()
    }
}
impl CatalogSource for DiciAsset {
    fn catalog(&self) -> impl Future<Output = Result<GlueCatalog>> {
        let dici_catalog = match self {
            DiciAsset::Core(CoreArgs { dici_catalog, .. }) => dici_catalog,
            DiciAsset::Iceberg(IcebergArgs { dici_catalog, .. }) => dici_catalog,
        };
        dici_catalog.catalog()
    }
}
impl TableReferenceSource for DiciAsset {
    async fn table_reference(&self) -> Result<TableReference> {
        match self {
            DiciAsset::Core(CoreArgs { asset, .. }) => asset.table_reference().await,
            DiciAsset::Iceberg(IcebergArgs { asset, .. }) => asset.table_reference().await,
        }
    }
}
impl TableIdentitySource for DiciAsset {
    async fn table_ident(&self) -> Result<TableIdent> {
        match self {
            DiciAsset::Core(CoreArgs {
                asset: CoreAsset { fxf },
                management_client,
                ..
            }) => {
                let inventory = management_client
                    .fetch_inventory_by_fxf(fxf.into())
                    .await
                    .context("Could not fetch inventory")?;
                TableIdent::from_strs([
                    inventory.id.iceberg_location.iceberg_location,
                    inventory.id.schema_table.schema_table,
                ])
                .context("Failed to parse table ident from core asset")
            }
            DiciAsset::Iceberg(IcebergArgs {
                asset:
                    IcebergAsset {
                        location,
                        schema_table,
                    },
                ..
            }) => TableIdent::from_strs([location, schema_table])
                .context("Failed to parse table ident from iceberg asset"),
        }
    }
}
