use crate::api::client::{DiciAsset, DiciClient};
use crate::api::traits::{ClientSource, TableIdentitySource, TableReferenceSource};
use anyhow::{Context, Result};
use datafusion::common::TableReference;
use iceberg::TableIdent;
use typed_builder::TypedBuilder;
pub type CoreFxf = String;
pub type IcebergLocation = String;
pub type IcebergSchemaTable = String;
#[derive(TypedBuilder, Clone)]
pub struct CoreAsset {
    #[builder(setter(into))]
    pub fxf: CoreFxf,
}
#[derive(TypedBuilder, Clone)]
pub struct IcebergAsset {
    #[builder(setter(into))]
    pub location: IcebergLocation,
    #[builder(setter(into))]
    pub schema_table: IcebergSchemaTable,
}
impl ClientSource for DiciAsset {
    fn client(&self) -> &DiciClient {
        match self {
            DiciAsset::Core {
                dici_client: client,
                ..
            } => client,
            DiciAsset::Iceberg { client, .. } => client,
        }
    }
}
impl TableReferenceSource for DiciAsset {
    async fn table_reference(&self) -> Result<TableReference> {
        Ok(match self {
            DiciAsset::Core {
                asset: CoreAsset { fxf },
                ..
            } => TableReference::Bare {
                table: fxf.as_str().into(),
            },
            DiciAsset::Iceberg {
                asset: IcebergAsset { schema_table, .. },
                ..
            } => TableReference::Bare {
                table: schema_table.as_str().into(),
            },
        })
    }
}
impl TableIdentitySource for DiciAsset {
    async fn table_ident(&self) -> Result<TableIdent> {
        match self {
            DiciAsset::Core {
                asset: CoreAsset { fxf },
                management_client,
                ..
            } => {
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
            DiciAsset::Iceberg {
                asset:
                    IcebergAsset {
                        location,
                        schema_table,
                    },
                ..
            } => TableIdent::from_strs([location, schema_table])
                .context("Failed to parse table ident from iceberg asset"),
        }
    }
}
