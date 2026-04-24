# Rule Fixture Matrix

- upstream source: DavidAnson/markdownlint default branch
- total rules: 53
- manual required: 8
- missing fixtures: 0
- stale fixtures: 0

| Rule | Check Pass | Check Fail | Fix | Config Valid | Config Invalid | Edge | Manual Required |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| MD001 | 1 | 1 | 0 | 1 | 1 | 1 |  |
| MD003 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD004 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD005 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local fix overlaps with MD007/list indentation strategy and is unsafe in global fix mode |
| MD007 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD009 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local regex implementation reports diagnostics only and has no safe trailing-space fix_info yet |
| MD010 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local regex implementation reports diagnostics only and has no safe tab replacement fix_info yet |
| MD011 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD012 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local fix is multi-line and current fix engine intentionally skips multi-line edits |
| MD013 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD014 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD018 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD019 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD020 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD021 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local fix overlaps with MD027 blockquote spacing in global fix mode |
| MD022 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD023 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD024 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD025 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD026 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD027 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local fix overlaps with MD021 blockquote spacing in global fix mode |
| MD028 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD029 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD030 | 1 | 1 | 0 | 1 | 1 | 0 | fix unsupported: local list marker spacing fix range is not safe enough to lock before/after output yet |
| MD031 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD032 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD033 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD034 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD035 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD036 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD037 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD038 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD039 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD040 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD041 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD042 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD043 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD044 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD045 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD046 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD047 | 1 | 1 | 1 | 1 | 1 | 0 |  |
| MD048 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD049 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD050 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD051 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD052 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD053 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD054 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD055 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD056 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD058 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD059 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD060 | 1 | 1 | 0 | 3 | 5 | 0 | fix unsupported: table column style requires strategy-aware table formatting before safe fix can be locked |
