# kml Dogfooding

This document records how this repository uses `kml` against its own Markdown files.

## Commands

Check-only:

```bash
make dogfood
```

Apply safe fixes:

```bash
make dogfood-fix
```

Machine-readable report:

```bash
make dogfood-json
```

Archived OpenSpec documents are checked only when explicitly requested:

```bash
make dogfood-archive
```

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

## Initial Run

Status: completed with known findings.

First command:

```bash
make dogfood
```

Result:

- files: 29
- files with issues: 29
- issues: 483
- fixable: 56
- exit code: 1 after switching to `.markdownlint-dogfood.json`

The first run classified findings into:

- `safe-fix`: can be fixed by `make dogfood-fix`
- `manual-doc`: requires intentional documentation edits
- `cli-ux`: indicates a CLI behavior or output issue
- `known-exclusion`: intentionally excluded from the default dogfood scope

## Usability Findings

| ID | Command | Expected | Actual | Decision |
| --- | --- | --- | --- | --- |
| CLI-001 | `make dogfood` with `.markdownlint.json` discovery | Existing repository config can be used directly | Config validation rejects official aliases such as `first-line-heading`, `no-duplicate-heading`, `no-inline-html`, and `MD022.lines_below` | Use `.markdownlint-dogfood.json` for this dogfood change; move alias/property parity into compatibility work |
| CLI-002 | `make dogfood` with `.markdownlint-dogfood.json` | Configured properties reduce noisy diagnostics | `MD013.line_length`, `MD044.names`, and similar properties do not affect rule execution consistently | Track property application in compatibility work before making dogfood a blocking gate |
| CLI-003 | `make dogfood` on OpenSpec task files | Task checkboxes are not treated as links | MD039 reports spaces inside `[ ]` and `[x]` task checkbox text | Treat as a rule false-positive candidate for upstream comparison |
| CLI-004 | `make dogfood-fix` decision | Safe fixes can be previewed before writing | CLI has `--diff`, but fix mode writes files and has no dry-run mode | Do not apply dogfood fixes in this change; consider dry-run fix support later |

## Fix Decision

No Markdown files were rewritten in the initial dogfood pass.

Although 56 diagnostics were reported as fixable, automatic fix was skipped because some
diagnostics appear to be false positives and the CLI currently lacks a non-writing fix
preview mode.
