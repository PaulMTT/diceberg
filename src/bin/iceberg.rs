use anyhow::Result;
use diceberg::api::client::asset::IcebergAsset;
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::iceberg_scope::DicebergIcebergAsset;
use diceberg::api::traits::{ClientSource, TableSource};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
        IcebergAsset::builder()
            .location("_ac642f8374a4a7c17e855f828c41cf48")
            .schema_table("dbo_vendors")
            .build(),
    );
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
