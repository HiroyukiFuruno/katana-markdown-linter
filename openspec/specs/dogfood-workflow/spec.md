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

### Requirement: dogfood workflow SHALL exercise core command separation before release

dogfood workflow は、release 前に `check`、`fix`、`fmt` の責務分離を確認しなければならない（SHALL）。

#### Scenario: core command dogfood を実行する

- **WHEN** developer が release 前 dogfood を実行する
- **THEN** system は check-only path が対象 Markdown を書き換えないことを確認する
- **THEN** system は fix path が default-safe fix だけを適用することを確認する
- **THEN** system は fmt path が formatter policy の範囲だけを変更することを確認する
- **THEN** system は見つかった CLI usability findings を後続対応できる粒度で記録する

### Requirement: public confidence dogfood SHALL support representative external Markdown corpus

公開前 confidence dogfood は、repository 内 fixture だけでなく、実運用に近い外部 Markdown corpus を扱えなければならない（SHALL）。

#### Scenario: KatanA Markdown corpus を確認する

- **WHEN** developer が `KATANA_CHECKOUT` を指定して external dogfood を実行する
- **THEN** system は KatanA `docs/**/*.md` と `assets/**/*.md` を候補 corpus として扱う
- **THEN** system は binary asset を lint 対象に含めない
- **THEN** system は `check` 実行で source file を書き換えない
- **THEN** system は finding を source path、rule、分類とともに記録する

### Requirement: public confidence dogfood SHALL avoid private corpus coupling in required CI

公開前 confidence dogfood は、required CI を private checkout に依存させてはならない（SHALL NOT）。

#### Scenario: required CI を実行する

- **WHEN** GitHub Actions required CI が実行される
- **THEN** system は sibling KatanA checkout を必須にしない
- **THEN** system は public repository に置ける curated fixture または synthetic equivalent で再現性を確保する
- **THEN** system は external dogfood が未実行の場合、その理由を release evidence に記録する
