# Tasks: v0.12.15

## Phase 1 — Dead-code removal (C-1, C-2)

### Task 1.1 — Remove `_line_index` from `inside_code_span` (C-1)

**File**: `src/rules/markdown/inline/scan.rs`

- Remove `_line_index: usize` parameter from `inside_code_span` signature
- Remove leading `_` is no longer needed (parameter is gone)

**Call sites** (update all 3):

- `src/rules/markdown/inline/html.rs:31`
  - Before: `inside_code_span(code_spans, line_index, absolute_start)`
  - After: `inside_code_span(code_spans, absolute_start)`
- `src/rules/markdown/inline/links/mod.rs:96`
  - Before: `inside_code_span(code_spans, line_index, offset)`
  - After: `inside_code_span(code_spans, offset)`
- `src/rules/markdown/inline/links/autolink.rs:16`
  - Before: `inside_code_span(code_spans, line_index, absolute_start)`
  - After: `inside_code_span(code_spans, absolute_start)`

**Verification**: `cargo build --locked` compiles with no warnings.

### Task 1.2 — Direct indexing in 4 inline extractors (C-2)

Replace `.get(idx).copied().unwrap_or(false)` → `code_line_flags[idx]` in:

- `src/rules/markdown/inline/code_spans.rs:10`
- `src/rules/markdown/inline/html.rs:11`
- `src/rules/markdown/inline/links/mod.rs:20`
- `src/rules/markdown/inline/reference_definitions.rs:12`

**Verification**: `cargo test --all-features --locked` passes without regression.

## Phase 2 — Precision fix+ (P-1)

### Task 2.1 — MD052 `is_fixable = true` in `official_meta`

**File**: `src/rules/markdown/rules/md052.rs`

Add `official_meta()` override following MD048/MD055 pattern:

```rust
fn official_meta(&self) -> Option<OfficialRuleMeta> {
    let mut meta = crate::rules::markdown::catalog::get_official_meta("MD052")?;
    meta.is_fixable = true;
    Some(meta)
}
```

### Task 2.2 — MD052 `fix_info` in `evaluate_context`

**File**: `src/rules/markdown/rules/md052.rs`

Change diagnostic construction from `fix_info: None` to:

```rust
let fix_range = ctx.diagnostic_range(crate::rules::markdown::document::SourceRange {
    start: link.full_range.end - 2,
    end: link.full_range.end,
});
fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
    start_line: fix_range.start_line,
    start_column: fix_range.start_column,
    end_line: fix_range.end_line,
    end_column: fix_range.end_column,
    replacement: String::new(),
}),
```

### Task 2.3 — MD052 tests for fix_info

Add to `md052.rs` test module:

- `fix_removes_trailing_brackets` — `[ref][]` → fix_info is Some with replacement `""`
- `fix_image_removes_trailing_brackets` — `![alt][]` → fix_info is Some with replacement `""`
- `fix_info_none_for_full_reference` — `[ref][label]` → fix_info is None (no diagnostic)

## Phase 3 — Quality Gates

### Task 3.1 — Compile and unit tests

```
cargo test --all-features --locked
```

Expected: all tests pass including new MD052 fix tests.

### Task 3.2 — AST lint

```
make ast-lint
```

Expected: 15 tests pass.

### Task 3.3 — Performance check

```
make perf-check-strict
```

Expected: ratio ≤ 1.40x for all benchmarks. Refresh baseline only if improvement is observed.

### Task 3.4 — Coverage gate

```
make coverage-blocking
```

Expected: uncovered ≤ baseline. Update baseline if new tests cover new lines (net neutral or
better).

### Task 3.5 — Public confidence

```
make public-confidence
```

Expected: `unclassified_count: 0`, `release_blocking_issues: []`.

### Task 3.6 — Dogfood and release check

```
make dogfood-refresh-baseline   # only if dogfood diagnostics changed
make release-check VERSION=v0.12.15
```

## Definition of Done

- [ ] C-1: `_line_index` removed from scan.rs and 3 call sites; `cargo build` emits zero warnings
- [ ] C-2: 4 extractors use direct indexing; `cargo test --all-features --locked` passes
- [ ] P-1: MD052 emits `fix_info: Some(...)` for collapsed references; `is_fixable = true` in catalog
- [ ] All quality gates pass
- [ ] Cargo.toml version bumped to 0.12.15
- [ ] CHANGELOG updated
- [ ] PR merged and release published
