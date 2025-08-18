use crate::api::traits::TableReferenceSource;
use datafusion::common::TableReference;
use typed_builder::TypedBuilder;
pub type IcebergLocation = String;
pub type IcebergSchemaTable = String;
#[derive(TypedBuilder, Clone)]
pub struct IcebergAsset {
    #[builder(setter(into))]
    pub location: IcebergLocation,
    #[builder(setter(into))]
    pub schema_table: IcebergSchemaTable,
}
impl TableReferenceSource for IcebergAsset {
    async fn table_reference(&self) -> anyhow::Result<TableReference> {
        Ok(TableReference::Bare {
            table: self.schema_table.as_str().into(),
        })
    }
}
