## ADDED Requirements

### Requirement: v0.12.21 SHALL run a KatanA feedback sweep after v0.12.20

`v0.12.21` は、`v0.12.20` の性能確認完了後に KatanA 側ドキュメントを external corpus として確認しなければならない（SHALL）。

#### Scenario: KatanA ドキュメントを読む

- **WHEN** developer が `KATANA_CHECKOUT` を指定して external dogfood または public confidence を実行する
- **THEN** system は KatanA 側の複数 Markdown 文書を対象にする
- **THEN** system は check-only 実行で KatanA 側ファイルを書き換えない
- **THEN** system は finding を source path、rule、分類、対応方針とともに記録する

### Requirement: KatanA feedback issues SHALL be classified before v0.12.x closeout

KatanA feedback で見つかった issue は、0.12.x closeout 前に分類されなければならない（SHALL）。

#### Scenario: feedback issue を分類する

- **WHEN** KatanA feedback sweep で finding が見つかる
- **THEN** system は finding を `false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression`、`docs-only` のいずれかに分類する
- **THEN** system は release-blocking issue と non-blocking follow-up を分ける
- **THEN** release-blocking issue は `v0.12.21` の bugfix 対象として tasks に残す

#### Scenario: issue がない場合

- **WHEN** KatanA feedback sweep で release-blocking issue が見つからない
- **THEN** system は `v0.12.21` を by-design 宣言と closeout 証跡の patch として扱う
- **THEN** system は `v0.13.0` へ進める状態を roadmap に記録する
