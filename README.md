# Frame Runtime API CLI

Call into WASM Runtime API for debugging purposes.

## Example

First download some Substrate WASM runtime, like [Polkadot 1.2.0](https://github.com/polkadot-fellows/runtimes/releases/download/v1.2.0/polkadot_runtime-v1002000.compact.compressed.wasm).

Some example calls:
```bash
# Get the metadata
frame-runtime-api -r polkadot_runtime-v1002000.compact.compressed.wasm call Metadata metadata

# Get the versions
frame-runtime-api -r ~/Downloads/polkadot_runtime-v1002000.compact.compressed.wasm call Core version
```

Listing and finding metadata types:
```bash
# List all types
frame-runtime-api -r polkadot_runtime-v1002000.compact.compressed.wasm metadata list types

# Find a specific type (case-insensitive)
frame-runtime-api -r polkadot_runtime-v1002000.compact.compressed.wasm metadata find type runtimeversion
> sp_version::RuntimeVersion
```

## TODO

- [ ] Use V15 Runtime API metadata to decode results instead of hard-coding some known-good ones.
- [ ] Make passing arguments easier.
