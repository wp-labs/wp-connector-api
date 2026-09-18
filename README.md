# wp-connector-api

[![Crates.io](https://img.shields.io/crates/v/wp-connector-api.svg)](https://crates.io/crates/wp-connector-api)
[![Docs.rs](https://docs.rs/wp-connector-api/badge.svg)](https://docs.rs/wp-connector-api)
[![CI](https://img.shields.io/github/actions/workflow/status/wp-labs/wp-connector-api/ci.yml?branch=main)](https://github.com/wp-labs/wp-connector-api/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/wp-labs/wp-connector-api/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-connector-api)
[![Crates.io downloads](https://img.shields.io/crates/d/wp-connector-api)](https://crates.io/crates/wp-connector-api)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust Edition](https://img.shields.io/badge/edition-2024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

`wp-connector-api` defines the sink/source runtime traits, connector configuration helpers, and shared error types that power WarpParse ingestion pipelines. The crate ships as part of the `warp-pase-system` workspace and is consumed by services that need to register connectors or embed the WarpParse runtime.


## Repository Layout
- `src/config/` — connector kind adapters plus `ParamMap` builders.
- `src/runtime/` — connector registry, sink/source factories, async handles, and stream/event definitions.
- `src/errors/` — sink/source error variants, summaries, and helper traits.
- `tests/` — integration coverage, e.g., `demo_connector.rs` ensures the trait surfaces stay ergonomic.
- `docs/` — rendered API notes in both English and Chinese for downstream consumers.



## License
This crate is distributed under the [Apache License 2.0](LICENSE). Update `[workspace.package].authors` plus the tail of `LICENSE` if you redistribute builds under a different publisher name.

## 中文简介
`wp-connector-api` 提供 WarpParse 连接器运行时接口、配置解析工具与统一错误定义。
