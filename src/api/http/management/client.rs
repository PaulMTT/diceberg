use crate::api::http::management::config::ManagementConfig;
use crate::api::http::management::model::inventory::Inventory;
use crate::api::http::management::model::registration::Registration;
use crate::api::http::management::model::sync::IcebergLocationSync;
use crate::api::http::management::model::version::GitConfig;
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use typed_builder::TypedBuilder;
#[derive(TypedBuilder, Clone)]
pub struct ManagementClient {
    #[builder(default)]
    http_client: Client,
    #[builder(default)]
    config: ManagementConfig,
}
impl Default for ManagementClient {
    fn default() -> Self {
        Self::builder().build()
    }
}
impl ManagementClient {
    pub async fn fetch_inventories(&self) -> Result<Vec<Inventory>> {
        self.http_client
            .get(format!("{}/inventory", self.config.address))
            .send()
            .await
            .context("Request to dici management failed")?
            .json::<Vec<Inventory>>()
            .await
            .context("Deserializing dici management response failed")
    }
    pub async fn fetch_inventory_by_fxf(&self, fxf: String) -> Result<Inventory> {
        let response = self
            .http_client
            .get(format!("{}/inventory/fxf/{}", self.config.address, fxf))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("Inventory not found")),
            status if status.is_success() => response
                .json::<Inventory>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_inventories_by_iceberg_location(
        &self,
        iceberg_location: String,
    ) -> Result<Vec<Inventory>> {
        let response = self
            .http_client
            .get(format!(
                "{}/inventory/iceberg/{}",
                self.config.address, iceberg_location
            ))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Inventory>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_registrations(&self) -> Result<Vec<Registration>> {
        self.http_client
            .get(format!("{}/registration", self.config.address))
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
            .get(format!("{}/query/{}", self.config.address, path))
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
            .post(format!("{}/query/{}", self.config.address, path))
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
    pub async fn fetch_registration_by_iceberg_location(
        &self,
        iceberg_location: String,
    ) -> Result<Registration> {
        let response = self
            .http_client
            .get(format!(
                "{}/registration/iceberg/{}",
                self.config.address, iceberg_location
            ))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("Registration not found")),
            status if status.is_success() => response
                .json::<Registration>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_inventories_by_domain(&self, domain: String) -> Result<Vec<Inventory>> {
        let response = self
            .http_client
            .get(format!(
                "{}/inventory/domain/{}",
                self.config.address, domain
            ))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Inventory>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_version(&self) -> Result<GitConfig> {
        let response = self
            .http_client
            .get(format!("{}/version", self.config.address))
            .send()
            .await
            .context("Request to dici management /version failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("Version information not found")),
            status if status.is_success() => response
                .json::<GitConfig>()
                .await
                .context("Deserializing /version response into GitConfig failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_inventories_updated_since(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<Inventory>> {
        let response = self
            .http_client
            .get(format!("{}/inventory/updated", self.config.address))
            .query(&[("since", since.to_rfc3339())])
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Inventory>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_inventories_by_iceberg_location_and_table(
        &self,
        iceberg_location: String,
        schema_table: String,
    ) -> Result<Vec<Inventory>> {
        let response = self
            .http_client
            .get(format!(
                "{}/inventory/iceberg/{}/{}",
                self.config.address, iceberg_location, schema_table
            ))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Inventory>>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn fetch_inventory_by_id(
        &self,
        domain: String,
        iceberg_location: String,
        schema_table: String,
    ) -> Result<Inventory> {
        let response = self
            .http_client
            .get(format!(
                "{}/inventory/id/{}/{}/{}",
                self.config.address, domain, iceberg_location, schema_table
            ))
            .send()
            .await
            .context("Request to dici management failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!(
                "Inventory not found for domain={}, location={}, table={}",
                domain,
                iceberg_location,
                schema_table
            )),
            status if status.is_success() => response
                .json::<Inventory>()
                .await
                .context("Deserializing dici management response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn sync_table(
        &self,
        iceberg_location: String,
        schema_table: String,
    ) -> Result<Vec<Inventory>> {
        let body = serde_json::json!({
            "icebergLocation": iceberg_location,
            "schemaTable": schema_table,
        });
        let response = self
            .http_client
            .post(format!("{}/sync", self.config.address))
            .json(&body)
            .send()
            .await
            .context("Request to /sync failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Ok(vec![]),
            status if status.is_success() => response
                .json::<Vec<Inventory>>()
                .await
                .context("Deserializing /sync response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn sync_table_domain(
        &self,
        domain: String,
        iceberg_location: String,
        schema_table: String,
    ) -> Result<Inventory> {
        let body = serde_json::json!({
            "domain": domain,
            "icebergLocation": iceberg_location,
            "schemaTable": schema_table,
        });
        let response = self
            .http_client
            .post(format!("{}/sync/domain", self.config.address))
            .json(&body)
            .send()
            .await
            .context("Request to /sync/domain failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("Inventory not found for domain sync")),
            status if status.is_success() => response
                .json::<Inventory>()
                .await
                .context("Deserializing /sync/domain response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
    pub async fn sync_iceberg_location(
        &self,
        iceberg_location: String,
    ) -> Result<IcebergLocationSync> {
        let response = self
            .http_client
            .get(format!(
                "{}/sync/iceberg/{}",
                self.config.address, iceberg_location
            ))
            .send()
            .await
            .context("Request to /sync/iceberg/{location} failed")?;
        match response.status() {
            StatusCode::NOT_FOUND => Err(anyhow!("Iceberg location not found")),
            status if status.is_success() => response
                .json::<IcebergLocationSync>()
                .await
                .context("Deserializing /sync/iceberg response failed"),
            status => Err(anyhow!("Unexpected status code: {}", status)),
        }
    }
}
