# Residual production-readiness risks — 2026-10-12

All items below are open until supported by the relevant tests, measurements and CI artifacts.

1. **Critical: potential native-file media data loss.** Previously, `format::load` silently ignored a missing or unreadable embedded `media/<id>` ZIP entry; this branch makes embedded entries mandatory and adds a regression test. CI confirmation remains required.
2. **Critical: closing unsaved work.** `docs/gaps.md` reports issue #60: `file.close` may discard an edited presentation without prompting. Needs independent reproduction and regression test; not resolved by this branch.
3. **Native file security/robustness.** Decompression limit applies per archive entry; aggregated limits and ZIP adversarial inputs need dedicated review and tests.
4. **PPTX interoperability.** Existing synthetic tests do not prove compatibility with third-party decks; avoid vendor-copied assets.
5. **Native UI and browser E2E.** No platform-specific execution evidence was collected on this fork at baseline.
6. **Workspace-wide coverage.** Initial instrumented job deliberately covers only three high-risk crates. No global percentage or quality threshold is justified.
7. **CI reproducibility.** GitHub Actions on a new fork may be disabled, and cross-platform jobs are not all present in the normal PR matrix. Any absent run is BLOCKED/UNVERIFIED, not passed.

Priority: obtain exact-SHA GitHub Actions results, then fix failures before widening coverage. No upstream changes or automatic merges.
