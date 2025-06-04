use crate::api::client::asset::IcebergAsset;
use crate::api::client::base::DicebergClient;
use crate::api::traits::{ClientSource, TableIdentity};
use anyhow::{Context, Result};
use iceberg::TableIdent;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct DicebergIcebergAsset {
    client: DicebergClient,
    asset: IcebergAsset,
}

impl TableIdentity for DicebergIcebergAsset {
    async fn table_ident(&self) -> Result<TableIdent> {
        TableIdent::from_strs([&self.asset.location, &self.asset.schema_table])
            .context("Failed to parse table ident from iceberg asset")
    }
}

impl ClientSource for DicebergIcebergAsset {
    fn client(&self) -> &DicebergClient {
        &self.client
    }
}
