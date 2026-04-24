## Context

KML already has a repeatable internal performance benchmark and baseline gate.
That benchmark is useful for KML-to-KML regression detection, but it does not
answer whether the `kml` CLI is broadly competitive with peer Rust Markdown
linters. `mado` publishes hyperfine-style CLI benchmarks, and `rumdl` exposes a
feature-rich CLI with `check`, `fmt`, `check --fix`, JSON output, and broad
markdownlint compatibility.

External comparison must account for different rule coverage, different config
formats, and different exit-code behavior. A diagnostics-heavy check command is
expected to exit non-zero for all tools, so benchmark commands need an adapter
layer that treats expected violation exits as successful benchmark iterations.

## Goals / Non-Goals

**Goals:**

- Measure `kml`, `mado`, and `rumdl` as external CLI binaries under the same
  process model.
- Provide clean-corpus, dirty-corpus, and fix benchmarks.
- Separate default-rule comparison from common-subset comparison.
- Generate JSON and Markdown reports suitable for local review and issue/PR
  discussion.
- Keep cross-tool benchmarks opt-in and deterministic enough for local
  investigation without making CI depend on third-party tools.

**Non-Goals:**

- This change does not require KML to beat `mado` or `rumdl`.
- This change does not change KML lint behavior to match either tool.
- This change does not publish third-party benchmark claims as release
  marketing.
- This change does not add external CLI installation to the required CI gate.

## Decisions

### Decision: use an external CLI harness around hyperfine-compatible commands

The repository will add a script that builds command lines for each tool and can
execute them through `hyperfine` when available. The script will also support a
fallback timing path so developers can still generate a rough report without
installing `hyperfine`.

Alternative considered: call each tool directly from a Rust benchmark. That
would make setup simpler for KML but would not match the process model used by
published CLI benchmarks.

### Decision: normalize expected violation exit codes

The benchmark harness will wrap each check command so an expected violation
exit does not fail the benchmark runner. Tool errors such as missing binaries,
invalid configs, or unexpected exit codes will still fail the case.

Alternative considered: use only clean corpora. That hides important runtime
behavior because diagnostics-heavy documents are common in lint adoption and
exercise output/reporting paths differently.

### Decision: isolate fix benchmarks with per-run workspace copies

Fix benchmarks will copy the source corpus into a temporary per-tool workspace
before each measured run. The source corpus is never mutated by a benchmark
case. The report records whether a tool supports the requested fix command.

Alternative considered: run fix commands against the same workspace and reset
with git checkout. That is faster to write but unsafe for generated corpora and
does not work well outside a git worktree.

### Decision: include default and common-subset modes

Default mode measures each tool as a user would likely run it. Common-subset
mode enables only a small shared set of markdownlint-style rules that are
available across the compared tools. The report labels these modes separately
and MUST NOT imply that default-mode timings are pure engine-speed comparisons.

Alternative considered: only compare default mode. That is easy but unfair
because rule count, config behavior, and parser features differ substantially.

### Decision: keep report-first behavior

The cross-tool benchmark will produce report data even when one optional tool is
missing. Missing tools are represented as skipped cases with reason strings. The
required KML case must always run.

Alternative considered: fail the whole command if `mado` or `rumdl` is missing.
That makes local setup brittle and blocks KML-only regression investigation.

## Risks / Trade-offs

- [Risk] Cross-tool results can be misread as exact apples-to-apples
  performance truth. Mitigation: report mode, corpus, enabled rules, tool
  versions, and skipped cases explicitly.
- [Risk] Third-party CLI flags can change. Mitigation: keep tool command
  templates isolated in the harness and document supported versions in the
  report.
- [Risk] Hyperfine can be missing. Mitigation: provide a fallback
  runner and mark the timing method in the report.
- [Risk] Common-subset configs can drift from tool support. Mitigation: make
  config generation deterministic and validate commands before timing.

## Migration Plan

Add the benchmark harness and fixtures behind new Makefile targets. Keep the
existing `make check` and `make perf-check` unchanged. Developers can run the
cross-tool benchmark locally after installing optional peer tools. If the
benchmark reveals severe KML slowness, follow-up performance changes must be
planned separately from this measurement contract.

## Open Questions

None.
