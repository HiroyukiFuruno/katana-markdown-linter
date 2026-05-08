# Quality Investigation Report: kml check / fix (v0.20.0 Readiness)

## Summary

Investigation conducted to evaluate the practical quality of `kml check` and `kml check --fix` before the `v0.20.0` release.

The investigation revealed one **critical quality issue** in the `MD037` (no-space-in-emphasis) rule that can corrupt Markdown structure. Other core rules (`MD007`, `MD022`, `MD032`, etc.) are stable and provide safe, accurate fixes.

## Key Findings

### 1. Critical Issue: MD037 Corruption (Release-Blocking)

The `MD037` rule misinterprets separate bold/italic spans on the same line as a single span with spaces. Its automated fix merges these spans, resulting in broken Markdown.

- **Problematic Case:** `**Note:** Neovim support is provided as a **docs-only sample**.`
- **Incorrect Fix Suggestion:** `**Neovim support is provided as a**` (Merging the two bold spans and deleting text in between).
- **Impact:** High. Automated fixes can destroy document structure without warning.
- **Classification:** **Release-Blocking**. This rule should be disabled or fixed before release.

### 2. Stable Rules and Fixes (Non-Blocking)

The following rules were verified to work correctly and provide safe fixes:

- **MD007 (ul-indent):** Correctly catches and fixes inconsistent unordered list indentation.
- **MD022 (blanks-around-headings):** Correctly identifies missing blank lines around headings and inserts them safely.
- **MD032 (blanks-around-lists):** Correctly identifies missing blank lines around lists and inserts them safely.
- **MD034 (no-bare-urls):** Correctly wraps bare URLs in angle brackets.
- **MD038 (no-space-in-code):** Correctly removes spaces inside inline code spans.

### 3. Detection Accuracy (False Negatives)

Detection is generally accurate for enabled rules. No significant "missed detections" were found for core structural rules when they are enabled in the configuration.

## Recommendations

1. **Fix or Disable MD037:** The current implementation of `MD037` is unsafe. It must be either fixed to properly handle multiple spans on one line or disabled by default/in dogfood config until fixed.
2. **Promote Safe Fixes:** Continue promoting `--fix` for structural rules (`MD007`, `MD022`, `MD032`) as they are reliable.
3. **Documentation:** Explicitly state in the release notes which rules are considered "safe" for automated fixing.

## Investigation Details

- **Target Files:** `README.md`, `docs/**/*.md`, `openspec/**/*.md` (excluding archives).
- **Tools Used:** `kml check`, `kml check --fix`, `git diff`, custom test cases.
- **Date:** 2026-05-08
