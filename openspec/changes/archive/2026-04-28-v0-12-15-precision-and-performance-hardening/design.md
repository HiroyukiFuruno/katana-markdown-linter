# Design: v0.12.15

## C-1: Remove dead `_line_index` parameter

### Current state (scan.rs)

```rust
pub(super) fn inside_code_span(
    code_spans: &[InlineCodeSpan],
    _line_index: usize,   // never read after partition_point rewrite
    offset: usize,
) -> bool {
    let idx = code_spans.partition_point(|span| span.full_range.start <= offset);
    idx > 0 && offset < code_spans[idx - 1].full_range.end
}
```

Call sites (all pass a `line_index` they compute but do not need):

| File | Line | Call |
|---|---|---|
| `inline/html.rs` | 31 | `inside_code_span(code_spans, line_index, absolute_start)` |
| `inline/links/mod.rs` | 96 | `inside_code_span(code_spans, line_index, offset)` |
| `inline/links/autolink.rs` | 16 | `inside_code_span(code_spans, line_index, absolute_start)` |

### After

```rust
pub(super) fn inside_code_span(
    code_spans: &[InlineCodeSpan],
    offset: usize,
) -> bool {
    let idx = code_spans.partition_point(|span| span.full_range.start <= offset);
    idx > 0 && offset < code_spans[idx - 1].full_range.end
}
```

Call sites drop the `line_index` argument.

### Equivalence

The parameter was already unused (`_` prefix); removal changes only the signature arity, not any
computed value.

---

## C-2: Direct indexing in inline extractors

### Invariant

`build_code_line_flags(lines.len(), &code_blocks)` returns `Vec<bool>` of length exactly
`lines.len()`. Every enumerated `idx` from `lines.iter().enumerate()` satisfies
`0 <= idx < lines.len()`, making `code_line_flags[idx]` infallible.

### Change (identical pattern in all 4 extractors)

Before:
```rust
if code_line_flags.get(idx).copied().unwrap_or(false) {
    continue;
}
```

After:
```rust
if code_line_flags[idx] {
    continue;
}
```

Files: `code_spans.rs:10`, `html.rs:11`, `links/mod.rs:20`, `reference_definitions.rs:12`.

### Equivalence

Same boolean value; `unwrap_or(false)` branch is unreachable by invariant, so the output is
identical for all valid inputs.

---

## P-1: MD052 collapsed reference safe-fix

### Problem

`[ref][]` triggers MD052 with `fix_info: None`. The fix is simple: remove the trailing `[]`.

### Fix range

For a collapsed reference `[ref][]`, `link.full_range` covers the entire token including `[]`.
The trailing `[]` occupies the last 2 bytes of `full_range`:

```
full_range.end - 2  ..  full_range.end
```

Replacement: empty string (deletion).

### Implementation

In `md052.rs::evaluate_context`:

1. Override `official_meta()` to set `is_fixable = true` (same pattern as MD048, MD055).
2. Build fix_range: `SourceRange { start: link.full_range.end - 2, end: link.full_range.end }`.
3. Convert via `ctx.diagnostic_range(fix_range)`.
4. Set `fix_info: Some(DiagnosticFix { ..fix_range_coords, replacement: String::new() })`.

### Correctness conditions

- `link.full_range.end >= 2` is always true for any token matching `[...][]` (minimum 4 bytes).
- The fix is idempotent and non-overlapping with any other MD052 diagnostic on the same line
  because collapsed references cannot nest.
- Images `![alt][]` share the same `full_range` structure; the trailing `[]` is the same 2 bytes.

### Test additions

Extend `md052.rs` tests:

- `fix_removes_trailing_brackets`: verify `fix_info` is `Some` and `replacement` is `""` for
  `[ref][]`.
- `fix_image_removes_trailing_brackets`: same for `![alt][]`.
- `fix_info_none_for_full_reference`: `[ref][label]` must still have `fix_info: None`.
