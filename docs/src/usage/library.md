# Dici API Client & Store Library

This crate provides a Rust interface for interacting with the **Dici Management API** and managing **Iceberg-backed assets** via [Apache DataFusion](https://arrow.apache.org/datafusion/) and [Apache Iceberg](https://iceberg.apache.org/).

It is designed to support:

* Fetching **inventories** and **registrations** from the Dici Management service
* Querying **Iceberg tables** with SQL via DataFusion
* Managing **Core** and **Iceberg** assets through a unified abstraction: `DiciAsset`
* Integrating with **AWS Glue catalogs** for Iceberg table discovery

---

## Features

* **Dici Management Client**

    * Fetch inventories by FXF, domain, or Iceberg location
    * Fetch registrations by path, Iceberg location, or metadata
    * Retrieve Git version/build information from `/version`
    * Track inventories updated since a given timestamp

* **Assets**

    * `DiciAsset::core(fxf)` – resolve assets via the management service
    * `DiciAsset::iceberg(location, schema_table)` – create assets directly from Iceberg metadata
    * Builders `CoreArgs` and `IcebergArgs` let you customize `DiciCatalog` or `ManagementClient`

* **SQL Querying**

    * Run queries via DataFusion with `SqlAble`
    * Override table references with `ManuallySqlAble`
    * Inspect Iceberg schemas directly

* **Catalog Integration**

    * Backed by AWS Glue (`GlueCatalog`)
    * Configurable via environment variables
---

## Usage

### 1. Using `DiciAsset`

The **recommended entrypoint** is the `DiciAsset` enum.
It abstracts over **Core assets** (resolved through the management service) and **Iceberg assets** (direct location/schema reference).

```rust
use dici_client::api::store::asset::dici::DiciAsset;
use dici_client::api::store::asset::traits::sqlable::SqlAble;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Core asset resolved via FXF through management service
    let core_asset = DiciAsset::core("abcd-1234".into());

    let df = core_asset.sql("SELECT * FROM 'abcd-1234' LIMIT 5").await?;
    df.show().await?;

    // Iceberg asset built from warehouse path + schema.table
    let iceberg_asset = DiciAsset::iceberg(
        "icebergLocation".into(),
        "schemaTable".into(),
    );

    let df2 = iceberg_asset.sql("SELECT COUNT(*) FROM schemaTable").await?;
    df2.show().await?;

    Ok(())
}
```

---

### 2. Advanced Customization

If you need to customize the **management client** or **catalog**, use `CoreArgs` or `IcebergArgs` directly and convert them into a `DiciAsset`:

```rust
use dici_client::api::http::management::client::ManagementClient;
use dici_client::api::store::catalog::dici::DiciCatalog;
use dici_client::api::store::asset::dici::{CoreArgs, DiciAsset};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let management_client = ManagementClient::default();
    let dici_catalog = DiciCatalog::default();

    let core_args = CoreArgs::builder()
        .asset("abcd-1234".into())
        .management_client(management_client)
        .dici_catalog(dici_catalog)
        .build();

    let asset: DiciAsset = core_args.into();

    let df = asset.sql("SELECT * FROM 'abcd-1234' LIMIT 10").await?;
    df.show().await?;

    Ok(())
}
```

This also works with `IcebergArgs`.

---

### 3. SQL Querying: `SqlAble` vs `ManuallySqlAble`

There are **two traits** for running SQL queries:

#### `SqlAble`

Use when the asset already knows its **own table reference**.
Implemented for assets like `CoreAsset`, `IcebergAsset`, and `DiciAsset`.

```rust
use dici_client::api::store::asset::dici::DiciAsset;
use dici_client::api::store::asset::traits::sqlable::SqlAble;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let asset = DiciAsset::core("abcd-1234".into());

    // Runs against the asset’s built-in table reference
    let df = asset.sql("SELECT * FROM 'abcd-1234' LIMIT 5").await?;
    df.show().await?;

    Ok(())
}
```

➡️ Use this for **normal querying**.

---

#### `ManuallySqlAble`

Use when you want to **provide your own table reference** (aliasing, custom naming, disambiguating multiple assets).

```rust
use dici_client::api::store::asset::dici::DiciAsset;
use dici_client::api::store::asset::traits::manually_sqlable::ManuallySqlAble;
use datafusion::sql::TableReference;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let asset = DiciAsset::core("abcd-1234".into());

    // Give the table a custom alias
    let custom_ref = TableReference::Bare { table: "this".into() };

    let df = asset.sql_with_table_reference(
        "SELECT * FROM this WHERE some_column > 100",
        custom_ref,
    ).await?;

    df.show().await?;
    Ok(())
}
```

➡️ Use this when you need **explicit control** over the SQL context.

---

### 4. Management Client (direct use)

You can also interact with the management API directly via `ManagementClient`:

```rust
use dici_client::api::http::management::client::ManagementClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ManagementClient::default();

    let inventories = client.fetch_inventories().await?;
    println!("Found {} inventories", inventories.len());

    Ok(())
}
```

Available methods include:

* `fetch_inventories`, `fetch_inventory_by_fxf`, `fetch_inventories_by_domain`
* `fetch_registrations`, `fetch_registrations_by_path`, `fetch_registrations_by_path_and_metadata`
* `fetch_version`
* `fetch_inventories_updated_since`

---

## Environment Configuration

| Variable                  | Description                         |
| ------------------------- | ----------------------------------- |
| `DICI_MANAGEMENT_ADDRESS` | Base URL of the Dici Management API |
| `DICI_WAREHOUSE`          | Path to warehouse root for Glue     |

Both default to environment variables. If unset, construction will fail.

---

## Development Notes

* Async API built with **Tokio**
* HTTP via **reqwest**
* Data query via **Apache DataFusion**
* Table management with **Apache Iceberg**
* Catalog integration via **iceberg-catalog-glue**
* Configuration via **typed-builder** for ergonomic constructors
