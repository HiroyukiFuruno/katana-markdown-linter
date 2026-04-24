## Purpose

上流 markdownlint の rule catalog 変更を監視し、local 側との差分を運用可能な形で可視化する。

## Requirements

### Requirement: system SHALL detect upstream markdownlint rule catalog changes

システムは、upstream markdownlint の rule catalog に対する変更を検出しなければならない（SHALL）。

#### Scenario: MD0XX が増える

- **WHEN** upstream に新しい rule が追加される
- **THEN** system は local catalog との差分を報告する
- **THEN** system は未取り込みの rule を明示する

#### Scenario: MD0XX が削除される

- **WHEN** upstream から rule が削除または廃止される
- **THEN** system は local catalog との差分を報告する
- **THEN** system は deprecated / removed として可視化する

### Requirement: system SHALL classify rule lifecycle states

システムは、rule を active / deprecated / removed / missing の lifecycle state で分類しなければならない（SHALL）。

#### Scenario: lifecycle state を表示する

- **WHEN** system が rule diff を生成する
- **THEN** system は各 rule の状態を分類する
- **THEN** system は state ごとに異なる表示を行う
