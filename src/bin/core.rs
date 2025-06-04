use anyhow::Result;
use diceberg::api::iceberg::asset::CoreAsset;
use diceberg::api::iceberg::client::DicebergClient;
use diceberg::api::iceberg::core::DicebergClientCoreScoped;
use diceberg::api::iceberg::traits::TableSource;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client: DicebergClientCoreScoped =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let table = client.table().await?;

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
