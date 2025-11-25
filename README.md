# Frame Runtime API CLI

Call into WASM Runtime API for debugging purposes.

## Example

First download some Substrate WASM runtime, like [Polkadot 2.0.3](https://github.com/polkadot-fellows/runtimes/releases/download/v2.0.3/asset-hub-polkadot_runtime-v2000003.compact.compressed.wasm).

Some example calls:
```bash
# Get the metadata
frame-runtime-api -r asset-hub-polkadot_runtime-v2000003.compact.compressed.wasm call Metadata metadata

# Get the versions
frame-runtime-api -r .. call Core version
```

Listing and finding metadata types:
```bash
# List all types
frame-runtime-api -r .. metadata show types
>cumulus_pallet_parachain_system::pallet::Call
> cumulus_pallet_xcm::pallet::Call
...
> xcm::v5::traits::Error
> xcm_runtime_apis::authorized_aliases::OriginAliaser

# Show all pallet calls
frame-runtime-api -r .. metadata show types ".*::pallet::Call"
> cumulus_pallet_parachain_system::pallet::Call
> cumulus_pallet_xcm::pallet::Call
...
> snowbridge_pallet_system_frontend::pallet::Call
> staging_parachain_info::pallet::Call
```

If no runtime with `-r` is provided, it will try to find a `.wasm.` file in the current folder and
error if none or multiple are found.

## TODO

- [ ] Use V15 Runtime API metadata to decode results instead of hard-coding some known-good ones.
- [ ] Make passing arguments easier.
