## ADDED Requirements

### Requirement: dogfood workflow SHALL exercise core command separation before release

dogfood workflow は、release 前に `check`、`fix`、`fmt` の責務分離を確認しなければならない（SHALL）。

#### Scenario: core command dogfood を実行する

- **WHEN** developer が release 前 dogfood を実行する
- **THEN** system は check-only path が対象 Markdown を書き換えないことを確認する
- **THEN** system は fix path が default-safe fix だけを適用することを確認する
- **THEN** system は fmt path が formatter policy の範囲だけを変更することを確認する
- **THEN** system は見つかった CLI usability findings を後続対応できる粒度で記録する
