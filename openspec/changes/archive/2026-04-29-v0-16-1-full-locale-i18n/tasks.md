# Tasks

## Definition Of Ready

- [x] v0.16.0 PR is merged into `main`.
- [x] Implementation branch is based on latest `origin/main`.
- [x] KatanA 本体の supported locale は `en`, `ja`, `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`, `de`, `es`, `it` と確認済み。
- [x] OpenSpec proposal, design, delta specs, and tasks exist for this change.
- [x] User feedback: English の単純コピーは不可。各言語で自然な翻訳にする。

## 0. Planning

- [x] 0.1 Create v0.16.1 OpenSpec change.
- [x] 0.2 Capture description 系 API と rule document Markdown API の両方を scope に入れる。
- [x] 0.3 Capture English copy rejection as a verification requirement.

## 1. Locale Contract

- [x] 1.1 Extend `Locale` and resolver to all KatanA-supported locale codes.
- [x] 1.2 Keep CLI explicit locale strict and library / MCP locale input lenient.
- [x] 1.3 Split i18n modules so v0.16.1 additions do not enlarge one oversized source file.
- [x] 1.4 Add locale resolver tests for region, underscore, and charset variants.

## 2. Description Catalog

- [x] 2.1 Add natural rule description translations for `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`, `de`, `es`, and `it`.
- [x] 2.2 Keep English canonical descriptions available in localized catalog output.
- [x] 2.3 Add tests proving supported-locale descriptions are not English copies.
- [x] 2.4 Update CLI `kml rule` tests for at least one new non-Japanese locale.

## 3. Rule Document Markdown

- [x] 3.1 Add localized rule document Markdown for all supported non-English locales.
- [x] 3.2 Update documentation lookup so supported locale documents are resolved consistently.
- [x] 3.3 Add tests for `get_rule_documentation` and `rule_doc_get` with a new non-Japanese locale.
- [x] 3.4 Add coverage checks for missing localized rule documents.
- [x] 3.5 Add checks that localized prose is not an English copy.

## 4. MCP And Remote Surfaces

- [x] 4.1 Ensure local MCP `rule_list`, `rule_get`, and `rule_doc_get` return resolved locale and localized content.
- [x] 4.2 Ensure remote MCP `rule_list`, `rule_get`, and `rule_doc_get` match the local behavior.
- [x] 4.3 Add MCP contract tests for localized descriptions and localized Markdown content.

## 5. Documentation And Release Prep

- [x] 5.1 Update README locale support text in English.
- [x] 5.2 Update MCP documentation locale support text in English.
- [x] 5.3 Bump crate and package metadata to 0.16.1.
- [x] 5.4 Add CHANGELOG entry for v0.16.1.
- [x] 5.5 Archive this OpenSpec change after implementation is complete.

## 6. Verification

- [x] 6.1 Run `make fmt-check`.
- [x] 6.2 Run `make lint`.
- [x] 6.3 Run `make ast-lint`.
- [x] 6.4 Run `cargo test --workspace --locked`.
- [x] 6.5 Run `make dogfood`.
- [x] 6.6 Run `git diff --check`.
- [x] 6.7 Run `make release-check VERSION=v0.16.1`.

## Definition Of Done

- [x] Supported locale set matches KatanA 本体.
- [x] Description 系 API returns natural localized text for every supported locale.
- [x] Rule document Markdown API returns natural localized Markdown for every supported locale.
- [x] English copy and missing translation gates fail when coverage regresses.
- [x] Public docs describe the 10-locale support in English.
- [x] Release PR for v0.16.1 is ready.
