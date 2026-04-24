## Purpose

Repository-local dogfooding defines how `kml` is used against this repository's Markdown documents.

## Requirements

### Requirement: repository SHALL run kml against its Markdown documents

repository は、`kml` を用いて自身の Markdown 文書を check する dogfood workflow を提供しなければならない（SHALL）。

#### Scenario: dogfood check を実行する

- **WHEN** developer が dogfood check command を実行する
- **THEN** system は repository 内の対象 Markdown を再帰的に lint する
- **THEN** system は違反があれば非 zero exit code を返す
- **THEN** system は実行対象と除外対象を developer が確認できる形で定義する

### Requirement: dogfood workflow SHALL separate check from fix

dogfood workflow は、check-only と自動 fix を明確に分離しなければならない（SHALL）。

#### Scenario: safe fix を明示実行する

- **WHEN** developer が dogfood fix command を実行する
- **THEN** system は fixable rule のみを適用する
- **THEN** system は fix 後に残存違反を確認できる状態にする
- **THEN** system は check-only command で文書を書き換えない

### Requirement: dogfood workflow SHALL preserve archived OpenSpec documents by default

dogfood workflow は、履歴として保存された archived OpenSpec documents を既定の自動修正対象から除外しなければならない（SHALL）。

#### Scenario: archive を既定除外する

- **WHEN** developer が既定の dogfood command を実行する
- **THEN** system は `openspec/changes/archive/**` を自動修正対象から除外する
- **THEN** system は archive を確認したい場合の明示 command または option を提供する

### Requirement: dogfood workflow SHALL record actionable CLI usability findings

dogfood workflow は、実利用で見つかった CLI の課題を再現可能な findings として記録しなければならない（SHALL）。

#### Scenario: usability finding を記録する

- **WHEN** dogfood 実行で CLI の不足や違和感が見つかる
- **THEN** system は command、期待結果、実際の結果、対応判断を記録する
- **THEN** system は未対応 findings を次の OpenSpec change に移せる粒度にする
