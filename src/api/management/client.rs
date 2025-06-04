use crate::api::management::inventory::Inventory;
use anyhow::{Context, Result};
use reqwest::Client;
use std::env;
use typed_builder::TypedBuilder;

fn management_address_from_env() -> Result<String> {
    env::var("DICI_MANAGEMENT_ADDRESS").context("DICI_MANAGEMENT_ADDRESS is not set")
}

fn default_management_address() -> String {
    management_address_from_env()
        .context("Could not determine management address")
        .unwrap()
}

#[derive(TypedBuilder, Clone)]
pub struct DiciManagementClient {
    #[builder(default)]
    http_client: Client,
    #[builder(default = default_management_address(), setter(into))]
    management_address: String,
}

impl Default for DiciManagementClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl DiciManagementClient {
    pub async fn fetch_inventory_by_fxf(&self, fxf: String) -> Result<Inventory> {
        self.http_client
            .get(format!(
                "{}/inventory/fxf/{}",
                self.management_address.clone(),
                fxf
            ))
            .send()
            .await
            .context("Request to dici management failed")?
            .json::<Inventory>()
            .await
            .context("Deserializing dici management response failed")
    }
}
