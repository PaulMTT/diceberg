use crate::api::store::asset::traits::sqlable::TableReferenceSource;
use anyhow::Result;
use datafusion::common::TableReference;
use typed_builder::TypedBuilder;
pub type CoreFxf = String;
#[derive(TypedBuilder, Clone)]
pub struct CoreAsset {
    #[builder(setter(into))]
    pub fxf: CoreFxf,
}
impl TableReferenceSource for CoreAsset {
    async fn table_reference(&self) -> Result<TableReference> {
        Ok(TableReference::Bare {
            table: self.fxf.as_str().into(),
        })
    }
}
