use crate::mcp::handler::DiciServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};
use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions,
    error::SdkResult,
    mcp_server::{ServerRuntime, server_runtime},
};
pub async fn handle_serve_mcp() -> anyhow::Result<()> {
    run_mcp()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
async fn run_mcp() -> SdkResult<()> {
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
# **DICI Conceptual Knowledge Document**
This document defines the **entities**, **fields**, and **relationships** in the DICI domain model.
It explains how these concepts interconnect, how to traverse between them, and what constraints and semantics apply.
---
## **1. Core Entities**
---
### **1.1 Domain**
A **Domain** represents a tenant, organizational boundary, or logical namespace for datasets.
It provides top-level scoping: no dataset exists outside a domain.
**Fields:**
| Field    | Type   | Description                                                                                                         |
| -------- | ------ | ------------------------------------------------------------------------------------------------------------------- |
| `domain` | String | Unique domain name (e.g., `"cityofchicago"`, `"agency_xyz"`). Case-insensitive in meaning, but stored consistently. |
**Key Concepts:**
* A domain is authoritative for its datasets.
* Inventories are always scoped to a single domain.
* Registrations may carry domain metadata, but it is not guaranteed unless explicitly set.
---
### **1.2 Inventory**
An **Inventory** record is the public-facing representation of a dataset inside a domain.
**Purpose:**
* Maps internal Iceberg assets to stable, external-facing identifiers (`fourByFour`).
* Serves as the bridge between **internal identifiers** (`icebergLocation`, `schemaTable`) and **public identifiers** (`fourByFour`).
**Fields:**
| Field                | Type           | Description                                                      |
| -------------------- | -------------- | ---------------------------------------------------------------- |
| `id.domain.domain`   | String         | Domain name.                                                     |
| `id.icebergLocation` | String         | Internal dataset identifier, derived from the registration path. |
| `id.schemaTable`     | String         | Lowercase Iceberg table name within the domain.                  |
| `fourByFour`         | String         | Globally unique public dataset identifier, format `xxxx-xxxx`.   |
| `createdAt`          | DateTime (UTC) | Record creation time (ISO-8601 UTC).                             |
| `updatedAt`          | DateTime (UTC) | Last modification time (ISO-8601 UTC).                           |
**Constraints:**
* `(domain, schemaTable)` is unique.
* `(domain, fourByFour)` is unique.
* Each Inventory points to exactly one Iceberg asset.
**Key Concepts:**
* An **Iceberg asset** is defined as the pair `(icebergLocation, schemaTable)`.
* The same Iceberg asset may appear in multiple domains, but each Inventory must have its own `fourByFour`.
* An Inventory can be retrieved either by:
  * its `fourByFour`, or
  * the composite key `(domain, icebergLocation, schemaTable)`.
---
### **1.3 Registration**
A **Registration** record maps a canonical dataset `path` to its `icebergLocation`.
**Purpose:**
* Provides the authoritative record of how a dataset’s logical identity maps to its storage table.
* Supports optional metadata for organizational and operational context.
**Fields:**
| Field             | Type                | Description                                |
| ----------------- | ------------------- | ------------------------------------------ |
| `id.path`         | String              | Canonical dataset path.                    |
| `icebergLocation` | String              | Internal identifier, MD5 of path.          |
| `metadata`        | Map\<String,String> | Optional metadata, may include `"domain"`. |
| `createdAt`       | DateTime (UTC)      | Record creation time (ISO-8601 UTC).       |
| `updatedAt`       | DateTime (UTC)      | Last modification time (ISO-8601 UTC).     |
**Constraints:**
* `path → icebergLocation` is deterministic.
* Multiple paths cannot map to the same `icebergLocation` unless explicitly configured.
---
### **1.4 Iceberg Location**
An **Iceberg Location** is the deterministic internal identifier for a dataset’s Iceberg table.
**Fields:**
| Field             | Type   | Description                                          |
| ----------------- | ------ | ---------------------------------------------------- |
| `icebergLocation` | String | `"_" + 32` lowercase hex MD5 of a Registration path. |
**Key Concepts:**
* Derived from `path`.
* Functions as a join key between **Inventory** and **Registration**.
* Identifies the physical Iceberg table across the system.
---
### **1.5 Four-by-Four (FXF)**
A **Four-by-Four** is the globally unique public identifier for a dataset.
**Fields:**
| Field        | Type   | Description                                             |
| ------------ | ------ | ------------------------------------------------------- |
| `fourByFour` | String | Lowercase alphanumeric with a dash, format `xxxx-xxxx`. |
**Key Concepts:**
* Always globally unique.
* If an Iceberg asset exists in multiple domains, each Inventory entry for it gets a different `fourByFour`.
* `fourByFour` is the most common entry point into the system for external requests.
---
### **1.6 GitConfig**
A **GitConfig** is what the `/version` endpoint returns. It describes the build and source control state of the system.
**Purpose:**
* Provides reproducibility and traceability.
* Associates a running service instance with the repository commit it was built from.
**Key Concepts:**
* Not part of the data model relationships (Registration/Inventory).
* Useful for debugging and auditing builds, deployments, and dataset processing.
---
## **2. Field Index Across Entities**
| Field                    | Appears In              | Navigation Role                                                        |
| ------------------------ | ----------------------- | ---------------------------------------------------------------------- |
| `icebergLocation`        | Inventory, Registration | Primary join key linking public datasets (Inventory) to registrations. |
| `domain`                 | Inventory, Registration | Scope boundary for datasets.                                           |
| `fourByFour`             | Inventory               | Public dataset identifier.                                             |
| `path`                   | Registration            | Canonical logical key for computing `icebergLocation`.                 |
| `createdAt`, `updatedAt` | Inventory, Registration | Support historical analysis and build correlations.                    |
---
## **3. Navigation Matrix**
| From → To                    | Direct Linking Field(s)                                   |
| ---------------------------- | --------------------------------------------------------- |
| Inventory → Registration     | `icebergLocation`                                         |
| Registration → Inventory     | `icebergLocation`                                         |
| Inventory → Domain           | `domain`                                                  |
| Registration → Domain        | `metadata["domain"]` or via `icebergLocation` → Inventory |
| Inventory → FXF              | `fourByFour`                                              |
| FXF → Inventory              | `fourByFour` (globally unique)                            |
| Inventory → Iceberg Location | `icebergLocation`                                         |
| Iceberg Location → Inventory | `icebergLocation`                                         |
| Registration → Path          | `path`                                                    |
| Path → Registration          | `path`                                                    |
---
## **4. Traversal Principles**
* **Iceberg Location is the bridge**: all entity relationships pass through it.
* **Four-by-Four is globally unique**: the public-facing handle for Inventories.
* **Domain scoping disambiguates**: same Iceberg asset in different domains → different `fourByFour`.
* **Path determinism**: given a `path`, its `icebergLocation` can always be computed.
* **GitConfig is temporal**: links only by build time, not through entity keys.
---
## **5. Conceptual Relationships**
1. **Registration** maps `(path)` → `icebergLocation`.
2. **Inventory** maps `(domain, icebergLocation, schemaTable)` → `fourByFour`.
3. **Iceberg Asset** = `(icebergLocation, schemaTable)`.
   * May exist across domains.
   * Each Inventory assigns its own `fourByFour`.
4. **Iceberg Location** is the shared bridge between Registrations and Inventories.
5. **Four-by-Four** is always resolved via Inventory.
6. **Domain** scopes Inventories, but FXF uniqueness is global.
7. **GitConfig** describes builds, independent of dataset identifiers.
        "#.into()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };
    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = DiciServerHandler::default();
    let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);
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
