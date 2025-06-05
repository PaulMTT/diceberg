use anyhow::Result;
use diceberg::api::client::asset::CoreAsset;
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergCoreAsset;
use diceberg::api::traits::TableSource;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let asset: DicebergCoreAsset =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let table = asset.table().await?;

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
