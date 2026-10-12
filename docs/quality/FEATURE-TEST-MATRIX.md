# Functional testing traceability — initial risk-first inventory

As of 2026-10-12, base `0ec0cc5e815caea982a7deec51bc437f3ba187ad`. This is a **partial inventory from the source tree**, not proof that any area is verified. Status is `UNVERIFIED` until test runs and assertions are inspected.

| ID | Code area | Contract and critical risks | Existing test locations | Required next evidence | Status |
|---|---|---|---|---|---|
| FMT-001 | `crates/format` | Native ZIP manifest, media integrity, load/save roundtrip, legacy mimetype | `src/lib.rs` tests; new missing-media regression | Instrumented CI + corrupt entry corpus + file roundtrip | UNVERIFIED |
| PPTX-001 | `crates/pptx` | Import/export, corrupted/hostile XML, interoperability, no data loss | `tests/malformed.rs`, `roundtrip.rs`, `sniff.rs`, `sample.rs` | CI; independent synthetic fixture oracle; external legal corpus | UNVERIFIED |
| BIT-001 | `crates/bitstream` | Bounds, Exp-Golomb, NAL extraction and byte alignment | `src/lib.rs` unit/property tests | Coverage, boundary/property and fuzz CI | UNVERIFIED |
| ENG-001 | `crates/engine` | Command atomicity, undo/redo, crash guard, saving and closing | `src/tests.rs` | API integration and full editor/CLI roundtrip | UNVERIFIED |
| MODEL-001 | `crates/model` | Slide/master/layout ownership, unique IDs, serialization | `src/tests.rs` | CI/coverage and stateful property testing | UNVERIFIED |
| MEDIA-001 | `crates/media` plus codec crates | Media decoding, packet parsing, corrupt streams | `tests/decode.rs`, codec unit tests | Fixtures, fuzz corpus, CPU/platform cross-check | UNVERIFIED |
| UI-001 | `crates/ui-egui`, `apps/deckcraft` | Editor actions, selection, menus, dialogs, file recovery | No complete E2E evidence collected | Native UI smoke and E2E on Linux/Windows/macOS | UNVERIFIED |
| WEB-001 | `apps/deckcraft-web` | WASM build, user interactions, import/export | No browser E2E evidence collected | Browser real-session E2E and JS error checks | UNVERIFIED |
| CLI-001 | `apps/deckcraft-cli`, `crates/mcp` | Headless commands and protocol error handling | `crates/mcp/src/tests.rs` | Spawn actual CLI/MCP and assert protocol contracts | UNVERIFIED |

Use test-runner collection and line/function/branch reports to populate numeric evidence. Distinguish test-file counts from actual collected tests.
