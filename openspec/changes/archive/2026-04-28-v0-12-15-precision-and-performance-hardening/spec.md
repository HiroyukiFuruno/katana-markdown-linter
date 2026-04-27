# Spec: v0.12.15

## Capability: dead-code-removal

Removes provably unused code introduced during `v0.12.14` refactoring:

- `_line_index: usize` parameter in `scan::inside_code_span` — the `partition_point`-based
  implementation uses only `code_spans` and `offset`; the line index was never needed after that
  rewrite. Removing it shrinks each call site by one argument and eliminates the leading-underscore
  workaround.

- `.get(idx).copied().unwrap_or(false)` in 4 inline extractors — `code_line_flags` length is
  guaranteed by `build_code_line_flags(lines.len(), ...)`, so the `get`/`copied`/`unwrap_or`
  fallback path is unreachable. Direct indexing `code_line_flags[idx]` is the correct form.

**Behavioral contract**: identical output to `v0.12.14` for all inputs. No rule output changes.

## Capability: safe-fix-coverage (MD052)

MD052 (`reference-links-images`) now provides a safe fix for collapsed reference links and images.

**Trigger**: any inline link or image with `is_collapsed_reference()` == true (`[ref][]`,
`![alt][]`).

**Fix action**: delete the trailing `[]` (2-byte range at `full_range.end - 2 .. full_range.end`).
Replacement is the empty string.

**Safety conditions**:

1. The fix range is always within the document bounds (any collapsed reference token is ≥ 4 bytes).
2. The fix does not change link semantics: `[ref][]` is a collapsed reference to label `ref`;
   `[ref]` is a shortcut reference to the same label. CommonMark §6.6 treats them as equivalent.
3. The fix is idempotent: applying it once to `[ref][]` yields `[ref]`, which no longer triggers
   MD052.
4. Multiple collapsed references on one line produce non-overlapping fix ranges.

**Catalog metadata**: `is_fixable: true` is set in `official_meta()` override so the catalog,
CLI `--fix`, MCP `apply_fix`, and API surfaces all advertise fixability for MD052.
