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
# **DICI Conceptual Knowledge Document**

This document defines the **entities**, **fields**, and **relationships** in the DICI domain model.
It also explains how these concepts interconnect, how to traverse from one to another, and what constraints and semantics apply.

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

* A domain is **authoritative** for its datasets — an FXF in one domain is unrelated to the same FXF in another.
* Inventories are always scoped to a single domain.
* Registrations may carry domain information in metadata, but it is not guaranteed unless explicitly set.

---

### **1.2 Inventory**

An **Inventory** record maps a public-facing dataset to its internal Iceberg storage table.

**Purpose:**

* Serves as the bridge between **public identifiers** (FXF) and **internal storage identifiers** (Iceberg location).
* Holds metadata needed to identify and track a dataset within a domain.

**Fields:**

| Field                                  | Type           | Description                                                                                       |
| -------------------------------------- | -------------- | ------------------------------------------------------------------------------------------------- |
| `id.domain.domain`                     | String         | Domain name.                                                                                      |
| `id.iceberg_location.iceberg_location` | String         | Unique internal identifier for Iceberg table (`"_" + 32` lowercase hex MD5 of registration path). |
| `id.schema_table.schema_table`         | String         | Lowercase Iceberg table name within the domain.                                                   |
| `four_by_four.four_by_four`            | String         | FXF public dataset ID, lowercase format `xxxx-xxxx`.                                              |
| `created_at`                           | DateTime (UTC) | Record creation time (ISO-8601 UTC).                                                              |
| `updated_at`                           | DateTime (UTC) | Last modification time (ISO-8601 UTC).                                                            |

**Constraints:**

* `(domain, schema_table)` is unique.
* `(domain, four_by_four)` is unique.
* Each inventory points to exactly one Iceberg location.

---

### **1.3 Registration**

A **Registration** record maps a canonical dataset path to its Iceberg location, with optional metadata.

**Purpose:**

* Provides an authoritative record of how a dataset’s logical identity (path) maps to its physical Iceberg table.
* Can store metadata for organizational, operational, or analytical purposes.

**Fields:**

| Field                               | Type                 | Description                                                          |
| ----------------------------------- | -------------------- | -------------------------------------------------------------------- |
| `id.path.path`                      | String               | Canonical dataset path (logical identifier).                         |
| `iceberg_location.iceberg_location` | String               | `"_" + 32` lowercase hex MD5 of `path`.                              |
| `metadata`                          | Map\<String, String> | Arbitrary key-value pairs. May include `"domain"`, but not required. |
| `created_at`                        | DateTime (UTC)       | Record creation time (ISO-8601 UTC).                                 |
| `updated_at`                        | DateTime (UTC)       | Last modification time (ISO-8601 UTC).                               |

**Constraints:**

* `path` → `iceberg_location` is deterministic.
* Multiple paths cannot map to the same iceberg\_location unless explicitly configured.

---

### **1.4 Iceberg Location**

An **Iceberg Location** is an internal storage identifier for a dataset’s table.

**Fields:**

| Field              | Type   | Description                                          |
| ------------------ | ------ | ---------------------------------------------------- |
| `iceberg_location` | String | `"_" + 32` lowercase hex MD5 of a registration path. |

**Key Concepts:**

* Derived from the registration path; deterministic and reproducible.
* Functions as a **link key** between Inventory and Registration.
* Identifies an Iceberg table across the system.

---

### **1.5 Four-by-Four (FXF)**

A **Four-by-Four** is a public dataset identifier used in APIs, exports, and portals.

**Fields:**

| Field          | Type   | Description                                             |
| -------------- | ------ | ------------------------------------------------------- |
| `four_by_four` | String | Lowercase alphanumeric with a dash, format `xxxx-xxxx`. |

**Key Concepts:**

* Unique within a single domain.
* Does not expose internal storage details.
* Is the most common **entry point** into the system from external requests.

---

### **1.6 GitConfig**

A **GitConfig** represents a snapshot of the version-control state of the software at a given time.

**Purpose:**

* Enables traceability and reproducibility.
* Associates build and deployment states with dataset processing events.

**Fields:**

| Field                      | Type             |
| -------------------------- | ---------------- |
| `branch`                   | String           |
| `build.host`               | String           |
| `build.time`               | DateTime (UTC)   |
| `build.user.name`          | String           |
| `build.user.email`         | String           |
| `version`                  | String           |
| `build.number`             | String / Integer |
| `closest.tag.name`         | String           |
| `closest.tag.commit_count` | Integer          |
| `commit.author_time`       | DateTime (UTC)   |
| `commit.committer_time`    | DateTime (UTC)   |
| `commit.id.full`           | String           |
| `commit.id.abbrev`         | String           |
| `commit.describe`          | String           |
| `commit.message.full`      | String           |
| `commit.message.short`     | String           |
| `commit.user.name`         | String           |
| `commit.user.email`        | String           |
| `dirty`                    | Boolean          |
| `local_branch.ahead`       | Integer          |
| `local_branch.behind`      | Integer          |
| `remote_origin`            | String           |
| `tag`                      | String           |
| `tags`                     | List\<String>    |
| `total_commits`            | Integer          |

---

## **2. Field Index Across Entities**

This table shows **shared fields** that allow navigation between entities.

| Field                      | Appears In                         | Navigation Role                                                         |
| -------------------------- | ---------------------------------- | ----------------------------------------------------------------------- |
| `iceberg_location`         | Inventory, Registration            | **Primary join key** linking public datasets to internal registrations. |
| `domain`                   | Inventory, Registration (metadata) | Scope boundary for datasets.                                            |
| `four_by_four`             | Inventory                          | Public dataset key.                                                     |
| `path`                     | Registration                       | Canonical logical key for computing iceberg\_location.                  |
| `created_at`, `updated_at` | Inventory, Registration            | Temporal link for historical analysis or GitConfig correlation.         |

---

## **3. Navigation Matrix**

| From → To                    | Direct Linking Field(s)                                    |
| ---------------------------- | ---------------------------------------------------------- |
| Inventory → Registration     | `iceberg_location`                                         |
| Registration → Inventory     | `iceberg_location`                                         |
| Inventory → Domain           | `domain`                                                   |
| Registration → Domain        | `metadata["domain"]` or via `iceberg_location` → Inventory |
| Inventory → FXF              | `four_by_four`                                             |
| FXF → Inventory              | `four_by_four` + `domain`                                  |
| Inventory → Iceberg Location | `iceberg_location`                                         |
| Iceberg Location → Inventory | `iceberg_location`                                         |
| Registration → Path          | `path`                                                     |
| Path → Registration          | `path`                                                     |

---

## **4. Traversal Principles**

* **Iceberg Location is the central bridge**: almost all cross-entity relationships pass through it.
* **Domain scoping is strict**: FXF collisions across domains are valid; they are only unique within a domain.
* **Path determinism**: given a path, you can compute the iceberg\_location without looking it up.
* **FXF is entry for public queries**, Path is entry for internal workflows.
* **GitConfig links are temporal**: the only way to associate it with datasets is via matching timestamps to `created_at` / `updated_at`.

---

## **5. Conceptual Relationships**

1. **Inventory** maps `(domain, FXF)` ↔ `iceberg_location`.
2. **Registration** maps `(path)` ↔ `iceberg_location` (+ metadata).
3. **Iceberg Location** is the intersection point of public (Inventory) and internal (Registration) records.
4. **FXF** is always resolved via Inventory.
5. **Domain** can be resolved directly from Inventory or indirectly from Registration metadata.
6. **GitConfig** is correlated by time, not by explicit key.
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
