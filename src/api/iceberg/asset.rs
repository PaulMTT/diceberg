use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Clone)]
pub struct CoreAsset {
    #[builder(setter(into))]
    pub fxf: String,
}

#[derive(TypedBuilder, Clone)]
pub struct IcebergAsset {
    #[builder(setter(into))]
    pub location: String,
    #[builder(setter(into))]
    pub schema_table: String,
}