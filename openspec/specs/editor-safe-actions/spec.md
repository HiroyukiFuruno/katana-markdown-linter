# editor-safe-actions Specification

## Purpose

エディタ診断とクイック修正（quick fix）が、設定反映済みの lint 結果と安全な修正だけに基づくための契約を定義する。

## Requirements

### Requirement: editor diagnostics SHALL match configuration-aware lint results

editor diagnostics は、同じ file と config で実行した `kml check` の lint 判断と一致すること（SHALL）。

#### Scenario: configured rule option changes a diagnostic

- **WHEN** config が rule option を変更する。
- **AND** Markdown document がその option に依存する違反候補を含む。
- **THEN** editor diagnostic は config 反映後の結果だけを表示する。
- **AND** default config 前提の stale diagnostic を表示しない。

### Requirement: editor quick fixes SHALL be safe and configuration-aware

editor quick fixes は、config 反映済み diagnostics に紐づく safe fix のみを提示すること（SHALL）。

#### Scenario: diagnostic has a safe fix

- **WHEN** config 反映済み diagnostic が safe fix を持つ。
- **THEN** editor はその diagnostic に対応する quick fix を提示する。
- **AND** quick fix 適用後の document は同じ config で再診断される。

#### Scenario: rule is disabled by config

- **WHEN** config が rule を無効化している。
- **THEN** editor はその rule の diagnostic を表示しない。
- **AND** editor はその rule の quick fix も提示しない。

### Requirement: editor formatting SHALL not mask configuration failure

editor formatting は、config error がある workspace で誤って成功状態に見せてはならない（MUST NOT）。

#### Scenario: formatting requested with invalid config

- **WHEN** user が invalid config の workspace で document formatting を実行する。
- **THEN** editor は config error を確認できる状態にする。
- **AND** formatting が実行される場合でも、その evidence は config error と分離して記録される。
