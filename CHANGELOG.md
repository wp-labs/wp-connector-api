# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.0] - 2026-08-04

### ⚠️ BREAKING CHANGES

- 依赖 `wp-model-core` 0.8 → 0.9（上游新增 `Value::BigUint` / `DataType::BigInt` 变体）
- 依赖 `wp-source-types` 0.1 → 0.2（同步上游版本对齐）

### Changed

- 升级 `wp-model-core` 至 0.9、`wp-source-types` 至 0.2

## [0.11.2] - 2026-07-12

### Changed

- `Tags` 迁移至 `wp-source-types` v0.1.0，本地替换为 `pub use` 重导出
- `SourceEvent` / `SourceBatch` / `EventPreHook` 迁移至 `wp-source-types`，本地替换为重导出
- 新增 `wp-source-types` 依赖

## [0.11.0] - 2026-06-26

### Added

- `BatchMeta` 结构体：批次级运行时元信息.
- `AsyncRecordSink::sink_records_with_meta` 默认方法：引擎可把批次级 metadata 传给 sink，默认实现忽略 meta 回退到 `sink_records`，保证向后兼容

## [0.10.1] - 2026-05-03

### Changed
- Change `SinkReason::Sink` and `SourceReason::{SupplierError, Disconnect, Other}` to unit variants so domain reasons only carry classification.
- Change `SinkReason::sink(...)` and source detail helpers to return `StructError` values with message detail stored on `StructError`.
- Use `SinkReason::send_error(...)` to preserve `SendError` summaries in `StructError.detail`.

### Removed
- Remove `From<SendError<T>> for SinkReason`; converting directly to a reason would discard the send error summary.

## [0.10.0] - 2026-05-03

### Changed
- Bump orion-error from 0.7 to 0.8; rename `UvsReason` to `UnifiedReason`
- Replace `StructError::from(reason)` with `reason.to_err()` per 0.8 convention
- License changed from Elastic-2.0 to Apache-2.0

### Removed
- Remove `anyhow` dependency; `ConnectorKindAdapter::url_to_params` now returns `Result<ParamMap, String>`
- Remove `once_cell` dependency (unused)
- Remove `From<anyhow::Error> for SinkReason` impl

### Fixed
- Add explicit `derive_more` feature `from` for v2.x compatibility

## [0.9.0] - 2026-04-29

### Changed
- Migrate SinkReason and SourceReason to `derive(OrionError)`, replacing manual
  `ErrorCode`/`DomainReason` impls and `thiserror::Error`
- Bump actions/checkout from 5 to 6 in GitHub Actions workflow (2c3407b)

### Added
- Add plural defs methods to provider traits (f7aabc8)

## [0.7.0] - (Previous Release)

[Unreleased]: https://github.com/wp-labs/wp-connector-api/compare/v0.11.1...HEAD
[0.11.1]: https://github.com/wp-labs/wp-connector-api/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/wp-labs/wp-connector-api/compare/v0.10.1...v0.11.0
[0.10.0]: https://github.com/wp-labs/wp-connector-api/releases/tag/v0.10.0
[0.9.0]: https://github.com/wp-labs/wp-connector-api/releases/tag/v0.9.0
[0.7.0]: https://github.com/wp-labs/wp-connector-api/releases/tag/v0.7.0
