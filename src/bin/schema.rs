use anyhow::Result;
use diceberg::api::client::asset::{CoreAsset, IcebergAsset};
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergCoreAsset;
use diceberg::api::client::iceberg_scope::DicebergIcebergAsset;
use diceberg::api::traits::TableSource;

#[tokio::main]
async fn main() -> Result<()> {
    let asset: DicebergCoreAsset =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let fields = asset.schema().await?;
    fields.iter().for_each(|f| {
        println!("{:?}", f)
    });

    let client: DicebergIcebergAsset =
        DicebergClient::default().iceberg(
            IcebergAsset::builder()
                .location("_ac642f8374a4a7c17e855f828c41cf48")
                .schema_table("dbo_vendors")
                .build()
        );

    let fields = client.schema().await?;
    fields.iter().for_each(|f| {
        println!("{:?}", f)
    });
    Ok(())
}
