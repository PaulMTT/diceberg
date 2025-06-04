use anyhow::Result;
use diceberg::api::client::asset::{CoreAsset, IcebergAsset};
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergClientCoreScoped;
use diceberg::api::client::iceberg_scope::DicebergClientIcebergScoped;
use diceberg::api::traits::TableSource;

#[tokio::main]
async fn main() -> Result<()> {
    let client: DicebergClientCoreScoped =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let fields = client.schema().await?;
    fields.iter().for_each(|f| {
        println!("{:?}", f)
    });

    let client: DicebergClientIcebergScoped =
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
