## Why

v0.6.0 established the localized public surface. The next release should make
the core engine faster before expanding broader unsafe fix behavior or
distribution channels.

The current performance documentation already identifies `api_fix_large_document`
as the largest local regression. The fix pipeline repeatedly converts
line/column ranges back to byte offsets for every candidate fix. That is a
good v0.7.0 target because it is measurable, internal, and does not change the
public safety contract.

`rumdl` is useful as a reference point for performance culture: release-profile
benchmarks, editor-friendly stdout behavior, caching/watch as later work, and
CLI/product documentation. kml should not copy rumdl internals, but should keep
its own hot paths measurable against similar expectations.

## What Changes

- Optimize fix application range mapping so a single `fix_with_results` call does not rescan the entire document for every fix candidate.

- Keep public `lint`, `fix`, and `fix_with_results` behavior source-compatible.

- Add regression tests for Unicode, virtual EOF, and multi-line fix ranges after the optimized offset lookup.

- Refresh performance documentation and baseline after intentional improvement.

- Reframe the active roadmap so `v0.7.0` is performance, later work continues linter precision, unsafe fix mode, formatter productization, and distribution/tooling expansion separately.

## Scope

In scope:

- Internal fix application hot path.
- Performance benchmark/baseline updates.
- Roadmap updates for the next release sequence.
- A local `rumdl` reference summary in design notes.

Out of scope:

- Unsafe fix mode implementation.
- Watch mode or persistent cache implementation.
- New distribution channels such as npm, pip, Homebrew, or GitHub Action.
- Changing rule semantics to chase benchmark wins.

## Impact

- Large-document fix performance should improve without changing fixed output.
- Existing safe fix policy remains unchanged.
- Future performance work has a clearer sequence: internal hot paths first,
  then cache/watch and editor workflows when their contracts are specified.
