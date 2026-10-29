# Markdown syntax support

kml is a source-preserving linter. Its syntax context and fix pipeline share the
same implementation across the library, CLI, formatter, LSP, and MCP entrypoints.

## Specification baseline

The following primary sources were checked on October 29, 2026:

| Source | Published version | Scope |
| --- | --- | --- |
| [CommonMark](https://spec.commonmark.org/0.31.2/) | 0.31.2 | Core Markdown syntax |
| [GFM](https://github.github.com/gfm/) | 0.29-gfm | GitHub Flavored Markdown extensions |
| [GitHub writing documentation](https://docs.github.com/en/get-started/writing-on-github) | Continuously updated | Platform behavior beyond the published GFM specification |

GitHub alerts, footnotes, mathematics, and diagram rendering are not evidence of
a new CommonMark version. Syntax support and lint policy are separate: a valid
Markdown construct can still produce a configured style diagnostic.

## Shared boundaries

`DocumentContext` owns source positions and caches syntax information. The
`pulldown-cmark` adapter supplies fenced-code and multiline inline-code boundaries,
including container boundaries. Paired single-line backticks use the shared
source scanner when no fence or unmatched run requires block parsing. Source adapters retain the
original heading and table-cell byte ranges. Individual lint rules consume this
context rather than maintain their own backtick state machines. Escape parity and
delimiter lookup are shared source operations. Reversed-link repair candidates
come from the shared source adapter; valid emphasis/strong-emphasis spans come
from cached parser ranges. Escaped literal punctuation does not select a style
or become a syntax-repair candidate.
Shared source checks apply CommonMark delimiter rules. Every proposed style edit
is compared against the original parsed structure and literal content. The syntax
index records enclosing source scopes and event boundaries; proven independent
inline scopes can be compared locally, while reference-dependent or ambiguous
contexts retain full-document comparison. This preserves shortcut and collapsed
reference bindings, including previously unresolved labels. A style edit that
cannot preserve meaning keeps its diagnostic without a safe automatic fix.

An unmatched backtick still conservatively protects the remainder of its line
from automatic edits. This is an existing kml safety policy, not a claim that
CommonMark renders an unmatched backtick as code.

The regression suite covers escaped table pipes, tab-separated ATX headings,
fence indentation, closing-fence suffixes, multiline code, escaped backticks,
and source ranges for Unicode and CRLF input. kml does not claim complete
CommonMark, GFM, or GitHub renderer conformance.

## Fix responsibilities

Rules produce diagnostics and fix candidates. A shared fix engine selects
syntax repairs before content/style edits, followed by layout edits. Each
applied batch triggers a fresh check; coordinates from an earlier document
snapshot are never carried into the next batch.

The edit applicator resolves overlapping candidates deterministically and
applies edits to source bytes. Repeated content and an exhausted iteration
budget with effective edits remaining are errors. A document that fails
convergence is not written back. The single-pass
`MarkdownLinter::fix_with_results` API keeps its existing contract.

## Configuration boundaries

The public schema, runtime validation, and option conversion consume the same
metadata for common rule properties such as `enabled`. Configuration discovery
and preparation are shared within a CLI operation; the next operation reads
settings afresh. CLI file/stdin formatting
and editor formatting use the same layout policy with resolved enable flags and
numeric settings. A valid four-space list configuration therefore stays valid
after formatting. Non-layout fixes remain outside the formatter policy.
The formatter's intrinsic line-ending and terminal-newline normalization remains
separate from rule-driven layout fixes.

## Verification and future updates

- Run `just markdown-spec-check` for shared syntax and fix-boundary regressions.
- Run `just document-answer-fix` for independently reviewed document answers,
  repeated fix stability, and remaining safe-fix diagnostics. The corpus contains
  270 documents: 200 public samples and 70 original composite documents. Manifest
  validation rejects duplicate IDs and ambiguous input/answer mappings before
  launching kml. Temporary execution filenames use a shared ordinal allocator,
  independently of logical IDs, so case-folding filesystems cannot merge samples.
- The same gate runs a small configuration/structure matrix across library,
  file, and stdin entrypoints, with eight required cases. Independently reviewed mutation cases test actual
  repairs, while escaped-literal cases guard against unwanted edits. Style-only
  cases also compare parsed Markdown events to verify meaning preservation.
  Expected unfixable diagnostics are matched by rule and count; all other cases
  must remain diagnostic-free, and none may retain a safe fix.
- Run `just upstream-golden` for the markdownlint compatibility corpus.
- Run `just public-confidence`, `just perf-check-strict`, and the existing
  release quality gates before publication.

For a specification update, record the primary source and version, add failing
input and independently specified expected output, update the shared adapter,
and rerun these gates. Update an existing answer only when a documented syntax
or fix-policy correction establishes that the old answer is wrong; do not
regenerate answers from the implementation under test.
