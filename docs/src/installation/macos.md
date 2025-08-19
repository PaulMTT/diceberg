# Installation for macOS

Xcode is required for the Metal kernels, which enable running large language models efficiently on macOS.

## Xcode
You will need to install Xcode from the App Store.

### License
After which you will need to accept the license:
```shell
sudo xcodebuild -license
````

## Building with Features

On macOS, you must enable the **Metal backend** when installing.
If you also want to enable the MCP server, include the `mcp` feature.

Example (Metal only):

```bash
cargo install --path . --features metal
```

Example (Metal + MCP):

```bash
cargo install --path . --features "metal mcp"
```

⚠️ Note: only one backend should be enabled at a time (e.g. don’t enable `metal` and `cuda` together).
