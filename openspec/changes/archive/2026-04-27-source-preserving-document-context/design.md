## Design

## Architecture

`DocumentContext` を rule execution の中心に置く。

Flow: content enters `DocumentContext`, rules consume context for check/fix, and
an optional lazy AST cache remains secondary.

AST は source-preserving context の一部であり、主役ではない。rule は AST だけを信頼せず、fix range を作る時は line offsets と original source slice を使う。

## Design Decisions

- Eager に構築するのは line offsets、line info、front matter、fenced code block ranges までを初期値とする
- Headings、references、tables は rule family の移行に合わせて eager または once-cell style lazy cache のどちらかを計測して決める
- Optional AST は parser dependency を選定した後でも public API に露出しない
- `DocumentContext` は単一 file content の責務に閉じる。directory traversal、ignore、parallelism は CLI layer の責務として維持する
- `LintOptions` / config resolution は既存 API を維持し、rule enablement は context 生成とは分離する

## DocumentContext Shape

Initial context fields:

- `content: &str`
- `file_path: &Path`
- `lines: Vec<LineInfo>`
- `line_offsets: Vec<usize>`
- `front_matter: Option<Range<usize>>`
- `code_blocks: Vec<BlockRange>`
- `headings: Vec<Heading>`
- `references: Vec<Reference>`
- `tables: Vec<TableBlock>`
- lazy inline code spans
- lazy emphasis spans
- optional/lazy AST

All ranges SHALL be source-preserving. Byte ranges are used internally; public diagnostics remain line/column based.

## Rule API Migration

Add a context-based interface while preserving the current trait behavior.

The trait keeps `id`, `official_meta`, and legacy evaluate-by-file/content
behavior, then adds context/config evaluation for migrated rules.

Default `evaluate_context` can delegate to `evaluate` during migration. New or migrated rules use context directly.

## First Migration Set

Prioritize rule families that gain correctness or speed from shared structure:

- heading: `MD001`, `MD003`, `MD018`, `MD019`, `MD020`, `MD021`
- code fence: `MD031`, `MD040`, `MD046`, `MD048`
- reference: `MD051`, `MD052`, `MD053`, `MD054`
- table: `MD055`, `MD056`, `MD058`, `MD060`

This change does not require all listed rules to be fully migrated before landing. It requires the migration path, context contract, and at least one representative rule family to be complete.

## Fix Safety

Safe fix generation MUST use original source ranges from `DocumentContext`.

Rules MUST NOT generate fixes from AST-normalized text when doing so would lose source formatting, comments, spacing, or table alignment.

## Performance

The context builder must avoid duplicate full-document scans when practical. Expensive substructures should be lazy when only a subset of rules needs them.

Benchmarks must include:

- API lint large document
- API fix large document
- CLI many small files
- rule family microbenchmarks for migrated families

Performance acceptance:

- No unexplained regression is allowed in `make perf-check`
- If a migrated rule becomes slower, the change must document the reason and the correctness tradeoff
- A context index may be reverted to lazy construction if eager construction hurts many-small-file CLI behavior

## Compatibility

Public `lint(content, &LintOptions)` and `fix(content, &LintOptions)` remain stable.

CLI output order and JSON shape remain stable.

## Implementation Order

1. Add data model and source range helpers without changing rule behavior
2. Wire context-based API behind legacy-compatible adapters
3. Migrate one heading family and one reference/table/fence family
4. Add parity and benchmark gates for migrated families
5. Only then consider wider rule migration in follow-up changes

## Risks

- Context construction can become slower than current rule-local scans if too many structures are eager.
- AST parser behavior may not match markdownlint edge cases exactly.
- Migrating too many rules at once can obscure behavior regressions.

Mitigation:

- Start with a minimal eager context and lazy expensive fields.
- Keep upstream golden fixtures as the behavior gate.
- Migrate rule families incrementally.
