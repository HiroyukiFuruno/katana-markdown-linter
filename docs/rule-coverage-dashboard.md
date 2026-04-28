# Rule Coverage Dashboard

Generated from `tests/fixtures/rule-fixture-matrix.json`.

| Rule | Check | Safe Fix | Unsafe Fix | Config | Edge | Golden | Known Delta | Manual Required |
| --- | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |
| MD001 | 2 | 0 | 0 | 4 | 1 | baseline | no | fix requires author intent: changing heading levels can change document structure and anchors |
| MD003 | 2 | 2 | 0 | 4 | 2 | pending: not locked | no | - |
| MD004 | 2 | 1 | 0 | 4 | 1 | pending: not locked | no | - |
| MD005 | 2 | 1 | 0 | 2 | 1 | baseline | no | - |
| MD007 | 2 | 1 | 0 | 8 | 0 | pending: not locked | no | - |
| MD009 | 2 | 1 | 0 | 10 | 1 | pending: not locked | no | - |
| MD010 | 2 | 1 | 0 | 8 | 0 | pending: not locked | no | - |
| MD011 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD012 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD013 | 2 | 0 | 0 | 18 | 1 | pending: not locked | no | fix requires author intent: line wrapping can change prose, code, tables, or inline references |
| MD014 | 3 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD018 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD019 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD020 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD021 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD022 | 2 | 1 | 0 | 6 | 0 | baseline | no | - |
| MD023 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD024 | 2 | 0 | 0 | 4 | 0 | pending: not locked | no | fix requires author intent: duplicate heading fixes require choosing new heading text |
| MD025 | 2 | 1 | 0 | 6 | 0 | pending: not locked | no | - |
| MD026 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD027 | 2 | 1 | 0 | 5 | 0 | pending: not locked | no | - |
| MD028 | 3 | 0 | 0 | 2 | 0 | pending: not locked | no | fix requires author intent: official docs allow either separating adjacent quotes with prose or joining them by adding a blockquote marker |
| MD029 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD030 | 2 | 2 | 0 | 10 | 1 | baseline | no | - |
| MD031 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD032 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD033 | 2 | 0 | 0 | 6 | 0 | pending: not locked | no | fix requires author intent: removing or replacing inline HTML changes rendered output |
| MD034 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD035 | 2 | 1 | 0 | 4 | 1 | baseline | no | - |
| MD036 | 2 | 0 | 1 | 4 | 0 | pending: not locked | no | - |
| MD037 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD038 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD039 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD040 | 2 | 1 | 0 | 6 | 0 | pending: not locked | no | - |
| MD041 | 2 | 0 | 0 | 8 | 0 | pending: not locked | no | fix requires author intent: first heading text cannot be inferred safely |
| MD042 | 2 | 0 | 0 | 2 | 0 | pending: not locked | no | fix requires author intent: empty link or image targets require author-provided destinations |
| MD043 | 3 | 0 | 0 | 6 | 0 | pending: not locked | no | check requires configured MD043.headings; default markdownlint headings is empty<br>fix requires author intent: required headings require author-provided sections and order |
| MD044 | 3 | 1 | 0 | 8 | 0 | pending: not locked | no | check requires configured MD044.names; default markdownlint names is empty<br>fix requires: configured MD044.names to avoid guessing product capitalization |
| MD045 | 2 | 0 | 0 | 2 | 0 | pending: not locked | no | fix requires author intent: alt text requires image-specific author knowledge |
| MD046 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD047 | 2 | 1 | 0 | 2 | 0 | baseline | no | - |
| MD048 | 2 | 1 | 0 | 4 | 1 | baseline | no | - |
| MD049 | 2 | 2 | 0 | 4 | 0 | pending: not locked | no | - |
| MD050 | 2 | 2 | 0 | 4 | 0 | pending: not locked | no | - |
| MD051 | 6 | 1 | 0 | 6 | 0 | pending: not locked | no | - |
| MD052 | 2 | 0 | 0 | 6 | 0 | pending: not locked | no | safe fix is not provided because the missing reference destination cannot be inferred |
| MD053 | 2 | 1 | 0 | 4 | 0 | pending: not locked | no | - |
| MD054 | 3 | 1 | 0 | 14 | 0 | pending: not locked | no | check requires a disabled link style such as MD054.collapsed=false<br>fix requires: disabled style and an inline-safe reference definition |
| MD055 | 2 | 1 | 0 | 4 | 1 | baseline | no | - |
| MD056 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD058 | 2 | 1 | 0 | 2 | 0 | pending: not locked | no | - |
| MD059 | 2 | 0 | 0 | 4 | 0 | pending: not locked | no | fix requires author intent: descriptive link text requires replacement wording |
| MD060 | 4 | 3 | 0 | 8 | 3 | pending: not locked | no | - |
