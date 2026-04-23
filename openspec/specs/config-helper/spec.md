## Purpose

Markdownlint configuration helpers provide creation and validation support for `.markdownlint.json`.

## Requirements

### Requirement: system SHALL create a valid .markdownlint.json helper file

システムは、`.markdownlint.json` の初期値を生成する helper を提供しなければならない（SHALL）。

#### Scenario: config helper を作る

- **WHEN** developer が初期 config の作成を要求する
- **THEN** system は valid な `.markdownlint.json` を生成する
- **THEN** system は公式 default に沿った構造を出力する

### Requirement: system SHALL validate markdownlint configuration values

システムは、`.markdownlint.json` の設定値が公式 contract に合っているか検証しなければならない（SHALL）。

#### Scenario: config を検証する

- **WHEN** system が既存の `.markdownlint.json` を読み込む
- **THEN** system は rule ID、boolean、rule property の妥当性を確認する
- **THEN** system は不正な設定をエラーとして返す
