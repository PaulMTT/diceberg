# Installation

## Requirements
### MacOS
[macos requirements](macos.md)

## Installing

Install the release version of the tool:

```shell
cargo install --path .
````

Make sure to add cargo bin to your path like

```shell
export PATH="$HOME/.cargo/bin:$PATH"
```

### Installing with Features

`diceberg` uses [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to enable optional components.

#### AI Backends (required for `Ai` command)

To use the `Ai` subcommand, you must enable **one** Mistral backend.
Enabling a backend automatically brings in `mistralrs`, `ratatui`, `tui-markdown`, `crossterm`, and `hf-hub`.
The umbrella feature `ai` is automatically activated when any backend is chosen.

Available backends:

* `metal` (macOS / Metal)
* `cuda` (Linux / CUDA)
* `cudnn`
* `flash-attn`
* `accelerate`
* `mkl`
* `nccl`
* `ring`

Example install with Metal backend:

```bash
cargo install --path . --features metal
```

Example install with CUDA backend:

```bash
cargo install --path . --features cuda
```

#### MCP Server

MCP support is behind the `mcp` feature flag.
This enables the `Serve` subcommand.

Example install with MCP:

```bash
cargo install --path . --features mcp
```

#### Combining Features

You can combine an AI backend with MCP if you want both `Ai` and `Serve` available:

```bash
cargo install --path . --features "metal mcp"
```

---

⚠️ **Note:** only one backend should be chosen at a time (e.g. don’t enable both `metal` and `cuda` together).