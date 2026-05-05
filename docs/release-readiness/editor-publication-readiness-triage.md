# Editor Publication Readiness Triage

## Decision

`v0.18.7` is permanently skipped after the accidental publication. Do not
publish `v0.18.7` again, and do not continue with `v0.18.8`.

This readiness note is not a versioned release plan. It records why the current
VS Code and Zed extensions are not ready for marketplace publication and which
follow-up OpenSpec changes must be created before any marketplace release.

Marketplace publication remains No-Go until all of the following are covered by
separate implementation changes:

- configuration-aware diagnostics for `.markdownlint.json` and `.markdownlint.jsonc`
- editor-triggered formatting and safe quick fixes in both VS Code and Zed
- final editor dogfood evidence that covers diagnostics, formatting, safe fixes, and configuration changes
- a dedicated marketplace publication change with VS Code and Zed verification steps

## Current Capability

VS Code and Zed both start `kml lsp`. The LSP can return Markdown diagnostics,
formatting edits, range formatting edits, and safe quick fixes.

The current LSP path uses default lint options for diagnostics and quick fixes.
It does not resolve project configuration before reporting editor diagnostics or
fixes. That means the editor surface is not yet equivalent to `kml check` or
`kml fix` for a configured repository.

## Required Follow-Up Changes

Created separate OpenSpec changes for:

- `v0-19-0-editor-capability-completion`: editor LSP configuration loading, configuration-aware diagnostics, editor safe fixes, formatting validation, and final editor dogfood
- `v0-20-0-editor-marketplace-publication`: marketplace publication after the editor capability and dogfood evidence pass

## Local Evidence

Completed local evidence:

- `just dogfood`: passed with 1 current diagnostic, 3 baseline diagnostics, and 2 resolved diagnostics.
- `cat target/dogfood-report.json`: confirmed the only remaining diagnostic is the existing `docs/editor-integration.md` MD037 baseline item.
- `just VERSION=v0.19.0 release-target-check`: passed with `v0.18.6 -> v0.19.0`; all external channel states were absent for `v0.19.0`.
- Code review found that current editor diagnostics and quick fixes use default lint options instead of resolving project configuration.

`just dogfood-json` emits the same existing diagnostic and returns a non-zero
diagnostic exit code, so it is evidence output rather than a separate pass/fail
gate for this triage.

## References

- VS Code extension publishing: <https://code.visualstudio.com/api/working-with-extensions/publishing-extension>
- Zed extension publishing: <https://zed.dev/docs/extensions/developing-extensions>
