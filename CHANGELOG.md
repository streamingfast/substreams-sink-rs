# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/streamingfast/substreams-sink-rs/releases/tag/v0.2.0)

### Changed

- **Breaking**: protobuf encoding and decoding moved from [prost](https://github.com/tokio-rs/prost)
  to [buffa](https://github.com/anthropics/buffa). `prost`, `prost-types` and `base64` are no longer
  dependencies, and this crate now requires `substreams` 0.8.0 or above. See
  [Migrating from prost to buffa](https://github.com/streamingfast/substreams/blob/develop/docs/references/migrating-to-buffa.md).

- **Breaking**: `KvOperations` and `KvOperation` are now `KVOperations` and `KVOperation`, matching
  the Protobuf message names. The previous names remain as `#[deprecated]` aliases, so existing code
  keeps compiling with a warning, and will be removed in a future version.

- **Breaking**: removed the WASM query service API: the `store` module (`Store`, `StoreGet`,
  `StoreNew`), the `prelude` module, and the re-exported `error` and `memory` modules.

  These bound the host functions `get_key`, `get_many_keys`, `get_by_prefix` and `scan`, which
  `substreams-sink-kv` removed in
  [v2.2.0](https://github.com/streamingfast/substreams-sink-kv/releases/tag/v2.2.0), so no server has
  implemented them since. A module calling `Store::get` compiled but trapped at instantiation on a
  missing `host.get_key`. Query a KV store through the `GenericService` Connect-Web/gRPC API instead.

- **Breaking**: removed the `substreams-sink-core` crate, which only supported the removed query
  service. Its `register_panic` import named the `host` module, while the live Substreams ABI
  provides it under `env`; a Substreams module gets the working equivalent from
  `substreams::register_panic_hook()`.

- The minimum supported Rust version is now 1.93, matching `substreams` 0.8.0.

- Releases are driven by [sfreleaser](https://github.com/streamingfast/sfreleaser), as in every other
  Rust crate in the organisation, replacing `bin/release.sh`.

## [0.1.3](https://github.com/streaminfast/substreams-sink-rs/releases/tag/v0.1.3)

* Migrated rust from `https://github.com/streaminfast/substreams-sink-kv` to this repo 