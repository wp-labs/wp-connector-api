# wp-connector-api

`wp-connector-api` defines the sink/source runtime traits, connector configuration helpers, and shared error types that power WarpParse ingestion pipelines. The crate ships as part of the `warp-pase-system` workspace and is consumed by services that need to register connectors or embed the WarpParse runtime.

## Highlights
- Declarative connector config parsing (`src/config`) with helpers to convert TOML maps into runtime parameters.
- Unified runtime traits for both sinks and sources (`src/runtime`) with async-friendly factories and control channels.
- Shared error taxonomy, summaries, and extensions (`src/errors`) so downstream crates can bubble up actionable diagnostics.
- Lightweight param/value utilities (`src/types.rs`) that mirror data structures from `wp-model-core` and `wp-parse-api`.

## Repository Layout
- `src/config/` — connector kind adapters plus `ParamMap` builders.
- `src/runtime/` — connector registry, sink/source factories, async handles, and stream/event definitions.
- `src/errors/` — sink/source error variants, summaries, and helper traits.
- `tests/` — integration coverage, e.g., `demo_connector.rs` ensures the trait surfaces stay ergonomic.
- `docs/` — rendered API notes in both English and Chinese for downstream consumers.

## Build & Test
```bash
# run inside this crate
cargo build
cargo test

# invoke from the workspace root
cargo build -p wp-connector-api
cargo test  -p wp-connector-api

# developer hygiene
cargo fmt --all
cargo clippy --all-targets --all-features -D warnings
cargo doc --open
```
Enable the optional `test_helpers` feature when experimentation requires additional fixtures.

## Development Workflow
Follow the contributor playbook in [AGENTS.md](AGENTS.md). That document covers structure, coding style, and release expectations for the entire workspace. In short, keep modules focused, document public APIs with `///`, avoid panics in library code, and make sure formatting, linting, and targeted tests succeed before requesting review.

## License
This crate is distributed under the [Elastic License 2.0](LICENSE). Update `[workspace.package].authors` plus the tail of `LICENSE` if you redistribute builds under a different publisher name.

## 中文简介
`wp-connector-api` 提供 WarpParse 连接器运行时接口、配置解析工具与统一错误定义。请参考 [AGENTS.md](AGENTS.md) 获取贡献指南，按 `cargo build/test -p wp-connector-api` 完成构建，并在提交前执行 `cargo fmt` 与 `cargo clippy`。仓库遵循 ELv2 许可，可根据需要调整作者署名。
