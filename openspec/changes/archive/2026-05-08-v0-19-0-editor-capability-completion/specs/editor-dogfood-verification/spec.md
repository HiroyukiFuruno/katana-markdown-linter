## ADDED Requirements

### Requirement: final editor dogfood SHALL cover diagnostics, formatting, fixes, and config changes

`v0.19.0` 系の完了前に、editor integration の最終 dogfood は diagnostics、formatting、safe fixes、config changes をまとめて検証すること（SHALL）。

#### Scenario: final dogfood runs

- **WHEN** `v0.19.0` 系の editor 機能実装が完了候補になる。
- **THEN** system は VS Code / Zed の editor-facing behavior を検証する。
- **AND** diagnostics、formatting、safe fixes、config changes の各結果を evidence に残す。

### Requirement: final editor dogfood SHALL classify every finding

最終 editor dogfood は、finding を未分類のまま残してはならない（MUST NOT）。

#### Scenario: dogfood finds an issue

- **WHEN** final editor dogfood が誤診断、誤修正、設定反映漏れ、UI/operation gap、または検証不能状態を検出する。
- **THEN** system は finding を分類し、release-blocking か follow-up かを記録する。
- **AND** release-blocking finding が残る場合は `v0.20.0` 公開へ進まない。

### Requirement: final editor dogfood SHALL preserve reproducible evidence

最終 editor dogfood は、後続の公開判断に使える再現可能な evidence を残すこと（SHALL）。

#### Scenario: dogfood passes

- **WHEN** final editor dogfood が release-blocking finding なしで完了する。
- **THEN** system は実行 command、対象 corpus、config、変更差分、diagnostic summary、fix summary を保存する。
- **AND** `v0.20.0` 公開 change はその evidence を前提条件として参照できる。
