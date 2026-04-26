## Purpose

Rule coverage exposes markdownlint-compatible rule metadata, checking, and fix behavior.

## Requirements

### Requirement: system SHALL expose every official markdownlint rule in the rule catalog

システムは、公式 markdownlint documentation にある active rule を rule catalog として公開し、deprecated / removed rule は lifecycle state として区別しなければならない（SHALL）。

#### Scenario: rule catalog を参照する

- **WHEN** developer が利用可能な rule 一覧を取得する
- **THEN** system は official docs にある active rule ID と説明を返す
- **THEN** system は各 rule の metadata を区別できる
- **THEN** system は deprecated / removed rule を active rule と混同しない

### Requirement: system SHALL check documents according to official markdownlint rule behavior

システムは、公式 markdownlint の rule behavior に準じて Markdown 文書を check しなければならない（SHALL）。

#### Scenario: 文書を check する

- **WHEN** system が Markdown 文書を解析する
- **THEN** system は rule ごとの違反を返す
- **THEN** system は rule の設定に従って enable / disable を判定する
- **THEN** system は fixture matrix の pass / fail 期待値と一致する判定を返す
- **THEN** system は境界値・イレギュラー条件に対しても仕様準拠の判定を返す

### Requirement: system SHALL support automatic fix only for rules that define fix behavior

システムは、upstream implementation または official documentation から安全な fix behavior を確認できる rule に対してのみ自動修正を適用しなければならない（SHALL）。

#### Scenario: fix を実行する

- **WHEN** system が document fix を要求される
- **THEN** system は fixable rule の修正だけを適用する
- **THEN** system は fix 非対応 rule を無理に変更しない
- **THEN** system は fix 非対応 rule の理由を metadata として保持する

### Requirement: system SHALL generate a rule fixture parity matrix from official markdownlint documentation

システムは、公式 markdownlint rule document から rule ごとの fixture parity matrix を生成しなければならない（SHALL）。

#### Scenario: fixture matrix を生成する

- **WHEN** developer が upstream rule document を入力する
- **THEN** system は rule id、aliases、tags、parameters、fixability を抽出する
- **THEN** system は check / fix / config / edge の fixture coverage を rule ごとに出力する
- **THEN** system は自動抽出できない記載を `manual_required` として可視化する

### Requirement: system SHALL report stale or incomplete rule fixtures

システムは、公式 markdownlint document と local fixture matrix の乖離を報告しなければならない（SHALL）。

#### Scenario: fixture drift を検出する

- **WHEN** upstream rule document が変更される
- **THEN** system は local fixture matrix と比較する
- **THEN** system は missing fixture、stale fixture、manual_required の数を報告する
- **THEN** system は未知の fixture drift を品質ゲートで検出できる

### Requirement: system SHALL publish a rule coverage dashboard

システムは、rule ごとの coverage と compatibility 状態を一覧できる dashboard を公開しなければならない（SHALL）。

#### Scenario: dashboard を生成する

- **WHEN** developer が coverage dashboard generation を実行する
- **THEN** system は rule ID ごとの check、fix、config、edge、golden comparison、known delta の状態を出力する
- **THEN** system は missing coverage と failing golden comparison を区別して表示する
- **THEN** system は dashboard を Markdown または JSON の再利用可能な形式で生成する

### Requirement: rule coverage gate SHALL fail on unknown golden deltas

rule coverage gate は、許可されていない upstream golden delta を失敗として扱わなければならない（SHALL）。

#### Scenario: unknown delta を検出する

- **WHEN** golden comparison が known delta にない差分を検出する
- **THEN** system は gate を failure にする
- **THEN** system は該当 rule、fixture、expected、actual を report する

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

- **WHEN** developer が issue inventory を更新する
- **THEN** system は rule ごとに `bug`、`test-gap`、`design-debt`、`ci-gap` のいずれかで分類する
- **THEN** system は今回対応対象と後続 change 対象を区別する
- **THEN** system は user-reported false positive が chat history のみに残らないよう tasks に記録する

### Requirement: false-positive fixes SHALL include rule-local and document-level tests

誤検知修正は、rule-local test と document-level test の両方で固定されなければならない（SHALL）。

#### Scenario: 誤検知を修正する

