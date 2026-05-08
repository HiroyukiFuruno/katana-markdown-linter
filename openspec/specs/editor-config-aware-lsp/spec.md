# editor-config-aware-lsp Specification

## Purpose

エディタ向け言語サーバー（LSP）が、CLI と同じ markdownlint 設定探索・読み込み結果を文書ごとに反映する契約を定義する。

## Requirements

### Requirement: LSP SHALL resolve project markdownlint configuration per document

LSP は、Markdown document ごとに CLI と同じ `.markdownlint.json` / `.markdownlint.jsonc` 探索・読み込み結果を使うこと（SHALL）。

#### Scenario: document has workspace config

- **WHEN** editor が workspace 内の Markdown document を開く。
- **AND** その workspace に `.markdownlint.json` または `.markdownlint.jsonc` が存在する。
- **THEN** LSP はその config を解決して diagnostics に反映する。
- **AND** config で無効化された rule の diagnostics を返さない。

#### Scenario: document has no config

- **WHEN** editor が config のない workspace 内の Markdown document を開く。
- **THEN** LSP は CLI と同じ default config を使う。
- **AND** config がないことを error として扱わない。

### Requirement: LSP SHALL surface configuration errors instead of silently falling back

LSP は、不正な config を検出した場合に default config へ黙って fallback してはならない（MUST NOT）。

#### Scenario: config file is invalid

- **WHEN** `.markdownlint.json` または `.markdownlint.jsonc` が不正な内容を含む。
- **THEN** LSP は config error を editor から確認できる形で返す。
- **AND** その document の通常 diagnostics / fix は誤った config 前提で成功扱いにしない。

### Requirement: LSP SHALL recompute diagnostics when configuration changes

LSP は、config file 変更後に開いている Markdown document の diagnostics を再計算すること（SHALL）。

#### Scenario: config disables an active rule

- **WHEN** editor が `.markdownlint.json` を変更して、現在表示中の rule を無効化する。
- **THEN** LSP は対象 document の diagnostics を再発行する。
- **AND** 無効化された rule の diagnostic は消える。
