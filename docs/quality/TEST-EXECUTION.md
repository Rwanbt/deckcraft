# Quality gate execution and provenance

- Starting repository: `Rwanbt/deckcraft`
- Base: `main` / `0ec0cc5e815caea982a7deec51bc437f3ba187ad` (same SHA as upstream at reconnaissance time).
- Branch: `test/native-format-integrity-and-coverage`.
- Audit date: 2026-10-12.
- Environment: repository inspected using the GitHub integration. This session's local container has neither Rust/Cargo nor network access to clone GitHub, so **no local tests were executed**.
- Baseline fork workflow runs observed: **0**.
- Existing full gate: `.github/workflows/ci.yml` runs `cargo xtask ci` (fmt, clippy, workspace tests, attribution, layering, wasm).
- Added gate: `.github/workflows/quality-coverage.yml` for instrumented native-format/bitstream/PPTX coverage.
- Do not mark tests passing until GitHub Actions supplies a successful run for the exact pushed SHA.
- Linux: pending verification. Windows/macOS: not included in initial new job. WASM/E2E: unverified.
- Collected test count and measured coverage: **not yet available**.
- PR policy: fork-internal draft until checks, coverage and behavior are verified; no upstream PR and no automatic merge.
