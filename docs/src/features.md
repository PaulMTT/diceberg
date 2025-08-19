# Features

- [Lookup registrations](../../src/cli/info/lookup/registration.rs)
- [Lookup inventories](../../src/cli/info/lookup/inventory.rs)
- [Get schema of table](../../src/cli/info/table/schema.rs)
- [Execute sql against table](../../src/cli/sql.rs)
- [CLI that exposes everything through sub commands](../../src/cli/mod.rs)
- Lib with table/sql api, catalog/context access
- [Table manifest size, table data size](../../src/cli/info/table/stats)
- [Table partitions](../../src/cli/info/table/partition.rs)
- [Snapshot history, snapshot details](../../src/cli/info/table/history)
- [JSON or IPC(arrow format) sql output format](../../src/cli/sql.rs)
- [IPC print utility](../../src/cli/util/ipc/print.rs)
- [Execute sql against dataframes(IPC) input to stdin](../../src/cli/util/ipc/query.rs)
- [MCP server](../../src/cli/serve/mcp.rs)
- [Embedded local LLM chat with Ratatui terminal UI](../../src/cli/ai.rs)