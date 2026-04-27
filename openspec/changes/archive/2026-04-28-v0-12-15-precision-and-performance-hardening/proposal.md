# Proposal: v0.12.15 — Dead-Code Removal and Precision Fix+ Expansion

## Why

`v0.12.14` eliminated the major hot-path costs (O(L×b) scan, O(s) inside_code_span, String alloc in
backtick scanner). Two clean-up items remain that were deferred as non-blocking follow-ups:

1. **Dead `_line_index` parameter** in `scan::inside_code_span` — the parameter is unused after the
   `partition_point` rewrite but still appears in the public(super) signature and at every call site
   (`html.rs`, `links/mod.rs`, `autolink.rs`). It is dead code and compiles with a leading
   underscore only to suppress the warning.

2. **Defensive `.get(idx).copied().unwrap_or(false)` pattern** in all 4 inline extractors —
   `code_line_flags` is built by `build_code_line_flags(lines.len(), &code_blocks)`, guaranteeing
   exactly `lines.len()` elements. The `get` + `copied` + `unwrap_or` chain is provably unreachable
   and masks the invariant. Direct `code_line_flags[idx]` is correct and clearer.

In addition, the precision fix+ roadmap continuation begins: MD052 currently emits a warning for
`[ref][]` but leaves `fix_info: None`, meaning the user must fix manually. The fix is safe and
mechanical: delete the trailing `[]` to produce `[ref]`.

## What

- **C-1**: Remove `_line_index: usize` from `inside_code_span` signature; update 3 call sites.
- **C-2**: Replace `.get(idx).copied().unwrap_or(false)` with `code_line_flags[idx]` in
  `code_spans.rs`, `html.rs`, `links/mod.rs`, and `reference_definitions.rs`.
- **P-1**: Add `fix_info` to MD052 diagnostics (safe-fix: delete trailing `[]`); set
  `is_fixable = true` in `official_meta()`.

## Capabilities

- `dead-code-removal`: eliminate provably unused parameter and unreachable fallback branches
- `safe-fix-coverage`: MD052 collapsed reference links and images gain a one-step auto-fix

## Impact

- Binary size: marginally smaller (one fewer unused parameter slot per call)
- Fix coverage: MD052 becomes fixable for the first time
- No behavioral change for any rule output (C-1 and C-2 are refactors with equivalent semantics)

## Non-Goals

- No new rules or rule changes beyond MD052 fix+
- No MD046/MD043/MD056 fixes in this change (sequenced to next patches per precision-first cadence)
- No MD013 wrap fix (deferred to unsafe-fix mode, v0.13.x)
