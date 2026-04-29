#Document Answer Scenario 031

This release note intentionally combines [ spaced link ](#Section-031) with ` inline code ` so the answer fixture can verify multiple safe edits in one meaningful document.

## Section 031

Use this service at https://example.com/scenario-031 and keep `[literal](#Wrong-Case)` inside code spans unchanged. The paragraph also keeps an inline image ![logo](https://example.com/logo-031.png) so URL parsing has surrounding Markdown.

### 案内 031

* spaced emphasis * appears next to a Unicode heading link [案内](#案内-031). The same section records a reviewed table and list context so the fixture is not a single-rule string.

| step | status |
| --- | --- |
| collect | ready |
| compare | ready |

```toml
[tool.kml]
url = "https://example.invalid/raw-031"
label = "[ spaced link ]"
```

The final paragraph references [Heading](#Section-031) and keeps [inside](https://example.com/path-031) untouched.
