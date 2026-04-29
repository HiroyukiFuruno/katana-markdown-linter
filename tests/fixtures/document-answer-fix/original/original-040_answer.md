# Document Answer Scenario 040

This release note intentionally combines [spaced link](#section-040) with `inline code` so the answer fixture can verify multiple safe edits in one meaningful document.

## Section 040

Use this service at <https://example.com/scenario-040> and keep `[literal](#Wrong-Case)` inside code spans unchanged. The paragraph also keeps an inline image ![logo](https://example.com/logo-040.png) so URL parsing has surrounding Markdown.

### 案内 040

*spaced emphasis* appears next to a Unicode heading link [案内](#案内-040). The same section records a reviewed table and list context so the fixture is not a single-rule string.

| step | status |
| --- | --- |
| collect | ready |
| compare | ready |

```toml
[tool.kml]
url = "https://example.invalid/raw-040"
label = "[ spaced link ]"
```

The final paragraph references [Heading](#section-040) and keeps [inside](https://example.com/path-040) untouched.
