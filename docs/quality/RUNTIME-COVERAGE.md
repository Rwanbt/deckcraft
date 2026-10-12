# Runtime coverage verification

## Status

New instrumented runner for the stateful application core. This reports four packages:
`deckcraft-engine`, `deckcraft-model`, `deckcraft-mcp`, and `deckcraft-cli`.

Coverage is **not yet measured** for this selection at the time of commit. The GitHub Actions job must complete for the exact pull request head before any percentage is recorded.

Run in CI:

```sh
cargo install cargo-llvm-cov --locked
cargo llvm-cov --locked -p deckcraft-engine -p deckcraft-model -p deckcraft-mcp -p deckcraft-cli --lcov --output-path coverage-runtime.lcov
cargo llvm-cov report --json --output-path coverage-runtime.json
cargo llvm-cov report --summary-only
```

The job executes the actual CLI integration tests plus the selected crates' unit/integration tests. It uploads machine-readable LCOV/JSON evidence and leaves the existing full `cargo xtask ci` quality gate unchanged.

Do not compare a tool-wide total including dependencies against per-crate source coverage. Extract covered/total lines from the project source filenames, keep per-crate numbers and report any excluded paths. Other crates, GPU rendering and desktop/web E2E require separate validation.
