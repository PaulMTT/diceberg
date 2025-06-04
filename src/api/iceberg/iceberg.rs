use crate::api::iceberg::asset::IcebergAsset;
use crate::api::iceberg::client::DicebergClient;
use crate::api::iceberg::traits::{ClientSource, TableIdentity};
use anyhow::{Context, Result};
use iceberg::TableIdent;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct DicebergClientIcebergScoped {
    pub client: DicebergClient,
    pub asset: IcebergAsset,
}

impl TableIdentity for DicebergClientIcebergScoped {
    async fn table_ident(&self) -> Result<TableIdent> {
        TableIdent::from_strs([&self.asset.location, &self.asset.schema_table])
            .context("Failed to parse table ident from iceberg asset")
    }
}

impl ClientSource for DicebergClientIcebergScoped {
    fn client(&self) -> &DicebergClient {
        &self.client
    }
}
