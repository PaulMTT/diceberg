use anyhow::Result;
use diceberg::api::client::asset::{CoreAsset, IcebergAsset};
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergCoreAsset;
use diceberg::api::client::iceberg_scope::DicebergIcebergAsset;
use diceberg::api::traits::TableSource;

#[tokio::main]
async fn main() -> Result<()> {
    let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
        IcebergAsset::builder()
            .location("_ac642f8374a4a7c17e855f828c41cf48")
            .schema_table("dbo_vendors")
            .build(),
    );

    let records = asset
        .sql("select count(vendorname) from dbo_vendors")
        .await?
        .explain(false, false)?
        .collect()
        .await?;
    assert_eq!(1, records.len());

    let asset: DicebergCoreAsset =
        DicebergClient::default().core(CoreAsset::builder().fxf("yfc6-7rgw").build());

    let records = asset
        .sql("select count(vendorname) from 'yfc6-7rgw'")
        .await?
        .explain(false, false)?
        .collect()
        .await?;
    assert_eq!(1, records.len());

    Ok(())
}
