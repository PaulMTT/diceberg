use anyhow::Result;
use arrow_json::ArrayWriter;
use clap::Parser;
use diceberg::api::client::asset::{CoreAsset, IcebergAsset};
use diceberg::api::client::base::DicebergClient;
use diceberg::api::client::core_scope::DicebergCoreAsset;
use diceberg::api::client::iceberg_scope::DicebergIcebergAsset;
use diceberg::api::traits::TableSource;
use diceberg::cli::{
    Cli, Commands, InfoAsset, InfoCoreArgs, InfoIcebergArgs, InfoKind, SqlAsset, SqlCoreArgs,
    SqlIcebergArgs,
};
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { kind } => handle_info(kind).await?,
        Commands::Sql { asset } => handle_sql(asset).await?,
    }
    Ok(())
}

async fn handle_info(kind: InfoKind) -> Result<()> {
    match kind {
        InfoKind::Schema { asset } => handle_info_schema(asset).await?,
    }
    Ok(())
}

async fn handle_info_schema(asset: InfoAsset) -> Result<()> {
    match asset {
        InfoAsset::Core(InfoCoreArgs { fxf }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let fields = asset.schema().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &fields)?;
        }
        InfoAsset::Iceberg(InfoIcebergArgs {
            location,
            schema_table,
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            let fields = asset.schema().await?;
            serde_json::to_writer_pretty(std::io::stdout(), &fields)?;
        }
    }
    Ok(())
}

async fn handle_sql(sql_command: SqlAsset) -> Result<()> {
    match sql_command {
        SqlAsset::Core(SqlCoreArgs { fxf, query }) => {
            let asset: DicebergCoreAsset =
                DicebergClient::default().core(CoreAsset::builder().fxf(fxf).build());
            let records = asset.sql(query.as_str()).await?.collect().await?;
            let mut writer = ArrayWriter::new(io::stdout());
            writer.write_batches(&records.iter().collect::<Vec<_>>())?;
            writer.finish()?;
        }
        SqlAsset::Iceberg(SqlIcebergArgs {
            location,
            schema_table,
            query,
        }) => {
            let asset: DicebergIcebergAsset = DicebergClient::default().iceberg(
                IcebergAsset::builder()
                    .location(location)
                    .schema_table(schema_table)
                    .build(),
            );
            let records = asset.sql(query.as_str()).await?.collect().await?;
            let mut writer = ArrayWriter::new(io::stdout());
            writer.write_batches(&records.iter().collect::<Vec<_>>())?;
            writer.finish()?;
        }
    }
    Ok(())
}
