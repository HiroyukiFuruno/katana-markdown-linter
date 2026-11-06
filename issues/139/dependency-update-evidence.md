# Dependency update evidence

upstream release: [KAL 0.5.4](https://github.com/HiroyukiFuruno/katana-ast-lint/releases/tag/v0.5.4)
is resolved from crates.io. The npm publishing job uses Node.js 26 LTS.

api migration note: KAL is a development dependency. The repository uses the
same standard runner, configuration, scan scope, and thresholds. The migration
keeps Markdown rule IDs, diagnostics, edit ordering, and public APIs compatible.

verification evidence: The unchanged AST gate detected 120 function-length
violations before the refactor. The new isolated workspace cases exercise the
same gate for production lines after an external test module declaration and
the 30-line function-body threshold. Both cases expose missed detections with
KAL 0.5.3 and pass with 0.5.4. The npm package check passed on Node.js 26.13.0,
and actionlint validated the workflow.
