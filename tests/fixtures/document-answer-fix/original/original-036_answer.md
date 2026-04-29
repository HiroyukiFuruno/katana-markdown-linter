# Document Answer Scenario 036

This release note intentionally combines [spaced link](#section-036) with `inline code` so the answer fixture can verify multiple safe edits in one meaningful document.

## Section 036

Use this service at <https://example.com/scenario-036> and keep `[literal](#Wrong-Case)` inside code spans unchanged. The paragraph also keeps an inline image ![logo](https://example.com/logo-036.png) so URL parsing has surrounding Markdown.

### 案内 036

*spaced emphasis* appears next to a Unicode heading link [案内](#案内-036). The same section records a reviewed table and list context so the fixture is not a single-rule string.

| step | status |
| --- | --- |
| collect | ready |
| compare | ready |

```toml
[tool.kml]
url = "https://example.invalid/raw-036"
label = "[ spaced link ]"
```

The final paragraph references [Heading](#section-036) and keeps [inside](https://example.com/path-036) untouched.
