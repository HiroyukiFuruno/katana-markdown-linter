# Document Answer Scenario 035

This release note intentionally combines [spaced link](#section-035) with `inline code` so the answer fixture can verify multiple safe edits in one meaningful document.

## Section 035

Use this service at <https://example.com/scenario-035> and keep `[literal](#Wrong-Case)` inside code spans unchanged. The paragraph also keeps an inline image ![logo](https://example.com/logo-035.png) so URL parsing has surrounding Markdown.

### 案内 035

*spaced emphasis* appears next to a Unicode heading link [案内](#案内-035). The same section records a reviewed table and list context so the fixture is not a single-rule string.

| step | status |
| --- | --- |
| collect | ready |
| compare | ready |

```toml
[tool.kml]
url = "https://example.invalid/raw-035"
label = "[ spaced link ]"
```

The final paragraph references [Heading](#section-035) and keeps [inside](https://example.com/path-035) untouched.
