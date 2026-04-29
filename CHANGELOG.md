# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2026-04-29

### Changed
- Migrate SinkReason and SourceReason to `derive(OrionError)`, replacing manual
  `ErrorCode`/`DomainReason` impls and `thiserror::Error`
- Bump actions/checkout from 5 to 6 in GitHub Actions workflow (2c3407b)

### Added
- Add plural defs methods to provider traits (f7aabc8)

## [0.7.0] - (Previous Release)

[Unreleased]: https://github.com/wp-labs/wp-connector-api/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/wp-labs/wp-connector-api/releases/tag/v0.9.0
[0.7.0]: https://github.com/wp-labs/wp-connector-api/releases/tag/v0.7.0
