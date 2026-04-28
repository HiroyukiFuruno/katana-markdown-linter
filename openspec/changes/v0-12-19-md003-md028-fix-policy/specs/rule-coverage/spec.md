## ADDED Requirements

### Requirement: MD003 safe fix SHALL preserve heading meaning and source ranges

`MD003` の safe-fix は、見出しレベルと本文を保持し、対象行だけを置き換えなければならない（SHALL）。

#### Scenario: setext H1 を ATX H1 に変換する

- **WHEN** system が `Heading\n=======\n` を `MD003` の fix 対象として処理する
- **THEN** system は `# Heading\n` へ変換する
- **THEN** system は見出し本文を変更しない
- **THEN** system は underline 行を残さない

#### Scenario: setext H2 を ATX H2 に変換する

- **WHEN** system が `Heading\n-------\n` を `MD003` の fix 対象として処理する
- **THEN** system は `## Heading\n` へ変換する
- **THEN** system は見出しレベルを保持する

#### Scenario: front matter と horizontal rule を修正しない

- **WHEN** system が front matter delimiter または独立した horizontal rule を含む Markdown を処理する
- **THEN** system はそれらを `MD003` safe-fix 対象にしない
- **THEN** system は既存の誤検知回避を維持する

### Requirement: MD028 fix policy SHALL be decided before enabling automatic fixes

`MD028` は、文意を変えない安全条件が定義できる場合にだけ自動修正を提供しなければならない（SHALL）。

#### Scenario: safe subset を実装できる場合

- **WHEN** developer が `MD028` の safe subset を定義する
- **THEN** system はその subset を fixture と unit test で固定する
- **THEN** system は GFM Alert 間の空行を修正対象にしない
- **THEN** system は `MD028` を safe-fix allowlist に追加する

#### Scenario: safe subset を定義できない場合

- **WHEN** developer が `MD028` の自動修正に人間の意図が必要だと判断する
- **THEN** system は `MD028` を `Diagnostic only` のまま維持する
- **THEN** system は `v0.12.21` の by-design 宣言対象に `MD028` を追加する
- **THEN** system は README と fixture matrix に理由を反映する
