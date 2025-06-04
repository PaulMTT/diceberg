use anyhow::Result;
use diceberg::api::iceberg::asset::IcebergAsset;
use diceberg::api::iceberg::client::DicebergClient;
use diceberg::api::iceberg::iceberg::DicebergClientIcebergScoped;
use diceberg::api::iceberg::traits::TableSource;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client: DicebergClientIcebergScoped = DicebergClient::default().iceberg(
        IcebergAsset::builder()
            .location("_ac642f8374a4a7c17e855f828c41cf48")
            .schema_table("dbo_vendors")
            .build(),
    );
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
