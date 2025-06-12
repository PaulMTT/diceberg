use typed_builder::TypedBuilder;

pub type CoreFxf = String;
pub type IcebergLocation = String;
pub type IcebergSchemaTable = String;

#[derive(TypedBuilder, Clone)]
pub struct CoreAsset {
    #[builder(setter(into))]
    pub fxf: CoreFxf,
}

#[derive(TypedBuilder, Clone)]
pub struct IcebergAsset {
    #[builder(setter(into))]
    pub location: IcebergLocation,
    #[builder(setter(into))]
    pub schema_table: IcebergSchemaTable,
}
