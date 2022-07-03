# Wondrium player and downloader

## Building

### Building the CLI

Run cargo build in the cli folder:

```
cd wondrium_cli
cargo build
```

### Building the Desktop app

To build, install the Rust WebAssembly target and tauri-cli:

```
rustup target add wasm32-unknown-unknown
cargo install tauri-cli
```

then build using tauri-cli:

```
cd wondrium_tauri
cargo tauri build
```
