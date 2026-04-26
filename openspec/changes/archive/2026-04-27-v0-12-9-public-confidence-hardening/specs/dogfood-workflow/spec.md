## ADDED Requirements

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