- **WHEN** developer が false positive を修正する
- **THEN** system は該当 rule の最小再現 test を追加する
- **THEN** system は mixed document fixture に再発条件を追加する
- **THEN** system は unrelated diagnostics が増えていないことを確認する

### Requirement: precision fixes SHALL cover false positives and false negatives

精度修正（precision fix）は誤検知と検出漏れの両方を扱わなければならない（SHALL）。

#### Scenario: precision issue を修正する

- **WHEN** developer が rule behavior を変更する
- **THEN** system は issue を `bug`、`test-gap`、`design-debt`、`perf-risk` のいずれかで分類する
- **THEN** system は最小再現のルール単位テスト（rule-local test）を追加する
- **THEN** system は必要に応じて文書単位fixture（document-level fixture）に複数構文が混ざった再発条件を追加する

### Requirement: context-sensitive rule tests SHALL include structural exclusion cases

context-sensitive rule test は、通常本文ではない構造の除外ケースを含まなければならない（SHALL）。

#### Scenario: structural exclusion を検証する

- **WHEN** rule が URL、link、table、heading、list、command prompt、emphasis に見える行を評価する
- **THEN** system は inline code、fenced code block、HTML block、table block、reference definition 内の期待値を検証する
- **THEN** system は unrelated diagnostics が増えていないことを確認する
- **THEN** system は fixture 追加だけで済むものと実装修正（implementation fix）が必要なものを区別する

### Requirement: linter hardening SHALL prioritize existing rule correctness before distribution expansion

システムは、配布経路の拡大より前に既存 rule の正しさを優先して固めなければならない（SHALL）。

#### Scenario: core rule issue を棚卸しする

- **WHEN** developer が `v0.12.4` の実装に着手する
- **THEN** system は既存 rule の誤検知、検出漏れ、fix 事故、test gap を分類する
- **THEN** system は patch release で扱う対象と後続 change に送る対象を区別する
- **THEN** system は fixture 追加だけで済むものと実装修正が必要なものを区別する

### Requirement: safe fix behavior SHALL converge and remain idempotent

safe fix behavior は収束し、再実行時に不要な差分を増やしてはならない（SHALL）。

#### Scenario: safe fix を再実行する

- **WHEN** system が `check --fix` または `fix` を同じ content に複数回実行する
- **THEN** system は初回で適用できる default-safe fix を適用する
- **THEN** system は再実行時に同じ修正を繰り返し適用しない
- **THEN** system は残存違反を diagnostics として報告し続ける
- **THEN** system は unsafe fix を default-safe fix と混同しない

### Requirement: check, fix, and fmt corpus SHALL include mixed Markdown structures

`check`、`fix`、`fmt` の corpus は、複数構文が混ざる Markdown 文書を含まなければならない（SHALL）。

#### Scenario: mixed corpus を検証する

- **WHEN** developer が corpus tests を実行する
- **THEN** system は fenced code block、inline code、HTML block、table、nested list、reference link を含む fixture を検証する
- **THEN** system は check diagnostics、fixed content、formatted content を別々に検証する
- **THEN** system は unrelated diagnostics や不要な formatting diff が増えていないことを確認する

### Requirement: link-family precision SHALL use shared parser evidence

link 系 rule の精度改善は、共有 parser または `DocumentContext` の token index に基づかなければならない（SHALL）。

#### Scenario: link 系 rule を評価する

- **WHEN** `MD034`、`MD051`、`MD052`、`MD054`、`MD059` が document を評価する
- **THEN** system は inline code、image、reference definition、autolink を通常本文と混同しない
- **THEN** system は rule ごとの独立した簡易 parser を増やさない
- **THEN** system は誤検知と検出漏れを rule-local test と document-level fixture の両方で固定する

### Requirement: parser migration SHALL document non-migrated rules

parser migration は、移譲しない rule の理由を明文化しなければならない（SHALL）。

#### Scenario: rule を移譲しない

- **WHEN** rule が `v0.12.5` で shared parser へ移譲されない
- **THEN** system は理由を `already-context-safe`、`line-local-by-spec`、`future-ast-required`、`blocked` のいずれかで記録する
- **THEN** system は future work が必要なものを後続 version の tasks に残す
