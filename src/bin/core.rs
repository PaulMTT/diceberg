use anyhow::Result;
use diceberg::api::client::asset::CoreAsset;
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergClientCoreScoped;
use diceberg::api::traits::TableSource;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client: DicebergClientCoreScoped =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let table = client.table().await?;

    let metadata = table.metadata();
    let schema = metadata.current_schema();
    let struct_type = schema.as_struct();
    struct_type.fields().iter().for_each(|f| {
        println!("{:?}", f)
    });


    let stream = table
        .scan()
        .select(["vendorname"])
        .build()?
        .to_arrow()
        .await?;

    let _data: Vec<_> = stream.try_collect().await?;
    println!("{:?}", _data);
    Ok(())
}
