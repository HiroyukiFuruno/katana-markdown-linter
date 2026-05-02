# kml Dogfooding

This document records how this repository uses `kml` against its own Markdown files.

## Commands

Check-only:

~~~bash
just dogfood
~~~

Apply safe fixes:

~~~bash
just dogfood-fix
~~~

Machine-readable report:

~~~bash
just dogfood-json
~~~

Refresh the committed baseline after intentional cleanup:

~~~bash
just dogfood-refresh-baseline
~~~

Archived OpenSpec documents are checked only when explicitly requested:

~~~bash
just dogfood-archive
~~~

Curated public confidence corpus:

~~~bash
just public-confidence
~~~

Optional KatanA external corpus:

~~~bash
KATANA_CHECKOUT=/path/to/katana just external-katana-dogfood
~~~

## Gate Model

`just dogfood` is a regression gate. It runs `kml` against maintained Markdown
and compares the current diagnostics with `tests/fixtures/dogfood-baseline.json`.
It is included in `just check` and the GitHub CI workflow.

The gate fails when a diagnostic fingerprint exceeds the committed baseline.
The fingerprint uses path, rule ID, message, and trimmed source line so ordinary
line movement does not create noise.

When documentation cleanup intentionally removes diagnostics, run
`just dogfood-refresh-baseline` and commit the reduced baseline.

## Default Scope

Included targets:

- `README.md`
- `docs`
- `openspec`

Default exclusions:

- `openspec/changes/archive/**`
- `target/**`

The default command uses `--force-exclude` so explicit directory inputs still honor the archive exclusion.

The default command uses `.markdownlint-dogfood.json` instead of `.markdownlint.json`. The existing repository config includes official markdownlint aliases and one property shape that the current validator does not accept yet; that compatibility gap is tracked below as a CLI finding.

The dogfood config is staged. Rules with known noisy property handling or false
positives are disabled until rule parity work can make them useful as a blocking
repository gate.

## Initial Run

Status: completed with known findings, then converted into a baseline gate.

First command:

~~~bash
just dogfood
~~~

Result:

- files: 29
- files with issues: 29
- issues: 483
- fixable: 56
- exit code: 1 after switching to `.markdownlint-dogfood.json`

Current baseline:

- diagnostics: 73
- behavior: `just dogfood` exits successfully unless new diagnostics appear
- raw report: `target/dogfood-report.json`

The first run classified findings into:

- `safe-fix`: can be fixed by `just dogfood-fix`
- `manual-doc`: requires intentional documentation edits
- `cli-ux`: indicates a CLI behavior or output issue
- `known-exclusion`: intentionally excluded from the default dogfood scope

## Usability Findings

| ID | Command | Expected | Actual | Decision |
| --- | --- | --- | --- | --- |
| CLI-001 | `just dogfood` with `.markdownlint.json` discovery | Existing repository config can be used directly | Config validation rejects official aliases such as `first-line-heading`, `no-duplicate-heading`, `no-inline-html`, and `MD022.lines_below` | Use `.markdownlint-dogfood.json` for this dogfood change; move alias/property parity into compatibility work |
| CLI-002 | `just dogfood` with `.markdownlint-dogfood.json` | Configured properties reduce noisy diagnostics | `MD013.line_length`, `MD044.names`, and similar properties do not affect rule execution consistently | Track property application in compatibility work before making dogfood a blocking gate |
| CLI-003 | `just dogfood` on OpenSpec task files | Task checkboxes are not treated as links | MD039 reports spaces inside `[ ]` and `[x]` task checkbox text | Treat as a rule false-positive candidate for upstream comparison |
| CLI-004 | `just dogfood-fix` decision | Safe fixes can be previewed before writing | CLI has `--diff`, but fix mode writes files and has no dry-run mode | Do not apply dogfood fixes in this change; consider dry-run fix support later |

## Fix Decision

No Markdown files were rewritten in the initial dogfood pass.

Although 56 diagnostics were reported as fixable, automatic fix was skipped because some
diagnostics appear to be false positives and the CLI currently lacks a non-writing fix
preview mode.

## Public Confidence Corpus

`just public-confidence` runs `check`, `fix`, repeated `fix`, `fmt`, repeated
`fmt`, and final `check` against
`tests/fixtures/public-confidence/corpus`. It writes machine-readable evidence
to `target/public-confidence-report.json`.

The curated fixture includes links, images, inline HTML, fenced code, tables,
reference definitions, and mixed Japanese/English text. It is committed so
release gates never depend on a private sibling checkout.

`just external-katana-dogfood` uses the same runner against KatanA
`docs/**/*.md` and `assets/**/*.md` when `KATANA_CHECKOUT` is set. It records
the real-document inventory and classified findings without modifying the
KatanA checkout. Remaining true-positive diagnostics are evidence, not a
release blocker for this crate.
