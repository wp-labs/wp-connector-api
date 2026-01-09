# Repository Guidelines

## Project Structure & Module Organization
`wp-connector-api` is a Rust library crate living inside the broader `warp-pase-system` workspace. Source modules reside in `src/`, and unit tests live alongside their implementations within `#[cfg(test)]` blocks. The crate consumes workspace siblings such as `../../wp-data-model`, so keep cross-crate paths stable and prefer reusing shared models instead of reinventing types. No binaries are shipped from this repo; the exported traits and helpers are consumed by upstream services.

## Build, Test, and Development Commands
- `cargo build` — compile the library from this crate directory.
- `cargo test` — run the unit-test suite; add `-p wp-connector-api` when invoking from the workspace root.
- `cargo doc --open` — generate API docs locally for quick contract reviews.
- `cargo fmt --all` and `cargo clippy --all-targets --all-features -D warnings` — enforce formatting and linting before sending changes.

## Coding Style & Naming Conventions
Target Rust 2021 defaults (4-space indentation, 100-column wraps). Modules and functions use `snake_case`, types and traits use `CamelCase`, and constants stay in `SCREAMING_SNAKE_CASE`. Document public APIs with `///` comments and prefer small, focused modules. Run `cargo fmt` after editing to keep style consistent across the workspace.

## Testing Guidelines
Keep unit tests close to the code they cover and name them `test_*` for clarity. Async scenarios should use `#[tokio::test]`. Execute `cargo test` inside this crate or `cargo test -p wp-connector-api` at the workspace root to make sure shared dependencies build. When adding new surface area, target the relevant module plus any shared data-model crates affected.

## Commit & Pull Request Guidelines
Follow Conventional Commits, scoping by crate when it adds context (e.g., `feat(wp-connector-api): add raw event mapper`). PRs should link issues, summarize behavioral changes, call out new configs, and include logs or screenshots when they help reviewers. Confirm `cargo fmt`, `cargo clippy`, and targeted `cargo test` runs before requesting review.

## Security & Configuration Tips
Never commit secrets or example keys; rely on workspace configuration files checked into `warp-pase-system`. Treat all inputs as untrusted and avoid `unwrap`/`expect` in library paths—bubble up meaningful errors instead. Keep dependency versions aligned with the workspace to benefit from shared audits and coordinated updates.
