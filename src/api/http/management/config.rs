use anyhow::Context;
use std::env;
use typed_builder::TypedBuilder;
fn management_address_from_env() -> anyhow::Result<String> {
    env::var("DICI_MANAGEMENT_ADDRESS").context("DICI_MANAGEMENT_ADDRESS is not set")
}
fn default_management_address() -> String {
    management_address_from_env()
        .context("Could not determine management address")
        .unwrap()
}
pub type ManagementAddress = String;
#[derive(TypedBuilder, Clone)]
pub struct ManagementConfig {
    #[builder(default = default_management_address(), setter(into))]
    pub address: ManagementAddress,
}
impl Default for ManagementConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
