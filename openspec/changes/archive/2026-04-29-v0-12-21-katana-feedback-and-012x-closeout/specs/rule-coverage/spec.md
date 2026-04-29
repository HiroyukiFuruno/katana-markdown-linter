## ADDED Requirements

### Requirement: remaining diagnostic-only rules SHALL have by-design reasons

0.12.x closeout 時点で safe-fix を提供しない active rule は、理由を by-design として説明しなければならない（SHALL）。

#### Scenario: by-design rule map を更新する

- **WHEN** developer が README の rule map を更新する
- **THEN** system は残り diagnostic-only rule に短い理由を付ける
- **THEN** system は `Needs triage` のまま残さない
- **THEN** system は未着手なのか、意図的に実装しないのかを区別できる表示にする

### Requirement: by-design declaration SHALL include current remaining rule set

by-design 宣言は、`v0.12.20` 完了時点の残り rule set を対象にしなければならない（SHALL）。

#### Scenario: remaining rule set を確定する

- **WHEN** `v0.12.21` に着手する
- **THEN** system は `MD001`、`MD013`、`MD024`、`MD033`、`MD041`、`MD042`、`MD043`、`MD045`、`MD059` を候補として確認する
- **THEN** `MD028` が `v0.12.19` で safe-fix 実装されなかった場合、候補に追加する
- **THEN** system は README、fixture matrix、rule-fix feasibility document の表示を揃える

### Requirement: by-design rules SHALL not be default-safe fixed

by-design と宣言した rule は、default safe-fix で変更してはならない（SHALL NOT）。

#### Scenario: fix を実行する

- **WHEN** system が `kml fix` または `check --fix` を実行する
- **THEN** system は by-design rule の診断を残す
- **THEN** system は by-design rule の修正候補を default safe-fix に混ぜない
- **THEN** system は unsafe fix と by-design 診断を混同しない
