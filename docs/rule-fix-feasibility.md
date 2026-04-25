# Rule Fix Feasibility

This document records the `v0.8.0` safe-fix prioritization for rules that were
diagnostic-only at the start of the change.

## States

| State | Meaning |
| --- | --- |
| `safe-now` | Deterministic rewrite is local and does not require author intent. |
| `safe-after-context` | Rewrite can be safe after structural source ranges or collision guards are available. |
| `unsafe-candidate` | Rewrite may be useful, but must require explicit unsafe opt-in. |
| `manual-required` | Correct output requires author knowledge or new content. |

## Classification

| Rule | State | Reason |
| --- | --- | --- |
| `MD001` | `unsafe-candidate` | Changing heading levels can change document structure and anchors. |
| `MD003` | `safe-after-context` | Setext/ATX conversion needs source ranges and style-specific guards. |
| `MD013` | `manual-required` | Line wrapping can change prose, code, tables, or inline references. |
| `MD024` | `manual-required` | Duplicate heading fixes require choosing new heading text. |
| `MD028` | `unsafe-candidate` | Official docs allow either separating quotes or joining them with `>`. |
| `MD033` | `manual-required` | Removing or replacing inline HTML changes rendered output. |
| `MD035` | `safe-now` | Horizontal rule style normalization preserves Markdown meaning. Implemented in `v0.8.0` first batch. |
| `MD036` | `unsafe-candidate` | Converting emphasis to heading changes structure and anchors. Exposed as an unsafe fix candidate in `v0.9.0`. |
| `MD041` | `manual-required` | First heading text cannot be inferred safely. |
| `MD042` | `manual-required` | Empty URL or image targets require author-provided destinations. |
| `MD043` | `manual-required` | Required headings require author-provided sections and order. |
| `MD045` | `manual-required` | Alt text requires image-specific author knowledge. |
| `MD046` | `safe-after-context` | Indented/fenced conversion needs block ranges and code-content guards. |
| `MD048` | `safe-now` | Fence marker conversion uses block ranges and skips collision-prone code blocks. Implemented in `v0.8.1`. |
| `MD052` | `manual-required` | Missing reference definitions require destinations or labels. |
| `MD055` | `safe-now` | Table pipe style can be fixed when `DocumentContext` marks the table safe. Implemented in `v0.8.0` first batch. |
| `MD056` | `manual-required` | Column count fixes require deciding missing cell content. |
| `MD059` | `manual-required` | Descriptive anchor text requires replacement wording. |

## First Batch

`MD035` and `MD055` are the first implementation batch. `MD035` is local and
deterministic, and it exposed an existing check precision issue: YAML front
matter delimiters must not seed the `consistent` horizontal rule style. `MD055`
reuses parsed table blocks so pipe-like text inside fenced code is not treated
as a table.
