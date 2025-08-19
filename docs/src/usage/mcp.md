# DICI MCP Server

The **Data and Insights Cloud Integration (DICI) MCP Server** is a [Model Context Protocol](https://modelcontextprotocol.io/) (MCP) implementation that exposes DICI domain concepts, schemas, and queries through structured tools.

It enables clients and LLMs to traverse the DICI data model, retrieve schemas, execute SQL, and inspect inventories, registrations, and version metadata.

---

## Features

The server provides a toolbox of MCP tools grouped into the following categories:

* **Schema Retrieval**

    * `asset_get_schema_by_fxf` – Get schema by public dataset FXF.
    * `asset_get_schema_by_iceberg` – Get schema by Iceberg location + schema table.

* **SQL Execution**

    * `asset_execute_sql_by_fxf` – Run SQL against a dataset identified by FXF.
    * `asset_execute_sql_by_iceberg` – Run SQL directly against an Iceberg table.

* **Inventory Management**

    * `inventory_get_by_fxf` – Retrieve Inventory by FXF.
    * `inventory_list_by_iceberg_location` – Find all Inventories sharing an Iceberg location.
    * `inventory_list_by_domain` – List Inventories within a domain.
    * `inventory_list_updated_since` – List Inventories updated since a timestamp.

* **Registration Management**

    * `registration_get_by_iceberg_location` – Retrieve Registration by Iceberg location.
    * `registration_list_by_path` – List Registrations by canonical path.
    * `registration_query_by_path_and_metadata` – Search Registrations by path + metadata.

* **System Metadata**

    * `version_get` – Retrieve Git configuration and build metadata.
    * `get_date_time` – Get current UTC date/time.

---

## Architecture

* **`DiciServerHandler`** – Implements the MCP server handler, dispatching tool calls.
* **`DiciToolBox`** – A macro-generated dispatcher containing all registered tools.
* **`DiciCallableTool`** – A trait implemented by each tool to unify async execution.
* **State** – Shared context (`DiciServerHandlerState`) that holds:

    * `ManagementClient` for Inventory, Registration, and Version queries.
    * `DiciCatalog` for schema resolution and SQL execution.

---

## Conceptual Model

The server includes an embedded **Conceptual Knowledge Document**, defining the entities and their relationships:

* **Domain** – Organizational scope for datasets; all inventories belong to a domain.
* **Inventory** – Maps `(domain, FXF)` to an Iceberg location and schema table.
* **Registration** – Maps canonical paths to Iceberg locations, with optional metadata.
* **Iceberg Location** – Internal identifier (`_` + 32 lowercase hex MD5 of path).
* **Four-by-Four (FXF)** – Public dataset identifier (`xxxx-xxxx`), unique within a domain.
* **GitConfig** – Snapshot of version control state, build, and commit metadata.

### Core Relationships

* `Inventory` and `Registration` join via `iceberg_location`.
* `FXF` resolves datasets through Inventory.
* Domains scope FXF uniqueness.
* Paths deterministically compute Iceberg locations.
* `GitConfig` links temporally to entity updates.

---

## Entrypoint

The MCP server runs via:

```
dici serve mcp
```

This launches the server over stdio transport, exposing all registered tools to MCP-compatible clients.