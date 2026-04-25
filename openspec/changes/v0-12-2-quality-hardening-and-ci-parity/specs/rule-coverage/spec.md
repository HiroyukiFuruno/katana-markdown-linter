## ADDED Requirements

### Requirement: system SHALL maintain file-level false-positive regression coverage

システムは、単体 rule test だけでなく、実文書に近い複数行 Markdown fixture で誤検知回帰を防がなければならない（SHALL）。

#### Scenario: mixed Markdown fixture を検証する

- **WHEN** developer が regression tests を実行する
- **THEN** system は HTML block、inline code、fenced code block、math、table、nested list、reference link を含む fixture を検証する
- **THEN** system は code block 内の Markdown 風テキストを通常本文として診断しない
- **THEN** system は HTML attribute 内 URL や inline code 内 URL を bare URL として診断しない

### Requirement: context-sensitive rules SHALL be tracked by risk category

システムは、context-sensitive rule の残課題を risk category 付きで追跡しなければならない（SHALL）。

#### Scenario: rule risk を棚卸しする

- **WHEN** developer が `v0.12.2` の issue inventory を更新する
- **THEN** system は rule ごとに `bug`、`test-gap`、`design-debt`、`ci-gap` のいずれかで分類する
- **THEN** system は `v0.12.2` 対応対象と後続 change 対象を区別する
- **THEN** system は user-reported false positive が chat history のみに残らないよう tasks に記録する

### Requirement: false-positive fixes SHALL include rule-local and document-level tests

誤検知修正は、rule-local test と document-level test の両方で固定されなければならない（SHALL）。

#### Scenario: 誤検知を修正する

- **WHEN** developer が false positive を修正する
- **THEN** system は該当 rule の最小再現 test を追加する
- **THEN** system は mixed document fixture に再発条件を追加する
- **THEN** system は unrelated diagnostics が増えていないことを確認する
