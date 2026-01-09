# wp-connector-api

![CI](https://github.com/wp-labs/wp-connector-api/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-connector-api/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-connector-api)
![License](https://img.shields.io/badge/License-Elastic%202.0-green.svg)


`wp-connector-api` defines the sink/source runtime traits, connector configuration helpers, and shared error types that power WarpParse ingestion pipelines. The crate ships as part of the `warp-pase-system` workspace and is consumed by services that need to register connectors or embed the WarpParse runtime.


## Repository Layout
- `src/config/` — connector kind adapters plus `ParamMap` builders.
- `src/runtime/` — connector registry, sink/source factories, async handles, and stream/event definitions.
- `src/errors/` — sink/source error variants, summaries, and helper traits.
- `tests/` — integration coverage, e.g., `demo_connector.rs` ensures the trait surfaces stay ergonomic.
- `docs/` — rendered API notes in both English and Chinese for downstream consumers.



## License
This crate is distributed under the [Elastic License 2.0](LICENSE). Update `[workspace.package].authors` plus the tail of `LICENSE` if you redistribute builds under a different publisher name.

## 中文简介
`wp-connector-api` 提供 WarpParse 连接器运行时接口、配置解析工具与统一错误定义。
