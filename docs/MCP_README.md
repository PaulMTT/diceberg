# DICI MCP Server Tools

This module implements the **server-side handler** and **tool definitions** for the **DICI** Model Context Protocol (MCP) server.
It exposes a **set of callable tools** (management, asset, datetime, etc.) via `rust_mcp_sdk` so that clients can discover and invoke them dynamically.

---

## 🎯 Design Goals

* **Unified tool interface** — All tools implement a single async trait (`DiciCallableTool`) for consistent execution.
* **Declarative tool definition** — Each tool is annotated with `#[mcp_tool]` metadata (name, description, hints) to be auto-registered with MCP.
* **Macro-based dispatch** — The `tool_box_with_dispatch!` macro generates an enum + dispatcher, avoiding boilerplate.
* **Separation of concerns** — The MCP server handler only coordinates requests; actual business logic is in tool implementations.
* **Consistent error handling** — Helper functions (`into_call_err`, `json_as_text`) enforce a standard output shape and formatting.

---

## 📐 Core Abstractions

### **Server State & Handler**

#### `DiciServerHandlerState`

Holds the state required for tool execution:

* `management_client: ManagementClient` — interface to the DICI management API.
* `dici_client: DiciClient` — interface to the core DICI service.

#### `DiciServerHandler`

Implements the MCP `ServerHandler` trait with two key responsibilities:

* `handle_list_tools_request` — returns metadata for all available tools.
* `handle_call_tool_request` — deserializes the request into a `DiciToolBox` variant and invokes its `call_tool` method.

---

### **Tool Abstraction**

#### `DiciCallableTool` (Trait)

Async interface implemented by all tools:

```rust
async fn call_tool(&self, state: &DiciServerHandlerState)
    -> Result<CallToolResult, CallToolError>;
```

This allows the handler to treat all tools uniformly, regardless of their input/output types.

#### `tool_box_with_dispatch!` (Macro)

Generates:

* A `DiciToolBox` enum with one variant per tool.
* An implementation of `DiciCallableTool` that pattern-matches the variant and calls the inner tool.

Example usage:

```rust
tool_box_with_dispatch!(
    DiciToolBox,
    [GetDateTimeTool, InventoryGetByFxf, VersionGet, ...]
);
```

---

### **Tool Metadata & Implementation**

Each tool is:

1. Defined as a struct holding its input parameters (using `serde` for (de)serialization).
2. Annotated with `#[mcp_tool]` to specify:

    * `name`, `title`, `description`
    * Safety and idempotency hints
    * Optional metadata JSON
3. Implements `DiciCallableTool` to:

    * Access `DiciServerHandlerState` for API calls
    * Perform domain-specific logic
    * Return results as pretty JSON via `json_as_text`

---

## 🛠 Tool Categories

### **System / Utility**

* **`GetDateTimeTool`** — Returns current UTC timestamp and epoch seconds.

### **Asset Tools**

* **`AssetGetSchemaByFxf`** — Fetch table schema by Core FXF ID.
* **`AssetGetSchemaByIceberg`** — Fetch table schema by Iceberg location & schema table.

### **Inventory Tools**

* **`InventoryGetByFxf`** — Fetch inventory record by FXF ID.
* **`InventoryListByIcebergLocation`** — List inventories for a given Iceberg location.
* **`InventoryListByDomain`** — List inventories by Socrata domain.
* **`InventoryListUpdatedSince`** — List inventories updated after a given timestamp.

### **Registration Tools**

* **`RegistrationListByPath`** — List registrations for a canonical path.
* **`RegistrationGetByIcebergLocation`** — Fetch registration by Iceberg location.
* **`RegistrationQueryByPathAndMetadata`** — Query registrations with exact metadata matches.

### **Version Tool**

* **`VersionGet`** — Retrieve Git build/version metadata from `/version`.

---

## ⚙️ Helper Functions

* **`into_call_err(e)`** — Wraps any error into a standard MCP `CallToolError` with `SdkError::internal_error`.
* **`json_as_text(value)`** — Pretty-prints any serializable value as JSON in a `CallToolResult::text_content`.

These ensure all tools produce **consistent, human-readable output** and **uniform error shapes**.

---

## 🔄 Request Flow

1. **List Tools**
   Client sends `ListToolsRequest` → Handler returns `ListToolsResult` with metadata from `DiciToolBox::tools()`.

2. **Call Tool**
   Client sends `CallToolRequest` → Handler:

    * Deserializes params into correct `DiciToolBox` variant.
    * Calls its `call_tool` method with `DiciServerHandlerState`.
    * Returns a `CallToolResult` (or `CallToolError`).

---

## 🔌 Extensibility

* **Adding a tool**:

    1. Define the input struct with `serde` derive.
    2. Annotate with `#[mcp_tool(...)]`.
    3. Implement `DiciCallableTool` with the logic.
    4. Add the type to the `tool_box_with_dispatch!` macro.

* **Changing output format**:

    * Adjust `json_as_text` for global pretty-printing rules.

* **Error handling**:

    * All tools should use `into_call_err` to ensure uniform error schema.
