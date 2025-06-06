use crate::api::management::inventory::Inventory;
use crate::api::management::registration::Registration;
use anyhow::{anyhow, Context, Result};
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
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
    pub async fn fetch_inventories(&self) -> Result<Vec<Inventory>> {
        self.http_client
            .get(format!("{}/inventory", self.management_address.clone(),))
            .send()
            .await
            .context("Request to dici management failed")?
            .json::<Vec<Inventory>>()
            .await
            .context("Deserializing dici management response failed")
    }

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

    pub async fn fetch_inventories_by_iceberg_location(
        &self,
        iceberg_location: String,
    ) -> Result<Vec<Inventory>> {
        self.http_client
            .get(format!(
                "{}/inventory/iceberg/{}",
                self.management_address.clone(),
                iceberg_location
            ))
            .send()
            .await
            .context("Request to dici management failed")?
            .json::<Vec<Inventory>>()
            .await
            .context("Deserializing dici management response failed")
    }

    pub async fn fetch_registrations(&self) -> Result<Vec<Registration>> {
        self.http_client
            .get(format!("{}/registration", self.management_address.clone()))
            .send()
            .await
            .context("Request to dici management failed")?
            .json::<Vec<Registration>>()
            .await
            .context("Deserializing dici management response failed")
    }

    pub async fn fetch_registrations_by_path(&self, path: String) -> Result<Vec<Registration>> {
        let response = self
            .http_client
            .get(format!(
                "{}/registration/{}",
                self.management_address.clone(),
                path
            ))
            .send()
            .await
            .context("Request to dici management failed")?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Registration>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }

    pub async fn fetch_registrations_by_path_and_metadata(
        &self,
        path: String,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<Registration>> {
        let response = self
            .http_client
            .post(format!("{}/query/{}", self.management_address, path))
            .json(&metadata)
            .send()
            .await
            .context("Request to dici management failed")?;

        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Registration>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
}
