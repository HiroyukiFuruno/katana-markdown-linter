## ADDED Requirements

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
- **THEN** system は未知のfixture driftを品質ゲートで検出できる
