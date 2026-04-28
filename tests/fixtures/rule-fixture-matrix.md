# Rule Fixture Matrix

- upstream source: DavidAnson/markdownlint default branch
- total rules: 53
- manual required: 13
- missing fixtures: 0
- stale fixtures: 0

| Rule | Check Pass | Check Fail | Fix | Unsafe Fix | Config Valid | Config Invalid | Edge | Manual Required |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| MD001 | 1 | 1 | 0 | 0 | 2 | 2 | 1 | fix requires author intent: changing heading levels can change document structure and anchors |
| MD003 | 1 | 1 | 2 | 0 | 2 | 2 | 2 |  |
| MD004 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD005 | 1 | 1 | 1 | 0 | 1 | 1 | 1 |  |
| MD007 | 1 | 1 | 1 | 0 | 4 | 4 | 0 |  |
| MD009 | 1 | 1 | 1 | 0 | 5 | 5 | 1 |  |
| MD010 | 1 | 1 | 1 | 0 | 4 | 4 | 0 |  |
| MD011 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD012 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD013 | 1 | 1 | 0 | 0 | 9 | 9 | 1 | fix requires author intent: line wrapping can change prose, code, tables, or inline references |
| MD014 | 2 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD018 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD019 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD020 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD021 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD022 | 1 | 1 | 1 | 0 | 3 | 3 | 0 |  |
| MD023 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD024 | 1 | 1 | 0 | 0 | 2 | 2 | 0 | fix requires author intent: duplicate heading fixes require choosing new heading text |
| MD025 | 1 | 1 | 1 | 0 | 3 | 3 | 0 |  |
| MD026 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD027 | 1 | 1 | 1 | 0 | 3 | 2 | 0 |  |
| MD028 | 2 | 1 | 0 | 0 | 1 | 1 | 0 | fix requires author intent: official docs allow either separating adjacent quotes with prose or joining them by adding a blockquote marker |
| MD029 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD030 | 1 | 1 | 2 | 0 | 5 | 5 | 1 |  |
| MD031 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD032 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD033 | 1 | 1 | 0 | 0 | 3 | 3 | 0 | fix requires author intent: removing or replacing inline HTML changes rendered output |
| MD034 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD035 | 1 | 1 | 1 | 0 | 2 | 2 | 1 |  |
| MD036 | 1 | 1 | 0 | 1 | 2 | 2 | 0 |  |
| MD037 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD038 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD039 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD040 | 1 | 1 | 1 | 0 | 3 | 3 | 0 |  |
| MD041 | 1 | 1 | 0 | 0 | 4 | 4 | 0 | fix requires author intent: first heading text cannot be inferred safely |
| MD042 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | fix requires author intent: empty link or image targets require author-provided destinations |
| MD043 | 2 | 1 | 0 | 0 | 3 | 3 | 0 | check requires configured MD043.headings; default markdownlint headings is empty<br>fix requires author intent: required headings require author-provided sections and order |
| MD044 | 2 | 1 | 1 | 0 | 4 | 4 | 0 | check requires configured MD044.names; default markdownlint names is empty; fix requires: configured MD044.names to avoid guessing product capitalization |
| MD045 | 1 | 1 | 0 | 0 | 1 | 1 | 0 | fix requires author intent: alt text requires image-specific author knowledge |
| MD046 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD047 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD048 | 1 | 1 | 1 | 0 | 2 | 2 | 1 |  |
| MD049 | 1 | 1 | 2 | 0 | 2 | 2 | 0 |  |
| MD050 | 1 | 1 | 2 | 0 | 2 | 2 | 0 |  |
| MD051 | 5 | 1 | 1 | 0 | 3 | 3 | 0 |  |
| MD052 | 1 | 1 | 0 | 0 | 3 | 3 | 0 | safe fix is not provided because the missing reference destination cannot be inferred |
| MD053 | 1 | 1 | 1 | 0 | 2 | 2 | 0 |  |
| MD054 | 2 | 1 | 1 | 0 | 7 | 7 | 0 | check requires a disabled link style such as MD054.collapsed=false; fix requires: disabled style and an inline-safe reference definition |
| MD055 | 1 | 1 | 1 | 0 | 2 | 2 | 1 |  |
| MD056 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD058 | 1 | 1 | 1 | 0 | 1 | 1 | 0 |  |
| MD059 | 1 | 1 | 0 | 0 | 2 | 2 | 0 | fix requires author intent: descriptive link text requires replacement wording |
| MD060 | 3 | 1 | 3 | 0 | 3 | 5 | 3 |  |
