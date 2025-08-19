# Diceberg!

This project provides a suite of utilities for working with DICI assets and related operations.

* 📚 **Rust library** for interacting with DICI Management APIs and Iceberg-backed assets
* 💻 **Command-line interface (CLI)** for inventories, registrations, schemas, and queries
* 🔌 **MCP server** that exposes DICI concepts to LLMs via [Model Context Protocol](https://modelcontextprotocol.io/)
* 🐭 **AI terminal chat UI** powered by [`ratatui`](https://github.com/ratatui-org/ratatui) and [`mistralrs`](https://github.com/EricLBuehler/mistral.rs)

---

## ✨ Features

### 📚 Library

* DataFrame API for accessing raw DICI asset Iceberg tables
* HTTP client for the DICI Management API

### 💻 CLI

* Lookup registrations and inventories
* Inspect schemas, snapshot history, partitions, and stats
* Execute SQL against tables with JSON or Arrow IPC output
* IPC utilities for printing and querying DataFrames

### 🔌 MCP Server

* Schema retrieval and SQL execution
* Inventory and registration management
* Version and metadata queries

### 🐭 AI Chat

* Interactive terminal UI for LLM-powered chat sessions

---

## 📖 Documentation

👉 Full documentation available at: [socrata.github.io/diceberg](https://socrata.github.io/diceberg/)
