## ADDED Requirements

### Requirement: MD037 emphasis detection SHALL follow CommonMark left-flanking rules

`MD037` (`no-space-in-emphasis`) は、強調記号（`*`, `_`, `**`, `__` 等）の開始位置判定で、句読点直後のスペース付き強調と、通常強調の閉じ記号を区別しなければならない（SHALL）。具体的には、直前文字が行頭 / 空白 / `([{"'` の場合に加えて、直前が句読点 `.!?,;:` の場合もスペース付き強調の開始候補として扱う。一方で、同種・同長の直前 marker が通常強調の開始として成立しており、現在 marker がその閉じ記号に見える場合は、後続 marker と誤って対応付けてはならない。

#### Scenario: 句読点直後のスペース付き強調を検出する

- **WHEN** `Hello.* spaced *.` のように句読点直後に開始するスペース付き強調を含む文書を `kml check` で検査する
- **THEN** system は MD037 違反を 1 件報告する
- **WHEN** 同じ文書を `kml check --fix` で修正する
- **THEN** system は内側の余分な空白を削除した `Hello.*spaced*.` 形へ変換する

#### Scenario: 同一行内の独立した強調範囲を連結しない

- **WHEN** `**Note:** Neovim support is provided as a **docs-only sample**.` のように同一行内に独立した強調範囲を 2 つ含む文書を `kml check --fix` で修正する
- **THEN** system は MD037 違反を報告しない
- **THEN** 文書の Markdown 構造は変更されない

#### Scenario: 通常強調の閉じ記号を開始候補として再利用しない

- **WHEN** `*Note:* Neovim support is provided as * docs-only sample *.` のように通常強調の直後に同種・同長のスペース付き強調を含む文書を `kml check --fix` で修正する
- **THEN** system は `*Note:*` の閉じ `*` を開始候補として再利用しない
- **THEN** system は後続の `* docs-only sample *` だけを `*docs-only sample*` へ修正する
