use anyhow::Context;
use std::env;
use typed_builder::TypedBuilder;
fn warehouse_from_env() -> anyhow::Result<String> {
    env::var("DICI_WAREHOUSE").context("DICI_WAREHOUSE is not set")
}
fn default_warehouse() -> String {
    warehouse_from_env()
        .context("Could not determine warehouse")
        .unwrap()
}
pub type Warehouse = String;
#[derive(TypedBuilder, Clone)]
pub struct DiciConfig {
    #[builder(default = default_warehouse(), setter(into))]
    pub warehouse: Warehouse,
}
impl Default for DiciConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
