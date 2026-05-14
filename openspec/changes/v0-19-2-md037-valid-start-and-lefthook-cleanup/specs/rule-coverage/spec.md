## ADDED Requirements

### Requirement: MD037 emphasis detection SHALL follow CommonMark left-flanking rules

`MD037` (`no-space-in-emphasis`) は、強調記号（`*`, `_`, `**`, `__` 等）の開始位置判定に CommonMark left-flanking delimiter run の必要十分条件を適用しなければならない（SHALL）。具体的には、直前文字が行頭 / 空白 / `([{"'` の場合に加えて、直前が句読点 `.!?,;:` であっても直後が non-空白なら強調記号の開始候補として扱う。直後が空白または行末の強調記号は、たとえ直前が句読点であっても開始候補から除外する。

#### Scenario: 句読点直後のスペース付き強調を検出する

- **WHEN** `Hello.* spaced *.` のように句読点直後に開始するスペース付き強調を含む文書を `kml check` で検査する
- **THEN** system は MD037 違反を 1 件報告する
- **WHEN** 同じ文書を `kml check --fix` で修正する
- **THEN** system は内側の余分な空白を削除した `Hello.*spaced*.` 形へ変換する

#### Scenario: 同一行内の独立した強調範囲を連結しない

- **WHEN** `**Note:** Neovim support is provided as a **docs-only sample**.` のように同一行内に独立した強調範囲を 2 つ含む文書を `kml check --fix` で修正する
- **THEN** system は MD037 違反を報告しない
- **THEN** 文書の Markdown 構造は変更されない

#### Scenario: 直後が空白の強調記号は開始候補から除外する

- **WHEN** ある `*` の直前が `.` で直後が空白であるとき
- **THEN** system はその `*` を強調範囲の開始候補として扱わない
- **THEN** 後続の独立した強調記号と誤って対応付けない
