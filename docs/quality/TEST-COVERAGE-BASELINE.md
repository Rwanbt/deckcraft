# Instrumented test coverage — baseline

Date: 2026-10-12. Repository: `Rwanbt/deckcraft`. Base: `main` at `0ec0cc5e815caea982a7deec51bc437f3ba187ad`.

**Status: UNMEASURED** at branch creation. The fork had no recorded GitHub Actions runs and no coverage artifacts to inspect. A source file with tests is not evidence of measured line, function or branch coverage.

## Reproducible initial measurement

The new `Quality coverage (core formats)` workflow collects the first **partial** baseline on pull requests and manual dispatches. Inspect the job for the exact tested commit and download `coverage-core-<SHA>`, which contains `coverage-core.lcov` and `coverage-core.json`.

The command is:

```bash
cargo install cargo-llvm-cov --locked
cargo llvm-cov --locked -p deckcraft-bitstream -p deckcraft-format -p deckcraft-pptx --lcov --output-path coverage-core.lcov
cargo llvm-cov report --json --output-path coverage-core.json
cargo llvm-cov report --summary-only
```

This is **not workspace coverage**. The remaining crates, platform features and desktop/web E2E are out of the initial instrumented scope. Do not infer global coverage from the result. No numerical percentage is asserted until the workflow completes.

## Expansion gate

1. Confirm the partial report and archive its exact SHA/artifact.
2. Extend instrumentation to the remaining workspace crates by compatible targets/features; document exclusions.
3. Add contract/E2E evidence, branch coverage, mutation/fuzz campaigns and anti-regression thresholds based on real measurements.
4. Never set a 100% threshold before the data supports it.
