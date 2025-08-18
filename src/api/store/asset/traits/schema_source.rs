use crate::api::store::asset::traits::table_source::TableSource;
use iceberg::spec::NestedFieldRef;

pub trait SchemaSource: TableSource {
    fn schema(&self) -> impl Future<Output = anyhow::Result<Vec<NestedFieldRef>>>;
}

impl<T> SchemaSource for T
where
    T: TableSource,
{
    async fn schema(&self) -> anyhow::Result<Vec<NestedFieldRef>> {
        Ok(self
            .table()
            .await?
            .metadata()
            .current_schema()
            .as_struct()
            .fields()
            .to_vec())
    }
}
