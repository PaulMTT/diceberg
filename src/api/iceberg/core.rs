use crate::api::iceberg::asset::CoreAsset;
use crate::api::iceberg::client::DicebergClient;
use crate::api::iceberg::traits::{ClientSource, TableIdentity};
use crate::api::management::client::DiciManagementClient;
use anyhow::{Context, Result};
use iceberg::TableIdent;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct DicebergClientCoreScoped {
    pub client: DicebergClient,
    pub asset: CoreAsset,
    #[builder(default)]
    pub management_client: DiciManagementClient,
}

impl TableIdentity for DicebergClientCoreScoped {
    async fn table_ident(&self) -> Result<TableIdent> {
        let inventory = self
            .management_client
            .fetch_inventory_by_fxf(self.asset.fxf.clone())
            .await
            .context("Could not fetch inventory")?;
        TableIdent::from_strs([
            inventory.id.iceberg_location.iceberg_location,
            inventory.id.schema_table.schema_table,
        ])
            .context("Failed to parse table ident from core asset")
    }
}

impl ClientSource for DicebergClientCoreScoped {
    fn client(&self) -> &DicebergClient {
        &self.client
    }
}
