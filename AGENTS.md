# Repository Guidelines

## Project Structure & Module Organization
- Root contains `Cargo.toml`, `README.md`, and `TODO.md`.
- Code lives in `crates/skygen` for the generator and `crates/generated/<provider>` for produced SDKs.
- Templates and static assets: `crates/skygen/assets`.
- OpenAPI specs in `crates/skygen/open-api-specs`; custom logic primarily under `crates/skygen/src`.
- Targets and intermediate artifacts land in `target/` (git-ignored).

## Build, Test, and Development Commands
- `cargo build -p skygen` – compiles the generator crate.
- `cargo run -p skygen -- generate -s <spec> -o <output>` – main entry to turn OpenAPI specs into bindings; use bundled specs for smoke tests.
- `cargo fmt` / `cargo fmt -- --check` – ensures formatting consistency.
- `cargo check` – fast verification without producing binaries.

## Coding Style & Naming Conventions
- Use Rust 2021 defaults with 4-space indentation; no tabs.
- Prefer explicit module imports; keep `pub use` re-exports in `mod.rs` tidy.
- Name generators and transformers descriptively (`schema_registry.rs`, `param_resolver.rs` etc.).
- Derive comments sparingly but document non-obvious transformations (e.g., schema normalization).

## Testing Guidelines
- Unit and integration tests should live in `crates/skygen/src/**/tests.rs` or `tests/`.
- Test names follow `function_under_test_expected_behavior`.
- Use `cargo test -p skygen` before submitting; add fixtures under `crates/skygen/assets` when mocking specs.
- For generator output diffing, keep golden files in `crates/generated/*` and re-run `cargo run` to refresh.

## Commit & Pull Request Guidelines
- Commit messages follow imperative style (“Add schema registry cache”).
- Group related changes; avoid large “mixed bag” commits.
- PRs should describe motivation, highlight schema/spec updates, and link relevant issues.
- Include reproduction steps or commands (`cargo run ...`) plus test evidence; screenshots only when UI assets change.

## Security & Configuration Tips
- Specs may embed credentials or URLs; scrub sensitive data before committing.
- When adding new providers or specs, document source versions in the PR and keep them under `open-api-specs`.
