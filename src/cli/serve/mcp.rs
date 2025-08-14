use crate::mcp::handler::DiciServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    error::SdkResult, mcp_server::{server_runtime, ServerRuntime}, McpServer,
    StdioTransport,
    TransportOptions,
};

pub async fn handle_serve_mcp() -> anyhow::Result<()> {
    run_mcp()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}

async fn run_mcp() -> SdkResult<()> {
    // 1) Server details and capabilities
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "The data and insights cloud integration (DICI) model context protocol (MCP) server.".to_string(),
            version: "0.1.0".to_string(),
            title: Some("The data and insights cloud integration (DICI) model context protocol (MCP) server.".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some(r#"
This MCP server retrieves and traverses relationships among:

* **Registrations** — map a canonical path (S3 key or arbitrary) to a computed Iceberg location and metadata (incl. domain).
* **Inventories** — track a Socrata Core dataset (FXF) for a given Iceberg table within a domain.
* **GitConfig** — expose build and Git version metadata.
* **Asset Table Schemas** — retrieve the table schema for a Core FXF or Iceberg table.

You can **start from any known field or object** and retrieve **any other**.

---

## Entities & Field Paths

### Inventory

Represents one Socrata Core dataset for one Iceberg table in one domain.

* `id.domain.domain` — **String** — Socrata tenant name
* `id.iceberg_location.iceberg_location` — **String** — `"_" + 32` lowercase hex (MD5 of Registration path)
* `id.schema_table.schema_table` — **String** — lowercase Iceberg table name
* `four_by_four.four_by_four` — **String** — FXF Socrata dataset ID (`XXXX-XXXX`)
* `created_at` — **DateTime<Utc>** — ISO-8601 UTC
* `updated_at` — **DateTime<Utc>** — ISO-8601 UTC

---

### Registration

Maps a canonical S3 key or arbitrary path to a computed Iceberg location and metadata.

* `id.path` — **String** — Canonical path string
* `iceberg_location.iceberg_location` — **String** — `"_" + 32` lowercase hex
* `metadata` — **Map\<String,String>** — Arbitrary keys; `"domain"` required for publish
* `created_at` — **DateTime<Utc>** — ISO-8601 UTC
* `updated_at` — **DateTime<Utc>** — ISO-8601 UTC

---

### GitConfig

Singleton build/version descriptor from `/version`.

* `branch` — **String** — Current Git branch
* `build.host` — **String** — Build host
* `build.time` — **String** — ISO-8601
* `build.user.email` / `build.user.name` — **String** — Build user info
* `build.version` — **String** — Build version
* `build.number` — **Option<String>** — Build number
* `closest.tag.name` — **Option<String>** — Closest Git tag
* `closest.tag.commit.count` — **Option<String>** — Commits since tag
* `commit.id.full` / `.abbrev` / `.describe` / `.describe_short` — **String** — Commit IDs
* `commit.message.full` / `.short` — **String** — Commit messages
* `commit.author.time` / `commit.committer.time` / `commit.time` — **String** — Commit timestamps
* `commit.user.email` / `commit.user.name` — **String** — Commit user info
* `dirty` — **String** — Dirty working tree flag
* `local.branch.ahead` / `local.branch.behind` — **String** — Ahead/behind counts
* `remote.origin.url` — **String** — Git remote URL
* `tag` / `tags` — **Option<String>** — Tag metadata
* `total.commit.count` — **String** — Total commit count

---

### Asset Table Schema

Returned by schema retrieval tools (`asset_get_schema_by_fxf` / `asset_get_schema_by_iceberg`).

* **Type** — JSON Array of field definitions (exactly as returned by `asset.schema()`)
* **Format** — Pretty-printed JSON

---

## Direct Retrieval Tools (Single-Hop)

* `inventory_get_by_fxf(four_by_four.four_by_four)` → **Inventory**
* `inventory_list_by_iceberg_location(id.iceberg_location.iceberg_location)` → **Inventory\[]**
* `inventory_list_by_domain(id.domain.domain)` → **Inventory\[]**
* `inventory_list_updated_since(since: ISO-8601 UTC)` → **Inventory\[]**
* `registration_list_by_path(id.path)` → **Registration\[]**
* `registration_get_by_iceberg_location(iceberg_location.iceberg_location)` → **Registration**
* `registration_query_by_path_and_metadata(id.path, metadata)` → **Registration\[]**
* `version_get()` → **GitConfig**
* `asset_get_schema_by_fxf(fxf: four_by_four.four_by_four)` → **[FieldDef]** (JSON array)
* `asset_get_schema_by_iceberg(location: id.iceberg_location.iceberg_location, schema_table: id.schema_table.schema_table)` → **[FieldDef]** (JSON array)

---

## Key Relationships

* `Registration.iceberg_location.iceberg_location`
  \= `Inventory.id.iceberg_location.iceberg_location`
* `Registration.metadata["domain"]`
  \= `Inventory.id.domain.domain`
* `four_by_four.four_by_four` **uniquely identifies** an **Inventory**
* (`id.iceberg_location.iceberg_location`, `id.schema_table.schema_table`) **uniquely identify** an Iceberg asset

---

## Retrieval Matrix (Recipes)

**Legend**

* **HAVE** — what you start with
* **NEED** — what you want
* **STEP 1 / STEP 2** — tool(s) to call
* **GET** — fields returned at that step

> Use exactly the tool names and field paths below.

---

### HAVE: `four_by_four.four_by_four`

* **NEED: Inventory**

  * STEP 1: `inventory_get_by_fxf` → **GET:** full **Inventory**
* **NEED: Registration**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.iceberg_location.iceberg_location`
  * STEP 2: `registration_get_by_iceberg_location` → **GET:** full **Registration**
* **NEED: Schema (by FXF)**

  * STEP 1: `asset_get_schema_by_fxf` → **GET:** table schema (JSON array)
* **NEED: id.domain.domain**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.domain.domain`
* **NEED: id.iceberg\_location.iceberg\_location**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.iceberg_location.iceberg_location`
* **NEED: id.schema\_table.schema\_table**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.schema_table.schema_table`
* **NEED: created\_at / updated\_at (Inventory)**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `created_at`, `updated_at`
* **NEED: id.path**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.iceberg_location.iceberg_location`
  * STEP 2: `registration_get_by_iceberg_location` → **GET:** `id.path`
* **NEED: metadata / metadata.domain**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.iceberg_location.iceberg_location`
  * STEP 2: `registration_get_by_iceberg_location` → **GET:** `metadata`
* **NEED: created\_at / updated\_at (Registration)**

  * STEP 1: `inventory_get_by_fxf` → **GET:** `id.iceberg_location.iceberg_location`
  * STEP 2: `registration_get_by_iceberg_location` → **GET:** `created_at`, `updated_at`

---

### HAVE: `id.iceberg_location.iceberg_location` + `id.schema_table.schema_table`

* **NEED: Schema (by Iceberg)**

  * STEP 1: `asset_get_schema_by_iceberg` → **GET:** table schema (JSON array)

---

### HAVE: `id.iceberg_location.iceberg_location`

* **NEED: Registration**

  * STEP 1: `registration_get_by_iceberg_location` → **GET:** full **Registration**
* **NEED: Inventory**

  * STEP 1: `inventory_list_by_iceberg_location` → **GET:** full **Inventory(s)**
* **NEED: id.path**

  * STEP 1: `registration_get_by_iceberg_location` → **GET:** `id.path`
* **NEED: metadata / metadata.domain**

  * STEP 1: `registration_get_by_iceberg_location` → **GET:** `metadata`
* **NEED: created\_at / updated\_at (Registration)**

  * STEP 1: `registration_get_by_iceberg_location` → **GET:** `created_at`, `updated_at`
* **NEED: id.domain.domain**

  * STEP 1: `inventory_list_by_iceberg_location` → **GET:** `id.domain.domain`
* **NEED: id.schema\_table.schema\_table**

  * STEP 1: `inventory_list_by_iceberg_location` → **GET:** `id.schema_table.schema_table`
* **NEED: four\_by\_four.four\_by\_four**

  * STEP 1: `inventory_list_by_iceberg_location` → **GET:** `four_by_four.four_by_four`
* **NEED: created\_at / updated\_at (Inventory)**

  * STEP 1: `inventory_list_by_iceberg_location` → **GET:** `created_at`, `updated_at`

---

### HAVE: — (nothing)

* **NEED: GitConfig**

  * STEP 1: `version_get` → **GET:** full **GitConfig** object
* **NEED: GitConfig field(s)**

  * STEP 1: `version_get` → **GET:** requested field(s)
        "#.into()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    // 2) stdio transport
    let transport = StdioTransport::new(TransportOptions::default())?;

    // 3) our DateTime handler
    let handler = DiciServerHandler::default();

    // 4) create server
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);

    // 5) start server
    if let Err(start_error) = server.start().await {
        eprintln!(
            "{}",
            start_error
                .rpc_error_message()
                .unwrap_or(&start_error.to_string())
        );
    }
    Ok(())
}
